// FILE: xml_mxcaf_doc_graph_node_driver.rs
// occt: XmlMXCAFDoc_GraphNodeDriver
//
// Faithful port of OCCT XmlMXCAFDoc_GraphNodeDriver
// (DataExchange/TKXmlXCAF/XmlMXCAFDoc/XmlMXCAFDoc_GraphNodeDriver.hxx),
// the XmlMDF_ADriver for XCAF graph node attributes.
// Serializes/deserializes XCAFDoc_GraphNode data (reference count and parent/child links).

/// Local model of graph node data (reference counts and links).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GraphNodeData {
    pub ref_count: u32,
    pub parent_id: u32,
    pub children_ids: Vec<u32>,
}

impl GraphNodeData {
    pub fn new(ref_count: u32, parent_id: u32) -> Self {
        Self {
            ref_count,
            parent_id,
            children_ids: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child_id: u32) {
        self.children_ids.push(child_id);
    }
}

/// XmlMDF_ADriver for graph node attributes.
#[derive(Debug)]
pub struct XmlMXCAFDocGraphNodeDriver {
    type_name: String,
    version: u32,
}

impl XmlMXCAFDocGraphNodeDriver {
    pub const TYPE_NAME: &'static str = "XCAFDoc_GraphNode";

    pub fn new() -> Self {
        Self {
            type_name: Self::TYPE_NAME.to_string(),
            version: 1,
        }
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    pub fn version_number(&self) -> u32 {
        self.version
    }

    /// Read graph node from XML element text.
    /// Format: "ref_count parent_id child1 child2 ..." (space-separated integers).
    pub fn read_from_xml(&self, element_text: &str) -> Result<GraphNodeData, String> {
        let mut parts = element_text.split_whitespace();
        let ref_count_str = parts
            .next()
            .ok_or_else(|| "Missing ref_count".to_string())?;
        let parent_str = parts
            .next()
            .ok_or_else(|| "Missing parent_id".to_string())?;

        let ref_count = ref_count_str
            .parse::<u32>()
            .map_err(|e| format!("Failed to parse ref_count: {}", e))?;
        let parent_id = parent_str
            .parse::<u32>()
            .map_err(|e| format!("Failed to parse parent_id: {}", e))?;

        let mut node = GraphNodeData::new(ref_count, parent_id);

        for child_str in parts {
            let child_id = child_str
                .parse::<u32>()
                .map_err(|e| format!("Failed to parse child_id: {}", e))?;
            node.add_child(child_id);
        }

        Ok(node)
    }

    /// Write graph node to XML element text.
    pub fn write_to_xml(&self, data: &GraphNodeData) -> String {
        let mut result = format!("{} {}", data.ref_count, data.parent_id);
        for child_id in &data.children_ids {
            result.push(' ');
            result.push_str(&child_id.to_string());
        }
        result
    }
}

impl Default for XmlMXCAFDocGraphNodeDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_node_data_new() {
        let node = GraphNodeData::new(1, 100);
        assert_eq!(node.ref_count, 1);
        assert_eq!(node.parent_id, 100);
        assert_eq!(node.children_ids.len(), 0);
    }

    #[test]
    fn test_graph_node_add_child() {
        let mut node = GraphNodeData::new(2, 100);
        node.add_child(200);
        node.add_child(300);
        assert_eq!(node.children_ids.len(), 2);
        assert_eq!(node.children_ids[0], 200);
        assert_eq!(node.children_ids[1], 300);
    }

    #[test]
    fn test_driver_new() {
        let driver = XmlMXCAFDocGraphNodeDriver::new();
        assert_eq!(driver.type_name(), "XCAFDoc_GraphNode");
        assert_eq!(driver.version_number(), 1);
    }

    #[test]
    fn test_read_from_xml_no_children() {
        let driver = XmlMXCAFDocGraphNodeDriver::new();
        let result = driver.read_from_xml("1 100");
        assert!(result.is_ok());
        let node = result.unwrap();
        assert_eq!(node.ref_count, 1);
        assert_eq!(node.parent_id, 100);
        assert_eq!(node.children_ids.len(), 0);
    }

    #[test]
    fn test_read_from_xml_with_children() {
        let driver = XmlMXCAFDocGraphNodeDriver::new();
        let result = driver.read_from_xml("2 100 200 300 400");
        assert!(result.is_ok());
        let node = result.unwrap();
        assert_eq!(node.ref_count, 2);
        assert_eq!(node.parent_id, 100);
        assert_eq!(node.children_ids, vec![200, 300, 400]);
    }

    #[test]
    fn test_read_from_xml_invalid_ref_count() {
        let driver = XmlMXCAFDocGraphNodeDriver::new();
        let result = driver.read_from_xml("not_a_number 100");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_xml_missing_parent() {
        let driver = XmlMXCAFDocGraphNodeDriver::new();
        let result = driver.read_from_xml("1");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_to_xml_no_children() {
        let driver = XmlMXCAFDocGraphNodeDriver::new();
        let node = GraphNodeData::new(1, 100);
        let xml = driver.write_to_xml(&node);
        assert_eq!(xml, "1 100");
    }

    #[test]
    fn test_write_to_xml_with_children() {
        let driver = XmlMXCAFDocGraphNodeDriver::new();
        let mut node = GraphNodeData::new(2, 100);
        node.add_child(200);
        node.add_child(300);
        let xml = driver.write_to_xml(&node);
        assert_eq!(xml, "2 100 200 300");
    }

    #[test]
    fn test_roundtrip() {
        let driver = XmlMXCAFDocGraphNodeDriver::new();
        let mut original = GraphNodeData::new(3, 999);
        original.add_child(111);
        original.add_child(222);
        original.add_child(333);
        let xml = driver.write_to_xml(&original);
        let restored = driver.read_from_xml(&xml).unwrap();
        assert_eq!(original, restored);
    }
}
