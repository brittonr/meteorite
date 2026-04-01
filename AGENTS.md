# Agent Notes

## Build
- `nix develop` then `cargo check` / `cargo nextest run`
- First build requires `git add Cargo.lock` before `nix develop` works (crane needs it tracked)
- `nix run nixpkgs#cargo -- generate-lockfile` to bootstrap the lockfile without a devshell

## Architecture
- Virtual workspace with crates under `crates/`
- `met-core` has theming, Size, Variant -- everything else depends on it
- `met-widgets` has the basic UI components (Button, Input, etc.)
- `met-layout` has layout primitives (VStack, HStack, Grid, Container, Sidebar, Split)
- `met-hooks` has custom Dioxus hooks
- `met-table` has the data table component
- `met-overlay` has modal/dialog/tooltip/toast
- `meteorite` re-exports all crates (like dioxus re-exports its sub-crates)
- `showcase` is the demo app (binary, depends on everything)

## Dioxus patterns
- Components use `#[derive(Props)]` + `#[component]` macro
- `rsx!` macro for JSX-like templates
- Props with defaults via `#[props(default)]` or `#[props(default = value)]`
- Signals are Copy -- `let value = use_signal(|| x)` then `value.read()` / `value.set()`
- Closures that mutate signals: assign signal to a local `let mut w = signal; w.set(...)` to avoid FnMut vs Fn issues
- For loops in rsx with closures that capture loop vars: clone into the block scope or use `.into_iter()` and extract values before the rsx

## Gotchas
- Dioxus 0.6.3 rsx closures require `'static` borrows -- can't borrow from `props` in event handlers inside `for` loops. Clone the values first.
- `workspace.metadata.crane.name` in root Cargo.toml silences crane warnings for virtual workspaces
