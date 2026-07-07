// FILE: interface_translates.rs
// occt: Interface_Translates

use std::collections::HashMap;

/// Manages translation of messages
pub struct InterfaceTranslates {
    translations: HashMap<String, String>,
}

impl InterfaceTranslates {
    pub fn new() -> Self {
        InterfaceTranslates {
            translations: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: &str, translation: &str) {
        self.translations.insert(key.to_string(), translation.to_string());
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.translations.get(key).cloned()
    }

    pub fn translate(&self, key: &str) -> String {
        self.translations
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    pub fn count(&self) -> usize {
        self.translations.len()
    }
}

impl Default for InterfaceTranslates {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let trans = InterfaceTranslates::new();
        assert_eq!(trans.count(), 0);
    }

    #[test]
    fn test_add_get() {
        let mut trans = InterfaceTranslates::new();
        trans.add("hello", "bonjour");
        assert_eq!(trans.get("hello"), Some("bonjour".to_string()));
    }

    #[test]
    fn test_translate() {
        let mut trans = InterfaceTranslates::new();
        trans.add("key", "value");
        assert_eq!(trans.translate("key"), "value");
        assert_eq!(trans.translate("unknown"), "unknown");
    }
}
