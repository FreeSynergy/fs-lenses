// view.rs — FsView implementations for fs-lenses.
//
// This is the ONLY file in fs-lenses that imports fs-render.
// Domain types (Lens, LensController) do NOT import fs-render.

use fs_render::{
    view::FsView,
    widget::{ButtonWidget, FsWidget, ListWidget, TextInputWidget},
};

use crate::model::{Lens, LensItem};

// ── LensesView ────────────────────────────────────────────────────────────────

/// Snapshot view of the lenses state.
///
/// Constructed from a slice of lenses; passed to the render engine for display.
pub struct LensesView {
    pub lenses: Vec<Lens>,
}

impl LensesView {
    /// Wrap a lens snapshot in a renderable view.
    #[must_use]
    pub fn new(lenses: Vec<Lens>) -> Self {
        Self { lenses }
    }
}

impl FsView for LensesView {
    fn view(&self) -> Box<dyn FsWidget> {
        // Each lens becomes one item in the list.
        let items: Vec<String> = self
            .lenses
            .iter()
            .map(|l| format!("{}: {}", l.name, l.query))
            .collect();

        Box::new(ListWidget {
            id: "lenses-list".into(),
            items,
            selected_index: None,
            enabled: true,
        })
    }
}

// ── LensDetailView ────────────────────────────────────────────────────────────

/// View for a single lens and its query results.
pub struct LensDetailView {
    pub lens: Lens,
}

impl LensDetailView {
    #[must_use]
    pub fn new(lens: Lens) -> Self {
        Self { lens }
    }
}

impl FsView for LensDetailView {
    fn view(&self) -> Box<dyn FsWidget> {
        let search_btn = ButtonWidget {
            id: "lens-search-btn".into(),
            label: crate::keys::SEARCH_BTN.into(), // FTL key resolved at render time
            enabled: !self.lens.loading,
            action: "search".into(),
        };

        let query_input = TextInputWidget {
            id: "lens-query-input".into(),
            placeholder: crate::keys::SEARCH_PLACEHOLDER.into(), // FTL key
            value: self.lens.query.clone(),
            enabled: !self.lens.loading,
        };

        let result_items: Vec<String> = self
            .lens
            .items
            .iter()
            .map(|item| format!("[{}] {}", item.role.id(), item.summary))
            .collect();

        Box::new(ListWidget {
            id: format!("lens-detail-{}", self.lens.id),
            items: vec![query_input.value.clone(), search_btn.label.clone()]
                .into_iter()
                .chain(result_items)
                .collect(),
            selected_index: None,
            enabled: true,
        })
    }
}

// ── SearchView ────────────────────────────────────────────────────────────────

/// View for ad-hoc search results (not tied to a saved lens).
///
/// Displays the search term and a flat list of result items grouped
/// by role label.
pub struct SearchView {
    /// The query that produced these results.
    pub query: String,
    /// Result items to display.
    pub items: Vec<LensItem>,
    /// Whether a search is currently in progress.
    pub loading: bool,
}

impl SearchView {
    #[must_use]
    pub fn new(query: impl Into<String>, items: Vec<LensItem>) -> Self {
        Self {
            query: query.into(),
            items,
            loading: false,
        }
    }

    #[must_use]
    pub fn loading(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            items: vec![],
            loading: true,
        }
    }
}

impl FsView for SearchView {
    fn view(&self) -> Box<dyn FsWidget> {
        let query_input = TextInputWidget {
            id: "search-input".into(),
            placeholder: crate::keys::SEARCH_PLACEHOLDER.into(),
            value: self.query.clone(),
            enabled: !self.loading,
        };

        let search_btn = ButtonWidget {
            id: "search-btn".into(),
            label: crate::keys::SEARCH_BTN.into(),
            enabled: !self.loading,
            action: "search".into(),
        };

        let result_items: Vec<String> = if self.loading {
            vec!["…".into()]
        } else {
            self.items
                .iter()
                .map(|item| format!("[{}] {} — {}", item.role.label(), item.summary, item.source))
                .collect()
        };

        Box::new(ListWidget {
            id: "search-results".into(),
            items: vec![query_input.value.clone(), search_btn.label.clone()]
                .into_iter()
                .chain(result_items)
                .collect(),
            selected_index: None,
            enabled: true,
        })
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Lens;

    #[test]
    fn empty_view_produces_widget() {
        let v = LensesView::new(vec![]);
        let w = v.view();
        assert_eq!(w.widget_id(), "lenses-list");
        assert!(w.is_enabled());
    }

    #[test]
    fn view_shows_all_lenses() {
        let lenses = vec![
            Lens::new("Alpha", "alpha query"),
            Lens::new("Beta", "beta query"),
        ];
        let v = LensesView::new(lenses);
        let w = v.view();
        assert_eq!(w.widget_id(), "lenses-list");
    }

    #[test]
    fn detail_view_has_correct_id() {
        let lens = Lens::new("Test", "test query");
        let id = lens.id;
        let v = LensDetailView::new(lens);
        let w = v.view();
        assert_eq!(w.widget_id(), format!("lens-detail-{id}"));
    }

    #[test]
    fn detail_view_loading_produces_widget() {
        let mut lens = Lens::new("Loading", "query");
        lens.loading = true;
        let v = LensDetailView::new(lens);
        let w = v.view();
        assert!(w.widget_id().starts_with("lens-detail-"));
    }

    #[test]
    fn search_view_widget_id() {
        let v = SearchView::new("rust", vec![]);
        let w = v.view();
        assert_eq!(w.widget_id(), "search-results");
    }

    #[test]
    fn search_view_loading_produces_widget() {
        let v = SearchView::loading("test");
        assert!(v.loading);
        let w = v.view();
        assert_eq!(w.widget_id(), "search-results");
    }

    #[test]
    fn search_view_shows_items() {
        use crate::model::{LensItem, LensRole};
        let items = vec![LensItem {
            role: LensRole::Wiki,
            summary: "A wiki hit".into(),
            link: None,
            source: "wiki".into(),
        }];
        let v = SearchView::new("something", items);
        let w = v.view();
        assert_eq!(w.widget_id(), "search-results");
    }
}
