//! Composes a `Page` with the real WinUI runtime and checks that `Frame`
//! navigation resolves the page through the app's metadata provider and calls
//! the Rust overrides with the composed page.
//!
//! Needs the Windows App SDK runtime and an interactive session, so it is
//! ignored by default:
//!
//! ```text
//! cargo test -p winui3 --no-default-features --features XamlApp_Navigation --test page -- --ignored
//! ```

mod common;

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use windows_core::{Error, IInspectable, IUnknown, Interface, Result, HSTRING};
use winui3::{
    xaml_typename, Activatable,
    Microsoft::UI::Xaml::{
        Application,
        Controls::{Frame, IPageOverrides, Page, XamlControlsResources},
        LaunchActivatedEventArgs,
        Markup::IXamlType,
        Navigation::{NavigatingCancelEventArgs, NavigationEventArgs},
    },
    XamlApp, XamlAppOverrides, XamlCustomType, XamlPage, XamlPageOverrides,
};

const PAGE_TYPE: &str = "TestPage";

/// Runtime activity on the UI thread, in order. Page identities are held
/// strongly while recording so two pages can never share an address; the log
/// is rendered to text and cleared before the application exits, since WinUI
/// objects must not outlive `Start`.
enum Event {
    /// The runtime activated a composed page through the resolved type.
    Activated(IUnknown),
    NavigatedTo(IUnknown),
    NavigatingFrom(IUnknown),
    NavigatedFrom(IUnknown),
    /// `Navigate2` returned this result and the frame then showed this content.
    Navigated(bool, IUnknown),
}

/// `Event`s rendered with pages numbered in activation order.
fn render(events: &[Event]) -> Vec<String> {
    let pages: Vec<&IUnknown> = events
        .iter()
        .filter_map(|event| match event {
            Event::Activated(page) => Some(page),
            _ => None,
        })
        .collect();
    let page = |id: &IUnknown| match pages.iter().position(|page| *page == id) {
        Some(index) => format!("page {}", index + 1),
        None => "unknown page".to_string(),
    };
    events
        .iter()
        .map(|event| match event {
            Event::Activated(id) => format!("Activated({})", page(id)),
            Event::NavigatedTo(id) => format!("NavigatedTo({})", page(id)),
            Event::NavigatingFrom(id) => format!("NavigatingFrom({})", page(id)),
            Event::NavigatedFrom(id) => format!("NavigatedFrom({})", page(id)),
            Event::Navigated(ok, id) => format!("Navigated({ok}, {})", page(id)),
        })
        .collect()
}

#[derive(Default)]
struct Observed {
    resolved: bool,
    rendered: Vec<String>,
    lifetime: Option<PageLifetime>,
    error: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct PageLifetime {
    alive_with_overrides: bool,
    drops_with_overrides: u32,
    alive_after_release: bool,
    drops_after_release: u32,
}

thread_local! {
    static EVENTS: RefCell<Vec<Event>> = const { RefCell::new(Vec::new()) };
    static OBSERVED: RefCell<Observed> = RefCell::new(Observed::default());
}

fn record(event: Event) {
    EVENTS.with(|events| events.borrow_mut().push(event));
}

fn identity<I: Interface>(value: &I) -> Result<IUnknown> {
    value.cast()
}

struct App;

impl XamlAppOverrides for App {
    fn OnLaunched(&self, base: &Application, _: Option<&LaunchActivatedEventArgs>) -> Result<()> {
        let checked = navigate_twice(base).and_then(|()| observe_page_lifetime());
        let rendered = EVENTS.with(|events| render(&std::mem::take(&mut *events.borrow_mut())));
        OBSERVED.with(|observed| {
            let mut observed = observed.borrow_mut();
            observed.rendered = rendered;
            match checked {
                Ok(lifetime) => observed.lifetime = Some(lifetime),
                Err(error) => observed.error = Some(error.to_string()),
            }
        });
        // Ending the application here is what lets `Start` return to the test.
        base.Exit()
    }

