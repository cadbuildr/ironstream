// FILE: b_rep_tools_purge_locations.rs
// occt: BRepTools_PurgeLocations

use std::collections::HashMap;

/// Removes redundant locations from a shape.
pub struct BRepToolsPurgeLocations {
    location_map: HashMap<u32, String>,
    removed_count: usize,
    merged_count: usize,
}

impl BRepToolsPurgeLocations {
    /// Create a new purge locations processor.
    pub fn new() -> Self {
        Self {
            location_map: HashMap::new(),
            removed_count: 0,
            merged_count: 0,
        }
    }

    /// Add a location entry.
    pub fn add_location(&mut self, id: u32, location: String) {
        self.location_map.insert(id, location);
    }

    /// Mark a location as removed.
    pub fn mark_removed(&mut self, id: u32) {
        self.location_map.remove(&id);
        self.removed_count += 1;
    }

    /// Mark locations as merged.
    pub fn mark_merged(&mut self, source: u32, target: u32) {
        if self.location_map.contains_key(&source) {
            self.location_map.remove(&source);
            self.merged_count += 1;
        }
    }

    /// Get the number of removed locations.
    pub fn removed_count(&self) -> usize {
        self.removed_count
    }

    /// Get the number of merged locations.
    pub fn merged_count(&self) -> usize {
        self.merged_count
    }

    /// Get total locations.
    pub fn total_locations(&self) -> usize {
        self.location_map.len()
    }

    /// Perform purge operation.
    pub fn purge(&mut self) -> usize {
        let before = self.location_map.len();
        // Remove entries with default/identity location
        self.location_map.retain(|_, loc| loc != "Identity");
        let after = self.location_map.len();
        before - after
    }

    /// Clear all data.
    pub fn clear(&mut self) {
        self.location_map.clear();
        self.removed_count = 0;
        self.merged_count = 0;
    }
}

impl Default for BRepToolsPurgeLocations {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = BRepToolsPurgeLocations::new();
        assert_eq!(p.total_locations(), 0);
        assert_eq!(p.removed_count(), 0);
    }

    #[test]
    fn test_add_location() {
        let mut p = BRepToolsPurgeLocations::new();
        p.add_location(1, "Trans(1,0,0)".to_string());

        assert_eq!(p.total_locations(), 1);
    }

    #[test]
    fn test_mark_removed() {
        let mut p = BRepToolsPurgeLocations::new();
        p.add_location(1, "Trans(1,0,0)".to_string());
        p.mark_removed(1);

        assert_eq!(p.total_locations(), 0);
        assert_eq!(p.removed_count(), 1);
    }

    #[test]
    fn test_purge() {
        let mut p = BRepToolsPurgeLocations::new();
        p.add_location(1, "Identity".to_string());
        p.add_location(2, "Trans(1,0,0)".to_string());

        let removed = p.purge();
        assert_eq!(removed, 1);
        assert_eq!(p.total_locations(), 1);
    }

    #[test]
    fn test_clear() {
        let mut p = BRepToolsPurgeLocations::new();
        p.add_location(1, "loc".to_string());
        p.mark_removed(1);

        p.clear();
        assert_eq!(p.total_locations(), 0);
        assert_eq!(p.removed_count(), 0);
    }
}
