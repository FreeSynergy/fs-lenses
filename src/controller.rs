// controller.rs — LensController: domain logic via LensRegistry strategy.
//
// Design Pattern: Strategy — LensController holds a `dyn LensRegistry`;
//                 swap the registry to change persistence without changing
//                 any other layer.

use std::sync::Arc;

use crate::model::{Lens, LensItem};
use crate::query::LensQueryEngine;
use crate::registry::{InMemoryLensRegistry, LensRegistry};

// ── LensController ────────────────────────────────────────────────────────────

/// Shared, cheaply-clonable controller for lens operations.
///
/// Owns a [`LensRegistry`] strategy and a [`LensQueryEngine`].
/// All callers (gRPC, REST, CLI, UI) share the same `Arc`-wrapped controller.
#[derive(Clone)]
pub struct LensController {
    registry: Arc<dyn LensRegistry>,
    engine: Arc<LensQueryEngine>,
}

impl Default for LensController {
    fn default() -> Self {
        Self::new()
    }
}

impl LensController {
    /// Create a controller backed by the default in-memory registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            registry: Arc::new(InMemoryLensRegistry::new()),
            engine: Arc::new(LensQueryEngine::new()),
        }
    }

    /// Create a controller with an explicit registry (e.g. for tests or
    /// when wiring a SQLite backend in G2).
    #[must_use]
    pub fn with_registry(registry: impl LensRegistry + 'static) -> Self {
        Self {
            registry: Arc::new(registry),
            engine: Arc::new(LensQueryEngine::new()),
        }
    }

    /// List all saved lenses.
    pub fn list(&self) -> Vec<Lens> {
        self.registry.list()
    }

    /// Retrieve a single lens by ID.
    pub fn get(&self, id: i64) -> Option<Lens> {
        self.registry.get(id)
    }

    /// Create and persist a new lens.
    pub fn create(&self, name: String, query: String) -> Lens {
        self.registry.create(name, query)
    }

    /// Delete a lens.  Returns `true` if it existed.
    pub fn delete(&self, id: i64) -> bool {
        self.registry.delete(id)
    }

    /// Refresh a lens: run the query engine and update cached items.
    /// Returns the new items, or an empty vec if the lens was not found.
    pub async fn refresh(&self, lens_id: i64) -> Vec<LensItem> {
        if let Some(lens) = self.registry.get(lens_id) {
            let items = self.engine.refresh_lens(&lens).await;
            self.registry.update_items(lens_id, items.clone());
            items
        } else {
            vec![]
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_list() {
        let ctrl = LensController::new();
        ctrl.create("Test".into(), "q".into());
        assert_eq!(ctrl.list().len(), 1);
    }

    #[test]
    fn get_existing() {
        let ctrl = LensController::new();
        let lens = ctrl.create("A".into(), "q".into());
        assert!(ctrl.get(lens.id).is_some());
    }

    #[test]
    fn delete_existing() {
        let ctrl = LensController::new();
        let lens = ctrl.create("B".into(), "q".into());
        assert!(ctrl.delete(lens.id));
        assert!(ctrl.list().is_empty());
    }

    #[tokio::test]
    async fn refresh_populates_items() {
        let ctrl = LensController::new();
        let lens = ctrl.create("C".into(), "rust".into());
        let items = ctrl.refresh(lens.id).await;
        assert!(!items.is_empty());
        // Items are also stored in the registry
        let updated = ctrl.get(lens.id).unwrap();
        assert!(!updated.items.is_empty());
        assert!(updated.last_refreshed.is_some());
    }

    #[tokio::test]
    async fn refresh_unknown_lens_returns_empty() {
        let ctrl = LensController::new();
        assert!(ctrl.refresh(999).await.is_empty());
    }
}
