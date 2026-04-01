## Context

meteorite and subwayrat are sibling projects in `~/git/`. meteorite targets
Dioxus (web/desktop), subwayrat targets ratatui (terminal). Three pure-logic
modules were copy-pasted from subwayrat into meteorite and have started
diverging in minor ways (field renames, formatting, comment drift).

Both projects use Nix flakes with crane for builds. Both are virtual
workspaces with crates under `crates/`.

## Goals / Non-Goals

**Goals:** Single source of truth for fuzzy scoring, CalDate, and tree
visible-row computation. Both projects depend on the shared crate. Bug fixes
land once.

**Non-Goals:** Unifying markdown rendering, table logic, or anything that
touches a UI framework. Not trying to build a general-purpose utility crate —
only extracting logic that's already duplicated.

## Decisions

### 1. Crate Location

**Choice:** New repository at `~/git/ratcore` with its own flake.

**Rationale:** Both meteorite and subwayrat are independent projects. Nesting
the shared crate inside either one creates an awkward ownership dynamic. A
third repo keeps the dependency direction clean.

**Alternative:** Monorepo containing all three. Rejected — the projects have
different release cadences and contributor sets.

**Implementation:** During development, both projects use `path = "../ratcore"`
in their workspace `Cargo.toml`. For releases, publish to crates.io or use
git dependencies.

### 2. Tree Data Access

**Choice:** Use subwayrat's `TreeData` trait with `usize` node IDs.

**Rationale:** The trait-based approach decouples the walk algorithm from
storage. meteorite currently uses `String` IDs in a flat vec — this is a
convenience wrapper, not a fundamental design. meteorite can implement
`TreeData` for its data model (or switch to `SimpleTree` + `usize` IDs).

**Alternative:** Use meteorite's `Vec<TreeItem>` flat model. Rejected — it
bakes a specific storage layout into the shared crate. The trait is more
flexible and subwayrat already depends on it for custom tree data sources.

**Implementation:** `ratcore` exports `TreeData`, `SimpleTree`, `VisibleRow`,
`compute_visible_rows`. meteorite's `met-tree` implements `TreeData` for its
`Vec<TreeItem>` as a thin adapter. subwayrat's `rat-tree` drops its local
`model.rs` and re-exports from `ratcore`.

### 3. Fuzzy Field Naming

**Choice:** Use `index: usize` (meteorite's name) for `ScoredMatch`.

**Rationale:** `index` is shorter and the field is always set by the caller
after scoring (the scorer always returns 0). The name doesn't matter much —
pick one and update the other.

**Alternative:** `candidate_idx`. Rejected — more verbose for no clarity gain.

### 4. Module Structure

**Choice:** Three top-level modules: `fuzzy`, `caldate`, `tree`.

```
ratcore/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── fuzzy.rs
│   ├── caldate.rs
│   └── tree.rs       # TreeData, SimpleTree, VisibleRow, compute_visible_rows
```

**Rationale:** Flat and obvious. Each module corresponds to one of the three
extracted concerns. No nested module trees for ~300 lines of code.

## Risks / Trade-offs

**[Cross-repo coordination]** — Changes to ratcore require updating both
downstream projects. Mitigated by keeping the API surface tiny and stable.
These modules haven't changed meaningfully in weeks; they're settled logic.

**[meteorite tree migration]** — meteorite's tree currently uses `String` IDs.
Switching to `usize` IDs (or writing a `TreeData` adapter) is a small
refactor in `met-tree`. The adapter approach avoids changing meteorite's
public API.

**[Build dependency]** — Both projects gain a build-time dependency on ratcore.
Since ratcore has zero deps and ~300 lines, compile time impact is negligible.
