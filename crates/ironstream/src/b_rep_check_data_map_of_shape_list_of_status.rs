// FILE: b_rep_check_data_map_of_shape_list_of_status.rs
// occt: BRepCheck_DataMapOfShapeListOfStatus

use std::collections::HashMap;

/// Status enumeration for shape checking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Ok,
    InvalidRange,
    EmptyWire,
    RedundantEdge,
    SelfIntersectingWire,
    SelfIntersectingFace,
}

/// Data map from shape ID to list of status values.
pub struct BrepcheckDataMapOfShapeListOfStatus {
    data: HashMap<usize, Vec<Status>>,
}

impl BrepcheckDataMapOfShapeListOfStatus {
    /// Creates a new empty data map.
    pub fn new() -> Self {
        BrepcheckDataMapOfShapeListOfStatus {
            data: HashMap::new(),
        }
    }

    /// Adds a status to the list for a shape.
    pub fn add(&mut self, shape_id: usize, status: Status) {
        self.data.entry(shape_id).or_insert_with(Vec::new).push(status);
    }

    /// Gets the list of statuses for a shape.
    pub fn get(&self, shape_id: usize) -> Option<&[Status]> {
        self.data.get(&shape_id).map(|v| v.as_slice())
    }

    /// Returns the number of shapes with statuses.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Removes all statuses for a shape.
    pub fn remove(&mut self, shape_id: usize) -> Option<Vec<Status>> {
        self.data.remove(&shape_id)
    }

    /// Clears all data.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Returns an iterator over shape IDs.
    pub fn keys(&self) -> impl Iterator<Item = &usize> {
        self.data.keys()
    }

    /// Returns an iterator over (shape_id, statuses) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (usize, &Vec<Status>)> {
        self.data.iter().map(|(k, v)| (*k, v))
    }

    /// Checks if a shape has any status.
    pub fn contains(&self, shape_id: usize) -> bool {
        self.data.contains_key(&shape_id)
    }
}

impl Default for BrepcheckDataMapOfShapeListOfStatus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let map = BrepcheckDataMapOfShapeListOfStatus::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_add_status() {
        let mut map = BrepcheckDataMapOfShapeListOfStatus::new();
        map.add(1, Status::Ok);
        assert_eq!(map.len(), 1);
        assert!(map.contains(1));
    }

    #[test]
    fn test_get_status() {
        let mut map = BrepcheckDataMapOfShapeListOfStatus::new();
        map.add(5, Status::InvalidRange);
        map.add(5, Status::EmptyWire);
        let statuses = map.get(5).unwrap();
        assert_eq!(statuses.len(), 2);
        assert!(statuses.contains(&Status::InvalidRange));
        assert!(statuses.contains(&Status::EmptyWire));
    }

    #[test]
    fn test_multiple_shapes() {
        let mut map = BrepcheckDataMapOfShapeListOfStatus::new();
        map.add(1, Status::Ok);
        map.add(2, Status::InvalidRange);
        map.add(3, Status::SelfIntersectingWire);
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn test_remove() {
        let mut map = BrepcheckDataMapOfShapeListOfStatus::new();
        map.add(1, Status::Ok);
        assert!(map.contains(1));
        let removed = map.remove(1);
        assert!(removed.is_some());
        assert!(!map.contains(1));
    }

    #[test]
    fn test_clear() {
        let mut map = BrepcheckDataMapOfShapeListOfStatus::new();
        map.add(1, Status::Ok);
        map.add(2, Status::InvalidRange);
        assert_eq!(map.len(), 2);
        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_keys_iterator() {
        let mut map = BrepcheckDataMapOfShapeListOfStatus::new();
        map.add(1, Status::Ok);
        map.add(2, Status::InvalidRange);
        map.add(3, Status::EmptyWire);
        let keys: Vec<_> = map.keys().copied().collect();
        assert_eq!(keys.len(), 3);
    }

    #[test]
    fn test_iter() {
        let mut map = BrepcheckDataMapOfShapeListOfStatus::new();
        map.add(1, Status::Ok);
        map.add(2, Status::InvalidRange);
        let pairs: Vec<_> = map.iter().collect();
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn test_status_variants() {
        assert_eq!(Status::Ok, Status::Ok);
        assert_ne!(Status::Ok, Status::InvalidRange);
    }

    #[test]
    fn test_add_multiple_to_same_shape() {
        let mut map = BrepcheckDataMapOfShapeListOfStatus::new();
        map.add(1, Status::Ok);
        map.add(1, Status::InvalidRange);
        map.add(1, Status::EmptyWire);
        let statuses = map.get(1).unwrap();
        assert_eq!(statuses.len(), 3);
        assert_eq!(map.len(), 1);
    }
}
