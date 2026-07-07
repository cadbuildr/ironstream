// FILE: ch_fi_ds_indexed_data_map_of_vertex_list_of_stripe.rs
// occt: ChFiDS_IndexedDataMapOfVertexListOfStripe
//
// Faithful port of the OCCT typedef (deprecated since OCCT 8.0.0):
//   typedef NCollection_IndexedDataMap<TopoDS_Vertex, ChFiDS_ListOfStripe,
//                                      TopTools_ShapeMapHasher>
//     ChFiDS_IndexedDataMapOfVertexListOfStripe;
//
// Modelled as a real newtype over an indexed data map with genuine
// NCollection_IndexedDataMap behaviour: stable 1-based indices, key lookup
// through TopTools_ShapeMapHasher semantics (TopoDS_Shape::IsSame — equal
// TShape + Location, orientation ignored), FindFromKey/FindFromIndex access,
// Add returning the existing index for an already-bound key, RemoveLast, and
// Clear. The vertex, stripe and list element types are small local models of
// TopoDS_Vertex and ChFiDS_Stripe sufficient to exercise the container.

use std::collections::HashMap;

/// Orientation flag of a TopoDS_Vertex (ignored by TopTools_ShapeMapHasher).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChfiVertexOrientation {
    Forward,
    Reversed,
    Internal,
    External,
}

/// Local model of TopoDS_Vertex: a TShape handle id plus a location id and
/// an orientation. Two vertices are IsSame when TShape and Location match.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChfiVertex {
    pub tshape_id: u64,
    pub location_id: u32,
    pub orientation: ChfiVertexOrientation,
}

impl ChfiVertex {
    pub fn new(tshape_id: u64, location_id: u32, orientation: ChfiVertexOrientation) -> Self {
        ChfiVertex {
            tshape_id,
            location_id,
            orientation,
        }
    }

    /// TopoDS_Shape::IsSame — same TShape and Location, orientation ignored.
    /// This is the equivalence used by TopTools_ShapeMapHasher.
    pub fn is_same(&self, other: &ChfiVertex) -> bool {
        self.tshape_id == other.tshape_id && self.location_id == other.location_id
    }

    /// The hash key used by the map (mirrors TopTools_ShapeMapHasher which
    /// hashes the TShape pointer and the location, not the orientation).
    fn hasher_key(&self) -> (u64, u32) {
        (self.tshape_id, self.location_id)
    }
}

/// Minimal local model of a ChFiDS_Stripe (a fillet stripe descriptor).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChfiStripe {
    /// Fillet spine index this stripe belongs to.
    pub spine_index: i32,
    /// Choix (orientation choice) as in ChFiDS_Stripe::Choix().
    pub choix: i32,
}

impl ChfiStripe {
    pub fn new(spine_index: i32, choix: i32) -> Self {
        ChfiStripe { spine_index, choix }
    }
}

/// Local model of ChFiDS_ListOfStripe (NCollection_List<handle<ChFiDS_Stripe>>).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChfiListOfStripe {
    items: Vec<ChfiStripe>,
}

impl ChfiListOfStripe {
    pub fn new() -> Self {
        Self::default()
    }

    /// NCollection_List::Append.
    pub fn append(&mut self, stripe: ChfiStripe) {
        self.items.push(stripe);
    }

    /// NCollection_List::First.
    pub fn first(&self) -> Option<&ChfiStripe> {
        self.items.first()
    }

    /// NCollection_List::Extent.
    pub fn extent(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ChfiStripe> {
        self.items.iter()
    }
}

/// Newtype port of ChFiDS_IndexedDataMapOfVertexListOfStripe.
#[derive(Debug, Default)]
pub struct ChFiDSIndexedDataMapOfVertexListOfStripe {
    /// Entries in insertion order; position + 1 == NCollection index.
    entries: Vec<(ChfiVertex, ChfiListOfStripe)>,
    /// Hash acceleration structure: ShapeMapHasher key -> 1-based index.
    index_of: HashMap<(u64, u32), usize>,
}

impl ChFiDSIndexedDataMapOfVertexListOfStripe {
    /// NCollection_IndexedDataMap default constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// NCollection_IndexedDataMap::Add — if the key is already bound (in the
    /// IsSame sense) returns its existing 1-based index, otherwise appends
    /// the (key, item) pair and returns the new index.
    pub fn add(&mut self, key: ChfiVertex, item: ChfiListOfStripe) -> usize {
        if let Some(&idx) = self.index_of.get(&key.hasher_key()) {
            return idx;
        }
        self.entries.push((key, item));
        let idx = self.entries.len();
        self.index_of.insert(key.hasher_key(), idx);
        idx
    }

