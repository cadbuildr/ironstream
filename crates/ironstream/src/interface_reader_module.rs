// FILE: interface_reader_module.rs
// occt: Interface_ReaderModule

use std::sync::Arc;

pub type FileReaderDataHandle = Arc<dyn std::any::Any>;
pub type CheckHandle = Arc<dyn std::any::Any>;
pub type TransientHandle = Arc<dyn std::any::Any>;

/// Defines unitary operations required to read an Entity from a File
pub trait InterfaceReaderModule {
    /// Translates the type of record to a positive Case Number
    fn case_num(&self, data: &FileReaderDataHandle, num: i32) -> i32;

    /// Performs the effective loading from data to entity
    fn read(&self, casenum: i32, data: &FileReaderDataHandle, num: i32,
            ach: &mut CheckHandle, ent: &mut TransientHandle);

    /// Specific operator (create+read)
    fn new_read(&self, casenum: i32, data: &FileReaderDataHandle, num: i32,
                ach: &mut CheckHandle, ent: &mut TransientHandle) -> bool {
        false
    }
}

/// Default implementation
pub struct DefaultReaderModule;

impl InterfaceReaderModule for DefaultReaderModule {
    fn case_num(&self, _data: &FileReaderDataHandle, _num: i32) -> i32 {
        0
    }

    fn read(&self, _casenum: i32, _data: &FileReaderDataHandle, _num: i32,
            _ach: &mut CheckHandle, _ent: &mut TransientHandle) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_num() {
        let module = DefaultReaderModule;
        let data: FileReaderDataHandle = Arc::new(());
        assert_eq!(module.case_num(&data, 0), 0);
    }

    #[test]
    fn test_new_read() {
        let module = DefaultReaderModule;
        let data: FileReaderDataHandle = Arc::new(());
        let mut ach: CheckHandle = Arc::new(());
        let mut ent: TransientHandle = Arc::new(());
        assert!(!module.new_read(0, &data, 0, &mut ach, &mut ent));
    }
}
