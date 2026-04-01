## Why

meteorite (Dioxus component library) and subwayrat (ratatui widget library) share
copy-pasted pure-logic modules that have already started drifting. Fuzzy scoring
has a renamed field (`index` vs `candidate_idx`). CalDate methods are identical
today but there's no mechanism preventing silent divergence on the next bugfix.
The tree visible-row algorithm is structurally the same but uses different ID
types and data-access patterns.

A shared crate eliminates the duplication and gives both projects a single place
to fix bugs and add tests for this logic.

## What Changes

Extract three pure-logic modules into a new crate (working name: `ratcore`)
that both meteorite and subwayrat depend on. The crate has zero UI dependencies —
no dioxus, no ratatui.

- **Fuzzy scoring**: `ScoredMatch` + `fuzzy_score()` (~90 lines)
- **CalDate**: date struct + weekday/days-in-month/day arithmetic (~120 lines)
- **Tree visible rows**: `TreeData` trait + `VisibleRow` + `compute_visible_rows()` (~100 lines)

## Capabilities

### New Capabilities
- `shared-fuzzy`: Single fuzzy scoring implementation used by both projects
- `shared-caldate`: Single CalDate implementation used by both projects
- `shared-tree-model`: Single tree flattening algorithm behind a trait, used by both projects

### Modified Capabilities
- `met-command-palette`: Replaces internal `fuzzy.rs` with re-export from `ratcore`
- `met-datepicker`: Replaces internal `CalDate` with re-export from `ratcore`
- `met-tree`: Replaces internal `model.rs` with adapter over `ratcore::tree`
- `rat-fuzzy`: Replaces internal `score.rs` with re-export from `ratcore`
- `rat-datepicker`: Replaces internal `CalDate` with re-export from `ratcore`
- `rat-tree`: Replaces internal `model.rs` with re-export from `ratcore`

## Out of Scope

- Markdown rendering — architectures diverged (AST-based vs direct-render). Not worth unifying.
- Table/widget logic — too coupled to their UI frameworks.
- Anything that imports dioxus or ratatui. The shared crate is pure logic only.

## Impact

- **New repo**: `ratcore` lives in its own repository (or as a path dependency during development)
- **APIs**: Public API is the union of what both projects already use, minus the field name inconsistency
- **Dependencies**: Both projects gain a dependency on `ratcore`. `ratcore` itself has zero dependencies.
- **Testing**: Existing tests from both projects migrate into `ratcore`. Both projects keep integration tests that verify their adapters.
