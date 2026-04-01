## Phase 1: Create ratcore crate

- [x] Create `~/git/ratcore` repo with `Cargo.toml` (edition 2024, zero deps), `flake.nix`, `src/lib.rs`
- [x] Extract `fuzzy.rs` — take subwayrat's `score.rs`, rename `candidate_idx` → `index`, add all tests from both projects
- [x] Extract `caldate.rs` — take meteorite's `calendar.rs` CalDate + day/month methods (cleaner formatting), add `month_name()` from subwayrat's version
- [x] Extract `tree.rs` — take subwayrat's `model.rs` wholesale (`TreeData` trait, `SimpleTree`, `VisibleRow`, `compute_visible_rows`, all tests)
- [x] Verify `cargo test` passes in ratcore — 30 tests pass

## Phase 2: Wire up subwayrat

- [x] Add `ratcore` as path dependency in subwayrat workspace `Cargo.toml`
- [x] `rat-fuzzy/score.rs`: replace local types and `fuzzy_score` with re-exports from `ratcore::fuzzy`
- [x] `rat-fuzzy/state.rs` + `render.rs`: rename `candidate_idx` → `index` (3 call sites)
- [x] `rat-datepicker/calendar.rs`: replace local `CalDate` with `ratcore::caldate::CalDate`, use `month_name_short()` for widget title
- [x] `rat-tree/model.rs`: replace local `TreeData`, `SimpleTree`, `VisibleRow`, `compute_visible_rows` with re-exports from `ratcore::tree`
- [x] Verify `cargo test` passes across subwayrat workspace — all crates green

## Phase 3: Wire up meteorite

- [x] Add `ratcore` as path dependency in meteorite workspace `Cargo.toml`
- [x] `met-command-palette/fuzzy.rs`: replace local `ScoredMatch` and `fuzzy_score` with re-exports from `ratcore::fuzzy`
- [x] `met-datepicker/calendar.rs`: replace local `CalDate` + methods with `ratcore::caldate::CalDate`
- [x] `met-tree/model.rs`: implement `ratcore::tree::TreeData` for meteorite's `Vec<TreeItem>` via `TreeItemsAdapter`, convert String↔usize IDs at the boundary, keep local `VisibleRow` with String fields
- [x] Verify `cargo test` passes across meteorite workspace — 27 tests pass

## Phase 4: Cleanup

- [x] Remove dead code — unused `child_ids` in met-tree, unused `Style` import in rat-datepicker, unused `i` variable in rat-fuzzy render
- [x] Final test run: meteorite 27 tests pass, subwayrat full suite passes, ratcore 30 tests pass
