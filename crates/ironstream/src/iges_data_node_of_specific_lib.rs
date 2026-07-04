// FILE: iges_data_node_of_specific_lib.rs
// occt: IGESData_NodeOfSpecificLib

//! Node for the SpecificLib library linked list structure.

#[derive(Clone, Debug)]
pub struct NodeOfSpecificLib {
    value: String,
}

impl NodeOfSpecificLib {
    pub fn new(value: &str) -> Self {
        NodeOfSpecificLib {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Default for NodeOfSpecificLib {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let node = NodeOfSpecificLib::new("test");
        assert_eq!(node.value(), "test");
    }

    #[test]
    fn test_default() {
        let node = NodeOfSpecificLib::default();
        assert_eq!(node.value(), "");
    }
}
