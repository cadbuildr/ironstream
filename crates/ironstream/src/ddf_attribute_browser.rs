// FILE: ddf_attribute_browser.rs
// occt: DDF_AttributeBrowser

//! Browser for TDF attributes with pluggable test, open, and text functions.

use std::sync::Mutex;

/// A TDF attribute placeholder.
#[derive(Clone, Debug)]
pub struct TdfAttribute {
    id: u32,
    data: String,
}

impl TdfAttribute {
    pub fn new(id: u32, data: &str) -> Self {
        TdfAttribute {
            id,
            data: data.to_string(),
        }
    }
}

/// Function pointer types for attribute browsing.
pub type TestFn = fn(&TdfAttribute) -> bool;
pub type OpenFn = fn(&TdfAttribute) -> String;
pub type TextFn = fn(&TdfAttribute) -> String;

/// Global browser chain.
static BROWSER_CHAIN: Mutex<Vec<Box<DdfAttributeBrowser>>> = Mutex::new(Vec::new());

/// DDF_AttributeBrowser: attribute browser with pluggable functions.
pub struct DdfAttributeBrowser {
    test: TestFn,
    open: OpenFn,
    text: TextFn,
}

impl DdfAttributeBrowser {
    /// Create a new attribute browser with test, open, and text functions.
    pub fn new(test: TestFn, open: OpenFn, text: TextFn) -> Self {
        DdfAttributeBrowser { test, open, text }
    }

    /// Test if this browser matches an attribute.
    pub fn test(&self, attr: &TdfAttribute) -> bool {
        (self.test)(attr)
    }

    /// Get the "open" string representation of an attribute.
    pub fn open(&self, attr: &TdfAttribute) -> String {
        (self.open)(attr)
    }

    /// Get the "text" string representation of an attribute.
    pub fn text(&self, attr: &TdfAttribute) -> String {
        (self.text)(attr)
    }

    /// Register this browser in the global chain.
    pub fn register(self) {
        let mut chain = BROWSER_CHAIN.lock().unwrap();
        chain.push(Box::new(self));
    }

    /// Find a browser that matches the given attribute.
    pub fn find_browser(attr: &TdfAttribute) -> Option<String> {
        let chain = BROWSER_CHAIN.lock().unwrap();

        for browser in chain.iter() {
            if browser.test(attr) {
                return Some(browser.open(attr));
            }
        }

        None
    }

    /// Clear all registered browsers.
    pub fn clear_all() {
        BROWSER_CHAIN.lock().unwrap().clear();
    }

    /// Get the number of registered browsers.
    pub fn browser_count() -> usize {
        BROWSER_CHAIN.lock().unwrap().len()
    }
}

/// A simpler implementation using closures.
pub struct SimpleBrowser {
    test: Box<dyn Fn(&TdfAttribute) -> bool>,
    open: Box<dyn Fn(&TdfAttribute) -> String>,
    text: Box<dyn Fn(&TdfAttribute) -> String>,
}

impl SimpleBrowser {
    pub fn new<T, O, Tx>(test: T, open: O, text: Tx) -> Self
    where
        T: Fn(&TdfAttribute) -> bool + 'static,
        O: Fn(&TdfAttribute) -> String + 'static,
        Tx: Fn(&TdfAttribute) -> String + 'static,
    {
        SimpleBrowser {
            test: Box::new(test),
            open: Box::new(open),
            text: Box::new(text),
        }
    }

    pub fn test(&self, attr: &TdfAttribute) -> bool {
        (self.test)(attr)
    }

    pub fn open(&self, attr: &TdfAttribute) -> String {
        (self.open)(attr)
    }

    pub fn text(&self, attr: &TdfAttribute) -> String {
        (self.text)(attr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_fn_always_true(attr: &TdfAttribute) -> bool {
        let _ = attr;
        true
    }

    fn test_fn_by_id(attr: &TdfAttribute) -> bool {
        attr.id == 42
    }

    fn open_fn_default(attr: &TdfAttribute) -> String {
        format!("Attribute {}", attr.id)
    }

    fn text_fn_default(attr: &TdfAttribute) -> String {
        format!("Data: {}", attr.data)
    }

    #[test]
    fn test_browser_creation() {
        let browser = DdfAttributeBrowser::new(test_fn_always_true, open_fn_default, text_fn_default);
        let attr = TdfAttribute::new(1, "test");

        assert!(browser.test(&attr));
        assert_eq!(browser.open(&attr), "Attribute 1");
        assert_eq!(browser.text(&attr), "Data: test");
    }

    #[test]
    fn test_browser_with_id_filter() {
        let browser = DdfAttributeBrowser::new(test_fn_by_id, open_fn_default, text_fn_default);

        let attr1 = TdfAttribute::new(42, "match");
        let attr2 = TdfAttribute::new(1, "no_match");

        assert!(browser.test(&attr1));
        assert!(!browser.test(&attr2));
    }

    #[test]
    fn test_register_and_find_browser() {
        DdfAttributeBrowser::clear_all();

        let browser1 =
            DdfAttributeBrowser::new(test_fn_always_true, open_fn_default, text_fn_default);
        browser1.register();

        assert_eq!(DdfAttributeBrowser::browser_count(), 1);

        let attr = TdfAttribute::new(1, "test");
        let result = DdfAttributeBrowser::find_browser(&attr);
        assert!(result.is_some());
    }

    #[test]
    fn test_multiple_browsers() {
        DdfAttributeBrowser::clear_all();

        let browser1 =
            DdfAttributeBrowser::new(test_fn_by_id, open_fn_default, text_fn_default);
        browser1.register();

        let browser2 =
            DdfAttributeBrowser::new(test_fn_always_true, open_fn_default, text_fn_default);
        browser2.register();

        assert_eq!(DdfAttributeBrowser::browser_count(), 2);
    }

    #[test]
    fn test_simple_browser_with_closures() {
        let browser = SimpleBrowser::new(
            |attr| attr.id > 5,
            |attr| format!("ID: {}", attr.id),
            |attr| format!("Data: {}", attr.data),
        );

        let attr = TdfAttribute::new(10, "hello");
        assert!(browser.test(&attr));
        assert_eq!(browser.open(&attr), "ID: 10");
        assert_eq!(browser.text(&attr), "Data: hello");
    }

    #[test]
    fn test_clear_browsers() {
        DdfAttributeBrowser::clear_all();

        let browser =
            DdfAttributeBrowser::new(test_fn_always_true, open_fn_default, text_fn_default);
        browser.register();
        assert_eq!(DdfAttributeBrowser::browser_count(), 1);

        DdfAttributeBrowser::clear_all();
        assert_eq!(DdfAttributeBrowser::browser_count(), 0);
    }

    #[test]
    fn test_tdf_attribute_creation() {
        let attr = TdfAttribute::new(99, "custom_data");
        assert_eq!(attr.id, 99);
        assert_eq!(attr.data, "custom_data");
    }
}
