// FILE: t_obj_t_int_sparse_array.rs
// occt: TObj_TIntSparseArray

//! OCAF Attribute to store a set of positive integer values in the OCAF
//! tree; each value is identified by a positive integer ID over an
//! NCollection_SparseArray.
//! Faithful port of `TObj_TIntSparseArray` (.hxx + .cxx): GUID
//! "7016dc0c-b118-4433-8ef3-aecdccc79198", positive-only IDs and values
//! (Standard_OutOfRange -> panic), modification-delta maintenance in
//! `myOldMap` with the AbsentValue = -1 sentinel and the exact
//! backupValue compression rule, Restore-from-delta, BackupCopy that
//! MOVES the delta, and SetDoBackup control. The C++ "my transaction is
//! older than the current one" condition is modeled by an explicit
//! `in_older_transaction` flag.

use std::collections::HashMap;

/// The GUID of TObj_TIntSparseArray.
pub const TOBJ_TINTSPARSEARRAY_GUID: &str = "7016dc0c-b118-4433-8ef3-aecdccc79198";

/// Internal constant marking absent values in the backup array.
pub const ABSENT_VALUE_TISA: i32 = -1;

/// OCAF attribute holding a sparse array of positive ints.
pub struct TObjTIntSparseArray {
    vector: HashMap<usize, i32>,
    old_map: HashMap<usize, i32>,
    do_backup: bool,
    /// Models `Transaction() < Label().Data()->Transaction()`: deltas are
    /// recorded only when the attribute was created in an older
    /// transaction than the current one.
    in_older_transaction: bool,
}

impl Default for TObjTIntSparseArray {
    fn default() -> Self {
        TObjTIntSparseArray::new()
    }
}

impl TObjTIntSparseArray {
    /// Empty constructor (`myDoBackup = true`).
    pub fn new() -> Self {
        TObjTIntSparseArray {
            vector: HashMap::new(),
            old_map: HashMap::new(),
            do_backup: true,
            in_older_transaction: true,
        }
    }

