// FILE: iges_data_global_node_of_writer_lib.rs
// occt: IGESData_GlobalNodeOfWriterLib

//! Global node for the WriterLib library chain structure.

#[derive(Clone, Debug)]
pub struct GlobalNodeOfWriterLib {
    name: String,
}

impl GlobalNodeOfWriterLib {
    pub fn new(name: &str) -> Self {
        GlobalNodeOfWriterLib {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for GlobalNodeOfWriterLib {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let node = GlobalNodeOfWriterLib::new("test");
        assert_eq!(node.name(), "test");
    }

    #[test]
    fn test_default() {
        let node = GlobalNodeOfWriterLib::default();
        assert_eq!(node.name(), "");
    }
}
