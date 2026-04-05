// search.rs — SearchStrategy trait (Strategy Pattern).
//
// Design Pattern: Strategy — swap search backends without changing LensQueryEngine
//                 or any other caller.
//
//   SearchStrategy (trait)
//     ├── DemoSearchStrategy   — deterministic demo items; used until bus is wired
//     └── BusSearchStrategy    — publishes to fs-bus and collects responses (G2)

use crate::model::{LensItem, LensRole};

// ── SearchStrategy (Strategy) ─────────────────────────────────────────────────

/// Pluggable search backend for lens queries.
///
/// Implementors receive a free-text query and return a list of
/// [`LensItem`]s from whatever sources they know about.
pub trait SearchStrategy: Send + Sync {
    /// Run a search and return matching items.
    fn search(&self, query: &str) -> Vec<LensItem>;
}

// ── DemoSearchStrategy ────────────────────────────────────────────────────────

/// Deterministic demo strategy — returns one item per known role.
///
/// Used as the default until `fs-bus` gRPC routing is wired in G2.
pub struct DemoSearchStrategy;

impl SearchStrategy for DemoSearchStrategy {
    fn search(&self, query: &str) -> Vec<LensItem> {
        vec![
            LensItem {
                role: LensRole::Wiki,
                summary: format!("Wiki: results for '{query}'"),
                link: Some(format!("http://wiki.local/search?q={query}")),
                source: "outline-wiki".into(),
            },
            LensItem {
                role: LensRole::Chat,
                summary: format!("Chat: 3 messages mentioning '{query}'"),
                link: Some("http://chat.local/search".into()),
                source: "matrix".into(),
            },
            LensItem {
                role: LensRole::Git,
                summary: format!("Git: 2 repositories matching '{query}'"),
                link: Some("http://git.local/explore".into()),
                source: "forgejo".into(),
            },
            LensItem {
                role: LensRole::Tasks,
                summary: format!("Tasks: 5 open tasks for '{query}'"),
                link: Some("http://tasks.local/".into()),
                source: "vikunja".into(),
            },
        ]
    }
}

// ── BusSearchStrategy ─────────────────────────────────────────────────────────

/// Bus-backed search strategy (G2 — not yet wired).
///
/// When `fs-bus` gRPC routing is available, this strategy publishes a
/// `search.query` event and collects `search.result` responses from all
/// services that subscribe to it.
///
/// Until then it returns an empty result set so callers can already compile
/// against it and switch strategies at runtime without code changes.
pub struct BusSearchStrategy;

impl SearchStrategy for BusSearchStrategy {
    fn search(&self, _query: &str) -> Vec<LensItem> {
        // Bus routing not yet connected — returns empty until G2.
        vec![]
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_returns_items_for_any_query() {
        let s = DemoSearchStrategy;
        let items = s.search("rust");
        assert!(!items.is_empty());
    }

    #[test]
    fn demo_includes_all_roles() {
        let s = DemoSearchStrategy;
        let items = s.search("test");
        let roles: Vec<_> = items.iter().map(|i| i.role.id()).collect();
        assert!(roles.contains(&"wiki".to_string()));
        assert!(roles.contains(&"chat".to_string()));
        assert!(roles.contains(&"git".to_string()));
        assert!(roles.contains(&"tasks".to_string()));
    }

    #[test]
    fn demo_embeds_query_in_summaries() {
        let s = DemoSearchStrategy;
        let items = s.search("freeSynergy");
        assert!(items.iter().any(|i| i.summary.contains("freeSynergy")));
    }

    #[test]
    fn bus_strategy_returns_empty() {
        let s = BusSearchStrategy;
        assert!(s.search("anything").is_empty());
    }
}
