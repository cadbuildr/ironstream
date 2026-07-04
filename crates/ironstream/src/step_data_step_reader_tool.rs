// FILE: step_data_step_reader_tool.rs
// occt: StepData_StepReaderTool

//! Tool for reading STEP data
pub struct StepDataStepReaderTool;

impl StepDataStepReaderTool {
    //! Creates a StepReaderTool
    pub fn new() -> Self {
        StepDataStepReaderTool
    }
}

impl Default for StepDataStepReaderTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_reader_tool_new() {
        let _tool = StepDataStepReaderTool::new();
    }
}
