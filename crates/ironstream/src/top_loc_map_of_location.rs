// FILE: top_loc_map_of_location.rs
// occt: TopLoc_MapOfLocation, TopLoc_Location, TopLoc_MapIteratorOfMapOfLocation

use std::collections::HashSet;

/// TopLoc_Location: A composite transformation object (simplified for map purposes).
///
/// For this map implementation, we only need a hashable representation.
/// In OCCT, locations track coordinate transformations; here we use a simple
/// wrapper that can be hashed and compared.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Location {
    id: usize, // Unique identifier for the location
}

impl Location {
    /// Creates a new location with a given ID.
    pub fn new(id: usize) -> Self {
        Location { id }
    }

    /// Returns the location ID.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns the identity location (ID 0).
    pub fn identity() -> Self {
        Location { id: 0 }
    }

    /// Checks if this location is the identity transformation.
    pub fn is_identity(&self) -> bool {
        self.id == 0
    }
}

impl Default for Location {
    fn default() -> Self {
        Location::identity()
    }
}

/// TopLoc_MapOfLocation: A set-like container for Location objects.
///
/// This is a deprecated typedef wrapper over std::collections::HashSet<TopLoc_Location>.
#[derive(Clone, Debug)]
pub struct MapOfLocation {
    data: HashSet<Location>,
}

impl MapOfLocation {
    /// Creates a new empty map.
    pub fn new() -> Self {
        MapOfLocation {
            data: HashSet::new(),
        }
    }

    /// Adds a location to the map.
    /// Returns true if the location was newly inserted, false if it was already present.
    pub fn add(&mut self, location: Location) -> bool {
        self.data.insert(location)
    }

    /// Removes a location from the map.
    /// Returns true if the location was present, false otherwise.
    pub fn remove(&mut self, location: &Location) -> bool {
        self.data.remove(location)
    }

    /// Returns true if the map contains the given location.
    pub fn contains(&self, location: &Location) -> bool {
        self.data.contains(location)
    }

    /// Returns the number of locations in the map.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Returns the number of locations in the map (OCCT alias).
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Clears the map.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Returns an iterator over the locations.
    pub fn iter(&self) -> impl Iterator<Item = &Location> {
        self.data.iter()
    }

    /// Returns true if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for MapOfLocation {
    fn default() -> Self {
        Self::new()
    }
}

/// TopLoc_MapIteratorOfMapOfLocation: Iterator for MapOfLocation.
pub struct MapIterator {
    locations: Vec<Location>,
    index: usize,
}

impl MapIterator {
    /// Creates a new iterator over the map.
    pub fn new(map: &MapOfLocation) -> Self {
        MapIterator {
            locations: map.data.iter().cloned().collect(),
            index: 0,
        }
    }

    /// Returns true if there is a next element.
    pub fn is_more(&self) -> bool {
        self.index < self.locations.len()
    }

    /// Advances to the next element.
    pub fn next(&mut self) {
        self.index += 1;
    }

    /// Returns the current location.
    pub fn current(&self) -> Option<&Location> {
        self.locations.get(self.index)
    }

    /// Returns the current location (OCCT alias).
    pub fn value(&self) -> Option<&Location> {
        self.current()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_new() {
        let loc = Location::new(42);
        assert_eq!(loc.id(), 42);
        assert!(!loc.is_identity());
    }

    #[test]
    fn test_location_identity() {
        let loc = Location::identity();
        assert!(loc.is_identity());
        assert_eq!(loc.id(), 0);
    }

    #[test]
    fn test_location_default() {
        let loc = Location::default();
        assert_eq!(loc.id(), 0);
    }

    #[test]
    fn test_map_new() {
        let map = MapOfLocation::new();
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_map_add() {
        let mut map = MapOfLocation::new();
        let loc1 = Location::new(1);
        let loc2 = Location::new(2);

        assert!(map.add(loc1.clone()));
        assert!(!map.add(loc1.clone())); // Adding same location returns false
        assert!(map.add(loc2.clone()));
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_map_contains() {
        let mut map = MapOfLocation::new();
        let loc = Location::new(5);

        assert!(!map.contains(&loc));
        map.add(loc.clone());
        assert!(map.contains(&loc));
    }

    #[test]
    fn test_map_remove() {
        let mut map = MapOfLocation::new();
        let loc = Location::new(3);

        map.add(loc.clone());
        assert_eq!(map.size(), 1);
        assert!(map.remove(&loc));
        assert_eq!(map.size(), 0);
        assert!(!map.remove(&loc)); // Already removed
    }

    #[test]
    fn test_map_clear() {
        let mut map = MapOfLocation::new();
        map.add(Location::new(1));
        map.add(Location::new(2));
        map.clear();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_map_iterator() {
        let mut map = MapOfLocation::new();
        map.add(Location::new(1));
        map.add(Location::new(2));
        map.add(Location::new(3));

        let mut iter = MapIterator::new(&map);
        let mut count = 0;
        while iter.is_more() {
            assert!(iter.current().is_some());
            iter.next();
            count += 1;
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn test_map_iter() {
        let mut map = MapOfLocation::new();
        map.add(Location::new(1));
        map.add(Location::new(2));

        let collected: Vec<_> = map.iter().collect();
        assert_eq!(collected.len(), 2);
    }
}
