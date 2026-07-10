<#
.SYNOPSIS
Resolves, downloads, and stages the .winmd files that `cargo run -p bindgen` expects
in bindgen/winmd/, for a given Windows App SDK release.

.DESCRIPTION
Version resolution goes through the Microsoft.WindowsAppSDK *meta-package*: its nuspec
pins the exact versions of the split packages (Foundation, InteractiveExperiences,
WinUI) that make up a WinAppSDK release. The split packages do NOT version in lockstep
with the SDK — e.g. WinAppSDK 2.1 ships InteractiveExperiences 2.0.13 — so resolving
each split package independently by version number does not work.

The WebView2 package version is the minimum pinned by the resolved
Microsoft.WindowsAppSDK.WinUI nuspec.

The set of .winmd files to stage is read from the `--in` lines of bindgen/etc/winui3.txt,
so this script never needs updating when the expected file list changes.

Unless -SkipRuntime is given, the script also downloads Microsoft.WindowsAppSDK.Runtime
and prints the bootstrap constants (runtime version u64, package family name) from its
WindowsAppSDK-VersionInfo.json — needed when adding a new WindowsAppSDKVersion variant
to winui3/src/bootstrap/mod.rs.

Downloads are cached in $env:TEMP\winui3-winmd-cache; delete that directory to force
a re-download.

.PARAMETER Version
WinAppSDK version to stage. Either major.minor (e.g. "2.1") — resolves to the earliest
stable (non-preview, non-experimental) patch of that minor — or an exact meta-package
version (e.g. "2.1.3").

.PARAMETER SkipRuntime
Skip downloading Microsoft.WindowsAppSDK.Runtime and printing the bootstrap constants.
Use for plain re-generation when no new WindowsAppSDKVersion variant is being added.

.EXAMPLE
./bindgen/fetch-winmd.ps1 -Version 2.1
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [ValidatePattern('^\d+\.\d+(\.\d+)?$')]
    [string]$Version,

    [switch]$SkipRuntime
)

$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'  # Invoke-WebRequest is much faster without the progress bar

$flatContainer = 'https://api.nuget.org/v3-flatcontainer'
$cacheDir = Join-Path $env:TEMP 'winui3-winmd-cache'
$winmdDir = Join-Path $PSScriptRoot 'winmd'
$etcFile = Join-Path $PSScriptRoot 'etc\winui3.txt'

New-Item -ItemType Directory -Force $cacheDir | Out-Null
New-Item -ItemType Directory -Force $winmdDir | Out-Null

function Get-NuspecXml([string]$PackageId, [string]$PackageVersion) {
    $id = $PackageId.ToLowerInvariant()
    $path = Join-Path $cacheDir "$id.$PackageVersion.nuspec"
    if (-not (Test-Path $path)) {
        Invoke-WebRequest "$flatContainer/$id/$PackageVersion/$id.nuspec" -OutFile $path
    }
    $xml = New-Object System.Xml.XmlDocument
    $xml.Load($path)  # XmlDocument.Load handles the UTF-8 BOM these nuspecs carry
    return $xml
}

# Nuspec dependency versions may be exact ranges like "[2.1.3]"; a bare version means
# "this or newer", so in both cases the lower bound is the version that release shipped with.
function Get-DependencyVersion([xml]$Nuspec, [string]$DependencyId) {
    $dep = $Nuspec.GetElementsByTagName('dependency') | Where-Object { $_.id -eq $DependencyId }
    if (-not $dep) {
        throw "package '$($Nuspec.package.metadata.id)' $($Nuspec.package.metadata.version) has no dependency on '$DependencyId'"
    }
    return @($dep)[0].version.Trim('[]()').Split(',')[0].Trim()
}

function Get-Nupkg([string]$PackageId, [string]$PackageVersion) {
    $id = $PackageId.ToLowerInvariant()
    $path = Join-Path $cacheDir "$id.$PackageVersion.nupkg"
    if (-not (Test-Path $path)) {
        Write-Host "  downloading $PackageId $PackageVersion..."
        Invoke-WebRequest "$flatContainer/$id/$PackageVersion/$id.$PackageVersion.nupkg" -OutFile $path
    } else {
        Write-Host "  using cached $PackageId $PackageVersion"
    }
    return $path
}

# --- Resolve the meta-package version -----------------------------------------------

Write-Host "Resolving Microsoft.WindowsAppSDK $Version..."
$index = Invoke-RestMethod "$flatContainer/microsoft.windowsappsdk/index.json"
$stable = $index.versions | Where-Object { $_ -notmatch '-' }

$parts = $Version.Split('.')
if ($parts.Count -eq 3) {
    $metaVersion = $stable | Where-Object { $_ -eq $Version }
    if (-not $metaVersion) {
        throw "Microsoft.WindowsAppSDK $Version is not a stable release on NuGet. Stable versions: $($stable -join ', ')"
    }
} else {
    # Earliest stable patch of the requested major.minor. Later patches of the same minor
    # are servicing releases; the earliest one is the API surface the minor introduced.
    $candidates = $stable | Where-Object {
        $p = $_.Split('.'); $p[0] -eq $parts[0] -and $p[1] -eq $parts[1]
    } | Sort-Object { [version]$_ }
    if (-not $candidates) {
        throw "no stable Microsoft.WindowsAppSDK $Version.x release on NuGet. Stable versions: $($stable -join ', ')"
    }
    $metaVersion = @($candidates)[0]
}
Write-Host "  meta-package: Microsoft.WindowsAppSDK $metaVersion"

