use windows::{
    core::{Interface, Ref, Result},
    Foundation::TypedEventHandler,
    Win32::System::WinRT::Composition::ICompositorDesktopInterop,
    UI::Composition::{Compositor, Desktop::DesktopWindowTarget},
};

use winui3::{
    bootstrap::PackageDependency,
    Microsoft::UI::{
        Colors, Composition::SystemBackdrops::MicaController,
        Dispatching::DispatcherQueueController, Windowing::AppWindow,
    },
    Reference,
};

fn main() -> Result<()> {
    winui3::init_apartment(winui3::ApartmentType::SingleThreaded)?;

    let _dependency = PackageDependency::initialize()?;

    let dispatcher_queue_controller = DispatcherQueueController::CreateOnCurrentThread()?;
    dispatcher_queue_controller
        .DispatcherQueue()?
        .EnsureSystemDispatcherQueue()?;

    let compositor = Compositor::new()?;

    let app_window = AppWindow::Create()?;
    app_window.AssociateWithDispatcherQueue(&dispatcher_queue_controller.DispatcherQueue()?)?;
    app_window.Closing(&TypedEventHandler::new(|sender: Ref<AppWindow>, _args| {
        sender.unwrap().DispatcherQueue()?.EnqueueEventLoopExit()
    }))?;

    let title_bar = app_window.TitleBar()?;
    title_bar.SetExtendsContentIntoTitleBar(true)?;
    title_bar.SetButtonBackgroundColor(Reference::new(Colors::Transparent()?))?;
    title_bar.SetButtonInactiveBackgroundColor(Reference::new(Colors::Transparent()?))?;

    app_window.Show()?;

    let target = create_window_target(&app_window, &compositor)?;
    target.SetRoot(&compositor.CreateContainerVisual()?)?;

    let mica_controller = MicaController::new()?;
    mica_controller.SetTargetWithWindowId(app_window.Id()?, &target)?;

    app_window.DispatcherQueue()?.RunEventLoop()
}

fn create_window_target(
    window: &AppWindow,
    compositor: &Compositor,
) -> Result<DesktopWindowTarget> {
    let desktop_window_target: ICompositorDesktopInterop = compositor.cast()?;
    let hwnd = unsafe { winui3::interop::GetWindowFromWindowId(window.Id()?)? };
    unsafe { desktop_window_target.CreateDesktopWindowTarget(hwnd, false) }
}
