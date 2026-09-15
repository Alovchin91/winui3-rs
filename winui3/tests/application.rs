//! Composes an `Application` with the real WinUI runtime and checks that the
//! runtime calls the Rust override with the composed singleton.
//!
//! Needs the Windows App SDK runtime and an interactive session, so it is
//! ignored by default:
//!
//! ```text
//! cargo test -p winui3 --no-default-features --features XamlApp --test application -- --ignored
//! ```

mod common;

use std::cell::RefCell;

use windows_core::{IUnknown, Interface, Result};
use winui3::{
    Microsoft::UI::Xaml::{Application, LaunchActivatedEventArgs},
    XamlApp, XamlAppOverrides,
};

/// What `OnLaunched` observed. Only plain data: WinUI objects must not outlive
/// `Start`, since the runtime is torn down by the time it returns.
#[derive(Default)]
struct Observed {
    /// Whether the `base` handed to the override was `Application::Current()`.
    base_is_current: Option<bool>,
    error: Option<String>,
}

thread_local! {
    static OBSERVED: RefCell<Observed> = RefCell::new(Observed::default());
}

struct App;

impl XamlAppOverrides for App {
    fn OnLaunched(&self, base: &Application, _: Option<&LaunchActivatedEventArgs>) -> Result<()> {
        let base_is_current = (|| -> Result<bool> {
            let base: IUnknown = base.cast()?;
            let current: IUnknown = Application::Current()?.cast()?;
            Ok(base == current)
        })();
        OBSERVED.with(|observed| {
            let mut observed = observed.borrow_mut();
            match base_is_current {
                Ok(value) => observed.base_is_current = Some(value),
                Err(error) => observed.error = Some(error.to_string()),
            }
        });
        // Ending the application here is what lets `Start` return to the test.
        base.Exit()
    }

    #[cfg(feature = "XamlApp_Navigation")]
    fn TryResolveXamlType(
        &self,
        _: &windows_core::HSTRING,
    ) -> Result<winui3::Microsoft::UI::Xaml::Markup::IXamlType> {
        Err(windows_core::Error::empty())
    }
}

#[test]
#[ignore = "needs the Windows App SDK runtime and an interactive session"]
fn launch_calls_on_launched_with_the_current_application() -> Result<()> {
    let observed = common::run_app(
        || XamlApp::compose(App),
        || OBSERVED.with(|observed| std::mem::take(&mut *observed.borrow_mut())),
    )?;
    assert_eq!(observed.error, None);
    assert_eq!(
        observed.base_is_current,
        Some(true),
        "OnLaunched ran with the singleton"
    );
    Ok(())
}
