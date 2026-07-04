// FILE: int_polyh_array_of_tangent_zones.rs
// occt: IntPolyh_ArrayOfTangentZones

//! Array of tangent zones between polyhedra.

/// Tangent zone between two polyhedra
#[derive(Clone)]
pub struct TangentZone {
    pub zone_id: i32,
    pub triangle1: i32,
    pub triangle2: i32,
}

/// Array of tangent zones
pub struct IntPolyhArrayOfTangentZones {
    zones: Vec<TangentZone>,
}

impl IntPolyhArrayOfTangentZones {
    /// Creates empty array
    pub fn new() -> Self {
        IntPolyhArrayOfTangentZones { zones: Vec::new() }
    }

    /// Returns number of zones
    pub fn count(&self) -> i32 {
        self.zones.len() as i32
    }

    /// Returns zone at index
    pub fn zone(&self, index: i32) -> Option<TangentZone> {
        self.zones.get(index as usize).cloned()
    }

    /// Adds a tangent zone
    pub fn add(&mut self, zone: TangentZone) {
        self.zones.push(zone);
    }

    /// Clears all zones
    pub fn clear(&mut self) {
        self.zones.clear();
    }
}

impl Default for IntPolyhArrayOfTangentZones {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tangent_zones_new() {
        let zones = IntPolyhArrayOfTangentZones::new();
        assert_eq!(zones.count(), 0);
    }

    #[test]
    fn test_tangent_zones_add() {
        let mut zones = IntPolyhArrayOfTangentZones::new();
        zones.add(TangentZone {
            zone_id: 0,
            triangle1: 0,
            triangle2: 0,
        });
        assert_eq!(zones.count(), 1);
    }

    #[test]
    fn test_tangent_zones_clear() {
        let mut zones = IntPolyhArrayOfTangentZones::new();
        zones.add(TangentZone {
            zone_id: 0,
            triangle1: 0,
            triangle2: 0,
        });
        zones.clear();
        assert_eq!(zones.count(), 0);
    }
}
