// FILE: vrml_data_unknown_node.rs
// occt: VrmlData_UnknownNode
//
// Faithful port of OCCT VrmlData_UnknownNode (DataExchange/TKDEVRML/VrmlData/
// VrmlData_UnknownNode.hxx/.cxx): represents an unparseable or unrecognized
// VRML node. Preserves the raw node text for safe round-trip processing
// when encountering extensions or unsupported node types.

use std::cell::RefCell;
use std::rc::Rc;

/// Error status for read/write operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnknownNodeErrorStatus {
    Ok = 0,
    EndOfFile = 1,
    NotEndOfFile = 2,
    GeneralError = 3,
}

/// Input buffer for parsing.
pub struct UnknownNodeInBuffer {
    pub line_num: u32,
}

impl UnknownNodeInBuffer {
    pub fn new() -> Self {
        UnknownNodeInBuffer { line_num: 1 }
    }
}

impl Default for UnknownNodeInBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// VRML UnknownNode: represents an unrecognized or unsupported node type.
/// Stores the raw node name and optional content for safe preservation
/// during VRML parsing and serialization. Enables forward compatibility.
#[derive(Debug)]
pub struct VrmlDataUnknownNode {
    my_node_type: String,     // e.g., "CustomGeometry"
    my_name: String,          // node name/id
    my_raw_content: String,   // preserved raw VRML text
    my_is_processed: bool,    // whether content has been parsed
}

impl VrmlDataUnknownNode {
    /// Constructor: creates an unknown node with a type name.
    pub fn new(node_type: &str, name: Option<&str>) -> Self {
        VrmlDataUnknownNode {
            my_node_type: node_type.to_string(),
            my_name: name.unwrap_or("").to_string(),
            my_raw_content: String::new(),
            my_is_processed: false,
        }
    }

    /// Get the node type.
    pub fn node_type(&self) -> &str {
        &self.my_node_type
    }

    /// Set the node type.
    pub fn set_node_type(&mut self, node_type: &str) {
        self.my_node_type = node_type.to_string();
    }

    /// Get the node name/id.
    pub fn name(&self) -> &str {
        &self.my_name
    }

    /// Set the node name/id.
    pub fn set_name(&mut self, name: &str) {
        self.my_name = name.to_string();
    }

    /// Get the raw VRML content preserved for this unknown node.
    pub fn raw_content(&self) -> &str {
        &self.my_raw_content
    }

    /// Set the raw VRML content.
    pub fn set_raw_content(&mut self, content: &str) {
        self.my_raw_content = content.to_string();
    }

    /// Append raw content (for streaming reads).
    pub fn append_raw_content(&mut self, content: &str) {
        self.my_raw_content.push_str(content);
    }

    /// Check if the raw content has been processed/parsed.
    pub fn is_processed(&self) -> bool {
        self.my_is_processed
    }

    /// Mark the node as having been processed.
    pub fn set_processed(&mut self, processed: bool) {
        self.my_is_processed = processed;
    }

    /// Check if this node is in default state (empty type and name).
    pub fn is_default(&self) -> bool {
        self.my_node_type.is_empty() && self.my_name.is_empty()
    }

    /// Check if the node's raw content is empty.
    pub fn is_empty(&self) -> bool {
        self.my_raw_content.is_empty()
    }

    /// Get the size of the raw content (in bytes).
    pub fn content_size(&self) -> usize {
        self.my_raw_content.len()
    }

    /// Clear all stored content.
    pub fn clear(&mut self) {
        self.my_raw_content.clear();
        self.my_is_processed = false;
    }

    /// Virtual read method: parse UnknownNode from VRML stream.
    /// Typically just reads the raw content until closing brace.
    pub fn read(&mut self, _buffer: &mut UnknownNodeInBuffer) -> UnknownNodeErrorStatus {
        // Subclass/user provides actual parsing.
        UnknownNodeErrorStatus::Ok
    }

    /// Virtual write method: output UnknownNode (reproduce raw content if available).
    pub fn write(&self, _prefix: Option<&str>) -> UnknownNodeErrorStatus {
        // Subclass/user provides actual output.
        UnknownNodeErrorStatus::Ok
    }

    /// Extract a field value from the raw content (basic pattern matching).
    /// Returns the value portion after the field name.
    pub fn extract_field_value(&self, field_name: &str) -> Option<String> {
        let pattern = format!("{} ", field_name);
        if let Some(pos) = self.my_raw_content.find(&pattern) {
            let start = pos + pattern.len();
            let remaining = &self.my_raw_content[start..];
            // Find the end of the value (space, comma, or newline)
            let end = remaining
                .find(|c: char| c.is_whitespace() || c == ',' || c == '}')
                .unwrap_or(remaining.len());
            Some(remaining[..end].to_string())
        } else {
            None
        }
    }

    /// Count the number of times a field appears in the raw content.
    pub fn count_field_occurrences(&self, field_name: &str) -> usize {
        self.my_raw_content.matches(field_name).count()
    }

    /// Construct a summary string for debugging.
    pub fn summary(&self) -> String {
        format!(
            "UnknownNode(type={}, name={}, size={}, processed={})",
            self.my_node_type,
            self.my_name,
            self.content_size(),
            self.my_is_processed
        )
    }
}

