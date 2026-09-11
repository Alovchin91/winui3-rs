<#
.SYNOPSIS
Checks every winui3 cargo feature in isolation (`cargo check -p winui3 --features <F>`).

.DESCRIPTION
`cargo check -p winui3 --all-features` cannot prove that each feature's dependency
list is complete: with all features enabled, a sibling feature may pull in the
`windows/*` feature that a broken feature forgot to declare, masking the gap.
Checking each feature alone is what proves every feature stands on its own.

The feature list is read from winui3/Cargo.toml's `[features]` table, so the script
automatically tracks regenerated feature tables.

The first sweep is slow: each distinct feature set builds its own variant of the
`windows` crate. Subsequent sweeps reuse the cached variants and are much faster.

For missing feature dependencies in generated code, fix the feature's entry in
`FEATURE_PATCHES` (bindgen/src/main.rs) and re-run `cargo run -p bindgen` — do not
hand-edit the generated feature table in winui3/Cargo.toml. For unresolved paths
in hand-written code, inspect the item's `#[cfg(feature = ...)]` gate instead.

.PARAMETER Feature
Optional subset of features to check. Default: every feature in the manifest.

.EXAMPLE
./bindgen/check-features.ps1
./bindgen/check-features.ps1 -Feature UI_Input_Interop, XamlApp
#>
[CmdletBinding()]
param([string[]]$Feature)

$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
$manifest = Join-Path $root 'winui3\Cargo.toml'

$names = @()
$inFeatures = $false
foreach ($line in Get-Content $manifest) {
    if ($line -match '^\[(.+)\]\s*$') { $inFeatures = ($Matches[1] -eq 'features'); continue }
    if ($inFeatures -and $line -match '^([A-Za-z0-9_]+)\s*=') { $names += $Matches[1] }
}
if (-not $names) { throw "no features found in $manifest" }

if ($Feature) {
    $unknown = $Feature | Where-Object { $names -notcontains $_ }
    if ($unknown) { throw "not features of winui3: $($unknown -join ', ')" }
    $names = $names | Where-Object { $Feature -contains $_ }
}

Push-Location $root
try {
    $failed = @()
    $i = 0
    foreach ($name in $names) {
        $i++
        $sw = [System.Diagnostics.Stopwatch]::StartNew()
        $out = & cargo check -p winui3 --quiet --features $name 2>&1
        $ok = ($LASTEXITCODE -eq 0)
        $sw.Stop()
        $status = if ($ok) { 'ok  ' } else { 'FAIL' }
        '{0,3}/{1} {2} {3,-32} {4,6:N0}s' -f $i, $names.Count, $status, $name, $sw.Elapsed.TotalSeconds
        if (-not $ok) {
            $failed += $name
            $out | Select-Object -Last 15 | ForEach-Object { "       $_" }
        }
    }
    ''
    if ($failed) {
        "FAILED ($($failed.Count)): $($failed -join ', ')"
        "For missing feature dependencies in generated code: update FEATURE_PATCHES in bindgen/src/main.rs, then re-run ``cargo run -p bindgen``."
        "For unresolved paths in hand-written code: inspect the item's #[cfg(feature = ...)] gate."
        exit 1
    }
    "All $($names.Count) features check individually."
} finally {
    Pop-Location
}
