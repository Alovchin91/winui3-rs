#![allow(non_snake_case, non_camel_case_types)]

#[cfg(feature = "UI")]
struct InteropImpl {
    pfnGetWindowIdFromWindow: Option<
        unsafe extern "system" fn(
            hwnd: windows::Win32::Foundation::HWND,
            windowId: *mut crate::Microsoft::UI::WindowId,
        ) -> windows_core::HRESULT,
    >,

    pfnGetWindowFromWindowId: Option<
        unsafe extern "system" fn(
            windowId: crate::Microsoft::UI::WindowId,
            hwnd: *mut windows::Win32::Foundation::HWND,
        ) -> windows_core::HRESULT,
    >,
}

#[cfg(feature = "UI")]
unsafe fn EnsureInteropImplLoaded() -> windows_core::Result<&'static InteropImpl> {
    static S_IMPL: std::sync::OnceLock<InteropImpl> = std::sync::OnceLock::new();

    if let Some(value) = S_IMPL.get() {
        return Ok(value);
    }

    let s_impl = unsafe {
        let module = windows::Win32::System::LibraryLoader::GetModuleHandleW(windows_core::w!(
            "Microsoft.Internal.FrameworkUdk.dll"
        ))
        .or_else(|_| {
            windows::Win32::System::LibraryLoader::LoadLibraryW(windows_core::w!(
                "Microsoft.Internal.FrameworkUdk.dll"
            ))
        })?;
        if module.is_invalid() {
            return Err(windows::Win32::Foundation::ERROR_INVALID_HANDLE.into());
        }
        let pfnGetWindowIdFromWindow = windows::Win32::System::LibraryLoader::GetProcAddress(
            module,
            windows_core::s!("Windowing_GetWindowIdFromWindow"),
        );
        let pfnGetWindowFromWindowId = windows::Win32::System::LibraryLoader::GetProcAddress(
            module,
            windows_core::s!("Windowing_GetWindowFromWindowId"),
        );
        #[allow(clippy::missing_transmute_annotations)]
        InteropImpl {
            pfnGetWindowFromWindowId: core::mem::transmute(pfnGetWindowFromWindowId),
            pfnGetWindowIdFromWindow: core::mem::transmute(pfnGetWindowIdFromWindow),
        }
    };

    Ok(S_IMPL.get_or_init(|| s_impl))
}

#[cfg(feature = "UI")]
pub unsafe fn GetWindowIdFromWindow(
    hwnd: windows::Win32::Foundation::HWND,
) -> windows_core::Result<crate::Microsoft::UI::WindowId> {
    let s_impl = EnsureInteropImplLoaded()?;
    let pfnGetWindowIdFromWindow = s_impl
        .pfnGetWindowIdFromWindow
        .ok_or_else(|| windows_core::Error::from(windows::Win32::Foundation::E_POINTER))?;

    let mut window_id = core::mem::zeroed();
    unsafe { pfnGetWindowIdFromWindow(hwnd, &mut window_id) }.map(|| window_id)
}

#[cfg(feature = "UI")]
pub unsafe fn GetWindowFromWindowId(
    windowId: crate::Microsoft::UI::WindowId,
) -> windows_core::Result<windows::Win32::Foundation::HWND> {
    let s_impl = EnsureInteropImplLoaded()?;
    let pfnGetWindowFromWindowId = s_impl
        .pfnGetWindowFromWindowId
        .ok_or_else(|| windows_core::Error::from(windows::Win32::Foundation::E_POINTER))?;

    let mut hwnd = core::mem::zeroed();
    unsafe { pfnGetWindowFromWindowId(windowId, &mut hwnd) }.map(|| hwnd)
}

#[cfg(feature = "UI_Xaml")]
#[windows_core::interface("eecdbf0e-bae9-4cb6-a68e-9598e1cb57bb")]
pub unsafe trait IWindowNative: windows_core::IUnknown {
    pub fn WindowHandle(
        &self,
        hwnd: *mut windows::Win32::Foundation::HWND,
    ) -> windows_core::HRESULT;
}
