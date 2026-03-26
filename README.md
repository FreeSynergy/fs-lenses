# fs-lenses

FreeSynergy Lenses — aggregated cross-service data views.

Part of the [FreeSynergy](https://github.com/FreeSynergy) platform.

## Purpose

Lenses let users define saved search queries that aggregate results from all
connected services (Wiki, Chat, Git, Tasks, IAM, ...) in one view. Results are
grouped by service role and can be opened directly in the embedded browser.

## Architecture

Follows the Provider Pattern (OOP, Dioxus):

- `LensesApp` — root Dioxus component
- `Lens` / `LensItem` / `LensRole` — domain model (model.rs)
- `LensQueryEngine` — queries the fs-bus for data; falls back to demo items

## Build

```bash
cargo build                    # default: desktop feature
cargo build --features web     # web target
```

## Dependencies

- **fs-libs** (`../fs-libs/`) — `fs-components`, `fs-i18n`
- **fs-desktop** (`../fs-desktop/vendor/dioxus-desktop`) — patched Dioxus desktop
