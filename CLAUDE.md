# CLAUDE.md – fs-lenses

## What is this?

FreeSynergy Lenses — saved search queries that aggregate data from all connected services.

## Rules

- Language in files: **English** (comments, code, variable names)
- Language in chat: **German**
- OOP everywhere: traits over match blocks, types carry their own behavior
- No CHANGELOG.md
- After every feature: commit directly

## Quality Gates (before every commit)

```
1. Design Pattern (Traits, Object hierarchy)
2. Structs + Traits — no impl code yet
3. cargo check
4. Impl (OOP)
5. cargo clippy --all-targets -- -D warnings
6. cargo fmt --check
7. Unit tests (min. 1 per public module)
8. cargo test
9. commit + push
```

Every lib.rs / main.rs must have:
```rust
#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings)]
```

## Architecture

Follows the Provider Pattern (OOP, Dioxus):
- `LensesApp` — root component
- `Lens` / `LensItem` / `LensRole` — domain model (model.rs)
- `LensQueryEngine` — bus query engine (query.rs)

## CSS Variables Prefix

Always `--fs-` (e.g., `--fs-color-primary`, `--fs-font-family`).

## Dependencies

- **fs-libs** (`../fs-libs/`) — `fs-components`, `fs-i18n`
- **fs-desktop** (`../fs-desktop/vendor/dioxus-desktop`) — patched Dioxus desktop
