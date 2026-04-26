# bindgen TODO

Open improvements for the bindgen tool, roughly ordered by priority.

## Wrap `windows_bindgen::bindgen()` calls with context

`src/main.rs:14` and `:17` use `.unwrap()`. A failure panics without a phase label.
Return `Result<(), Box<dyn Error>>` from `main` and propagate with context
(e.g. `"while generating WinUI 3 bindings"`). Low urgency.

## Preflight the WinMD file list

`src/main.rs:9` only checks that `bindgen/winmd/` exists. The `--in` lines in
`etc/winui3.txt` name eight specific `.winmd` files; if any are missing,
`windows-bindgen` fails with an unhelpful message. Parsing the config and
reporting missing files would greatly improve first-run setup for new
contributors.

## Resolve paths from `CARGO_MANIFEST_DIR`

`env::var("CARGO")` at `src/main.rs:6` only verifies invocation via cargo, not
cwd. All paths (`winui3/Cargo.toml`, `bindgen/winmd`, …) assume workspace-root
cwd. Using `env!("CARGO_MANIFEST_DIR")/..` would make the tool cwd-independent.

## Revisit the two-pass `windows-bindgen` workaround

`etc/winui3.txt:13-17` documents that `Windows.UI.Xaml.Interop` has to be
generated in a separate pass due to a bindgen bug. Worth re-checking on each
`windows-bindgen` version bump (currently `0.63.0`) to see if the workaround
can collapse to one pass.

## WebView2 escape hatch

`etc/winui3.txt:52-56` excludes all `Microsoft.Web.WebView2` and `*WebView2*`
types unconditionally. Consumers who need them today have no option beyond
forking the bindgen config. Consider a bindgen flag / env var to opt them in
behind a separate feature.
