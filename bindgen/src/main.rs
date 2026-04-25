use std::{env, fs, str};

use toml_edit::{value, Array, DocumentMut};

fn main() -> Result<(), &'static str> {
    env::var("CARGO")
        .map_err(|_| r"please run this tool with `cargo run -p bindgen` from the workspace root")?;

    if !fs::exists("bindgen/winmd").expect("failed to check if winmd dir exists") {
        return Err("please make sure to put WinUI 3 metadata in the bindgen/winmd dir");
    }

    println!("Generating Windows.UI.Xaml.Interop bindings...");
    windows_bindgen::bindgen(["--etc", "bindgen/etc/xaml_interop.txt"]).unwrap();

    println!("Generating WinUI 3 bindings...");
    windows_bindgen::bindgen(["--etc", "bindgen/etc/winui3.txt"]).unwrap();

    println!("Patching features...");
    patch_winui3_features();

    println!("Patching fn IApplicationFactory...");
    promote_factory("winui3/src/Microsoft/UI/Xaml/mod.rs", "IApplicationFactory");

    println!("Patching fn IPageFactory...");
    promote_factory(
        "winui3/src/Microsoft/UI/Xaml/Controls/mod.rs",
        "IPageFactory",
    );

    println!("Done.");
    Ok(())
}

// The full desired dependency list for each feature `windows-bindgen` can't
// emit on its own — both internal features and the `windows/*` pass-throughs
// that the generator doesn't infer.
#[rustfmt::skip]
const FEATURE_PATCHES: &[(&str, &[&str])] = &[
    ("Graphics_Display",               &["Graphics", "windows/Storage_Streams"]),
    ("UI",                             &["Foundation", "windows/UI"]),
    ("UI_Composition",                 &["UI", "windows/Foundation_Numerics", "windows/Graphics_Effects", "windows/UI_Composition"]),
    ("UI_Composition_SystemBackdrops", &["UI_Composition", "windows/UI_Core"]),
    ("UI_Content",                     &["UI", "windows/Graphics"]),
    ("UI_Input",                       &["UI", "windows/Graphics", "windows/System", "windows/UI_Core"]),
    ("UI_Input_DragDrop",              &["UI_Input", "windows/ApplicationModel_DataTransfer", "windows/Graphics_Imaging"]),
    ("UI_Text",                        &["UI", "windows/Storage_Streams", "windows/UI_Text"]),
    ("UI_Windowing",                   &["UI", "windows/Graphics"]),
    ("UI_Xaml",                        &["UI", "UI_Xaml_Interop", "windows/ApplicationModel_Activation", "windows/ApplicationModel_Core", "windows/ApplicationModel_DataTransfer_DragDrop", "windows/Foundation_Collections", "windows/Graphics_Imaging", "windows/UI_Core"]),
    ("UI_Xaml_Controls",               &["UI_Text", "UI_Xaml", "windows/ApplicationModel_Contacts", "windows/Devices_Geolocation", "windows/Globalization_NumberFormatting", "windows/Media_Casting", "windows/Media_Playback"]),
    ("UI_Xaml_Documents",              &["UI_Text", "UI_Xaml"]),
    ("UI_Xaml_Input",                  &["UI_Input", "UI_Xaml"]),
    ("UI_Xaml_Interop",                &[]),
    ("UI_Xaml_Markup",                 &["UI_Xaml", "windows/Storage_Streams"]),
    ("UI_Xaml_Media",                  &["UI_Xaml", "windows/Storage_Streams"]),
    ("UI_Xaml_Media_Imaging",          &["UI_Xaml_Media", "windows/ApplicationModel_Background"]),
    ("UI_Xaml_Printing",               &["UI_Xaml", "windows/Graphics_Printing"]),
];

fn patch_winui3_features() {
    const PATH: &str = "winui3/Cargo.toml";

    let source = fs::read_to_string(PATH).expect("failed to read winui3/Cargo.toml");
    let mut doc = source
        .parse::<DocumentMut>()
        .expect("failed to parse winui3/Cargo.toml");
    let features = doc["features"]
        .as_table_mut()
        .expect("missing [features] table");

    for &(name, deps) in FEATURE_PATCHES {
        assert!(
            features.contains_key(name),
            "feature `{name}` not emitted by windows-bindgen — stale FEATURE_PATCHES entry?",
        );
        let array: Array = deps.iter().copied().collect();
        features[name] = value(array);
    }

    fs::write(PATH, doc.to_string()).expect("failed to write winui3/Cargo.toml");
}

fn promote_factory(path: &str, factory: &str) {
    let needle = format!("  fn {factory}");
    let replacement = format!("  pub(crate) fn {factory}");

    let contents =
        fs::read_to_string(path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"));
    assert!(
        contents.contains(&needle),
        "{path}: `{needle}` not found — did windows-bindgen output format change?",
    );

    let patched = contents.replace(&needle, &replacement);
    fs::write(path, patched).unwrap_or_else(|e| panic!("failed to write {path}: {e}"));
}
