// FILE: top_tools_location_set.rs
// occt: TopTools_LocationSet

//! Set of topological locations for storage and retrieval.

/// Location representing position and orientation in 3D space
#[derive(Clone, Debug)]
pub struct Location {
    id: usize,
}

/// Collection of locations
pub struct TopToolsLocationSet {
    locations: Vec<Location>,
}

impl TopToolsLocationSet {
    /// Creates an empty location set
    pub fn new() -> Self {
        TopToolsLocationSet {
            locations: Vec::new(),
        }
    }

    /// Adds a location to the set and returns its index
    pub fn add(&mut self, loc: Location) -> usize {
        self.locations.push(loc);
        self.locations.len() - 1
    }

    /// Returns location at index
    pub fn location(&self, index: usize) -> Option<Location> {
        self.locations.get(index).cloned()
    }

    /// Returns number of locations in set
    pub fn count(&self) -> usize {
        self.locations.len()
    }

    /// Clears all locations
    pub fn clear(&mut self) {
        self.locations.clear();
    }

    /// Returns whether set is empty
    pub fn is_empty(&self) -> bool {
        self.locations.is_empty()
    }
}

impl Default for TopToolsLocationSet {
    fn default() -> Self {
        Self::new()
    }
}

impl Location {
    /// Creates a new location
    pub fn new(id: usize) -> Self {
        Location { id }
    }

    /// Returns location ID
    pub fn id(&self) -> usize {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_set_new() {
        let set = TopToolsLocationSet::new();
        assert!(set.is_empty());
        assert_eq!(set.count(), 0);
    }

    #[test]
    fn test_location_set_add() {
        let mut set = TopToolsLocationSet::new();
        let loc = Location::new(1);
        let idx = set.add(loc);
        assert_eq!(idx, 0);
        assert_eq!(set.count(), 1);
    }

    #[test]
    fn test_location_set_location() {
        let mut set = TopToolsLocationSet::new();
        let loc = Location::new(42);
        set.add(loc);
        let retrieved = set.location(0);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id(), 42);
    }

    #[test]
    fn test_location_set_clear() {
        let mut set = TopToolsLocationSet::new();
        set.add(Location::new(1));
        set.add(Location::new(2));
        assert_eq!(set.count(), 2);
        set.clear();
        assert_eq!(set.count(), 0);
        assert!(set.is_empty());
    }
}
