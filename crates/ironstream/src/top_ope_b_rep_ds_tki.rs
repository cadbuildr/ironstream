// FILE: top_ope_b_rep_ds_tki.rs
// occt: TopOpeBRepDS_TKI

use std::collections::HashMap;

/// Kind enumeration for elements
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TopOpeBRepDSKind {
    Point = 1,
    Curve = 2,
    Surface = 3,
    Solid = 4,
    Shell = 5,
    Face = 6,
    Wire = 7,
    Edge = 8,
    Vertex = 9,
}

/// Interference information
#[derive(Clone, Debug)]
pub struct TopOpeBRepDSInterference {
    id: i32,
}

impl TopOpeBRepDSInterference {
    pub fn new(id: i32) -> Self {
        TopOpeBRepDSInterference { id }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

/// Tool for managing interferences indexed by Kind and geometry ID.
/// Maps (Kind, GeometryID) -> List of Interferences
pub struct TopOpeBRepDSTKI {
    // Map from (Kind, GeometryID) to list of interferences
    map: HashMap<(TopOpeBRepDSKind, i32), Vec<TopOpeBRepDSInterference>>,
    // Iterator state
    iter_vec: Vec<(TopOpeBRepDSKind, i32, Vec<TopOpeBRepDSInterference>)>,
    iter_index: usize,
}

impl TopOpeBRepDSTKI {
    /// Create a new TKI
    pub fn new() -> Self {
        TopOpeBRepDSTKI {
            map: HashMap::new(),
            iter_vec: Vec::new(),
            iter_index: 0,
        }
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.map.clear();
        self.iter_vec.clear();
        self.iter_index = 0;
    }

    /// Check if a (Kind, GeometryID) pair is in the map
    pub fn is_bound(&self, kind: TopOpeBRepDSKind, geometry_id: i32) -> bool {
        self.map.contains_key(&(kind, geometry_id))
    }

    /// Get interferences for (Kind, GeometryID)
    pub fn interferences(
        &self,
        kind: TopOpeBRepDSKind,
        geometry_id: i32,
    ) -> Vec<TopOpeBRepDSInterference> {
        self.map
            .get(&(kind, geometry_id))
            .cloned()
            .unwrap_or_default()
    }

    /// Check if (Kind, GeometryID) has interferences
    pub fn has_interferences(&self, kind: TopOpeBRepDSKind, geometry_id: i32) -> bool {
        self.map
            .get(&(kind, geometry_id))
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    }

    /// Add a (Kind, GeometryID) entry
    pub fn add(&mut self, kind: TopOpeBRepDSKind, geometry_id: i32) {
        self.map
            .entry((kind, geometry_id))
            .or_insert_with(Vec::new);
    }

    /// Add an interference to (Kind, GeometryID)
    pub fn add_interference(
        &mut self,
        kind: TopOpeBRepDSKind,
        geometry_id: i32,
        interference: TopOpeBRepDSInterference,
    ) {
        self.map
            .entry((kind, geometry_id))
            .or_insert_with(Vec::new)
            .push(interference);
    }

    /// Initialize iterator
    pub fn init(&mut self) {
        self.iter_vec.clear();
        for ((kind, geom_id), interfs) in &self.map {
            self.iter_vec
                .push((*kind, *geom_id, interfs.clone()));
        }
        self.iter_index = 0;
    }

    /// Check if iterator has more elements
    pub fn more(&self) -> bool {
        self.iter_index < self.iter_vec.len()
    }

    /// Move to next element
    pub fn next(&mut self) {
        if self.more() {
            self.iter_index += 1;
        }
    }

    /// Get current value (Kind, GeometryID, Interferences)
    pub fn value(&self) -> Option<(TopOpeBRepDSKind, i32, Vec<TopOpeBRepDSInterference>)> {
        self.iter_vec.get(self.iter_index).cloned()
    }
}

impl Default for TopOpeBRepDSTKI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tki() {
        let tki = TopOpeBRepDSTKI::new();
        assert!(!tki.is_bound(TopOpeBRepDSKind::Point, 1));
    }

    #[test]
    fn test_add_entry() {
        let mut tki = TopOpeBRepDSTKI::new();
        tki.add(TopOpeBRepDSKind::Point, 1);
        assert!(tki.is_bound(TopOpeBRepDSKind::Point, 1));
    }

    #[test]
    fn test_add_interference() {
        let mut tki = TopOpeBRepDSTKI::new();
        let interf = TopOpeBRepDSInterference::new(10);
        tki.add_interference(TopOpeBRepDSKind::Point, 1, interf);
        assert!(tki.has_interferences(TopOpeBRepDSKind::Point, 1));
    }

    #[test]
    fn test_get_interferences() {
        let mut tki = TopOpeBRepDSTKI::new();
        let interf1 = TopOpeBRepDSInterference::new(10);
        let interf2 = TopOpeBRepDSInterference::new(20);
        tki.add_interference(TopOpeBRepDSKind::Curve, 5, interf1);
        tki.add_interference(TopOpeBRepDSKind::Curve, 5, interf2);
        let interfs = tki.interferences(TopOpeBRepDSKind::Curve, 5);
        assert_eq!(interfs.len(), 2);
    }

    #[test]
    fn test_iterator() {
        let mut tki = TopOpeBRepDSTKI::new();
        tki.add(TopOpeBRepDSKind::Point, 1);
        tki.add(TopOpeBRepDSKind::Curve, 2);
        tki.init();
        assert!(tki.more());
        let mut count = 0;
        while tki.more() {
            count += 1;
            tki.next();
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn test_clear() {
        let mut tki = TopOpeBRepDSTKI::new();
        tki.add(TopOpeBRepDSKind::Point, 1);
        assert!(tki.is_bound(TopOpeBRepDSKind::Point, 1));
        tki.clear();
        assert!(!tki.is_bound(TopOpeBRepDSKind::Point, 1));
    }
}