$metaNuspec = Get-NuspecXml 'Microsoft.WindowsAppSDK' $metaVersion
$foundationVersion = Get-DependencyVersion $metaNuspec 'Microsoft.WindowsAppSDK.Foundation'
$ixpVersion = Get-DependencyVersion $metaNuspec 'Microsoft.WindowsAppSDK.InteractiveExperiences'
$winuiVersion = Get-DependencyVersion $metaNuspec 'Microsoft.WindowsAppSDK.WinUI'
$runtimeVersion = Get-DependencyVersion $metaNuspec 'Microsoft.WindowsAppSDK.Runtime'

$winuiNuspec = Get-NuspecXml 'Microsoft.WindowsAppSDK.WinUI' $winuiVersion
$webview2Version = Get-DependencyVersion $winuiNuspec 'Microsoft.Web.WebView2'

Write-Host @"
  Microsoft.WindowsAppSDK.Foundation             $foundationVersion
  Microsoft.WindowsAppSDK.InteractiveExperiences $ixpVersion
  Microsoft.WindowsAppSDK.WinUI                  $winuiVersion
  Microsoft.WindowsAppSDK.Runtime                $runtimeVersion
  Microsoft.Web.WebView2                         $webview2Version
"@

# --- Stage the winmds ----------------------------------------------------------------

# bindgen/etc/winui3.txt's `--in` lines are the source of truth for which files to stage.
$expected = Get-Content $etcFile |
    Where-Object { $_ -match '^\s*bindgen/winmd/(\S+\.winmd)\s*$' } |
    ForEach-Object { $Matches[1] }
if (-not $expected) {
    throw "no 'bindgen/winmd/*.winmd' entries found in $etcFile"
}

$packages = @(
    @{ Id = 'Microsoft.WindowsAppSDK.Foundation';             Version = $foundationVersion },
    @{ Id = 'Microsoft.WindowsAppSDK.InteractiveExperiences'; Version = $ixpVersion },
    @{ Id = 'Microsoft.WindowsAppSDK.WinUI';                  Version = $winuiVersion },
    @{ Id = 'Microsoft.Web.WebView2';                         Version = $webview2Version }
)

Write-Host 'Downloading packages...'
$nupkgs = $packages | ForEach-Object { Get-Nupkg $_.Id $_.Version }

# Clear stale winmds first: a leftover file from a different SDK version would silently
# win over nothing (bindgen reads whatever is at the staged path).
Remove-Item (Join-Path $winmdDir '*.winmd') -Force -ErrorAction SilentlyContinue

Write-Host 'Staging winmds...'
Add-Type -AssemblyName System.IO.Compression.FileSystem
$staged = @{}
foreach ($nupkg in $nupkgs) {
    $zip = [System.IO.Compression.ZipFile]::OpenRead($nupkg)
    try {
        $found = $zip.Entries | Where-Object { $expected -contains $_.Name }
        foreach ($group in ($found | Group-Object Name)) {
            $leaf = $group.Name
            if ($staged.ContainsKey($leaf)) {
                Write-Warning "$leaf also found in $(Split-Path $nupkg -Leaf); keeping the copy from $($staged[$leaf])"
                continue
            }
            # Some packages carry one copy per target platform version (e.g.
            # InteractiveExperiences has both metadata/10.0.17763.0/ and
            # metadata/10.0.18362.0/). Take the highest-versioned one: higher TFM
            # winmds are supersets, and 10.0.18362.0 is the documented source
            # (see "Source packages" in bindgen/ARCHITECTURE.md).
            $entry = $group.Group | Sort-Object {
                $dir = Split-Path ([System.IO.Path]::GetDirectoryName($_.FullName)) -Leaf
                if ($dir -match '^\d+(\.\d+)+$') { [version]$dir } else { [version]'0.0' }
            } -Descending | Select-Object -First 1
            [System.IO.Compression.ZipFileExtensions]::ExtractToFile(
                $entry, (Join-Path $winmdDir $leaf), $true)
            $staged[$leaf] = "$(Split-Path $nupkg -Leaf) ($($entry.FullName))"
            Write-Host "  $leaf  <-  $($staged[$leaf])"
        }
    } finally {
        $zip.Dispose()
    }
}

$missing = $expected | Where-Object { -not $staged.ContainsKey($_) }
if ($missing) {
    throw "not found in any package: $($missing -join ', ')"
}
Write-Host "Staged $($staged.Count) winmd file(s) into $winmdDir"

# --- Bootstrap constants from the Runtime package ------------------------------------

if (-not $SkipRuntime) {
    Write-Host 'Fetching runtime version info...'
    $runtimeNupkg = Get-Nupkg 'Microsoft.WindowsAppSDK.Runtime' $runtimeVersion
    $zip = [System.IO.Compression.ZipFile]::OpenRead($runtimeNupkg)
    try {
        $entry = $zip.Entries | Where-Object { $_.Name -eq 'WindowsAppSDK-VersionInfo.json' } | Select-Object -First 1
        if (-not $entry) {
            throw "WindowsAppSDK-VersionInfo.json not found in Microsoft.WindowsAppSDK.Runtime $runtimeVersion"
        }
        $reader = New-Object System.IO.StreamReader($entry.Open())
        try {
            $versionInfo = $reader.ReadToEnd() | ConvertFrom-Json
        } finally {
            $reader.Dispose()
        }
    } finally {
        $zip.Dispose()
    }

    $rt = $versionInfo.Runtime
    Write-Host @"

Bootstrap constants (winui3/src/bootstrap/mod.rs — see winui3/ARCHITECTURE.md
'Adding a new WinAppSDK version'; skip if the version enum already has this variant):

  runtime version:     $($rt.Version.HexUInt16)_u64  ($($rt.Version.String))
  package family name: $($rt.Packages.Framework.PackageFamilyName)
"@
}

Write-Host "`nNext: run ``cargo run -p bindgen`` from the workspace root."
