//! Runs one end-to-end composition test as a WinUI application on a dedicated
//! thread.
//!
//! The UI thread parks until process exit after `Application::Start` returns.
//! This avoids a known STA teardown crash when the thread exits after `Frame`
//! navigation.

use std::{sync::mpsc, thread, time::Duration};

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
    thread::spawn(move || {
        let started = start(compose);
        let observed = observe();
        let _ = sender.send(started.map(|()| observed));
        loop {
            thread::park();
        }
    });
    receiver
        .recv_timeout(Duration::from_secs(60))
        .expect("the application did not exit within 60 seconds")
}

fn start(compose: impl Fn() -> Result<Application> + Send + 'static) -> Result<()> {
    winui3::init_apartment(winui3::ApartmentType::SingleThreaded)?;
    let _dependency = PackageDependency::initialize()?;
    Application::Start(&ApplicationInitializationCallback::new(move |_| {
        compose()?;
        Ok(())
    }))
}
