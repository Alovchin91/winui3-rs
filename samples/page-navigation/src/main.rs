mod app;
mod sample_page;

use windows::{
    core::{h, Ref, Result},
    Foundation::TypedEventHandler,
    Win32::{
        Foundation::E_POINTER,
        UI::HiDpi::{SetThreadDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2},
    },
};

use winui3::{
    bootstrap::PackageDependency,
    Microsoft::{
        Windows::ApplicationModel::Resources::ResourceManager,
        UI::Xaml::{
            Application, ApplicationInitializationCallback, ResourceManagerRequestedEventArgs,
        },
    },
    XamlApp,
};

use app::App;

fn main() -> Result<()> {
    winui3::init_apartment(winui3::ApartmentType::SingleThreaded)?;

    unsafe { SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };

    let _dependency = PackageDependency::initialize()?;

    Application::Start(&ApplicationInitializationCallback::new(|_| {
        let app = XamlApp::compose(App)?;

        // Workaround for a bug in WinAppSDK v1.8, see https://github.com/microsoft/WindowsAppSDK/issues/5940
        app.ResourceManagerRequested(&TypedEventHandler::new(
            |_, args: Ref<ResourceManagerRequestedEventArgs>| {
                let args = args.as_ref().ok_or(E_POINTER)?;
                let resource_manager = ResourceManager::CreateInstance(h!("resources.pri"))?;
                args.SetCustomResourceManager(&resource_manager)?;
                Ok(())
            },
        ))?;

        Ok(())
    }))
}
