// registry.rs — LensRegistry trait (Strategy Pattern).
//
// Design Pattern: Strategy — swap storage backends without touching LensController.
//
//   LensRegistry (trait)
//     └── InMemoryLensRegistry   — ephemeral; default + tests
//     └── SqliteLensRegistry     — durable; wired when fs-db is ready (G2)

use crate::model::{Lens, LensItem};

// ── LensRegistry (Strategy) ───────────────────────────────────────────────────

/// Pluggable storage strategy for saved lenses.
///
/// Implement this trait to add a new persistence backend.  The default backend
/// is [`InMemoryLensRegistry`]; a durable SQLite backend will be added in G2
/// once the `fs-db` `DbEngine` trait is stable.
pub trait LensRegistry: Send + Sync {
    /// List all saved lenses.
    fn list(&self) -> Vec<Lens>;

    /// Retrieve a single lens by its ID.
    fn get(&self, id: i64) -> Option<Lens>;

    /// Persist a new lens and return it (with its assigned ID).
    fn create(&self, name: String, query: String) -> Lens;

    /// Remove a lens by ID.  Returns `true` if the lens existed.
    fn delete(&self, id: i64) -> bool;

    /// Replace the cached items on an existing lens and stamp `last_refreshed`.
    /// Returns `true` if the lens was found and updated.
    fn update_items(&self, id: i64, items: Vec<LensItem>) -> bool;
}

// ── InMemoryLensRegistry ──────────────────────────────────────────────────────

/// Ephemeral in-memory registry backed by a `Mutex<Vec<Lens>>`.
///
/// State is lost on restart — use this for tests and as the default
/// until the SQLite backend is wired.
#[derive(Default)]
pub struct InMemoryLensRegistry {
    lenses: std::sync::Mutex<Vec<Lens>>,
}

impl InMemoryLensRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl LensRegistry for InMemoryLensRegistry {
    fn list(&self) -> Vec<Lens> {
        self.lenses.lock().unwrap().clone()
    }

    fn get(&self, id: i64) -> Option<Lens> {
        self.lenses
            .lock()
            .unwrap()
            .iter()
            .find(|l| l.id == id)
            .cloned()
    }

    fn create(&self, name: String, query: String) -> Lens {
        let lens = Lens::new(name, query);
        self.lenses.lock().unwrap().push(lens.clone());
        lens
    }

    fn delete(&self, id: i64) -> bool {
        let mut guard = self.lenses.lock().unwrap();
        let before = guard.len();
        guard.retain(|l| l.id != id);
        guard.len() < before
    }

    fn update_items(&self, id: i64, items: Vec<LensItem>) -> bool {
        let mut guard = self.lenses.lock().unwrap();
        if let Some(lens) = guard.iter_mut().find(|l| l.id == id) {
            lens.items = items;
            lens.last_refreshed = Some(chrono::Utc::now().to_rfc3339());
            true
        } else {
            false
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{LensItem, LensRole};

    fn reg() -> InMemoryLensRegistry {
        InMemoryLensRegistry::new()
    }

    #[test]
    fn create_and_list() {
        let r = reg();
        let lens = r.create("Alpha".into(), "q".into());
        let list = r.list();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, lens.id);
    }

    #[test]
    fn get_existing() {
        let r = reg();
        let lens = r.create("Beta".into(), "q".into());
        assert!(r.get(lens.id).is_some());
    }

    #[test]
    fn get_missing_returns_none() {
        let r = reg();
        assert!(r.get(999).is_none());
    }

    #[test]
    fn delete_existing() {
        let r = reg();
        let lens = r.create("Gamma".into(), "q".into());
        assert!(r.delete(lens.id));
        assert!(r.list().is_empty());
    }

    #[test]
    fn delete_missing_returns_false() {
        let r = reg();
        assert!(!r.delete(999));
    }

    #[test]
    fn update_items_stamps_refresh() {
        let r = reg();
        let lens = r.create("Delta".into(), "q".into());
        let items = vec![LensItem {
            role: LensRole::Wiki,
            summary: "hit".into(),
            link: None,
            source: "wiki".into(),
        }];
        assert!(r.update_items(lens.id, items));
        let updated = r.get(lens.id).unwrap();
        assert_eq!(updated.items.len(), 1);
        assert!(updated.last_refreshed.is_some());
    }

    #[test]
    fn update_items_missing_returns_false() {
        let r = reg();
        assert!(!r.update_items(999, vec![]));
    }
}
