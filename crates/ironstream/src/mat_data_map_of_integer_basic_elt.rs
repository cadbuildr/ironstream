// FILE: mat_data_map_of_integer_basic_elt.rs
// occt: MAT_DataMapOfIntegerBasicElt
// occt-ref: MAT_DataMapIteratorOfDataMapOfIntegerBasicElt

use std::collections::BTreeMap;

pub struct MATDataMapOfIntegerBasicElt {
    items: BTreeMap<i32, u32>,
}

impl MATDataMapOfIntegerBasicElt {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    pub fn bind(&mut self, key: i32, value: u32) {
        self.items.insert(key, value);
    }

    pub fn unbind(&mut self, key: i32) -> bool {
        self.items.remove(&key).is_some()
    }

    pub fn find(&self, key: i32) -> Option<u32> {
        self.items.get(&key).copied()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iterator(&self) -> MATDataMapIteratorOfDataMapOfIntegerBasicElt {
        MATDataMapIteratorOfDataMapOfIntegerBasicElt {
            items: self.items.iter().map(|(k, v)| (*k, *v)).collect(),
            index: 0,
        }
    }
}

impl Default for MATDataMapOfIntegerBasicElt {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MATDataMapIteratorOfDataMapOfIntegerBasicElt {
    items: Vec<(i32, u32)>,
    index: usize,
}

impl MATDataMapIteratorOfDataMapOfIntegerBasicElt {
    pub fn more(&self) -> bool {
        self.index < self.items.len()
    }

    pub fn next(&mut self) {
        if self.index < self.items.len() {
            self.index += 1;
        }
    }

    pub fn key(&self) -> Option<i32> {
        if self.index < self.items.len() {
            Some(self.items[self.index].0)
        } else {
            None
        }
    }

    pub fn value(&self) -> Option<u32> {
        if self.index < self.items.len() {
            Some(self.items[self.index].1)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let mut map = MATDataMapOfIntegerBasicElt::new();
        map.bind(1, 100);
        assert_eq!(map.find(1), Some(100));
        assert!(map.unbind(1));
        assert_eq!(map.find(1), None);
    }
}
