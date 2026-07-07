// FILE: bin_obj_mgt_s_relocation_table.rs
// occt: BinObjMgt_SRelocationTable

//! Deprecated type alias for backward compatibility.
//! Use NCollection_IndexedMap<Handle<Transient>> directly instead.

use std::collections::BTreeMap;

/// Deprecated alias for an indexed map of transient objects.
/// Maps indices to transient handles for relocation in binary I/O.
pub type BinObjMgtSRelocationTable = BTreeMap<usize, Box<dyn std::any::Any>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relocation_table_insert_and_retrieve() {
        let mut table: BinObjMgtSRelocationTable = BTreeMap::new();

        // Simulate storing a transient object reference
        let obj = Box::new(42usize);
        table.insert(1, obj);

        assert_eq!(table.len(), 1);
        assert!(table.contains_key(&1));
    }

    #[test]
    fn test_relocation_table_indexed_map_semantics() {
        let mut table: BinObjMgtSRelocationTable = BTreeMap::new();

        table.insert(0, Box::new("first"));
        table.insert(1, Box::new("second"));
        table.insert(2, Box::new("third"));

        // BTreeMap maintains sorted keys like NCollection_IndexedMap
        let keys: Vec<_> = table.keys().copied().collect();
        assert_eq!(keys, vec![0, 1, 2]);
    }

    #[test]
    fn test_relocation_table_clear() {
        let mut table: BinObjMgtSRelocationTable = BTreeMap::new();
        table.insert(5, Box::new(100));

        table.clear();
        assert_eq!(table.len(), 0);
    }
}
