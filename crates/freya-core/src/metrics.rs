use crate::{
    runner::Runner,
    tree::Tree,
};

/// Current metrics collected from a Freya runner and tree.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Metrics {
    pub alive_tasks: usize,
    pub scopes: usize,
    pub scope_values: usize,
    pub contexts: usize,
    pub reactive_contexts: usize,
    pub tree_nodes: usize,
    pub layout_nodes: usize,
    pub text_cache_size: usize,
    pub text_cache_users: usize,
    pub cached_assets: usize,
    pub resource_cache: Option<(usize, usize)>,
}

impl Metrics {
    pub fn new(
        runner: &Runner,
        tree: &Tree,
        cached_assets: usize,
        resource_cache: Option<(usize, usize)>,
    ) -> Self {
        let scope_storages = runner.scopes_storages.borrow();
        Self {
            alive_tasks: runner.tasks.borrow().len(),
            scopes: runner.scopes.len(),
            scope_values: scope_storages
                .values()
                .map(|storage| storage.values.len())
                .sum(),
            contexts: scope_storages
                .values()
                .map(|storage| storage.contexts.len())
                .sum(),
            reactive_contexts: scope_storages.len(),
            tree_nodes: tree.elements.len(),
            layout_nodes: tree.layout.size(),
            text_cache_size: tree.text_cache.len(),
            text_cache_users: tree.text_cache.users_len(),
            cached_assets,
            resource_cache,
        }
    }
}
