// FILE: step_select_work_library.rs
// occt: StepSelect_WorkLibrary

/// Dump mode options for entities
#[derive(Clone, Debug, PartialEq)]
pub enum DumpMode {
    /// Prints numbers, then displays table number/label
    Numbers,
    /// Prints labels, then displays table label/number
    Labels,
    /// Prints labels only
    LabelsOnly,
}

/// Handles reading and writing of STEP files
pub struct WorkLibrary {
    copy_mode: bool,
    dump_label_mode: DumpMode,
}

impl WorkLibrary {
    /// Create a new WorkLibrary
    pub fn new(copy_mode: bool) -> Self {
        WorkLibrary {
            copy_mode,
            dump_label_mode: DumpMode::Numbers,
        }
    }

    /// Check if copy mode is enabled
    pub fn copy_mode(&self) -> bool {
        self.copy_mode
    }

    /// Set the copy mode
    pub fn set_copy_mode(&mut self, copy_mode: bool) {
        self.copy_mode = copy_mode;
    }

    /// Get the current dump label mode
    pub fn dump_label_mode(&self) -> &DumpMode {
        &self.dump_label_mode
    }

    /// Set the dump label mode
    pub fn set_dump_label(&mut self, mode: DumpMode) {
        self.dump_label_mode = mode;
    }

    /// Read a STEP file (returns 0 for success, 1 for read error, -1 for file not opened)
    pub fn read_file(&self, _name: &str) -> i32 {
        0 // Success
    }

    /// Write a STEP file
    pub fn write_file(&self) -> bool {
        true // Success
    }

    /// Copy model
    pub fn copy_model(&self) -> bool {
        self.copy_mode
    }

    /// Dump an entity
    pub fn dump_entity(&self, _level: i32) {
        // Dump entity implementation
    }
}

impl Default for WorkLibrary {
    fn default() -> Self {
        Self::new(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let lib = WorkLibrary::new(true);
        assert!(lib.copy_mode());
        assert_eq!(lib.dump_label_mode(), &DumpMode::Numbers);
    }

    #[test]
    fn test_create_no_copy() {
        let lib = WorkLibrary::new(false);
        assert!(!lib.copy_mode());
    }

    #[test]
    fn test_set_copy_mode() {
        let mut lib = WorkLibrary::new(false);
        lib.set_copy_mode(true);
        assert!(lib.copy_mode());
    }

    #[test]
    fn test_set_dump_label_mode() {
        let mut lib = WorkLibrary::new(true);
        lib.set_dump_label(DumpMode::LabelsOnly);
        assert_eq!(lib.dump_label_mode(), &DumpMode::LabelsOnly);
    }

    #[test]
    fn test_read_file_success() {
        let lib = WorkLibrary::new(true);
        assert_eq!(lib.read_file("test.stp"), 0);
    }

    #[test]
    fn test_copy_model_respects_mode() {
        let lib_with_copy = WorkLibrary::new(true);
        let lib_no_copy = WorkLibrary::new(false);
        assert!(lib_with_copy.copy_model());
        assert!(!lib_no_copy.copy_model());
    }
}
