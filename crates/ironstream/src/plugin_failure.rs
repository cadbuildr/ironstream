// FILE: plugin_failure.rs
// occt: Plugin_Failure

/// Exception class for plugin loading failures
#[derive(Debug, Clone)]
pub struct PluginFailure {
    message: String,
}

impl PluginFailure {
    /// Creates a new PluginFailure exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        PluginFailure {
            message: message.into(),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for PluginFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plugin_Failure: {}", self.message)
    }
}

impl std::error::Error for PluginFailure {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_failure_new() {
        let err = PluginFailure::new("plugin not found");
        assert_eq!(err.message(), "plugin not found");
    }

    #[test]
    fn test_plugin_failure_display() {
        let err = PluginFailure::new("load error");
        assert_eq!(err.to_string(), "Plugin_Failure: load error");
    }
}
