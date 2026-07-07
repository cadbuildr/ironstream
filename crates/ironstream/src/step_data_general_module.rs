// FILE: step_data_general_module.rs
// occt: StepData_GeneralModule

//! A general module for STEP data handling
pub struct StepDataGeneralModule;

impl StepDataGeneralModule {
    //! Creates a GeneralModule
    pub fn new() -> Self {
        StepDataGeneralModule
    }
}

impl Default for StepDataGeneralModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general_module_new() {
        let _module = StepDataGeneralModule::new();
    }
}
