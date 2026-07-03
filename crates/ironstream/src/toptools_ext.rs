// FILE: toptools_ext.rs
// occt: TopTools_IndexedMapOfShape, TopTools_ListOfShape,
//       TopTools_DataMapOfShapeShape, TopTools_MapOfShape,
//       TopTools_IndexedDataMapOfShapeListOfShape

/// Minimal shape reference (substitutes TopoDS_Shape handle).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShapeRef {
    pub id: u32,
    pub shape_type: u8,  // 0=compound, 1=solid, 2=shell, 3=face, 4=wire, 5=edge, 6=vertex
}

impl ShapeRef {
    pub fn new(id: u32, shape_type: u8) -> Self { Self { id, shape_type } }
    pub fn is_null(&self) -> bool { self.id == 0 }
    pub fn shape_type_name(&self) -> &'static str {
        match self.shape_type {
            0 => "Compound", 1 => "Solid", 2 => "Shell",
            3 => "Face", 4 => "Wire", 5 => "Edge", 6 => "Vertex",
            _ => "Unknown",
        }
    }
}

// occt: TopTools_IndexedMapOfShape — maps integer indices 1..N to shapes
#[derive(Clone, Debug, Default)]
pub struct IndexedMapOfShape {
    pub shapes: Vec<ShapeRef>,
}

impl IndexedMapOfShape {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, s: ShapeRef) -> u32 {
        if let Some(pos) = self.shapes.iter().position(|x| x == &s) {
            return pos as u32 + 1;
        }
        self.shapes.push(s);
        self.shapes.len() as u32
    }

    pub fn find_index(&self, s: &ShapeRef) -> Option<u32> {
        self.shapes.iter().position(|x| x == s).map(|i| i as u32 + 1)
    }

    pub fn find_from_index(&self, idx: u32) -> Option<&ShapeRef> {
        self.shapes.get((idx as usize).saturating_sub(1))
    }

    pub fn extent(&self) -> usize { self.shapes.len() }
    pub fn contains(&self, s: &ShapeRef) -> bool { self.shapes.contains(s) }
    pub fn clear(&mut self) { self.shapes.clear(); }
    pub fn is_empty(&self) -> bool { self.shapes.is_empty() }
}

// occt: TopTools_ListOfShape — ordered list of shape references
#[derive(Clone, Debug, Default)]
pub struct ListOfShape {
    pub items: Vec<ShapeRef>,
}

impl ListOfShape {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, s: ShapeRef) { self.items.push(s); }
    pub fn prepend(&mut self, s: ShapeRef) { self.items.insert(0, s); }

    pub fn remove_first(&mut self) -> Option<ShapeRef> {
        if self.items.is_empty() { None } else { Some(self.items.remove(0)) }
    }

    pub fn first(&self) -> Option<&ShapeRef> { self.items.first() }
    pub fn last(&self) -> Option<&ShapeRef> { self.items.last() }
    pub fn size(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn clear(&mut self) { self.items.clear(); }

    pub fn contains(&self, s: &ShapeRef) -> bool { self.items.contains(s) }

    pub fn append_list(&mut self, other: &ListOfShape) {
        self.items.extend(other.items.iter().cloned());
    }
}

// occt: TopTools_MapOfShape — set of distinct shapes (no duplicates)
#[derive(Clone, Debug, Default)]
pub struct MapOfShape {
    pub shapes: Vec<ShapeRef>,
}

impl MapOfShape {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, s: ShapeRef) -> bool {
        if self.shapes.contains(&s) { return false; }
        self.shapes.push(s);
        true
    }

    pub fn remove(&mut self, s: &ShapeRef) -> bool {
        let before = self.shapes.len();
        self.shapes.retain(|x| x != s);
        self.shapes.len() < before
    }

    pub fn contains(&self, s: &ShapeRef) -> bool { self.shapes.contains(s) }
    pub fn extent(&self) -> usize { self.shapes.len() }
    pub fn is_empty(&self) -> bool { self.shapes.is_empty() }
    pub fn clear(&mut self) { self.shapes.clear(); }

    pub fn intersection(&self, other: &MapOfShape) -> MapOfShape {
        let shapes = self.shapes.iter().filter(|s| other.contains(s)).cloned().collect();
        MapOfShape { shapes }
    }

    pub fn union(&self, other: &MapOfShape) -> MapOfShape {
        let mut m = self.clone();
        for s in &other.shapes { m.add(s.clone()); }
        m
    }

    pub fn difference(&self, other: &MapOfShape) -> MapOfShape {
        let shapes = self.shapes.iter().filter(|s| !other.contains(s)).cloned().collect();
        MapOfShape { shapes }
    }
}

