# Shared Logic Crate Specification

## Purpose

Defines the public API of the `ratcore` crate — the pure-logic modules shared
between meteorite and subwayrat. Every type and function here has zero UI
framework dependencies.

## Requirements

### Requirement: Zero UI Dependencies

The crate MUST NOT depend on dioxus, ratatui, or any rendering framework.
It MUST compile with no default features and no transitive UI deps.

#### Scenario: Clean dependency tree

- GIVEN the ratcore crate
- WHEN `cargo tree` is run
- THEN no dioxus or ratatui crates appear in the output

### Requirement: Fuzzy Scoring

The crate MUST expose a `fuzzy_score(text, query) -> Option<ScoredMatch>` function
and a `ScoredMatch` struct with `index: usize`, `score: i32`, and `positions: Vec<usize>`.

#### Scenario: Prefix match scores higher than mid-word

- GIVEN candidates "Ship docs" and "Worship plan"
- WHEN scored against query "ship"
- THEN "Ship docs" has a higher score

#### Scenario: No match returns None

- GIVEN candidate "hello"
- WHEN scored against query "xyz"
- THEN the result is None

#### Scenario: Empty query matches everything

- GIVEN any candidate
- WHEN scored against an empty query
- THEN the result is Some with score 0 and empty positions

#### Scenario: Case insensitive matching

- GIVEN candidate "Ship Docs"
- WHEN scored against query "sd"
- THEN the result is Some (matches S and d)

#### Scenario: Consecutive matches beat scattered

- GIVEN candidates "abcdef" and "a_b_c_def"
- WHEN scored against query "abc"
- THEN the consecutive match scores higher

#### Scenario: Match positions tracked

- GIVEN candidate "Ship docs"
- WHEN scored against query "sd"
- THEN positions contains the indices of 'S' and 'd'

### Requirement: Calendar Date

The crate MUST expose a `CalDate` struct with `year: i32`, `month: u32`, `day: u32`
and methods: `weekday()`, `days_in_month()`, `next_day()`, `prev_day()`, `add_days(i32)`,
`next_month()`, `prev_month()`.

#### Scenario: Weekday calculation

- GIVEN CalDate 2026-04-01 (Wednesday)
- WHEN `weekday()` is called
- THEN the result is 2 (0=Monday convention)

#### Scenario: Leap year handling

- GIVEN CalDate with year 2024, month 2
- WHEN `days_in_month()` is called
- THEN the result is 29

#### Scenario: Non-leap year

- GIVEN CalDate with year 2023, month 2
- WHEN `days_in_month()` is called
- THEN the result is 28

#### Scenario: Month rollover

- GIVEN CalDate 2026-01-31
- WHEN `next_day()` is called
- THEN the result is 2026-02-01

#### Scenario: Year rollover

- GIVEN CalDate 2026-12-31
- WHEN `next_day()` is called
- THEN the result is 2027-01-01

#### Scenario: Backward day arithmetic

- GIVEN CalDate 2026-03-01
- WHEN `prev_day()` is called
- THEN the result is 2026-02-28

### Requirement: Tree Visible Rows

The crate MUST expose a `TreeData` trait, a `VisibleRow` struct, and a
`compute_visible_rows(data, expanded) -> Vec<VisibleRow>` function.

`TreeData` MUST use `usize` node IDs and provide: `root_count()`, `root(index)`,
`child_count(node)`, `child(node, index)`, `node_label(node)`, `node_icon(node)`,
`parent(node)`.

`VisibleRow` MUST contain: `node_id`, `depth`, `has_children`, `is_expanded`,
`is_last_sibling`, `ancestors_last: Vec<bool>`.

#### Scenario: All collapsed shows only roots

- GIVEN a tree with one root and children
- WHEN computed with an empty expanded set
- THEN only the root appears in the output

#### Scenario: Expand root shows direct children

- GIVEN a tree root(0) → [a(1), b(2)]
- WHEN computed with expanded = {0}
- THEN output is [0, 1, 2] with depths [0, 1, 1]

#### Scenario: Nested expand

- GIVEN root(0) → [a(1) → [a1(3), a2(4)], b(2)]
- WHEN computed with expanded = {0, 1}
- THEN output is [0, 1, 3, 4, 2]

#### Scenario: Collapse hides all descendants

- GIVEN the same tree fully expanded
- WHEN node 1 is removed from expanded
- THEN a1 and a2 disappear from visible rows

#### Scenario: Ancestor last-sibling tracking

- GIVEN root(0) → [a(1) → [a1(3)], b(2)], expanded = {0, 1}
- WHEN a1's ancestors_last is inspected
- THEN it is [true, false] — root is last at depth 0, a is not last at depth 1

### Requirement: SimpleTree Adapter

The crate MUST expose a `SimpleTree` struct constructable from
`Vec<(usize, Option<usize>, String)>` that implements `TreeData`.

#### Scenario: Build from flat entries

- GIVEN entries [(0, None, "root"), (1, Some(0), "child")]
- WHEN SimpleTree::new is called
- THEN root_count() is 1, child_count(0) is 1, node_label(1) is "child"
