pub mod bootstrap;

pub mod interop;

#[rustfmt::skip]
pub mod Microsoft;

#[rustfmt::skip]
pub mod Windows;

#[cfg(feature = "Foundation")]
mod reference;

#[cfg(feature = "XamlApp")]
mod xaml_app;

#[cfg(feature = "XamlApp_Navigation")]
mod xaml_page;

#[cfg(feature = "XamlApp_Navigation")]
mod xaml_types;

#[cfg(feature = "Foundation")]
pub use reference::Reference;

#[cfg(feature = "XamlApp")]
pub use xaml_app::{XamlApp, XamlAppOverrides};

#[cfg(feature = "XamlApp_Navigation")]
pub use xaml_page::{XamlPage, XamlPageOverrides};

#[cfg(feature = "XamlApp_Navigation")]
pub use xaml_types::XamlCustomType;

pub enum ApartmentType {
    MultiThreaded,
    SingleThreaded,
}

#[inline]
pub fn init_apartment(apartment_type: ApartmentType) -> windows_core::Result<()> {
    let roinit = match apartment_type {
        ApartmentType::MultiThreaded => windows::Win32::System::WinRT::RO_INIT_MULTITHREADED,
        ApartmentType::SingleThreaded => windows::Win32::System::WinRT::RO_INIT_SINGLETHREADED,
    };
    unsafe { windows::Win32::System::WinRT::RoInitialize(roinit) }
}

pub trait Activatable {
    fn activate() -> windows_core::Result<windows_core::IInspectable>;
}

#[cfg(feature = "UI_Xaml")]
pub fn xaml_typename<T: AsRef<str>>(type_name: T) -> Windows::UI::Xaml::Interop::TypeName {
    Windows::UI::Xaml::Interop::TypeName {
        Name: windows_core::HSTRING::from(type_name.as_ref()),
        Kind: Windows::UI::Xaml::Interop::TypeKind::Custom,
    }
}
