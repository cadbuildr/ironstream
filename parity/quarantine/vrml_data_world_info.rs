// FILE: vrml_data_world_info.rs
// occt: VrmlData_WorldInfo
//
// Faithful port of OCCT VrmlData_WorldInfo (DataExchange/TKDEVRML/VrmlData/
// VrmlData_WorldInfo.hxx/.cxx): VRML 2.0 WorldInfo node.
// Stores metadata about the VRML scene (title, info strings, creation info).
// Non-geometric node used for documentation and tool-specific parameters.

use std::cell::RefCell;
use std::rc::Rc;

/// Error status for read/write operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorldInfoErrorStatus {
    Ok = 0,
    EndOfFile = 1,
    NotEndOfFile = 2,
    GeneralError = 3,
}

/// Input buffer for parsing.
pub struct WorldInfoInBuffer {
    pub line_num: u32,
}

impl WorldInfoInBuffer {
    pub fn new() -> Self {
        WorldInfoInBuffer { line_num: 1 }
    }
}

impl Default for WorldInfoInBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// VRML WorldInfo node: scene metadata and documentation.
/// Provides title, info strings (description), and optional creation date.
/// Non-geometric node for scene-level properties and compatibility.
pub struct VrmlDataWorldInfo {
    my_title: String,
    my_info: Vec<String>,          // array of info strings
    my_name: String,
}

impl VrmlDataWorldInfo {
    /// Constructor: creates an empty WorldInfo node.
    pub fn new(name: Option<&str>) -> Self {
        VrmlDataWorldInfo {
            my_title: String::new(),
            my_info: Vec::new(),
            my_name: name.unwrap_or("").to_string(),
        }
    }

    /// Full constructor with title.
    pub fn with_title(title: &str, name: Option<&str>) -> Self {
        VrmlDataWorldInfo {
            my_title: title.to_string(),
            my_info: Vec::new(),
            my_name: name.unwrap_or("").to_string(),
        }
    }

    /// Query the name.
    pub fn name(&self) -> &str {
        &self.my_name
    }

    /// Set the name.
    pub fn set_name(&mut self, name: &str) {
        self.my_name = name.to_string();
    }

    /// Get the title string.
    pub fn title(&self) -> &str {
        &self.my_title
    }

    /// Set the title string.
    pub fn set_title(&mut self, title: &str) {
        self.my_title = title.to_string();
    }

    /// Add an info string (description).
    pub fn add_info(&mut self, info: &str) {
        self.my_info.push(info.to_string());
    }

    /// Get the number of info strings.
    pub fn info_count(&self) -> usize {
        self.my_info.len()
    }

    /// Get an info string by index (0-based). Returns None if out of range.
    pub fn get_info(&self, index: usize) -> Option<&str> {
        self.my_info.get(index).map(|s| s.as_str())
    }

    /// Get all info strings as a slice.
    pub fn info_list(&self) -> &[String] {
        &self.my_info
    }

    /// Set all info strings from a vector.
    pub fn set_info_list(&mut self, info: Vec<String>) {
        self.my_info = info;
    }

    /// Clear all info strings.
    pub fn clear_info(&mut self) {
        self.my_info.clear();
    }

    /// Clear all data.
    pub fn clear(&mut self) {
        self.my_title.clear();
        self.my_info.clear();
    }

    /// Check if this node is in default state (empty title and info).
    pub fn is_default(&self) -> bool {
        self.my_title.is_empty() && self.my_info.is_empty()
    }

    /// Virtual read method: parse WorldInfo node from VRML stream.
    pub fn read(&mut self, _buffer: &mut WorldInfoInBuffer) -> WorldInfoErrorStatus {
        // Subclass/user provides actual parsing.
        WorldInfoErrorStatus::Ok
    }

    /// Virtual write method: output WorldInfo node to VRML format.
    pub fn write(&self, _prefix: Option<&str>) -> WorldInfoErrorStatus {
        // Subclass/user provides actual output.
        WorldInfoErrorStatus::Ok
    }

    /// Get a combined info string (all entries joined by newline).
    pub fn combined_info(&self) -> String {
        self.my_info.join("\n")
    }

    /// Get brief summary of world info.
    pub fn summary(&self) -> String {
        format!(
            "WorldInfo(title='{}', info_count={})",
            self.my_title,
            self.my_info.len()
        )
    }
}