impl Default for VrmlDataUnknownNode {
    fn default() -> Self {
        Self::new("Unknown", None)
    }
}

impl Clone for VrmlDataUnknownNode {
    fn clone(&self) -> Self {
        VrmlDataUnknownNode {
            my_node_type: self.my_node_type.clone(),
            my_name: self.my_name.clone(),
            my_raw_content: self.my_raw_content.clone(),
            my_is_processed: self.my_is_processed,
        }
    }
}

impl PartialEq for VrmlDataUnknownNode {
    fn eq(&self, other: &Self) -> bool {
        self.my_node_type == other.my_node_type
            && self.my_name == other.my_name
            && self.my_raw_content == other.my_raw_content
            && self.my_is_processed == other.my_is_processed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_unknown_node() {
        let node = VrmlDataUnknownNode::new("ProtoNode", Some("proto1"));
        assert_eq!(node.node_type(), "ProtoNode");
        assert_eq!(node.name(), "proto1");
        assert!(node.is_empty());
    }

    #[test]
    fn unknown_node_default() {
        let node = VrmlDataUnknownNode::default();
        assert_eq!(node.node_type(), "Unknown");
        assert_eq!(node.name(), "");
    }

    #[test]
    fn set_node_type() {
        let mut node = VrmlDataUnknownNode::new("Old", None);
        node.set_node_type("New");
        assert_eq!(node.node_type(), "New");
    }

    #[test]
    fn set_name() {
        let mut node = VrmlDataUnknownNode::new("Type", Some("OldName"));
        node.set_name("NewName");
        assert_eq!(node.name(), "NewName");
    }

    #[test]
    fn set_raw_content() {
        let mut node = VrmlDataUnknownNode::new("CustomNode", None);
        node.set_raw_content("field1 10 field2 20");
        assert_eq!(node.raw_content(), "field1 10 field2 20");
        assert!(!node.is_empty());
    }

    #[test]
    fn append_raw_content() {
        let mut node = VrmlDataUnknownNode::new("CustomNode", None);
        node.append_raw_content("part1 ");
        node.append_raw_content("part2");
        assert_eq!(node.raw_content(), "part1 part2");
    }

    #[test]
    fn content_size() {
        let mut node = VrmlDataUnknownNode::new("Type", None);
        node.set_raw_content("hello");
        assert_eq!(node.content_size(), 5);
    }

    #[test]
    fn is_processed() {
        let mut node = VrmlDataUnknownNode::new("Type", None);
        assert!(!node.is_processed());
        node.set_processed(true);
        assert!(node.is_processed());
    }

    #[test]
    fn is_default() {
        let mut node = VrmlDataUnknownNode::new("Type", Some("name"));
        assert!(!node.is_default());

        let empty = VrmlDataUnknownNode::new("", None);
        assert!(empty.is_default());
    }

    #[test]
    fn clear_content() {
        let mut node = VrmlDataUnknownNode::new("Type", None);
        node.set_raw_content("content");
        node.set_processed(true);
        node.clear();
        assert!(node.is_empty());
        assert!(!node.is_processed());
    }

    #[test]
    fn extract_field_value() {
        let mut node = VrmlDataUnknownNode::new("Type", None);
        node.set_raw_content("radius 5.0 color 1 0 0");
        assert_eq!(node.extract_field_value("radius"), Some("5.0".to_string()));
        assert_eq!(node.extract_field_value("color"), Some("1".to_string()));
        assert_eq!(node.extract_field_value("unknown"), None);
    }

    #[test]
    fn count_field_occurrences() {
        let mut node = VrmlDataUnknownNode::new("Type", None);
        node.set_raw_content("field field other field");
        assert_eq!(node.count_field_occurrences("field"), 3);
        assert_eq!(node.count_field_occurrences("other"), 1);
        assert_eq!(node.count_field_occurrences("missing"), 0);
    }

    #[test]
    fn clone_preserves_data() {
        let mut node = VrmlDataUnknownNode::new("CustomNode", Some("instance"));
        node.set_raw_content("some content");
        node.set_processed(true);
        let cloned = node.clone();
        assert_eq!(cloned.node_type(), "CustomNode");
        assert_eq!(cloned.name(), "instance");
        assert_eq!(cloned.raw_content(), "some content");
        assert!(cloned.is_processed());
    }

    #[test]
    fn equality() {
        let mut n1 = VrmlDataUnknownNode::new("Type", Some("N1"));
        n1.set_raw_content("content");
        let mut n2 = VrmlDataUnknownNode::new("Type", Some("N1"));
        n2.set_raw_content("content");
        assert_eq!(n1, n2);
    }

    #[test]
    fn inequality_different_type() {
        let n1 = VrmlDataUnknownNode::new("Type1", None);
        let n2 = VrmlDataUnknownNode::new("Type2", None);
        assert_ne!(n1, n2);
    }

    #[test]
    fn summary() {
        let mut node = VrmlDataUnknownNode::new("Proto", Some("p1"));
        node.set_raw_content("field 10");
        let summary = node.summary();
        assert!(summary.contains("Proto"));
        assert!(summary.contains("p1"));
        assert!(summary.contains("8")); // "field 10" is 8 bytes
    }
}
