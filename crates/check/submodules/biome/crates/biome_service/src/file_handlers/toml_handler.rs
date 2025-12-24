use super::{Capabilities, EnabledForPath, ExtensionHandler};

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct TomlFileHandler {}

impl ExtensionHandler for TomlFileHandler {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            enabled_for_path: EnabledForPath {
                formatter: Some(|_path, _settings| true), // TOML formatting is always enabled
                linter: Some(|_path, _settings| true),    // TOML linting is always enabled  
                assist: None,
                search: None,
            },
            ..Default::default()
        }
    }
}
