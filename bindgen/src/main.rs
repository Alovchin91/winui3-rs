use std::{env, fs, str};

fn main() -> Result<(), &'static str> {
    env::var("CARGO")
        .map_err(|_| r"please run this tool with `cargo run -p bindgen` from the workspace root")?;

    if !fs::exists("bindgen/winmd").expect("failed to check if winmd dir exists") {
        return Err("please make sure to put WinUI 3 metadata in the bindgen/winmd dir");
    }

    println!("Generating Windows.UI.Xaml.Interop bindings...");
    windows_bindgen::bindgen(["--etc", "bindgen/src/xaml_interop.txt"]).unwrap();

    println!("Generating WinUI 3 bindings...");
    windows_bindgen::bindgen(["--etc", "bindgen/src/winui3.txt"]).unwrap();

    println!("Patching features...");
    patch_winui3_features();

    println!("Patching fn IApplicationFactory...");
    patch_application_factory();

    println!("Patching fn IPageFactory...");
    patch_page_factory();

    println!("Done.");
    Ok(())
}

fn patch_winui3_features() {
    let manifest =
        fs::read_to_string("winui3/Cargo.toml").expect("failed to read winui3/Cargo.toml");
    let manifest = manifest
        .replace(
            r#"Graphics_Display = ["Graphics"]"#,
            r#"Graphics_Display = ["Graphics", "windows/Storage_Streams"]"#,
        )
        .replace(
            r#"UI = ["Foundation"]"#,
            r#"UI = ["Foundation", "windows/UI"]"#,
        )
        .replace(
            r#"UI_Composition = ["UI"]"#,
            r#"UI_Composition = ["UI", "windows/Foundation_Numerics", "windows/Graphics_Effects", "windows/UI_Composition"]"#,
        )
        .replace(
            r#"UI_Composition_SystemBackdrops = ["UI_Composition"]"#,
            r#"UI_Composition_SystemBackdrops = ["UI_Composition", "windows/UI_Core"]"#,
        )
        .replace(
            r#"UI_Content = ["UI"]"#,
            r#"UI_Content = ["UI", "windows/Graphics"]"#,
        )
        .replace(
            r#"UI_Input = ["UI"]"#,
            r#"UI_Input = ["UI", "windows/Graphics", "windows/System", "windows/UI_Core"]"#,
        )
        .replace(
            r#"UI_Input_DragDrop = ["UI_Input"]"#,
            r#"UI_Input_DragDrop = ["UI_Input", "windows/ApplicationModel_DataTransfer", "windows/Graphics_Imaging"]"#,
        )
        .replace(
            r#"UI_Text = ["UI"]"#,
            r#"UI_Text = ["UI", "windows/Storage_Streams", "windows/UI_Text"]"#,
        )
        .replace(
            r#"UI_Windowing = ["UI"]"#,
            r#"UI_Windowing = ["UI", "windows/Graphics"]"#,
        )
        .replace(
            r#"UI_Xaml = ["UI"]"#,
            r#"UI_Xaml = ["UI", "UI_Xaml_Interop", "windows/ApplicationModel_Activation", "windows/ApplicationModel_Core", "windows/ApplicationModel_DataTransfer_DragDrop", "windows/Foundation_Collections", "windows/Graphics_Imaging", "windows/UI_Core"]"#,
        )
        .replace(
            r#"UI_Xaml_Controls = ["UI_Xaml"]"#,
            r#"UI_Xaml_Controls = ["UI_Text", "UI_Xaml", "windows/ApplicationModel_Contacts", "windows/Devices_Geolocation", "windows/Globalization_NumberFormatting", "windows/Media_Casting", "windows/Media_Playback"]"#,
        )
        .replace(
            r#"UI_Xaml_Documents = ["UI_Xaml"]"#,
            r#"UI_Xaml_Documents = ["UI_Text", "UI_Xaml"]"#,
        )
        .replace(
            r#"UI_Xaml_Input = ["UI_Xaml"]"#,
            r#"UI_Xaml_Input = ["UI_Input", "UI_Xaml"]"#,
        )
        .replace(
            r#"UI_Xaml_Interop = ["UI_Xaml"]"#,
            r#"UI_Xaml_Interop = []"#,
        )
        .replace(
            r#"UI_Xaml_Markup = ["UI_Xaml"]"#,
            r#"UI_Xaml_Markup = ["UI_Xaml", "windows/Storage_Streams"]"#,
        )
        .replace(
            r#"UI_Xaml_Media = ["UI_Xaml"]"#,
            r#"UI_Xaml_Media = ["UI_Xaml", "windows/Storage_Streams"]"#,
        )
        .replace(
            r#"UI_Xaml_Media_Imaging = ["UI_Xaml_Media"]"#,
            r#"UI_Xaml_Media_Imaging = ["UI_Xaml_Media", "windows/ApplicationModel_Background"]"#,
        )
        .replace(
            r#"UI_Xaml_Printing = ["UI_Xaml"]"#,
            r#"UI_Xaml_Printing = ["UI_Xaml", "windows/Graphics_Printing"]"#,
        );
    fs::write("winui3/Cargo.toml", &manifest).expect("failed to write winui3/Cargo.toml");
}

fn patch_application_factory() {
    const MICROSOFT_UI_XAML_MOD: &str = "winui3/src/Microsoft/UI/Xaml/mod.rs";

    let contents = fs::read_to_string(MICROSOFT_UI_XAML_MOD).expect("failed to read mod.rs");
    let contents = contents.replace(
        r#"fn IApplicationFactory"#,
        r#"pub(crate) fn IApplicationFactory"#,
    );
    fs::write(MICROSOFT_UI_XAML_MOD, &contents).expect("failed to write mod.rs");
}

fn patch_page_factory() {
    const MICROSOFT_UI_XAML_CONTROLS_MOD: &str = "winui3/src/Microsoft/UI/Xaml/Controls/mod.rs";

    let contents =
        fs::read_to_string(MICROSOFT_UI_XAML_CONTROLS_MOD).expect("failed to read mod.rs");
    let contents = contents.replace(r#"fn IPageFactory"#, r#"pub(crate) fn IPageFactory"#);
    fs::write(MICROSOFT_UI_XAML_CONTROLS_MOD, &contents).expect("failed to write mod.rs");
}
