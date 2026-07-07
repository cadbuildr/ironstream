// FILE: ddf_browser.rs
// occt: DDF_Browser

//! Browses a data framework from TDF.

use std::sync::Arc;
use std::collections::HashMap;

/// TDF_Data placeholder.
#[derive(Clone, Debug)]
pub struct TdfData {
    id: u32,
    labels: HashMap<String, TdfLabel>,
}

impl TdfData {
    pub fn new(id: u32) -> Self {
        TdfData {
            id,
            labels: HashMap::new(),
        }
    }

    pub fn add_label(&mut self, entry: &str) {
        self.labels.insert(entry.to_string(), TdfLabel::new(entry));
    }

    pub fn labels(&self) -> &HashMap<String, TdfLabel> {
        &self.labels
    }
}

/// TDF_Label placeholder.
#[derive(Clone, Debug)]
pub struct TdfLabel {
    entry: String,
}

impl TdfLabel {
    pub fn new(entry: &str) -> Self {
        TdfLabel {
            entry: entry.to_string(),
        }
    }

    pub fn entry(&self) -> &str {
        &self.entry
    }
}

/// Draw_Display placeholder.
#[derive(Clone, Debug)]
pub struct DrawDisplay;

impl DrawDisplay {
    pub fn new() -> Self {
        DrawDisplay
    }
}

impl Default for DrawDisplay {
    fn default() -> Self {
        Self::new()
    }
}

/// Draw_Drawable3D placeholder.
#[derive(Clone, Debug)]
pub struct DrawDrawable3d {
    name: String,
}

impl DrawDrawable3d {
    pub fn new(name: &str) -> Self {
        DrawDrawable3d {
            name: name.to_string(),
        }
    }
}

/// DDF_Browser: browse a TDF data framework.
#[derive(Clone, Debug)]
pub struct DdfBrowser {
    data: Arc<TdfData>,
    name: String,
}

impl DdfBrowser {
    /// Create a new browser for the given data framework.
    pub fn new(data: Arc<TdfData>, name: &str) -> Self {
        DdfBrowser {
            data,
            name: name.to_string(),
        }
    }

    /// Set the data framework.
    pub fn set_data(&mut self, data: Arc<TdfData>) {
        self.data = data;
    }

    /// Get the data framework.
    pub fn data(&self) -> &Arc<TdfData> {
        &self.data
    }

    /// Draw the browser on the display.
    pub fn draw_on(&self, _dis: &DrawDisplay) {
        // In real implementation: render the browser visualization
    }

    /// Create a copy of this browser.
    pub fn copy(&self) -> Self {
        self.clone()
    }

    /// Dump browser information.
    pub fn dump(&self) -> String {
        format!("DDF_Browser: name={}, data_id={}", self.name, self.data.id)
    }

    /// Whatis command for the browser.
    pub fn whatis(&self) -> String {
        format!("This is a DDF_Browser browsing framework {}", self.data.id)
    }

    /// Open root entries of the framework.
    pub fn open_root(&self) -> String {
        let mut entries = Vec::new();
        for (entry, _) in self.data.labels() {
            if entry.starts_with("0:") {
                entries.push(entry.clone());
            }
        }
        entries.sort();
        entries.join(" ")
    }

    /// Open sub-label entries of a label.
    pub fn open_label(&self, label: &TdfLabel) -> String {
        let parent_entry = label.entry();
        let mut entries = Vec::new();

        for (entry, _) in self.data.labels() {
            if entry.starts_with(parent_entry) && entry != parent_entry {
                entries.push(entry.clone());
            }
        }
        entries.sort();
        entries.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_creation() {
        let data = Arc::new(TdfData::new(1));
        let browser = DdfBrowser::new(data, "test_browser");

        assert_eq!(browser.name, "test_browser");
        assert_eq!(browser.data.id, 1);
    }

    #[test]
    fn test_set_data() {
        let data1 = Arc::new(TdfData::new(1));
        let mut browser = DdfBrowser::new(data1, "browser");

        let data2 = Arc::new(TdfData::new(2));
        browser.set_data(data2);

        assert_eq!(browser.data.id, 2);
    }

    #[test]
    fn test_dump() {
        let data = Arc::new(TdfData::new(42));
        let browser = DdfBrowser::new(data, "mybrowser");

        let dump = browser.dump();
        assert!(dump.contains("mybrowser"));
        assert!(dump.contains("42"));
    }

    #[test]
    fn test_whatis() {
        let data = Arc::new(TdfData::new(99));
        let browser = DdfBrowser::new(data, "test");

        let whatis = browser.whatis();
        assert!(whatis.contains("DDF_Browser"));
        assert!(whatis.contains("99"));
    }

    #[test]
    fn test_open_root() {
        let mut data = TdfData::new(1);
        data.add_label("0:1");
        data.add_label("0:2");
        data.add_label("1:1");

        let browser = DdfBrowser::new(Arc::new(data), "browser");
        let root = browser.open_root();

        assert!(root.contains("0:1"));
        assert!(root.contains("0:2"));
        assert!(!root.contains("1:1"));
    }

    #[test]
    fn test_open_label() {
        let mut data = TdfData::new(1);
        data.add_label("0:1");
        data.add_label("0:1:1");
        data.add_label("0:1:2");
        data.add_label("0:2");

        let browser = DdfBrowser::new(Arc::new(data), "browser");
        let label = TdfLabel::new("0:1");
        let children = browser.open_label(&label);

        assert!(children.contains("0:1:1") || children.contains("0:1:2"));
    }

    #[test]
    fn test_copy() {
        let data = Arc::new(TdfData::new(5));
        let browser = DdfBrowser::new(data, "original");

        let copied = browser.copy();
        assert_eq!(copied.name, "original");
        assert_eq!(copied.data.id, 5);
    }

    #[test]
    fn test_tdf_data_creation() {
        let data = TdfData::new(1);
        assert_eq!(data.id, 1);
    }

    #[test]
    fn test_tdf_label_creation() {
        let label = TdfLabel::new("0:1:2");
        assert_eq!(label.entry(), "0:1:2");
    }

    #[test]
    fn test_draw_display() {
        let _dis = DrawDisplay::new();
        // Just verify it can be created
    }
}
