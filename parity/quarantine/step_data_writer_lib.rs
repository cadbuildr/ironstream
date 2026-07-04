// FILE: step_data_writer_lib.rs
// occt: StepData_WriterLib

use std::collections::HashMap;

//! Library of writers for STEP data
pub struct StepDataWriterLib {
    writers: HashMap<String, String>,
}

impl StepDataWriterLib {
    //! Creates a WriterLib
    pub fn new() -> Self {
        StepDataWriterLib {
            writers: HashMap::new(),
        }
    }

    //! Adds a writer
    pub fn add(&mut self, name: &str, writer: &str) {
        self.writers.insert(name.to_string(), writer.to_string());
    }

    //! Returns a writer
    pub fn get(&self, name: &str) -> Option<&str> {
        self.writers.get(name).map(|s| s.as_str())
    }
}

impl Default for StepDataWriterLib {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_writer_lib_new() {
        let lib = StepDataWriterLib::new();
        assert!(lib.get("unknown").is_none());
    }

    #[test]
    fn test_add_writer() {
        let mut lib = StepDataWriterLib::new();
        lib.add("type1", "writer1");
        assert_eq!(lib.get("type1"), Some("writer1"));
    }
}
