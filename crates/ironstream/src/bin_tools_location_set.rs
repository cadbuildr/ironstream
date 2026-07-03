// FILE: bin_tools_location_set.rs
// occt: BinTools_LocationSet

use std::collections::HashMap;

/// Manages a set of locations with serialization support.
pub struct BinToolsLocationSet {
    locations: HashMap<u32, String>,
    next_id: u32,
}

impl BinToolsLocationSet {
    /// Create a new location set.
    pub fn new() -> Self {
        Self {
            locations: HashMap::new(),
            next_id: 1,
        }
    }

    /// Add a location.
    pub fn add_location(&mut self, location_str: String) -> u32 {
        let id = self.next_id;
        self.locations.insert(id, location_str);
        self.next_id += 1;
        id
    }

    /// Get a location by id.
    pub fn get_location(&self, id: u32) -> Option<&str> {
        self.locations.get(&id).map(|s| s.as_str())
    }

    /// Get the number of locations.
    pub fn location_count(&self) -> usize {
        self.locations.len()
    }

    /// Check if a location exists.
    pub fn contains(&self, id: u32) -> bool {
        self.locations.contains_key(&id)
    }

    /// Clear all locations.
    pub fn clear(&mut self) {
        self.locations.clear();
        self.next_id = 1;
    }

    /// Write the location set to bytes.
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(self.location_count().to_le_bytes());
        for (&id, loc) in &self.locations {
            result.extend(id.to_le_bytes());
            result.extend((loc.len() as u32).to_le_bytes());
            result.extend(loc.as_bytes());
        }
        result
    }
}

impl Default for BinToolsLocationSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let set = BinToolsLocationSet::new();
        assert_eq!(set.location_count(), 0);
    }

    #[test]
    fn test_add_location() {
        let mut set = BinToolsLocationSet::new();
        let id = set.add_location("Identity".to_string());

        assert_eq!(id, 1);
        assert_eq!(set.get_location(1), Some("Identity"));
        assert_eq!(set.location_count(), 1);
    }

    #[test]
    fn test_contains() {
        let mut set = BinToolsLocationSet::new();
        let id = set.add_location("Trans(1,0,0)".to_string());

        assert!(set.contains(id));
        assert!(!set.contains(id + 1));
    }

    #[test]
    fn test_clear() {
        let mut set = BinToolsLocationSet::new();
        set.add_location("loc".to_string());
        set.clear();

        assert_eq!(set.location_count(), 0);
    }

    #[test]
    fn test_serialize() {
        let mut set = BinToolsLocationSet::new();
        set.add_location("id".to_string());

        let data = set.serialize();
        assert!(!data.is_empty());
    }
}
