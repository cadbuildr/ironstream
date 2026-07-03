// FILE: topo_loc_ext.rs
// occt: TopLoc_Location, TopLoc_SHasher, TopLoc_IndexedMapOfLocation

/// A composed location: sequence of (displacement_id, power) pairs.
/// power is typically +1 (forward) or -1 (inverse).
/// occt: TopLoc_Location
#[derive(Clone, Debug, Default)]
pub struct TopoLocLocation {
    pub items: Vec<(u32, i32)>,  // (displacement_id, power)
}

impl TopoLocLocation {
    pub fn new() -> Self { Self::default() }

    pub fn identity() -> Self { Self::default() }

    pub fn from_displacement(disp_id: u32) -> Self {
        Self { items: vec![(disp_id, 1)] }
    }

    pub fn is_identity(&self) -> bool { self.items.is_empty() }

    /// Compose with another location: append its items.
    pub fn multiply(&self, other: &TopoLocLocation) -> Self {
        let mut items = self.items.clone();
        items.extend_from_slice(&other.items);
        Self { items }
    }

    /// Inverse location: reverse order and negate powers.
    pub fn inverted(&self) -> Self {
        let items = self.items.iter().rev().map(|(id, p)| (*id, -p)).collect();
        Self { items }
    }

    /// Power of the location.
    pub fn power(&self, p: i32) -> Self {
        if p == 0 { return Self::identity(); }
        let items = self.items.iter().map(|(id, pow)| (*id, pow * p)).collect();
        Self { items }
    }

    pub fn depth(&self) -> usize { self.items.len() }

    /// Hash of the location (simple XOR of displacement ids and powers).
    pub fn hash_code(&self) -> u64 {
        let mut h: u64 = 0;
        for (id, p) in &self.items {
            h ^= (*id as u64).wrapping_mul(31).wrapping_add(*p as u64);
        }
        h
    }

    /// Check equality by comparing item lists.
    pub fn is_equal(&self, other: &TopoLocLocation, tol: f64) -> bool {
        let _ = tol;
        self.items == other.items
    }
}

impl PartialEq for TopoLocLocation {
    fn eq(&self, other: &Self) -> bool { self.items == other.items }
}

/// Hash utility for TopLoc_Location.
/// occt: TopLoc_SHasher
pub struct TopoLocSHasher;

impl TopoLocSHasher {
    pub fn hash_code(loc: &TopoLocLocation, upper: u64) -> u64 {
        loc.hash_code() % upper.max(1)
    }

    pub fn is_equal(a: &TopoLocLocation, b: &TopoLocLocation) -> bool {
        a == b
    }
}

/// Indexed map of locations (1-based).
/// occt: TopLoc_IndexedMapOfLocation
#[derive(Clone, Debug, Default)]
pub struct TopoLocIndexedMapOfLocation {
    pub entries: Vec<TopoLocLocation>,
}

impl TopoLocIndexedMapOfLocation {
    pub fn new() -> Self { Self::default() }

    /// Add if not already present; return 1-based index.
    pub fn add(&mut self, loc: TopoLocLocation) -> usize {
        if let Some(pos) = self.entries.iter().position(|e| e == &loc) {
            return pos + 1;
        }
        self.entries.push(loc);
        self.entries.len()
    }

    pub fn find_index(&self, loc: &TopoLocLocation) -> Option<usize> {
        self.entries.iter().position(|e| e == loc).map(|p| p + 1)
    }

    pub fn find_key(&self, idx: usize) -> Option<&TopoLocLocation> {
        self.entries.get(idx.saturating_sub(1))
    }

    pub fn extent(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    pub fn clear(&mut self) { self.entries.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_is_identity() {
        let loc = TopoLocLocation::identity();
        assert!(loc.is_identity());
        assert_eq!(loc.depth(), 0);
    }

    #[test]
    fn multiply_locations() {
        let a = TopoLocLocation::from_displacement(1);
        let b = TopoLocLocation::from_displacement(2);
        let c = a.multiply(&b);
        assert_eq!(c.depth(), 2);
        assert!(!c.is_identity());
    }

    #[test]
    fn inverted_location() {
        let loc = TopoLocLocation::from_displacement(5);
        let inv = loc.inverted();
        assert_eq!(inv.items[0], (5, -1));
    }

    #[test]
    fn power_location() {
        let loc = TopoLocLocation::from_displacement(3);
        let p2 = loc.power(2);
        assert_eq!(p2.items[0].1, 2);
        let p0 = loc.power(0);
        assert!(p0.is_identity());
    }

    #[test]
    fn indexed_map_add_find() {
        let mut m = TopoLocIndexedMapOfLocation::new();
        let l1 = TopoLocLocation::from_displacement(1);
        let l2 = TopoLocLocation::from_displacement(2);
        let i1 = m.add(l1.clone());
        let i2 = m.add(l2.clone());
        let i1b = m.add(l1.clone()); // dup
        assert_eq!(i1, 1);
        assert_eq!(i2, 2);
        assert_eq!(i1b, 1); // same index
        assert_eq!(m.extent(), 2);
        assert_eq!(m.find_index(&l1), Some(1));
        assert!(m.find_key(1).is_some());
    }
}
