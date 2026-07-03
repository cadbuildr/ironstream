// FILE: plugin_macro.rs
// occt: Plugin_Macro

//! Macro for implementing C-style interface to plugin factories.
//! In Rust, this is represented as a module documentation.

/// This module provides the Plugin macro which implements C-style interface
/// function to get factory object from dynamically loaded library.
///
/// In C++:
/// ```
/// #define PLUGIN(name) \
///   extern "C" Standard_EXPORT Standard_Transient* PLUGINFACTORY(const Standard_GUID& aGUID) \
///   { \
///     return const_cast<Standard_Transient*>(name::Factory(aGUID).get()); \
///   }
/// ```
///
/// In Rust, plugins are loaded through the Plugin module directly.

/// Represents a plugin factory function
pub type PluginFactory = fn(&[u8; 16]) -> Option<String>;

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_factory(_guid: &[u8; 16]) -> Option<String> {
        Some("dummy".to_string())
    }

    #[test]
    fn test_plugin_factory() {
        let factory: PluginFactory = dummy_factory;
        let guid = [0u8; 16];
        let result = factory(&guid);
        assert_eq!(result, Some("dummy".to_string()));
    }
}
