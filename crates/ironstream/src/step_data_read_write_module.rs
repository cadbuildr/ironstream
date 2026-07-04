// FILE: step_data_read_write_module.rs
// occt: StepData_ReadWriteModule

//! A read-write module for STEP data
pub struct StepDataReadWriteModule;

impl StepDataReadWriteModule {
    //! Creates a ReadWriteModule
    pub fn new() -> Self {
        StepDataReadWriteModule
    }
}

impl Default for StepDataReadWriteModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write_module_new() {
        let _module = StepDataReadWriteModule::new();
    }
}