    fn TryResolveXamlType(&self, full_name: &HSTRING) -> Result<IXamlType> {
        // The runtime also probes this resolver for its own types (brushes from
        // the merged resources, the page's runtime class name); only our page
        // type is of interest here.
        if full_name == PAGE_TYPE {
            // Resolution must happen, but its frequency is a runtime cache detail.
            OBSERVED.with(|observed| observed.borrow_mut().resolved = true);
            XamlCustomType::<TestPage>::for_page(full_name)
        } else {
            Err(Error::empty())
        }
    }
}

/// Navigates a frame to the composed page twice, so the first instance sees
/// `OnNavigatingFrom` and `OnNavigatedFrom` and the second sees `OnNavigatedTo`.
fn navigate_twice(app: &Application) -> Result<()> {
    app.Resources()?
        .MergedDictionaries()?
        .Append(&XamlControlsResources::new()?)?;
    let frame = Frame::new()?;
    for _ in 0..2 {
        let navigated = frame.Navigate2(&xaml_typename(PAGE_TYPE))?;
        record(Event::Navigated(navigated, identity(&frame.Content()?)?));
    }
    Ok(())
}

/// Checks lifetime on a standalone native page before application shutdown,
/// without a frame or the event log retaining any of its interfaces.
fn observe_page_lifetime() -> Result<PageLifetime> {
    let drops = Rc::new(Cell::new(0));
    let page = XamlPage::compose(TestPage {
        drops: drops.clone(),
    })?;
    let overrides: IPageOverrides = page.cast()?;
    let weak = page.downgrade()?;
    drop(page);

    let alive_with_overrides = weak.upgrade().is_some();
    let drops_with_overrides = drops.get();
    drop(overrides);

    Ok(PageLifetime {
        alive_with_overrides,
        drops_with_overrides,
        alive_after_release: weak.upgrade().is_some(),
        drops_after_release: drops.get(),
    })
}

#[derive(Default)]
struct TestPage {
    drops: Rc<Cell<u32>>,
}

impl Drop for TestPage {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}

impl Activatable for TestPage {
    fn activate() -> Result<IInspectable> {
        let page = XamlPage::compose(Self::default())?;
        record(Event::Activated(identity(&page)?));
        Ok(page.into())
    }
}

impl XamlPageOverrides for TestPage {
    fn OnNavigatedFrom(&self, base: &Page, _: Option<&NavigationEventArgs>) -> Result<()> {
        record(Event::NavigatedFrom(identity(base)?));
        Ok(())
    }

    fn OnNavigatedTo(&self, base: &Page, _: Option<&NavigationEventArgs>) -> Result<()> {
        record(Event::NavigatedTo(identity(base)?));
        Ok(())
    }

    fn OnNavigatingFrom(&self, base: &Page, _: Option<&NavigatingCancelEventArgs>) -> Result<()> {
        record(Event::NavigatingFrom(identity(base)?));
        Ok(())
    }
}

#[test]
#[ignore = "needs the Windows App SDK runtime and an interactive session"]
fn navigation_calls_the_overrides_with_the_composed_page() -> Result<()> {
    let observed = common::run_app(
        || XamlApp::compose(App),
        || OBSERVED.with(|observed| std::mem::take(&mut *observed.borrow_mut())),
    )?;
    assert_eq!(observed.error, None);
    assert!(
        observed.resolved,
        "the runtime resolved the custom page type"
    );
    assert_eq!(
        observed.rendered,
        [
            "Activated(page 1)",
            "NavigatedTo(page 1)",
            "Navigated(true, page 1)",
            "NavigatingFrom(page 1)",
            "Activated(page 2)",
            "NavigatedFrom(page 1)",
            "NavigatedTo(page 2)",
            "Navigated(true, page 2)",
        ]
    );
    assert_eq!(
        observed.lifetime,
        Some(PageLifetime {
            alive_with_overrides: true,
            drops_with_overrides: 0,
            alive_after_release: false,
            drops_after_release: 1,
        }),
        "the standalone page stays alive through its overrides interface and releases its callback state exactly once"
    );
    Ok(())
}