    /// TObj_TIntSparseArray::GetID.
    pub fn get_id() -> &'static str {
        TOBJ_TINTSPARSEARRAY_GUID
    }

    /// TObj_TIntSparseArray::ID.
    pub fn id(&self) -> &'static str {
        Self::get_id()
    }

    /// Number of stored values.
    pub fn size(&self) -> usize {
        self.vector.len()
    }

    /// True if a value with the given ID is present.
    pub fn has_value(&self, id: usize) -> bool {
        self.vector.contains_key(&id)
    }

    /// Value by ID; panics when absent (NCollection_SparseArray::Value).
    pub fn value(&self, id: usize) -> i32 {
        *self
            .vector
            .get(&id)
            .expect("TObj_TIntSparseArray: no value stored with this ID")
    }

    /// Iterator over (id, value) pairs, id-ordered for determinism.
    pub fn iter(&self) -> impl Iterator<Item = (usize, i32)> {
        let mut items: Vec<(usize, i32)> = self.vector.iter().map(|(k, v)| (*k, *v)).collect();
        items.sort_by_key(|(k, _)| *k);
        items.into_iter()
    }

    /// Sets the value with the given ID (both must be positive).
    pub fn set_value(&mut self, id: usize, value: i32) {
        assert!(id >= 1 && value >= 1, "TObj_TIntSparseArray::SetValue");
        let mut old = ABSENT_VALUE_TISA;
        if let Some(existing) = self.vector.get_mut(&id) {
            if *existing == value {
                return; // no actual modification
            }
            old = *existing;
            *existing = value;
        } else {
            self.vector.insert(id, value);
        }
        if self.do_backup && self.in_older_transaction {
            self.backup_value(id, old, value);
        }
    }

    /// Unsets the value with the given ID.
    pub fn unset_value(&mut self, id: usize) {
        assert!(id >= 1, "TObj_TIntSparseArray::UnsetValue");
        let old = match self.vector.remove(&id) {
            Some(v) => v,
            None => return, // no actual modification
        };
        if self.do_backup && self.in_older_transaction {
            self.backup_value(id, old, ABSENT_VALUE_TISA);
        }
    }

    /// Clears the set (backing up all removed values).
    pub fn clear(&mut self) {
        if self.do_backup && self.in_older_transaction {
            let snapshot: Vec<(usize, i32)> = self.vector.iter().map(|(k, v)| (*k, *v)).collect();
            for (id, val) in snapshot {
                self.backup_value(id, val, ABSENT_VALUE_TISA);
            }
        }
        self.vector.clear();
    }

    /// backupValue: save the current value unless already saved; if the
    /// saved undo value equals the new value, drop the undo item.
    fn backup_value(&mut self, id: usize, curr_value: i32, new_value: i32) {
        if let Some(&undo) = self.old_map.get(&id) {
            if undo == new_value {
                self.old_map.remove(&id);
            }
        } else {
            self.old_map.insert(id, curr_value);
        }
    }

    /// NewEmpty.
    pub fn new_empty(&self) -> TObjTIntSparseArray {
        TObjTIntSparseArray::new()
    }

    /// BackupCopy: MOVES this delta into a fresh attribute.
    pub fn backup_copy(&mut self) -> TObjTIntSparseArray {
        let mut copy = self.new_empty();
        if !self.old_map.is_empty() {
            std::mem::swap(&mut copy.old_map, &mut self.old_map);
        }
        copy
    }

    /// Restore: applies the delta saved in `delta.old_map` to this.
    pub fn restore(&mut self, delta: &TObjTIntSparseArray) {
        let mut items: Vec<(usize, i32)> = delta.old_map.iter().map(|(k, v)| (*k, *v)).collect();
        items.sort_by_key(|(k, _)| *k);
        for (id, old) in items {
            if old == ABSENT_VALUE_TISA {
                self.unset_value(id);
            } else {
                self.set_value(id, old);
            }
        }
    }

    /// Paste: assigns the value vector into `into` (delta untouched).
    pub fn paste(&self, into: &mut TObjTIntSparseArray) {
        into.vector = self.vector.clone();
    }

    /// SetDoBackup — controls modification-delta maintenance.
    pub fn set_do_backup(&mut self, to_do: bool) {
        self.do_backup = to_do;
    }

    /// Models entering/leaving a transaction newer than the attribute's.
    pub fn set_in_older_transaction(&mut self, older: bool) {
        self.in_older_transaction = older;
    }

    /// ClearDelta.
    pub fn clear_delta(&mut self) {
        self.old_map.clear();
    }

    /// AfterUndo: the delta in `self` must be cleared; returns true.
    pub fn after_undo(&mut self) -> bool {
        self.clear_delta();
        true
    }

    /// Delta size (observability for tests).
    pub fn delta_size(&self) -> usize {
        self.old_map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guid_and_empty_state() {
        let arr = TObjTIntSparseArray::new();
        assert_eq!(arr.id(), "7016dc0c-b118-4433-8ef3-aecdccc79198");
        assert_eq!(arr.size(), 0);
        assert!(!arr.has_value(1));
    }

    #[test]
    fn set_get_unset() {
        let mut arr = TObjTIntSparseArray::new();
        arr.set_value(5, 42);
        arr.set_value(1000, 7);
        assert_eq!(arr.size(), 2);
        assert_eq!(arr.value(5), 42);
        assert_eq!(arr.value(1000), 7);
        arr.unset_value(5);
        assert!(!arr.has_value(5));
        assert_eq!(arr.size(), 1);
    }

    #[test]
    #[should_panic(expected = "SetValue")]
    fn zero_id_rejected() {
        let mut arr = TObjTIntSparseArray::new();
        arr.set_value(0, 1);
    }

    #[test]
    #[should_panic(expected = "SetValue")]
    fn non_positive_value_rejected() {
        let mut arr = TObjTIntSparseArray::new();
        arr.set_value(1, 0);
    }

    #[test]
    fn delta_records_previous_values() {
        let mut arr = TObjTIntSparseArray::new();
        arr.set_value(3, 10); // absent -> 10: delta stores AbsentValue
        assert_eq!(arr.delta_size(), 1);
        arr.set_value(3, 20); // 10 -> 20: id already backed up, kept
        assert_eq!(arr.delta_size(), 1);
        // Setting back to the undo value compresses the delta away:
        // undo value is AbsentValue, so unset removes the delta item.
        arr.unset_value(3);
        assert_eq!(arr.delta_size(), 0, "returning to the saved state clears the item");
    }

    #[test]
    fn restore_from_backup_copy_undoes_changes() {
        let mut arr = TObjTIntSparseArray::new();
        arr.set_value(1, 100);
        arr.set_value(2, 200);
        arr.clear_delta();

        // A "transaction": modify, then take the delta and restore.
        arr.set_value(1, 111);
        arr.unset_value(2);
        arr.set_value(9, 900);
        let delta = arr.backup_copy();
        assert_eq!(arr.delta_size(), 0, "BackupCopy moved the delta out");
        arr.restore(&delta);
        assert_eq!(arr.value(1), 100);
        assert_eq!(arr.value(2), 200);
        assert!(!arr.has_value(9));
    }

    #[test]
    fn clear_backs_up_all_values() {
        let mut arr = TObjTIntSparseArray::new();
        arr.set_value(1, 10);
        arr.set_value(2, 20);
        arr.clear_delta();
        arr.clear();
        assert_eq!(arr.size(), 0);
        assert_eq!(arr.delta_size(), 2);
        let delta = arr.backup_copy();
        arr.restore(&delta);
        assert_eq!(arr.value(1), 10);
        assert_eq!(arr.value(2), 20);
    }

    #[test]
    fn do_backup_flag_disables_delta() {
        let mut arr = TObjTIntSparseArray::new();
        arr.set_do_backup(false);
        arr.set_value(4, 44);
        assert_eq!(arr.delta_size(), 0);
        assert!(arr.after_undo());
    }

    #[test]
    fn paste_copies_values_only() {
        let mut src = TObjTIntSparseArray::new();
        src.set_value(7, 70);
        let mut dst = TObjTIntSparseArray::new();
        src.paste(&mut dst);
        assert_eq!(dst.value(7), 70);
        assert_eq!(dst.delta_size(), 0);
    }

    #[test]
    fn iterator_is_id_ordered() {
        let mut arr = TObjTIntSparseArray::new();
        arr.set_value(10, 1);
        arr.set_value(2, 2);
        arr.set_value(500, 3);
        let ids: Vec<usize> = arr.iter().map(|(k, _)| k).collect();
        assert_eq!(ids, vec![2, 10, 500]);
    }
}
