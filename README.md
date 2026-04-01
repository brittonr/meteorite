# meteorite

UI component library for [Dioxus](https://dioxuslabs.com/).

Workspace of reusable primitives and components, structured similarly to
[subwayrat](https://github.com/brittonr/subwayrat) (ratatui widgets) but
targeting Dioxus's reactive, cross-platform model.

## Crates

| Crate | What it does |
|-------|-------------|
| `met-core` | Theming, size/variant enums, shared types |
| `met-widgets` | Button, input, checkbox, select, slider, switch, badge, progress |
| `met-layout` | VStack, HStack, Grid, Container, Sidebar, Split |
| `met-hooks` | use_debounce, use_toggle, use_local_storage |
| `met-table` | Data table with sortable columns |
| `met-overlay` | Modal, Dialog, Tooltip, Toast |
| `meteorite` | Re-export crate -- pulls in everything |
| `showcase` | Demo app |

## Quick start

```bash
nix develop
cargo check
cargo nextest run
```

## Usage

Add the umbrella crate to pull in everything:

```toml
[dependencies]
meteorite = { path = "crates/meteorite" }
```

Or pick individual crates:

```toml
[dependencies]
met-widgets = { path = "crates/met-widgets" }
met-layout = { path = "crates/met-layout" }
```

## License

MIT
