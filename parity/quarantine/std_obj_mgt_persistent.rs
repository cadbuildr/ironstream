// FILE: std_obj_mgt_persistent.rs
// occt: StdObjMgt_Persistent

/// Root class for a temporary persistent object that reads data from a file
/// and then creates transient object using the data.
pub struct StdObjMgtPersistent {
    type_num: i32,
    ref_num: i32,
}

impl StdObjMgtPersistent {
    /// Create a new persistent object
    pub fn new() -> Self {
        StdObjMgtPersistent {
            type_num: 0,
            ref_num: 0,
        }
    }

    /// Returns the assigned persistent type number
    pub fn type_num(&self) -> i32 {
        self.type_num
    }

    /// Assigns a persistent type number to the object
    pub fn set_type_num(&mut self, num: i32) {
        self.type_num = num;
    }

    /// Returns the object reference number
    pub fn ref_num(&self) -> i32 {
        self.ref_num
    }

    /// Sets an object reference number
    pub fn set_ref_num(&mut self, num: i32) {
        self.ref_num = num;
    }

    /// Returns persistent type name
    pub fn p_name(&self) -> &str {
        "StdObjMgt_Persistent"
    }
}

impl Default for StdObjMgtPersistent {
    fn default() -> Self {
        Self::new()
    }
}

/// Sequence of persistent objects
pub struct SequenceOfPersistent {
    items: Vec<StdObjMgtPersistent>,
}

impl SequenceOfPersistent {
    /// Create a new empty sequence
    pub fn new() -> Self {
        SequenceOfPersistent {
            items: Vec::new(),
        }
    }

    /// Append a persistent object to the sequence
    pub fn append(&mut self, item: StdObjMgtPersistent) {
        self.items.push(item);
    }

    /// Get the number of items in the sequence
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the sequence is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get an item by index
    pub fn get(&self, index: usize) -> Option<&StdObjMgtPersistent> {
        self.items.get(index)
    }

    /// Get a mutable item by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut StdObjMgtPersistent> {
        self.items.get_mut(index)
    }

    /// Clear the sequence
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Default for SequenceOfPersistent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_persistent() {
        let persistent = StdObjMgtPersistent::new();
        assert_eq!(persistent.type_num(), 0);
        assert_eq!(persistent.ref_num(), 0);
        assert_eq!(persistent.p_name(), "StdObjMgt_Persistent");
    }

    #[test]
    fn test_set_type_num() {
        let mut persistent = StdObjMgtPersistent::new();
        persistent.set_type_num(42);
        assert_eq!(persistent.type_num(), 42);
    }

    #[test]
    fn test_set_ref_num() {
        let mut persistent = StdObjMgtPersistent::new();
        persistent.set_ref_num(99);
        assert_eq!(persistent.ref_num(), 99);
    }

    #[test]
    fn test_sequence_create_empty() {
        let seq = SequenceOfPersistent::new();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_sequence_append() {
        let mut seq = SequenceOfPersistent::new();
        let obj = StdObjMgtPersistent::new();
        seq.append(obj);

        assert_eq!(seq.len(), 1);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_sequence_get() {
        let mut seq = SequenceOfPersistent::new();
        let mut obj = StdObjMgtPersistent::new();
        obj.set_ref_num(42);
        seq.append(obj);

        let retrieved = seq.get(0);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().ref_num(), 42);
    }

    #[test]
    fn test_sequence_multiple_items() {
        let mut seq = SequenceOfPersistent::new();
        for i in 0..5 {
            let mut obj = StdObjMgtPersistent::new();
            obj.set_ref_num(i);
            seq.append(obj);
        }

        assert_eq!(seq.len(), 5);
        for i in 0..5 {
            assert_eq!(seq.get(i).unwrap().ref_num(), i);
        }
    }

    #[test]
    fn test_sequence_clear() {
        let mut seq = SequenceOfPersistent::new();
        seq.append(StdObjMgtPersistent::new());
        seq.append(StdObjMgtPersistent::new());

        assert_eq!(seq.len(), 2);
        seq.clear();
        assert!(seq.is_empty());
    }

    #[test]
    fn test_sequence_get_mut() {
        let mut seq = SequenceOfPersistent::new();
        seq.append(StdObjMgtPersistent::new());

        if let Some(obj) = seq.get_mut(0) {
            obj.set_ref_num(123);
        }

        assert_eq!(seq.get(0).unwrap().ref_num(), 123);
    }
}
