// FILE: b_rep_check_indexed_data_map_of_shape_result.rs
// occt: BRepCheck_IndexedDataMapOfShapeResult

use std::sync::Arc;
use std::collections::BTreeMap;

/// Check result information for a shape.
#[derive(Debug, Clone)]
pub struct Result {
    result_id: usize,
    error_count: i32,
    warning_count: i32,
}

impl Result {
    pub fn new(result_id: usize) -> Self {
        Result {
            result_id,
            error_count: 0,
            warning_count: 0,
        }
    }

    pub fn id(&self) -> usize {
        self.result_id
    }

    pub fn error_count(&self) -> i32 {
        self.error_count
    }

    pub fn warning_count(&self) -> i32 {
        self.warning_count
    }

    pub fn add_error(&mut self) {
        self.error_count += 1;
    }

    pub fn add_warning(&mut self) {
        self.warning_count += 1;
    }
}

/// Indexed data map from shape ID to check result (handle).
pub struct BrepcheckIndexedDataMapOfShapeResult {
    data: BTreeMap<usize, (usize, Arc<Result>)>,
    next_index: usize,
}

impl BrepcheckIndexedDataMapOfShapeResult {
    /// Creates a new empty map.
    pub fn new() -> Self {
        BrepcheckIndexedDataMapOfShapeResult {
            data: BTreeMap::new(),
            next_index: 1,
        }
    }

    /// Adds a shape and result, returning its index.
    pub fn add(&mut self, shape_id: usize, result: Arc<Result>) -> usize {
        let index = self.next_index;
        self.data.insert(index, (shape_id, result));
        self.next_index += 1;
        index
    }

    /// Gets result by 1-based index.
    pub fn get(&self, index: usize) -> Option<&Arc<Result>> {
        self.data.get(&index).map(|(_, r)| r)
    }

    /// Gets the shape ID by index.
    pub fn get_shape_id(&self, index: usize) -> Option<usize> {
        self.data.get(&index).map(|(s, _)| *s)
    }

    /// Finds the index of a shape.
    pub fn find_index(&self, shape_id: usize) -> Option<usize> {
        for (&idx, &(sid, _)) in &self.data {
            if sid == shape_id {
                return Some(idx);
            }
        }
        None
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Removes an entry by index.
    pub fn remove(&mut self, index: usize) -> Option<Arc<Result>> {
        self.data.remove(&index).map(|(_, r)| r)
    }

    /// Clears the map.
    pub fn clear(&mut self) {
        self.data.clear();
        self.next_index = 1;
    }

    /// Returns an iterator.
    pub fn iter(&self) -> impl Iterator<Item = (usize, &Arc<Result>)> {
        self.data.iter().map(|(&idx, (_, r))| (idx, r))
    }
}

impl Default for BrepcheckIndexedDataMapOfShapeResult {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_creation() {
        let result = Result::new(1);
        assert_eq!(result.id(), 1);
        assert_eq!(result.error_count(), 0);
        assert_eq!(result.warning_count(), 0);
    }

    #[test]
    fn test_result_errors_warnings() {
        let mut result = Result::new(1);
        result.add_error();
        result.add_error();
        result.add_warning();
        assert_eq!(result.error_count(), 2);
        assert_eq!(result.warning_count(), 1);
    }

    #[test]
    fn test_map_creation() {
        let map = BrepcheckIndexedDataMapOfShapeResult::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_map_add() {
        let mut map = BrepcheckIndexedDataMapOfShapeResult::new();
        let result = Arc::new(Result::new(1));
        let idx = map.add(100, result);
        assert_eq!(idx, 1);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_map_get() {
        let mut map = BrepcheckIndexedDataMapOfShapeResult::new();
        let result = Arc::new(Result::new(42));
        let idx = map.add(100, result.clone());
        assert_eq!(map.get(idx).unwrap().id(), 42);
    }

    #[test]
    fn test_map_get_shape_id() {
        let mut map = BrepcheckIndexedDataMapOfShapeResult::new();
        let result = Arc::new(Result::new(1));
        let idx = map.add(555, result);
        assert_eq!(map.get_shape_id(idx), Some(555));
    }

    #[test]
    fn test_map_find_index() {
        let mut map = BrepcheckIndexedDataMapOfShapeResult::new();
        let result = Arc::new(Result::new(1));
        let idx = map.add(777, result);
        assert_eq!(map.find_index(777), Some(idx));
        assert_eq!(map.find_index(999), None);
    }

    #[test]
    fn test_map_multiple() {
        let mut map = BrepcheckIndexedDataMapOfShapeResult::new();
        for i in 1..=5 {
            let result = Arc::new(Result::new(i));
            map.add(i * 100, result);
        }
        assert_eq!(map.len(), 5);
    }

    #[test]
    fn test_map_remove() {
        let mut map = BrepcheckIndexedDataMapOfShapeResult::new();
        let result = Arc::new(Result::new(1));
        let idx = map.add(100, result);
        assert_eq!(map.len(), 1);
        map.remove(idx);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_map_clear() {
        let mut map = BrepcheckIndexedDataMapOfShapeResult::new();
        for i in 1..=3 {
            let result = Arc::new(Result::new(i));
            map.add(i * 100, result);
        }
        assert_eq!(map.len(), 3);
        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_iter() {
        let mut map = BrepcheckIndexedDataMapOfShapeResult::new();
        for i in 1..=3 {
            let result = Arc::new(Result::new(i));
            map.add(i, result);
        }
        let count = map.iter().count();
        assert_eq!(count, 3);
    }
}
