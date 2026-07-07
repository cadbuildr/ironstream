// FILE: step_data_step_dumper.rs
// occt: StepData_StepDumper

//! Dumps STEP data to a readable format
pub struct StepDataStepDumper;

impl StepDataStepDumper {
    //! Creates a StepDumper
    pub fn new() -> Self {
        StepDataStepDumper
    }
}

impl Default for StepDataStepDumper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_dumper_new() {
        let _dumper = StepDataStepDumper::new();
    }
}
