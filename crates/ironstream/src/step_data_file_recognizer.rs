// FILE: step_data_file_recognizer.rs
// occt: StepData_FileRecognizer

//! Recognizes STEP file format
pub struct StepDataFileRecognizer;

impl StepDataFileRecognizer {
    //! Creates a FileRecognizer
    pub fn new() -> Self {
        StepDataFileRecognizer
    }
}

impl Default for StepDataFileRecognizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_recognizer_new() {
        let _recognizer = StepDataFileRecognizer::new();
    }
}
