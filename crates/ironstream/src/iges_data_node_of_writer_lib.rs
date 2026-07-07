// FILE: iges_data_node_of_writer_lib.rs
// occt: IGESData_NodeOfWriterLib

//! Node for the WriterLib library linked list structure.

#[derive(Clone, Debug)]
pub struct NodeOfWriterLib {
    value: String,
}

impl NodeOfWriterLib {
    pub fn new(value: &str) -> Self {
        NodeOfWriterLib {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Default for NodeOfWriterLib {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let node = NodeOfWriterLib::new("test");
        assert_eq!(node.value(), "test");
    }

    #[test]
    fn test_default() {
        let node = NodeOfWriterLib::default();
        assert_eq!(node.value(), "");
    }
}