// occt: TopTools_DataMapOfShapeShape — maps shape → shape
#[derive(Clone, Debug, Default)]
pub struct DataMapOfShapeShape {
    pub entries: Vec<(ShapeRef, ShapeRef)>,
}

impl DataMapOfShapeShape {
    pub fn new() -> Self { Self::default() }

    pub fn bind(&mut self, key: ShapeRef, value: ShapeRef) {
        if let Some(e) = self.entries.iter_mut().find(|(k, _)| k == &key) {
            e.1 = value;
        } else {
            self.entries.push((key, value));
        }
    }

    pub fn find(&self, key: &ShapeRef) -> Option<&ShapeRef> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn unbind(&mut self, key: &ShapeRef) -> bool {
        let before = self.entries.len();
        self.entries.retain(|(k, _)| k != key);
        self.entries.len() < before
    }

    pub fn is_bound(&self, key: &ShapeRef) -> bool {
        self.entries.iter().any(|(k, _)| k == key)
    }

    pub fn extent(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    pub fn clear(&mut self) { self.entries.clear(); }
}

// occt: TopTools_IndexedDataMapOfShapeListOfShape — shape → list-of-shapes
#[derive(Clone, Debug, Default)]
pub struct IndexedDataMapOfShapeListOfShape {
    pub entries: Vec<(ShapeRef, ListOfShape)>,
}

impl IndexedDataMapOfShapeListOfShape {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, key: ShapeRef, value: ListOfShape) -> u32 {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == &key) {
            self.entries[pos].1 = value;
            return pos as u32 + 1;
        }
        self.entries.push((key, value));
        self.entries.len() as u32
    }

    pub fn find_from_key(&self, key: &ShapeRef) -> Option<&ListOfShape> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn find_from_index(&self, idx: u32) -> Option<&ListOfShape> {
        self.entries.get((idx as usize).saturating_sub(1)).map(|(_, v)| v)
    }

    pub fn find_index(&self, key: &ShapeRef) -> Option<u32> {
        self.entries.iter().position(|(k, _)| k == key).map(|i| i as u32 + 1)
    }

    pub fn key(&self, idx: u32) -> Option<&ShapeRef> {
        self.entries.get((idx as usize).saturating_sub(1)).map(|(k, _)| k)
    }

    pub fn extent(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexed_map() {
        let mut m = IndexedMapOfShape::new();
        let s1 = ShapeRef::new(1, 3);
        let s2 = ShapeRef::new(2, 5);
        let i1 = m.add(s1.clone());
        let i2 = m.add(s2.clone());
        assert_eq!(i1, 1);
        assert_eq!(i2, 2);
        assert_eq!(m.find_index(&s1), Some(1));
        let dup = m.add(s1.clone());  // should return existing index
        assert_eq!(dup, 1);
        assert_eq!(m.extent(), 2);
    }

    #[test]
    fn list_of_shape() {
        let mut l = ListOfShape::new();
        l.append(ShapeRef::new(1, 3));
        l.append(ShapeRef::new(2, 5));
        assert_eq!(l.size(), 2);
        l.remove_first();
        assert_eq!(l.size(), 1);
    }

    #[test]
    fn map_of_shape_set_ops() {
        let mut a = MapOfShape::new();
        let mut b = MapOfShape::new();
        a.add(ShapeRef::new(1, 3));
        a.add(ShapeRef::new(2, 5));
        b.add(ShapeRef::new(2, 5));
        b.add(ShapeRef::new(3, 6));
        let inter = a.intersection(&b);
        assert_eq!(inter.extent(), 1);
        let union = a.union(&b);
        assert_eq!(union.extent(), 3);
    }

    #[test]
    fn data_map_shape_shape() {
        let mut m = DataMapOfShapeShape::new();
        let k = ShapeRef::new(1, 3);
        let v = ShapeRef::new(2, 3);
        m.bind(k.clone(), v.clone());
        assert_eq!(m.find(&k), Some(&v));
        assert!(m.unbind(&k));
        assert!(!m.is_bound(&k));
    }

    #[test]
    fn indexed_data_map() {
        let mut m = IndexedDataMapOfShapeListOfShape::new();
        let face = ShapeRef::new(1, 3);
        let mut edges = ListOfShape::new();
        edges.append(ShapeRef::new(10, 5));
        edges.append(ShapeRef::new(11, 5));
        m.add(face.clone(), edges);
        let list = m.find_from_key(&face).unwrap();
        assert_eq!(list.size(), 2);
    }
}
