// FILE: top_ope_b_rep_ds_data_map_of_check_status.rs
// occt: TopOpeBRepDS_DataMapOfCheckStatus, TopOpeBRepDS_CheckStatus

use std::collections::HashMap;

/// CheckStatus: Status for checking operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CheckStatus {
    OK = 0,
    Invalid = 1,
    Suspicious = 2,
}

impl CheckStatus {
    pub fn from_int(value: i32) -> Self {
        match value {
            0 => CheckStatus::OK,
            1 => CheckStatus::Invalid,
            2 => CheckStatus::Suspicious,
            _ => CheckStatus::OK,
        }
    }

    pub fn as_int(&self) -> i32 {
        *self as i32
    }
}

/// ShapeKey: Simple shape identifier for hashing.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShapeKey {
    id: usize,
}

impl ShapeKey {
    pub fn new(id: usize) -> Self {
        ShapeKey { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// DataMapOfCheckStatus: Maps Shape to CheckStatus.
#[derive(Clone, Debug)]
pub struct DataMapOfCheckStatus {
    data: HashMap<ShapeKey, CheckStatus>,
}

impl DataMapOfCheckStatus {
    pub fn new() -> Self {
        DataMapOfCheckStatus {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, shape: ShapeKey, status: CheckStatus) -> bool {
        self.data.insert(shape, status).is_none()
    }

    pub fn contains(&self, shape: &ShapeKey) -> bool {
        self.data.contains_key(shape)
    }

    pub fn find(&self, shape: &ShapeKey) -> Option<CheckStatus> {
        self.data.get(shape).copied()
    }

    pub fn remove(&mut self, shape: &ShapeKey) -> bool {
        self.data.remove(shape).is_some()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ShapeKey, &CheckStatus)> {
        self.data.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for DataMapOfCheckStatus {
    fn default() -> Self {
        Self::new()
    }
}

/// DataMapIterator: Iterator for the data map.
pub struct DataMapIterator {
    entries: Vec<(ShapeKey, CheckStatus)>,
    index: usize,
}

impl DataMapIterator {
    pub fn new(map: &DataMapOfCheckStatus) -> Self {
        DataMapIterator {
            entries: map.data.iter().map(|(k, v)| (k.clone(), *v)).collect(),
            index: 0,
        }
    }

    pub fn is_more(&self) -> bool {
        self.index < self.entries.len()
    }

    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn key(&self) -> Option<&ShapeKey> {
        self.entries.get(self.index).map(|(k, _)| k)
    }

    pub fn value(&self) -> Option<CheckStatus> {
        self.entries.get(self.index).map(|(_, v)| *v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_status_variants() {
        assert_eq!(CheckStatus::OK.as_int(), 0);
        assert_eq!(CheckStatus::Invalid.as_int(), 1);
        assert_eq!(CheckStatus::Suspicious.as_int(), 2);
    }

    #[test]
    fn test_check_status_from_int() {
        assert_eq!(CheckStatus::from_int(0), CheckStatus::OK);
        assert_eq!(CheckStatus::from_int(1), CheckStatus::Invalid);
        assert_eq!(CheckStatus::from_int(2), CheckStatus::Suspicious);
    }

    #[test]
    fn test_shape_key() {
        let key = ShapeKey::new(42);
        assert_eq!(key.id(), 42);
    }

    #[test]
    fn test_data_map_bind() {
        let mut map = DataMapOfCheckStatus::new();
        let shape = ShapeKey::new(1);
        assert!(map.bind(shape.clone(), CheckStatus::OK));
        assert!(!map.bind(shape, CheckStatus::Invalid)); // Already present
    }

    #[test]
    fn test_data_map_contains() {
        let mut map = DataMapOfCheckStatus::new();
        let shape = ShapeKey::new(5);
        assert!(!map.contains(&shape));

        map.bind(shape.clone(), CheckStatus::OK);
        assert!(map.contains(&shape));
    }

    #[test]
    fn test_data_map_find() {
        let mut map = DataMapOfCheckStatus::new();
        let shape = ShapeKey::new(3);
        map.bind(shape.clone(), CheckStatus::Suspicious);

        let status = map.find(&shape).unwrap();
        assert_eq!(status, CheckStatus::Suspicious);
    }

    #[test]
    fn test_data_map_remove() {
        let mut map = DataMapOfCheckStatus::new();
        let shape = ShapeKey::new(7);
        map.bind(shape.clone(), CheckStatus::OK);

        assert_eq!(map.size(), 1);
        assert!(map.remove(&shape));
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_data_map_clear() {
        let mut map = DataMapOfCheckStatus::new();
        map.bind(ShapeKey::new(1), CheckStatus::OK);
        map.bind(ShapeKey::new(2), CheckStatus::Invalid);
        map.clear();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_data_map_iterator() {
        let mut map = DataMapOfCheckStatus::new();
        map.bind(ShapeKey::new(1), CheckStatus::OK);
        map.bind(ShapeKey::new(2), CheckStatus::Invalid);

        let mut iter = DataMapIterator::new(&map);
        let mut count = 0;
        while iter.is_more() {
            assert!(iter.key().is_some());
            assert!(iter.value().is_some());
            iter.next();
            count += 1;
        }
        assert_eq!(count, 2);
    }
}
