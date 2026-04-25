use windows::core::{h, Error, Result, HSTRING};

use winui3::{
    xaml_typename,
    Microsoft::UI::Xaml::{
        Application,
        Controls::{Frame, XamlControlsResources},
        LaunchActivatedEventArgs,
        Markup::IXamlType,
        Window,
    },
    XamlAppOverrides,
};

use crate::sample_page::SamplePage;

pub struct App;

impl XamlAppOverrides for App {
    fn OnLaunched(
        &self,
        base: &Application,
        _args: Option<&LaunchActivatedEventArgs>,
    ) -> Result<()> {
        // Add default XAML controls resources to the main app's resource dictionary
        let xaml_controls_resources = XamlControlsResources::new()?;
        base.Resources()?
            .MergedDictionaries()?
            .Append(&xaml_controls_resources)?;

        let frame = Frame::new()?;
        let page_type = xaml_typename("SamplePage");
        frame.Navigate2(&page_type)?;

        let window = Window::new()?;
        window.SetTitle(h!("XAML navigation sample"))?;
        window.SetContent(&frame)?;
        window.Activate()?;

        Ok(())
    }

    fn TryResolveXamlType(&self, full_name: &HSTRING) -> Result<IXamlType> {
        if full_name == "SamplePage" {
            winui3::XamlCustomType::<SamplePage>::for_page(full_name)
        } else {
            Err(Error::empty())
        }
    }
}
