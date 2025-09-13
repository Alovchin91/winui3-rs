use std::{fmt, os::windows::ffi::OsStringExt, sync::OnceLock};
use windows::Win32::{
    Foundation::{ERROR_INSUFFICIENT_BUFFER, ERROR_SUCCESS},
    Storage::Packaging::Appx::{
        AddPackageDependency, AddPackageDependencyOptions_None,
        CreatePackageDependencyOptions_None, GetPackagePathByFullName,
        PackageDependencyLifetimeKind_Process, PackageDependencyProcessorArchitectures_None,
        RemovePackageDependency, TryCreatePackageDependency, PACKAGEDEPENDENCY_CONTEXT,
        PACKAGE_VERSION, PACKAGE_VERSION_0,
    },
};
use windows_core::{h, Result, HSTRING, PWSTR};

const WINDOWSAPPSDK_RUNTIME_VERSION_UINT64_V1_5: u64 = 0x1389003A01C00000_u64;
const WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME_V1_5: &HSTRING =
    h!("Microsoft.WindowsAppRuntime.1.5_8wekyb3d8bbwe");

const WINDOWSAPPSDK_RUNTIME_VERSION_UINT64_V1_6: u64 = 0x177000F200650000_u64;
const WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME_V1_6: &HSTRING =
    h!("Microsoft.WindowsAppRuntime.1.6_8wekyb3d8bbwe");

const WINDOWSAPPSDK_RUNTIME_VERSION_UINT64_V1_7: u64 = 0x1B5801B3009A0000_u64;
const WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME_V1_7: &HSTRING =
    h!("Microsoft.WindowsAppRuntime.1.7_8wekyb3d8bbwe");

const WINDOWSAPPSDK_RUNTIME_VERSION_UINT64_V1_8: u64 = 0x1F40026801300000_u64;
const WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME_V1_8: &HSTRING =
    h!("Microsoft.WindowsAppRuntime.1.8_8wekyb3d8bbwe");

pub enum WindowsAppSDKVersion {
    V1_5,
    V1_6,
    V1_7,
    V1_8,
}

impl WindowsAppSDKVersion {
    const fn get_runtime_version(&self) -> u64 {
        match self {
            WindowsAppSDKVersion::V1_5 => WINDOWSAPPSDK_RUNTIME_VERSION_UINT64_V1_5,
            WindowsAppSDKVersion::V1_6 => WINDOWSAPPSDK_RUNTIME_VERSION_UINT64_V1_6,
            WindowsAppSDKVersion::V1_7 => WINDOWSAPPSDK_RUNTIME_VERSION_UINT64_V1_7,
            WindowsAppSDKVersion::V1_8 => WINDOWSAPPSDK_RUNTIME_VERSION_UINT64_V1_8,
        }
    }

    const fn get_package_family_name(&self) -> &'static HSTRING {
        match self {
            WindowsAppSDKVersion::V1_5 => {
                WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME_V1_5
            }
            WindowsAppSDKVersion::V1_6 => {
                WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME_V1_6
            }
            WindowsAppSDKVersion::V1_7 => {
                WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME_V1_7
            }
            WindowsAppSDKVersion::V1_8 => {
                WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME_V1_8
            }
        }
    }
}

#[derive(Debug)]
struct PackageDependencyID(PWSTR);

unsafe impl Sync for PackageDependencyID {}
unsafe impl Send for PackageDependencyID {}

pub struct PackageDependency {
    ctx: PACKAGEDEPENDENCY_CONTEXT,
    package_full_name: HSTRING,
}

impl PackageDependency {
    pub fn initialize() -> Result<Self> {
        Self::initialize_version(WindowsAppSDKVersion::V1_8)
    }

    pub fn initialize_version(version: WindowsAppSDKVersion) -> Result<Self> {
        static RUNTIME_PACKAGE_FRAMEWORK_DEPENDENCY_ID: OnceLock<PackageDependencyID> =
            OnceLock::new();

        let dependency_id = match RUNTIME_PACKAGE_FRAMEWORK_DEPENDENCY_ID.get() {
            Some(dependency_id) => dependency_id,
            None => {
                let min_version = PACKAGE_VERSION {
                    Anonymous: PACKAGE_VERSION_0 {
                        Version: version.get_runtime_version(),
                    },
                };
                let dependency_id = unsafe {
                    TryCreatePackageDependency(
                        windows::Win32::Security::PSID::default(),
                        version.get_package_family_name(),
                        min_version,
                        PackageDependencyProcessorArchitectures_None,
                        PackageDependencyLifetimeKind_Process,
                        None,
                        CreatePackageDependencyOptions_None,
                    )
                }?;
                RUNTIME_PACKAGE_FRAMEWORK_DEPENDENCY_ID
                    .get_or_init(|| PackageDependencyID(dependency_id))
            }
        };

        let mut ctx = PACKAGEDEPENDENCY_CONTEXT::default();
        let mut package_full_name = PWSTR::null();

        unsafe {
            AddPackageDependency(
                dependency_id.0,
                0,
                AddPackageDependencyOptions_None,
                &mut ctx,
                Some(&mut package_full_name),
            )
        }?;

        Ok(Self {
            ctx,
            package_full_name: unsafe { package_full_name.to_hstring() },
        })
    }

    pub fn get_package_full_name(&self) -> &HSTRING {
        &self.package_full_name
    }

    pub fn get_package_path(&self) -> Result<std::path::PathBuf> {
        let mut path_length = 0_u32;
        match unsafe {
            GetPackagePathByFullName(&self.package_full_name, &raw mut path_length, None)
        } {
            ERROR_INSUFFICIENT_BUFFER => (),
            ERROR_SUCCESS => (),
            err => return Err(err.into()),
        }
        let mut path_vec = vec![0_u16; path_length as usize];
        unsafe {
            GetPackagePathByFullName(
                &self.package_full_name,
                &raw mut path_length,
                Some(PWSTR::from_raw(path_vec.as_mut_ptr())),
            )
        }
        .ok()?;
        // According to the documentation, pathLength includes the null-terminator.
        let path = std::ffi::OsString::from_wide(&path_vec[..path_length as usize - 1]);
        Ok(path.into())
    }

    fn uninitialize(&self) -> Result<()> {
        unsafe { RemovePackageDependency(self.ctx) }
    }
}

impl fmt::Debug for PackageDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PackageDependency")
            .field("package_full_name", &self.package_full_name)
            .finish_non_exhaustive()
    }
}

impl Drop for PackageDependency {
    fn drop(&mut self) {
        self.uninitialize()
            .expect("failed to remove package dependency")
    }
}
