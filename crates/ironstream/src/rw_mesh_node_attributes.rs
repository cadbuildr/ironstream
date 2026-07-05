// FILE: rw_mesh_node_attributes.rs
// occt: RWMesh_NodeAttributes

//! Attributes of a node in a mesh.
//! Contains metadata for mesh nodes including name, style, and optional metadata.

/// Attributes associated with a mesh node
#[derive(Clone, Debug, Default)]
pub struct RWMeshNodeAttributes {
    /// Name for the user (friendly name)
    pub name: String,
    /// Name within low-level format structure
    pub raw_name: String,
    /// Optional metadata associated with the node
    pub named_data: Option<NamedDataHandle>,
    /// Presentation style for the node
    pub style: PresentationStyle,
}

/// Handle to named data (placeholder for OCCT TDataStd_NamedData)
#[derive(Clone, Debug)]
pub struct NamedDataHandle;

/// Presentation style for mesh elements
#[derive(Clone, Debug, Default)]
pub struct PresentationStyle;

impl RWMeshNodeAttributes {
    /// Create new node attributes with default values
    pub fn new() -> Self {
        RWMeshNodeAttributes {
            name: String::new(),
            raw_name: String::new(),
            named_data: None,
            style: PresentationStyle::default(),
        }
    }

    /// Set the user-friendly name
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// Set the raw name (low-level format name)
    pub fn set_raw_name(&mut self, raw_name: impl Into<String>) {
        self.raw_name = raw_name.into();
    }

    /// Get the user-friendly name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the raw name
    pub fn raw_name(&self) -> &str {
        &self.raw_name
    }

    /// Set optional metadata
    pub fn set_named_data(&mut self, data: Option<NamedDataHandle>) {
        self.named_data = data;
    }

    /// Get optional metadata
    pub fn named_data(&self) -> Option<&NamedDataHandle> {
        self.named_data.as_ref()
    }

    /// Set the presentation style
    pub fn set_style(&mut self, style: PresentationStyle) {
        self.style = style;
    }

    /// Get the presentation style
    pub fn style(&self) -> &PresentationStyle {
        &self.style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_attributes_creation() {
        let attrs = RWMeshNodeAttributes::new();
        assert_eq!(attrs.name(), "");
        assert_eq!(attrs.raw_name(), "");
        assert!(attrs.named_data().is_none());
    }

    #[test]
    fn test_set_name() {
        let mut attrs = RWMeshNodeAttributes::new();
        attrs.set_name("test_node");
        assert_eq!(attrs.name(), "test_node");
    }

    #[test]
    fn test_set_raw_name() {
        let mut attrs = RWMeshNodeAttributes::new();
        attrs.set_raw_name("_node_42");
        assert_eq!(attrs.raw_name(), "_node_42");
    }

    #[test]
    fn test_default_construction() {
        let attrs = RWMeshNodeAttributes::default();
        assert_eq!(attrs.name(), "");
        assert_eq!(attrs.raw_name(), "");
        assert!(attrs.named_data().is_none());
    }

    #[test]
    fn test_clone() {
        let mut attrs = RWMeshNodeAttributes::new();
        attrs.set_name("original");
        attrs.set_raw_name("_orig");

        let attrs_clone = attrs.clone();
        assert_eq!(attrs_clone.name(), "original");
        assert_eq!(attrs_clone.raw_name(), "_orig");
    }
}
