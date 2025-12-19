//! Plugin builder.

/// Plugin builder for registering hooks.
pub struct PluginBuilder {
    _on_load_handlers: Vec<()>,
    _on_resolve_handlers: Vec<()>,
}

impl PluginBuilder {
    /// Create a new plugin builder.
    pub fn new() -> Self {
        Self {
            _on_load_handlers: Vec::new(),
            _on_resolve_handlers: Vec::new(),
        }
    }

    /// Register a load handler.
    pub fn on_load<F>(&mut self, _filter: &str, _namespace: Option<&str>, _handler: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        // TODO: Implement
    }

    /// Register a resolve handler.
    pub fn on_resolve<F>(&mut self, _filter: &str, _namespace: Option<&str>, _handler: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        // TODO: Implement
    }
}

impl Default for PluginBuilder {
    fn default() -> Self {
        Self::new()
    }
}
