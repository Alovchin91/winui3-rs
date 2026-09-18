//! Runs one end-to-end composition test as a WinUI application on a dedicated
//! thread.

use std::{
    sync::mpsc::{self, RecvTimeoutError},
    thread,
    time::Duration,
};

use windows_core::Result;
use winui3::{
    bootstrap::PackageDependency,
    Microsoft::UI::Xaml::{Application, ApplicationInitializationCallback},
};

/// Runs `compose` as a WinUI application on a new single-threaded apartment,
/// waits for it to exit, then evaluates `observe` on that thread and returns
/// the result. `observe` must return plain data; WinUI objects must not
/// outlive `Start`.
pub fn run_app<T: Send + 'static>(
    compose: impl Fn() -> Result<Application> + Send + 'static,
    observe: impl FnOnce() -> T + Send + 'static,
) -> Result<T> {
    let (sender, receiver) = mpsc::channel();
    let ui_thread = thread::spawn(move || {
        let started = start(compose);
        let observed = observe();
        let _ = sender.send(());
        started.map(|()| observed)
    });
    if receiver.recv_timeout(Duration::from_secs(60)) == Err(RecvTimeoutError::Timeout) {
        panic!("the application did not exit within 60 seconds");
    }
    ui_thread.join().expect("the UI thread panicked")
}

fn start(compose: impl Fn() -> Result<Application> + Send + 'static) -> Result<()> {
    winui3::init_apartment(winui3::ApartmentType::SingleThreaded)?;
    let _dependency = PackageDependency::initialize()?;
    Application::Start(&ApplicationInitializationCallback::new(move |_| {
        compose()?;
        Ok(())
    }))
}
