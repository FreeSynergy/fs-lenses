// query.rs — LensQueryEngine: runs a search strategy for a given lens.
//
// Design Pattern: Strategy (composition) — LensQueryEngine holds a
//                 `SearchStrategy` and delegates the actual search to it.
//
// Real bus integration will be added in G2 by swapping to BusSearchStrategy.

use std::sync::Arc;

use crate::model::{Lens, LensItem};
use crate::search::{DemoSearchStrategy, SearchStrategy};

// ── LensQueryEngine ───────────────────────────────────────────────────────────

/// Executes searches for saved lenses using a pluggable [`SearchStrategy`].
pub struct LensQueryEngine {
    strategy: Arc<dyn SearchStrategy>,
}

impl LensQueryEngine {
    /// Create an engine backed by the default demo strategy.
    #[must_use]
    pub fn new() -> Self {
        Self {
            strategy: Arc::new(DemoSearchStrategy),
        }
    }

    /// Create an engine with an explicit search strategy.
    #[must_use]
    pub fn with_strategy(strategy: impl SearchStrategy + 'static) -> Self {
        Self {
            strategy: Arc::new(strategy),
        }
    }

    /// Run the lens's saved query through the search strategy.
    pub fn refresh_lens(&self, lens: &Lens) -> Vec<LensItem> {
        self.strategy.search(&lens.query)
    }
}

impl Default for LensQueryEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Lens;

    #[test]
    fn refresh_returns_demo_items() {
        let engine = LensQueryEngine::new();
        let lens = Lens::new("test", "rust");
        let items = engine.refresh_lens(&lens);
        assert!(!items.is_empty());
    }

    #[test]
    fn demo_items_include_all_roles() {
        let engine = LensQueryEngine::new();
        let lens = Lens::new("test", "alpha");
        let items = engine.refresh_lens(&lens);
        let roles: Vec<_> = items.iter().map(|i| i.role.id()).collect();
        assert!(roles.contains(&"wiki".to_string()));
        assert!(roles.contains(&"chat".to_string()));
        assert!(roles.contains(&"git".to_string()));
        assert!(roles.contains(&"tasks".to_string()));
    }
}