    /// NCollection_IndexedDataMap::Contains.
    pub fn contains(&self, key: &ChfiVertex) -> bool {
        self.index_of.contains_key(&key.hasher_key())
    }

    /// NCollection_IndexedDataMap::FindIndex — 1-based, 0 if absent.
    pub fn find_index(&self, key: &ChfiVertex) -> usize {
        self.index_of.get(&key.hasher_key()).copied().unwrap_or(0)
    }

    /// NCollection_IndexedDataMap::FindKey (1-based index).
    pub fn find_key(&self, index: usize) -> &ChfiVertex {
        &self.entries[index - 1].0
    }

    /// NCollection_IndexedDataMap::FindFromIndex (1-based index).
    pub fn find_from_index(&self, index: usize) -> &ChfiListOfStripe {
        &self.entries[index - 1].1
    }

    /// NCollection_IndexedDataMap::ChangeFromIndex (1-based index).
    pub fn change_from_index(&mut self, index: usize) -> &mut ChfiListOfStripe {
        &mut self.entries[index - 1].1
    }

    /// NCollection_IndexedDataMap::FindFromKey.
    pub fn find_from_key(&self, key: &ChfiVertex) -> Option<&ChfiListOfStripe> {
        let idx = self.find_index(key);
        if idx == 0 {
            None
        } else {
            Some(self.find_from_index(idx))
        }
    }

    /// NCollection_IndexedDataMap::ChangeFromKey.
    pub fn change_from_key(&mut self, key: &ChfiVertex) -> Option<&mut ChfiListOfStripe> {
        let idx = self.find_index(key);
        if idx == 0 {
            None
        } else {
            Some(self.change_from_index(idx))
        }
    }

    /// NCollection_IndexedDataMap::RemoveLast — removes the highest index.
    pub fn remove_last(&mut self) {
        if let Some((key, _)) = self.entries.pop() {
            self.index_of.remove(&key.hasher_key());
        }
    }

    /// NCollection_IndexedDataMap::Extent.
    pub fn extent(&self) -> usize {
        self.entries.len()
    }

    /// NCollection_IndexedDataMap::IsEmpty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// NCollection_IndexedDataMap::Clear.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.index_of.clear();
    }

