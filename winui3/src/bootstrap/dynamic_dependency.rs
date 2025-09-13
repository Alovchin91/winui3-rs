#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use windows::Win32::Storage::Packaging::Appx::PACKAGE_VERSION;

/// MSIX Dynamic Dependency HRESULT: Windows App Runtime is not in the package graph.
pub const MDD_E_WINDOWSAPPRUNTIME_NOT_IN_PACKAGE_GRAPH: windows_core::HRESULT = windows_core::HRESULT(0x80040001_u32 as _);

/// MSIX Dynamic Dependency HRESULT: Data Store not found (Windows App Runtime's Main package not registered?)
pub const MDD_E_WINDOWSAPPRUNTIME_DATASTORE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80040002_u32 as _);

/// MSIX Dynamic Dependency: Bootstrap initialization request is incompatible with current Bootstrap initialization state.
pub const MDD_E_BOOTSTRAP_INITIALIZE_INCOMPATIBLE: windows_core::HRESULT = windows_core::HRESULT(0x80040014_u32 as _);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MddCreatePackageDependencyOptions(pub i32);
impl MddCreatePackageDependencyOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MddCreatePackageDependencyOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MddCreatePackageDependencyOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MddCreatePackageDependencyOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MddCreatePackageDependencyOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MddCreatePackageDependencyOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
/// Disable dependency resolution when pinning a package dependency.
pub const MddCreatePackageDependencyOptions_DoNotVerifyDependencyResolution: MddCreatePackageDependencyOptions = MddCreatePackageDependencyOptions(1i32);
pub const MddCreatePackageDependencyOptions_None: MddCreatePackageDependencyOptions = MddCreatePackageDependencyOptions(0i32);
/// Define the package dependency for the system, accessible to all users
/// (default is the package dependency is defined for a specific user).
/// This option requires the caller has adminitrative privileges.
pub const MddCreatePackageDependencyOptions_ScopeIsSystem: MddCreatePackageDependencyOptions = MddCreatePackageDependencyOptions(2i32);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MddPackageDependencyLifetimeKind(pub i32);
/// The lifetime artifact is an absolute filename or path.
/// The package dependency is implicitly deleted when this is deleted.
pub const MddPackageDependencyLifetimeKind_FilePath: MddPackageDependencyLifetimeKind = MddPackageDependencyLifetimeKind(1i32);
/// The current process is the lifetime artifact. The package dependency
/// is implicitly deleted when the process terminates.
pub const MddPackageDependencyLifetimeKind_Process: MddPackageDependencyLifetimeKind = MddPackageDependencyLifetimeKind(0i32);
/// The lifetime artifact is a registry key in the format
/// 'root\\subkey' where root is one of the following: HKLM, HKCU, HKCR, HKU.
/// The package dependency is implicitly deleted when this is deleted.
pub const MddPackageDependencyLifetimeKind_RegistryKey: MddPackageDependencyLifetimeKind = MddPackageDependencyLifetimeKind(2i32);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MddAddPackageDependencyOptions(pub i32);
impl MddAddPackageDependencyOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MddAddPackageDependencyOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MddAddPackageDependencyOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MddAddPackageDependencyOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MddAddPackageDependencyOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MddAddPackageDependencyOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const MddAddPackageDependencyOptions_None: MddAddPackageDependencyOptions = MddAddPackageDependencyOptions(0i32);
pub const MddAddPackageDependencyOptions_PrependIfRankCollision: MddAddPackageDependencyOptions = MddAddPackageDependencyOptions(1i32);
pub const MDD_PACKAGE_DEPENDENCY_RANK_DEFAULT: i32 = 0;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MddPackageDependencyProcessorArchitectures(pub i32);
impl MddPackageDependencyProcessorArchitectures {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MddPackageDependencyProcessorArchitectures {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MddPackageDependencyProcessorArchitectures {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MddPackageDependencyProcessorArchitectures {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MddPackageDependencyProcessorArchitectures {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MddPackageDependencyProcessorArchitectures {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const MddPackageDependencyProcessorArchitectures_Arm: MddPackageDependencyProcessorArchitectures = MddPackageDependencyProcessorArchitectures(8i32);
pub const MddPackageDependencyProcessorArchitectures_Arm64: MddPackageDependencyProcessorArchitectures = MddPackageDependencyProcessorArchitectures(16i32);
pub const MddPackageDependencyProcessorArchitectures_Neutral: MddPackageDependencyProcessorArchitectures = MddPackageDependencyProcessorArchitectures(1i32);
pub const MddPackageDependencyProcessorArchitectures_None: MddPackageDependencyProcessorArchitectures = MddPackageDependencyProcessorArchitectures(0i32);
pub const MddPackageDependencyProcessorArchitectures_X64: MddPackageDependencyProcessorArchitectures = MddPackageDependencyProcessorArchitectures(4i32);
pub const MddPackageDependencyProcessorArchitectures_X86: MddPackageDependencyProcessorArchitectures = MddPackageDependencyProcessorArchitectures(2i32);
pub const MddPackageDependencyProcessorArchitectures_X86OnArm64: MddPackageDependencyProcessorArchitectures = MddPackageDependencyProcessorArchitectures(32i32);

#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MDD_PACKAGEDEPENDENCY_CONTEXT(pub *mut core::ffi::c_void);
impl MDD_PACKAGEDEPENDENCY_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for MDD_PACKAGEDEPENDENCY_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

/// Define a package dependency. The criteria for a PackageDependency
/// (package family name, minimum version, etc)
/// may match multiple packages, but ensures Deployment won't remove
/// a package if it's the only one satisfying the PackageDependency.
///
/// **note** A package matching a PackageDependency pin can still be removed
///          as long as there's another package that satisfies the PackageDependency.
///          For example, if Fwk-v1 is installed and a PackageDependency specifies
///          MinVersion=1 and then Fwk-v2 is installed, Deployment could remove
///          Fwk-v1 because Fwk-v2 will satisfy the PackageDependency. After Fwk-v1
///          is removed Deployment won't remove Fwk-v2 because it's the only package
///          satisfying the PackageDependency. Thus Fwk-v1 and Fwk-v2 (and any other
///          package matching the PackageDependency) are 'loosely pinned'. Deployment
///          guarantees it won't remove a package if it would make a PackageDependency
///          unsatisfied.
///
/// A PackageDependency specifies criteria (package family, minimum version, etc)
/// and not a specific package. Deployment reserves the right to use a different
/// package (e.g. higher version) to satisfy the PackageDependency if/when
/// one becomes available.
///
/// * `user` the user scope of the package dependency. If NULL the caller's
///   user context is used. MUST be NULL if `MddCreatePackageDependencyOptions::ScopeIsSystem`
///   is specified
/// * `lifetimeArtifact` MUST be NULL if `lifetimeKind=MddPackageDependencyLifetimeKind::Process`
/// * `packageDependencyId` allocated via HeapAlloc; use HeapFree to deallocate
///
/// **note** MddTryCreatePackageDependency() fails if the PackageDependency cannot be resolved to a specific
///          package. This package resolution check is skipped if
///          MddCreatePackageDependencyOptions::DoNotVerifyDependencyResolution is specified. This is useful
///          for installers running as user contexts other than the target user (e.g. installers
///          running as LocalSystem).
#[allow(non_snake_case)]
#[inline]
pub unsafe fn MddTryCreatePackageDependency<P1, P5>(user: windows::Win32::Security::PSID, packagefamilyname: P1, minversion: PACKAGE_VERSION, packagedependencyprocessorarchitectures: MddPackageDependencyProcessorArchitectures, lifetimekind: MddPackageDependencyLifetimeKind, lifetimeartifact: P5, options: MddCreatePackageDependencyOptions) -> windows_core::Result<windows_core::PWSTR>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("Microsoft.WindowsAppRuntime.dll" "system" fn MddTryCreatePackageDependency(user : windows::Win32::Security:: PSID, packagefamilyname : windows_core::PCWSTR, minversion : PACKAGE_VERSION, packagedependencyprocessorarchitectures : MddPackageDependencyProcessorArchitectures, lifetimekind : MddPackageDependencyLifetimeKind, lifetimeartifact : windows_core::PCWSTR, options : MddCreatePackageDependencyOptions, packagedependencyid : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MddTryCreatePackageDependency(user, packagefamilyname.param().abi(), minversion, packagedependencyprocessorarchitectures, lifetimekind, lifetimeartifact.param().abi(), options, &mut result__).map(|| result__)
    }
}

/// Undefine a package dependency. Removing a pin on a PackageDependency is typically done at uninstall-time.
/// This implicitly occurs if the package dependency's 'lifetime artifact' (specified via MddTryCreatePackageDependency)
/// is deleted. Packages that are not referenced by other packages and have no pins are elegible to be removed.
///
/// **warn** MddDeletePackageDependency() requires the caller have administrative privileges
///          if the package dependency was pinned with MddCreatePackageDependencyOptions::ScopeIsSystem.
#[allow(non_snake_case)]
#[inline]
pub unsafe fn MddDeletePackageDependency<P0>(packagedependencyid: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("Microsoft.WindowsAppRuntime.dll" "system" fn MddDeletePackageDependency(packagedependencyid : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { MddDeletePackageDependency(packagedependencyid.param().abi()).ok() }
}

/// Resolve a previously-pinned PackageDependency to a specific package and
/// add it to the invoking process' package graph. Once the dependency has
/// been added other code-loading methods (LoadLibrary, CoCreateInstance, etc)
/// can find the binaries in the resolved package.
///
/// Package resolution is specific to a user and can return different values
/// for different users on a system.
///
/// Each successful MddAddPackageDependency() adds the resolved packaged to the
/// calling process' package graph, even if already present. There is no
/// duplicate 'detection' or 'filtering' applied by the API (multiple
/// references from a package is not harmful). Once resolution is complete
/// the package dependency stays resolved for that user until the last reference across
/// all processes for that user is removed via MddRemovePackageDependency (or
/// process termination).
///
/// MddAddPackageDependency() adds the resolved package to the caller's package graph,
/// per the rank specified. A process' package graph is a list of packages sorted by
/// rank in ascending order (-infinity...0...+infinity). If package(s) are present in the
/// package graph with the same rank as the call to MddAddPackageDependency the resolved
/// package is (by default) added after others of the same rank. To add a package
/// before others o the same rank, specify MddAddPackageDependencyOptions::PrependIfRankCollision.
///
/// Every MddAddPackageDependency can be balanced by a MddRemovePackageDependency
/// to remove the entry from the package graph. If the process terminates all package
/// references are removed, but any pins stay behind.
///
/// MddAddPackageDependency adds the resolved package to the process' package
/// graph, per the rank and options parameters. The process' package
/// graph is used to search for DLLs (per Dynamic-Link Library Search Order),
/// WinRT objects and other resources; the caller can now load DLLs, activate
/// WinRT objects and use other resources from the framework package until
/// MddRemovePackageDependency is called. The packageDependencyId parameter
/// must match a package dependency defined for the calling user or the
/// system (i.e. pinned with MddCreatePackageDependencyOptions::ScopeIsSystem) else
/// an error is returned.
///
/// * `packageDependencyContext` valid until passed to MddRemovePackageDependency()
/// * `packageFullName` allocated via HeapAlloc; use HeapFree to deallocate
#[allow(non_snake_case)]
#[inline]
pub unsafe fn MddAddPackageDependency<P0>(packagedependencyid: P0, rank: i32, options: MddAddPackageDependencyOptions, packagedependencycontext: *mut MDD_PACKAGEDEPENDENCY_CONTEXT, packagefullname: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("Microsoft.WindowsAppRuntime.dll" "system" fn MddAddPackageDependency(packagedependencyid : windows_core::PCWSTR, rank : i32, options : MddAddPackageDependencyOptions, packagedependencycontext : *mut MDD_PACKAGEDEPENDENCY_CONTEXT, packagefullname : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { MddAddPackageDependency(packagedependencyid.param().abi(), rank, options, packagedependencycontext as _, packagefullname.unwrap_or(core::mem::zeroed()) as _).ok() }
}

/// Remove a resolved PackageDependency from the current process' package graph
/// (i.e. undo MddAddPackageDependency). Used at runtime (i.e. the moral equivalent
/// of Windows' RemoveDllDirectory()).
///
/// **note** This does not unload loaded resources (DLLs etc). After removing
///          a package dependency any files loaded from the package can continue
///          to be used; future file resolution will fail to see the removed
///          package dependency.
#[allow(non_snake_case)]
#[inline]
pub unsafe fn MddRemovePackageDependency(packagedependencycontext: MDD_PACKAGEDEPENDENCY_CONTEXT) -> windows_core::Result<()> {
    windows_link::link!("Microsoft.WindowsAppRuntime.dll" "system" fn MddRemovePackageDependency(packagedependencycontext : MDD_PACKAGEDEPENDENCY_CONTEXT) -> windows_core::HRESULT);
    unsafe { MddRemovePackageDependency(packagedependencycontext).ok() }
}
