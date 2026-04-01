# ☄️ meteorite

UI component library for [Dioxus](https://dioxuslabs.com). Themed wrappers around [dioxus-primitives](https://github.com/DioxusLabs/components) with pluggable theming via CSS custom properties.

## Quick Start

```rust
use dioxus::prelude::*;
use meteorite::met_core::{Theme, ThemeProvider, Variant};
use meteorite::met_widgets::Button;

fn app() -> Element {
    rsx! {
        ThemeProvider { theme: Theme::dark(),
            Button { variant: Variant::Primary, "Click me" }
        }
    }
}
```

## Crates

| Crate | Description |
|---|---|
| `met-core` | Theme, palette, tokens, variant/size enums, CSS stylesheet |
| `met-widgets` | Button, Badge, Alert, Card, Icon, Input, Select, Checkbox, Switch, Slider, Progress, Tabs, Spinner, Loader, Form, FormField, Divider |
| `met-overlay` | Tooltip, Dialog, Modal, Popover, ContextMenu, Toast |
| `met-layout` | VStack, HStack, Container, Grid, Sidebar, Split |
| `met-hooks` | use_debounce, use_toggle, use_local_storage |
| `met-table` | Data table with sorting, inline editing, row selection, row numbers |
| `met-tree` | Tree view with expand/collapse, guide lines, keyboard nav |
| `met-command-palette` | Fuzzy-search ⌘K command palette overlay |
| `met-datepicker` | Calendar date picker and time input |
| `met-markdown` | Markdown renderer (headings, lists, code, links, images) |
| `meteorite` | Umbrella re-export of all crates |

## Components (24)

### Widgets (`met-widgets`)
- **Button** — variants, sizes, loading, disabled
- **Badge** — inline status labels
- **Alert** — dismissible banners (success/warning/danger/info)
- **Card** — container with Header, Body, Footer
- **Icon** — 37 SVG icons (Feather-style), 5 sizes
- **TextInput** — text field
- **Select** — dropdown with keyboard nav, typeahead (wraps primitives)
- **Checkbox** — tri-state with indicator (wraps primitives)
- **Switch** — toggle with thumb (wraps primitives)
- **Slider** — track + range + thumb (wraps primitives)
- **Progress** — bar with indicator (wraps primitives)
- **Tabs** — tab list + content panels (wraps primitives)
- **Spinner** — SVG animated spinner + LoadingOverlay
- **Loader** — dots, bars, pulse, skeleton patterns + ContentLoader
- **Divider** — horizontal/vertical separator (wraps primitives Separator)
- **Form** — FormGroup, FormLabel, FormInput, FormTextarea, FormSelect, FormCheckbox, FormError
- **FormField** — compound field: label + input + validation + help text

### Overlays (`met-overlay`)
- **Tooltip** — hover content (wraps primitives)
- **Dialog** — modal/non-modal with focus trapping (wraps primitives)
- **Modal** — always-modal convenience alias
- **Popover** — triggered flyout (wraps primitives)
- **ContextMenu** — right-click menu (wraps primitives)
- **Toast** — notification system with auto-dismiss (wraps primitives)

### Layout (`met-layout`)
- **VStack / HStack** — flex column/row with gap
- **Container** — centered max-width wrapper
- **Grid** — CSS grid with column count
- **Sidebar** — collapsible side panel
- **Split** — horizontal/vertical split pane

### Tree (`met-tree`)
- **Tree** — hierarchical tree view with expand/collapse, guide lines (│├└), keyboard nav (Arrow keys, Enter, Space), icons, selection events

### Command Palette (`met-command-palette`)
- **CommandPalette** — ⌘K-style fuzzy search overlay with match highlighting, keyboard nav (↑↓ Enter Escape), shortcuts display, backdrop dismiss

### Date Picker (`met-datepicker`)
- **DatePicker** — calendar grid with month navigation, today highlight, weekend styling, date selection
- **TimeInput** — HH:MM time entry with number inputs

### Markdown (`met-markdown`)
- **Markdown** — renders markdown string as themed HTML: headings (h1–h6), paragraphs, bold/italic/bold-italic, inline code, fenced code blocks with language class, links, images, blockquotes, ordered/unordered lists, horizontal rules

## Theming

All components read from `--met-*` CSS custom properties. Wrap your app in `ThemeProvider`:

```rust
// Presets
ThemeProvider { theme: Theme::dark(), /* ... */ }
ThemeProvider { theme: Theme::light(), /* ... */ }

// Runtime switching
let mut theme = use_signal(|| Theme::dark());
rsx! {
    ThemeProvider { theme: theme(),
        button { onclick: move |_| theme.set(Theme::light()), "Toggle" }
    }
}
```

### Custom themes

```rust
let theme = Theme::builder("corporate")
    .palette(Palette { primary: "#003366".into(), ..Palette::light() })
    .extra_color("brand", "#ff6600")      // → --met-brand + .met-brand class
    .extra_color("accent", "#9333ea")     // → --met-accent + .met-accent class
    .extra_token("shadow-lg", "0 8px 24px rgba(0,0,0,0.15)")  // → --met-shadow-lg
    .build();

// Use custom variants
Button { variant: Variant::Custom("brand".into()), "Buy now" }
```

### Nested themes

```rust
ThemeProvider { theme: Theme::dark(),
    Sidebar {}
    ThemeProvider { theme: Theme::light(),
        MainContent {}  // sees light theme
    }
}
```

### Reading theme in components

```rust
fn MyComponent() -> Element {
    let theme = use_theme();           // clones Theme
    let sig = use_theme_signal();      // Signal<Theme>, no clone
    // ...
}
```

## Features

```toml
[dependencies]
meteorite = { path = "..." }              # renderer-agnostic
meteorite = { path = "...", features = ["web"] }  # enables dioxus-primitives/web
```

The `web` feature gates `dioxus-primitives/web` (time/wasm-bindgen). Library crates stay cross-platform by default.

## Running the Showcase

```bash
cd crates/showcase
dx serve
```

## Architecture

```
ThemeProvider { theme: Theme::dark() }
  │
  ├─ <style> :root { --met-primary: #3b82f6; ... }   ← palette + tokens
  ├─ <style> .met-btn { color: var(--_c); ... }       ← component CSS
  │
  └─ Button { variant: Variant::Primary }
       ↓ class="met-btn met-primary"
       ↓ .met-primary → --_c: var(--met-primary)
       ↓ .met-btn → color: var(--_c)
       → picks up #3b82f6
```

Components emit `met-*` class names. Variant classes set `--_c` (color) and `--_bg` (background) local properties. Component rules reference those locals. Swap the theme and everything recolors.