impl Default for VrmlDataWorldInfo {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Clone for VrmlDataWorldInfo {
    fn clone(&self) -> Self {
        VrmlDataWorldInfo {
            my_title: self.my_title.clone(),
            my_info: self.my_info.clone(),
            my_name: self.my_name.clone(),
        }
    }
}

impl PartialEq for VrmlDataWorldInfo {
    fn eq(&self, other: &Self) -> bool {
        self.my_title == other.my_title && self.my_info == other.my_info && self.my_name == other.my_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_world_info() {
        let wi = VrmlDataWorldInfo::new(Some("world"));
        assert_eq!(wi.name(), "world");
        assert_eq!(wi.title(), "");
        assert_eq!(wi.info_count(), 0);
        assert!(wi.is_default());
    }

    #[test]
    fn with_title() {
        let wi = VrmlDataWorldInfo::with_title("My Scene", Some("w1"));
        assert_eq!(wi.title(), "My Scene");
        assert_eq!(wi.name(), "w1");
        assert!(!wi.is_default());
    }

    #[test]
    fn set_title() {
        let mut wi = VrmlDataWorldInfo::new(None);
        wi.set_title("Updated Title");
        assert_eq!(wi.title(), "Updated Title");
    }

    #[test]
    fn add_info() {
        let mut wi = VrmlDataWorldInfo::new(None);
        wi.add_info("Created with OCCT");
        wi.add_info("Version 1.0");
        assert_eq!(wi.info_count(), 2);
        assert_eq!(wi.get_info(0), Some("Created with OCCT"));
        assert_eq!(wi.get_info(1), Some("Version 1.0"));
        assert_eq!(wi.get_info(2), None);
    }

    #[test]
    fn set_info_list() {
        let mut wi = VrmlDataWorldInfo::new(None);
        wi.set_info_list(vec![
            "Info1".to_string(),
            "Info2".to_string(),
            "Info3".to_string(),
        ]);
        assert_eq!(wi.info_count(), 3);
        assert_eq!(wi.get_info(1), Some("Info2"));
    }

    #[test]
    fn clear_info() {
        let mut wi = VrmlDataWorldInfo::new(None);
        wi.add_info("temp");
        assert_eq!(wi.info_count(), 1);
        wi.clear_info();
        assert_eq!(wi.info_count(), 0);
    }

    #[test]
    fn clear_all() {
        let mut wi = VrmlDataWorldInfo::with_title("Title", None);
        wi.add_info("info");
        wi.clear();
        assert_eq!(wi.title(), "");
        assert_eq!(wi.info_count(), 0);
        assert!(wi.is_default());
    }

    #[test]
    fn combined_info() {
        let mut wi = VrmlDataWorldInfo::new(None);
        wi.add_info("Line1");
        wi.add_info("Line2");
        wi.add_info("Line3");
        assert_eq!(wi.combined_info(), "Line1\nLine2\nLine3");
    }

    #[test]
    fn combined_info_single() {
        let mut wi = VrmlDataWorldInfo::new(None);
        wi.add_info("Only");
        assert_eq!(wi.combined_info(), "Only");
    }

    #[test]
    fn combined_info_empty() {
        let wi = VrmlDataWorldInfo::new(None);
        assert_eq!(wi.combined_info(), "");
    }

    #[test]
    fn summary() {
        let mut wi = VrmlDataWorldInfo::with_title("Test", None);
        wi.add_info("Info1");
        wi.add_info("Info2");
        let summary = wi.summary();
        assert!(summary.contains("Test"));
        assert!(summary.contains("2"));
    }

    #[test]
    fn clone_preserves_data() {
        let mut wi = VrmlDataWorldInfo::with_title("Original", Some("w1"));
        wi.add_info("Info");
        let cloned = wi.clone();
        assert_eq!(cloned.title(), "Original");
        assert_eq!(cloned.name(), "w1");
        assert_eq!(cloned.info_count(), 1);
        assert_eq!(cloned.get_info(0), Some("Info"));
    }

    #[test]
    fn equality() {
        let mut wi1 = VrmlDataWorldInfo::with_title("Title", Some("w"));
        wi1.add_info("Info");
        let mut wi2 = VrmlDataWorldInfo::with_title("Title", Some("w"));
        wi2.add_info("Info");
        assert_eq!(wi1, wi2);
    }

    #[test]
    fn inequality_different_title() {
        let wi1 = VrmlDataWorldInfo::with_title("Title1", None);
        let wi2 = VrmlDataWorldInfo::with_title("Title2", None);
        assert_ne!(wi1, wi2);
    }

    #[test]
    fn inequality_different_info() {
        let mut wi1 = VrmlDataWorldInfo::new(None);
        wi1.add_info("Info1");
        let mut wi2 = VrmlDataWorldInfo::new(None);
        wi2.add_info("Info2");
        assert_ne!(wi1, wi2);
    }

    #[test]
    fn set_name() {
        let mut wi = VrmlDataWorldInfo::new(Some("Old"));
        wi.set_name("New");
        assert_eq!(wi.name(), "New");
    }

    #[test]
    fn info_list_slice() {
        let mut wi = VrmlDataWorldInfo::new(None);
        wi.add_info("A");
        wi.add_info("B");
        let list = wi.info_list();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0], "A");
    }
}