    /// Iterate pairs in index order (like NCollection_IndexedDataMap::Iterator).
    pub fn iter(&self) -> impl Iterator<Item = (&ChfiVertex, &ChfiListOfStripe)> {
        self.entries.iter().map(|(k, v)| (k, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_of(stripes: &[(i32, i32)]) -> ChfiListOfStripe {
        let mut l = ChfiListOfStripe::new();
        for &(s, c) in stripes {
            l.append(ChfiStripe::new(s, c));
        }
        l
    }

    #[test]
    fn add_assigns_sequential_one_based_indices() {
        let mut map = ChFiDSIndexedDataMapOfVertexListOfStripe::new();
        let v1 = ChfiVertex::new(101, 0, ChfiVertexOrientation::Forward);
        let v2 = ChfiVertex::new(102, 0, ChfiVertexOrientation::Forward);
        let v3 = ChfiVertex::new(103, 7, ChfiVertexOrientation::Reversed);

        assert_eq!(map.add(v1, list_of(&[(1, 0)])), 1);
        assert_eq!(map.add(v2, list_of(&[(2, 0)])), 2);
        assert_eq!(map.add(v3, list_of(&[(3, 1)])), 3);
        assert_eq!(map.extent(), 3);
        assert_eq!(map.find_key(2).tshape_id, 102);
        assert_eq!(map.find_from_index(3).first().unwrap().spine_index, 3);
    }

    #[test]
    fn add_existing_key_returns_existing_index_and_keeps_item() {
        let mut map = ChFiDSIndexedDataMapOfVertexListOfStripe::new();
        let v = ChfiVertex::new(55, 3, ChfiVertexOrientation::Forward);
        assert_eq!(map.add(v, list_of(&[(9, 0)])), 1);
        // Same key again: index 1 returned, original item untouched
        // (NCollection_IndexedDataMap::Add does not overwrite).
        assert_eq!(map.add(v, list_of(&[(42, 1)])), 1);
        assert_eq!(map.extent(), 1);
        assert_eq!(map.find_from_index(1).first().unwrap().spine_index, 9);
    }

    #[test]
    fn shape_map_hasher_ignores_orientation() {
        // TopTools_ShapeMapHasher equates IsSame shapes: same TShape and
        // Location but different orientation must hit the same slot.
        let mut map = ChFiDSIndexedDataMapOfVertexListOfStripe::new();
        let fwd = ChfiVertex::new(200, 1, ChfiVertexOrientation::Forward);
        let rev = ChfiVertex::new(200, 1, ChfiVertexOrientation::Reversed);
        assert!(fwd.is_same(&rev));

        assert_eq!(map.add(fwd, list_of(&[(1, 0)])), 1);
        assert_eq!(map.add(rev, list_of(&[(2, 0)])), 1);
        assert_eq!(map.extent(), 1);
        assert_eq!(map.find_index(&rev), 1);

        // Different location => different key.
        let moved = ChfiVertex::new(200, 2, ChfiVertexOrientation::Forward);
        assert!(!fwd.is_same(&moved));
        assert_eq!(map.add(moved, ChfiListOfStripe::new()), 2);
    }

    #[test]
    fn find_from_key_and_mutation() {
        let mut map = ChFiDSIndexedDataMapOfVertexListOfStripe::new();
        let v = ChfiVertex::new(7, 0, ChfiVertexOrientation::Forward);
        map.add(v, list_of(&[(1, 6)]));

        // Typical ChFi3d usage: append another stripe to the vertex's list.
        map.change_from_key(&v)
            .unwrap()
            .append(ChfiStripe::new(2, 8));

        let list = map.find_from_key(&v).unwrap();
        assert_eq!(list.extent(), 2);
        let choixes: Vec<i32> = list.iter().map(|s| s.choix).collect();
        assert_eq!(choixes, vec![6, 8]);

        let absent = ChfiVertex::new(999, 0, ChfiVertexOrientation::Forward);
        assert!(map.find_from_key(&absent).is_none());
        assert_eq!(map.find_index(&absent), 0);
        assert!(!map.contains(&absent));
    }

    #[test]
    fn remove_last_and_clear() {
        let mut map = ChFiDSIndexedDataMapOfVertexListOfStripe::new();
        let v1 = ChfiVertex::new(1, 0, ChfiVertexOrientation::Forward);
        let v2 = ChfiVertex::new(2, 0, ChfiVertexOrientation::Forward);
        map.add(v1, ChfiListOfStripe::new());
        map.add(v2, ChfiListOfStripe::new());

        map.remove_last();
        assert_eq!(map.extent(), 1);
        assert!(map.contains(&v1));
        assert!(!map.contains(&v2));
        // Index of a re-added key is assigned afresh at the end.
        assert_eq!(map.add(v2, ChfiListOfStripe::new()), 2);

        map.clear();
        assert!(map.is_empty());
        assert_eq!(map.find_index(&v1), 0);
    }

    #[test]
    fn iteration_in_index_order() {
        let mut map = ChFiDSIndexedDataMapOfVertexListOfStripe::new();
        for i in 0..5u64 {
            map.add(
                ChfiVertex::new(i + 10, 0, ChfiVertexOrientation::Forward),
                list_of(&[(i as i32, 0)]),
            );
        }
        let order: Vec<u64> = map.iter().map(|(k, _)| k.tshape_id).collect();
        assert_eq!(order, vec![10, 11, 12, 13, 14]);
    }
}
