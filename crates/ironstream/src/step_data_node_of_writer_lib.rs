// FILE: step_data_node_of_writer_lib.rs
// occt: StepData_NodeOfWriterLib

//! Node of writer library
pub struct StepDataNodeOfWriterLib;

impl StepDataNodeOfWriterLib {
    //! Creates a NodeOfWriterLib
    pub fn new() -> Self {
        StepDataNodeOfWriterLib
    }
}

impl Default for StepDataNodeOfWriterLib {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_new() {
        let _node = StepDataNodeOfWriterLib::new();
    }
}
