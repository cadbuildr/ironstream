// FILE: f_em_tool_h_assembly_table.rs
// occt: FEmTool_HAssemblyTable

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct AssemblyTable {}

pub type AssemblyTableHandle = Rc<RefCell<AssemblyTable>>;

/// Deprecated alias for handle assembly table.
#[derive(Clone, Debug)]
pub struct HAssemblyTable {
    table: AssemblyTableHandle,
}

impl HAssemblyTable {
    pub fn new() -> Self {
        HAssemblyTable {
            table: Rc::new(RefCell::new(AssemblyTable {})),
        }
    }

    pub fn table(&self) -> &AssemblyTableHandle {
        &self.table
    }
}

impl Default for HAssemblyTable {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_assembly_table_creation() {
        let h_table = HAssemblyTable::new();
        assert!(std::ptr::eq(h_table.table().as_ptr(), h_table.table().as_ptr()));
    }
}
