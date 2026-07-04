// FILE: iges_data_global_node_of_specific_lib.rs
// occt: IGESData_GlobalNodeOfSpecificLib

//! Global node for the SpecificLib library chain structure.

#[derive(Clone, Debug)]
pub struct GlobalNodeOfSpecificLib {
    name: String,
}

impl GlobalNodeOfSpecificLib {
    pub fn new(name: &str) -> Self {
        GlobalNodeOfSpecificLib {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for GlobalNodeOfSpecificLib {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let node = GlobalNodeOfSpecificLib::new("test");
        assert_eq!(node.name(), "test");
    }

    #[test]
    fn test_default() {
        let node = GlobalNodeOfSpecificLib::default();
        assert_eq!(node.name(), "");
    }
}
