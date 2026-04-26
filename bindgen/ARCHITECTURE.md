# bindgen architecture

The `bindgen` crate is an internal tool that regenerates the WinRT bindings
shipped in the `winui3` crate. It wraps `windows-bindgen` and post-processes
the output.

## When to re-run

Re-run `cargo run -p bindgen` when:

- Adding support for a new WinAppSDK version (the main trigger).
- Bumping the `windows-bindgen` workspace dependency.
- Adding a new top-level WinMD namespace, or changing the filters in
  `bindgen/etc/winui3.txt`.

## Inputs

The tool reads `.winmd` files from `bindgen/winmd/` (gitignored). The
directory must be populated manually before running. `bindgen/etc/winui3.txt`
lists the expected files.

### Source packages

The eight WinMDs come from four NuGet packages:

| NuGet package                               | Path inside the `.nupkg`       | WinMDs                                                                                  |
| ------------------------------------------- | ------------------------------ | --------------------------------------------------------------------------------------- |
| `Microsoft.WindowsAppSDK.Foundation`        | `metadata/`                    | `Microsoft.Windows.ApplicationModel.Resources.winmd`, `Microsoft.Windows.Storage.Pickers.winmd` |
| `Microsoft.WindowsAppSDK.InteractiveExperiences` | `metadata/10.0.18362.0/`  | `Microsoft.Foundation.winmd`, `Microsoft.Graphics.winmd`, `Microsoft.UI.winmd`          |
| `Microsoft.WindowsAppSDK.WinUI`             | `metadata/`                    | `Microsoft.UI.Text.winmd`, `Microsoft.UI.Xaml.winmd`                                    |
| `Microsoft.Web.WebView2`                    | `lib/`                         | `Microsoft.Web.WebView2.Core.winmd`                                                     |

### Picking versions

- **`Microsoft.WindowsAppSDK.*` packages** — use the **earliest available
  patch** release (non-experimental, non-preview) for the minor version
  you're adding. For WinAppSDK 1.8, that's the first stable 1.8.x.y on NuGet.
- **`Microsoft.Web.WebView2`** — use the **minimum version** that the chosen
  `Microsoft.WindowsAppSDK.WinUI` package depends on (check its `.nuspec`).

Extract each `.nupkg` (they're zips), copy the WinMDs from the paths above
into `bindgen/winmd/`, then run the tool. The nested subdirectory in
`InteractiveExperiences` (`metadata/10.0.18362.0/`) is only the source
location — everything ends up flat in `bindgen/winmd/`.

## Running

```bash
cargo run -p bindgen
```

Must be invoked from the workspace root. The tool runs two
`windows_bindgen::bindgen()` passes followed by two post-process steps:

1. **Pass 1** — generate `Windows.UI.Xaml.Interop` bindings (config:
   `bindgen/etc/xaml_interop.txt`). This has to be a separate pass because
   of a `windows-bindgen` bug — generating it alongside the rest of WinUI
   produces wrong output. Worth re-checking on each `windows-bindgen` bump
   (see `TODO.md`).
2. **Pass 2** — generate everything else (config:
   `bindgen/etc/winui3.txt`). WebView2 is filtered out here via
   `!Microsoft.Web.WebView2*` / `!Microsoft.UI.Xaml.*WebView2*` (WebView2 has
   its own Rust ecosystem).
3. **`patch_winui3_features()`** — rewrites feature entries in
   `winui3/Cargo.toml` with the full dependency list from `FEATURE_PATCHES`,
   using `toml_edit`. Covers `windows/*` deps the generator can't infer,
   plus special cases like `UI_Xaml_Interop = []`. Asserts each patched
   feature exists in the manifest.
4. **`promote_factory()`** — called once each for `IApplicationFactory` and
   `IPageFactory`. Promotes them from private to `pub(crate)` so
   `winui3/src/xaml_app.rs` / `xaml_page.rs` can reach them. Asserts the
   needle is present.

Both post-process steps panic on assertion failure rather than silently
no-opping, so a broken assumption after a `windows-bindgen` upgrade fails
fast at the exact source.

After regenerating, inspect the diff. If `cargo check` reveals unresolved
types in the generated code, add the missing dep to `FEATURE_PATCHES` in
`src/main.rs`.

## Known gap: breaking changes between WinAppSDK versions

There's currently no plan for how to handle a WinAppSDK release that breaks
compatibility with earlier supported versions. Each bump so far has been
additive — new types and methods, no source-breaking removals — so the
`WindowsAppSDKVersion` variants coexist cleanly in a single crate.

If a future release breaks APIs we re-export (e.g. renames or removes a type
currently in the bindings), we'd need to choose between: pinning consumers
to a single SDK via cargo features, producing separate crates per major
version, or declaring earlier SDKs unsupported. No decision has been made.
Not expected to come up often in practice.

## Backlog

Open items for the tool live in `TODO.md`.
