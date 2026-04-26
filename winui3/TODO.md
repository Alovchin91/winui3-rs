# winui3 TODO

Open backlog for the `winui3` crate, roughly ordered by priority.

Scope reminder: this crate is for **code-driven** WinUI 3 UI from Rust. XAML
markup support is explicitly a non-goal (see `ARCHITECTURE.md` →
"Non-goals: XAML markup files"). Items below stay inside that scope.

## Delegate `XamlSystemBaseType` to the platform provider

`xaml_types.rs:122-133` — `XamlCustomType::for_page` constructs an
`XamlSystemBaseType` stub for `Microsoft.UI.Xaml.Controls.Page` whose
`BaseType`, `IsConstructible`, `GetMember`, etc. all return `E_NOTIMPL`.
Navigation never walks past `FullName` / `UnderlyingType`, so it works
today, but it's a latent landmine: any future code path that introspects
the base chain hits `E_NOTIMPL`.

Generated `XamlTypeInfo.g.cs` doesn't fabricate base types — it looks them
up through the metadata provider. The equivalent here: query
`XamlControlsXamlMetaDataProvider::GetXamlTypeByFullName(
"Microsoft.UI.Xaml.Controls.Page")` and store the returned `IXamlType` as
the base. Removes the `E_NOTIMPL` slots and matches platform-generated
behaviour.

`XamlSystemBaseType` may then become unnecessary — re-evaluate whether to
delete it once nothing references it.

## `RunInitializer` hook for Rust-side DependencyProperties

`xaml_types.rs:217-220` — `XamlCustomType::RunInitializer` is a no-op with
a `// TODO` comment. In generated `XamlTypeInfo.g.cs`, `RunInitializer` is
where `DependencyProperty::Register(...)` calls fire on first use of the
type. Today no Rust API exposes DependencyProperties, so the no-op is
harmless.

If/when the crate grows a way for Rust code to declare
DependencyProperties, this is where the registration callback for the
owning type needs to fire. Design the user-facing DP API first; this hook
is the easy half.
