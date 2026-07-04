// FILE: step_data_global_node_of_writer_lib.rs
// occt: StepData_GlobalNodeOfWriterLib

//! Global node of writer library
pub struct StepDataGlobalNodeOfWriterLib;

impl StepDataGlobalNodeOfWriterLib {
    //! Creates a GlobalNodeOfWriterLib
    pub fn new() -> Self {
        StepDataGlobalNodeOfWriterLib
    }
}

impl Default for StepDataGlobalNodeOfWriterLib {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_node_new() {
        let _node = StepDataGlobalNodeOfWriterLib::new();
    }
}
