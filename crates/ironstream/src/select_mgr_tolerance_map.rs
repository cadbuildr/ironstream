// FILE: select_mgr_tolerance_map.rs
// occt: SelectMgr_ToleranceMap

use std::collections::HashMap;

/// Map for managing tolerance values in selection.
#[derive(Clone, Debug)]
pub struct ToleranceMap {
    pub tolerances: HashMap<i32, i32>,
    pub largest_key: i32,
    pub custom_tolerance: i32,
}

impl ToleranceMap {
    pub fn new() -> Self {
        Self {
            tolerances: HashMap::new(),
            largest_key: -1,
            custom_tolerance: -1,
        }
    }

    pub fn add(&mut self, tolerance: i32) {
        let count = self.tolerances.entry(tolerance).or_insert(0);
        *count += 1;

        if self.largest_key < 0 || tolerance > self.largest_key {
            self.largest_key = tolerance;
        }
    }

    pub fn decrement(&mut self, tolerance: i32) {
        if let Some(count) = self.tolerances.get_mut(&tolerance) {
            *count -= 1;
            if *count <= 0 {
                self.tolerances.remove(&tolerance);
                if tolerance == self.largest_key {
                    self.largest_key = self.tolerances.keys().max().copied().unwrap_or(-1);
                }
            }
        }
    }

    pub fn tolerance(&self) -> i32 {
        if self.largest_key < 0 {
            return 2;
        }
        if self.custom_tolerance < 0 {
            self.largest_key
        } else {
            self.largest_key + self.custom_tolerance
        }
    }

    pub fn set_custom_tolerance(&mut self, tol: i32) {
        self.custom_tolerance = tol;
    }

    pub fn reset_defaults(&mut self) {
        self.custom_tolerance = -1;
    }

    pub fn custom_tolerance(&self) -> i32 {
        self.custom_tolerance
    }

    pub fn is_custom_tol_set(&self) -> bool {
        self.custom_tolerance > 0
    }
}

impl Default for ToleranceMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_map() {
        let map = ToleranceMap::new();
        assert_eq!(map.tolerance(), 2);
    }

    #[test]
    fn add_tolerance() {
        let mut map = ToleranceMap::new();
        map.add(5);
        map.add(5);
        map.add(3);
        assert_eq!(map.largest_key, 5);
    }

    #[test]
    fn decrement_tolerance() {
        let mut map = ToleranceMap::new();
        map.add(5);
        map.add(3);
        map.decrement(5);
        assert_eq!(map.largest_key, 3);
    }

    #[test]
    fn custom_tolerance() {
        let mut map = ToleranceMap::new();
        map.add(5);
        assert!(!map.is_custom_tol_set());
        map.set_custom_tolerance(2);
        assert!(map.is_custom_tol_set());
        assert_eq!(map.tolerance(), 7);
    }

    #[test]
    fn reset_defaults() {
        let mut map = ToleranceMap::new();
        map.add(5);
        map.set_custom_tolerance(2);
        map.reset_defaults();
        assert!(!map.is_custom_tol_set());
    }
}
