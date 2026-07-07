// FILE: f_em_tool_assembly_table.rs
// occt: FEmTool_AssemblyTable

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct AssemblyTableElement {}

pub type AssemblyTableElementHandle = Rc<RefCell<AssemblyTableElement>>;

/// Deprecated alias for assembly table collection.
#[derive(Clone, Debug)]
pub struct AssemblyTable {
    items: Vec<AssemblyTableElementHandle>,
}

impl AssemblyTable {
    pub fn new() -> Self {
        AssemblyTable {
            items: Vec::new(),
        }
    }

    pub fn append(&mut self, item: AssemblyTableElementHandle) {
        self.items.push(item);
    }

    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}

impl Default for AssemblyTable {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        let table = AssemblyTable::new();
        assert!(table.is_empty());
    }

    #[test]
    fn test_table_append() {
        let mut table = AssemblyTable::new();
        let item = Rc::new(RefCell::new(AssemblyTableElement {}));
        table.append(item);
        assert_eq!(table.len(), 1);
    }
}
