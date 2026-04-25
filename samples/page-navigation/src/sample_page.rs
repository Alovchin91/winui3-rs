use windows::{
    core::{h, IInspectable, Result},
    Win32::Foundation::E_NOTIMPL,
};

use winui3::{
    Activatable,
    Microsoft::UI::Xaml::{
        Controls::{Grid, Page, TextBlock},
        HorizontalAlignment,
        Navigation::{NavigatingCancelEventArgs, NavigationEventArgs},
        VerticalAlignment,
    },
    XamlPage, XamlPageOverrides,
};

pub struct SamplePage;

impl Activatable for SamplePage {
    fn activate() -> Result<IInspectable> {
        XamlPage::compose(SamplePage).map(Into::into)
    }
}

impl XamlPageOverrides for SamplePage {
    fn OnNavigatedFrom(&self, _base: &Page, _args: Option<&NavigationEventArgs>) -> Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn OnNavigatedTo(&self, base: &Page, _args: Option<&NavigationEventArgs>) -> Result<()> {
        let grid = Grid::new()?;
        let text_block = TextBlock::new()?;
        text_block.SetText(h!("Sample page"))?;
        text_block.SetHorizontalAlignment(HorizontalAlignment::Center)?;
        text_block.SetVerticalAlignment(VerticalAlignment::Center)?;
        grid.Children()?.Append(&text_block)?;
        base.SetContent(&grid)?;
        Ok(())
    }

    fn OnNavigatingFrom(
        &self,
        _base: &Page,
        _args: Option<&NavigatingCancelEventArgs>,
    ) -> Result<()> {
        Err(E_NOTIMPL.into())
    }
}
