// FILE: ncollection_extra.rs
// occt: NCollection_Sequence, NCollection_Array2, NCollection_Map
//       NCollection_IndexedDataMap, NCollection_SparseArray,
//       NCollection_UBTree, NCollection_StlIterator

use std::collections::HashMap;

// occt: NCollection_Sequence // (indexed random-access list)
#[derive(Clone, Debug, Default)]
pub struct NcSequence<T> {
    pub items: Vec<T>,
}

impl<T: Clone> NcSequence<T> {
    pub fn new() -> Self { Self { items: Vec::new() } }

    pub fn append(&mut self, item: T) { self.items.push(item); }

    pub fn prepend(&mut self, item: T) { self.items.insert(0, item); }

    pub fn value(&self, idx: usize) -> Option<&T> { self.items.get(idx) }

    pub fn value_mut(&mut self, idx: usize) -> Option<&mut T> { self.items.get_mut(idx) }

    pub fn set_value(&mut self, idx: usize, v: T) {
        if idx < self.items.len() { self.items[idx] = v; }
    }

    pub fn remove(&mut self, idx: usize) -> Option<T> {
        if idx < self.items.len() { Some(self.items.remove(idx)) } else { None }
    }

    pub fn exchange(&mut self, i: usize, j: usize) {
        if i < self.items.len() && j < self.items.len() {
            self.items.swap(i, j);
        }
    }

    pub fn first(&self) -> Option<&T> { self.items.first() }
    pub fn last(&self) -> Option<&T> { self.items.last() }
    pub fn length(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn clear(&mut self) { self.items.clear(); }
    pub fn iter(&self) -> impl Iterator<Item = &T> { self.items.iter() }

    pub fn split(&mut self, at: usize) -> NcSequence<T> {
        let tail = self.items.split_off(at);
        NcSequence { items: tail }
    }

    pub fn contains(&self, item: &T) -> bool where T: PartialEq {
        self.items.contains(item)
    }
}

// occt: NCollection_Array2 // (2D dense array)
#[derive(Clone, Debug)]
pub struct NcArray2<T> {
    pub lower_row: i32,
    pub upper_row: i32,
    pub lower_col: i32,
    pub upper_col: i32,
    pub data: Vec<T>,
}

impl<T: Clone + Default> NcArray2<T> {
    pub fn new(lr: i32, ur: i32, lc: i32, uc: i32) -> Self {
        let nr = (ur - lr + 1).max(0) as usize;
        let nc = (uc - lc + 1).max(0) as usize;
        Self {
            lower_row: lr, upper_row: ur,
            lower_col: lc, upper_col: uc,
            data: vec![T::default(); nr * nc],
        }
    }

    pub fn nb_rows(&self) -> usize { (self.upper_row - self.lower_row + 1).max(0) as usize }
    pub fn nb_cols(&self) -> usize { (self.upper_col - self.lower_col + 1).max(0) as usize }

    fn idx(&self, row: i32, col: i32) -> Option<usize> {
        let r = (row - self.lower_row) as usize;
        let c = (col - self.lower_col) as usize;
        if r < self.nb_rows() && c < self.nb_cols() {
            Some(r * self.nb_cols() + c)
        } else {
            None
        }
    }

    pub fn value(&self, row: i32, col: i32) -> Option<&T> {
        self.idx(row, col).map(|i| &self.data[i])
    }

    pub fn value_mut(&mut self, row: i32, col: i32) -> Option<&mut T> {
        let i = self.idx(row, col)?;
        Some(&mut self.data[i])
    }

    pub fn set_value(&mut self, row: i32, col: i32, v: T) {
        if let Some(i) = self.idx(row, col) { self.data[i] = v; }
    }

    pub fn size(&self) -> usize { self.data.len() }
}

// occt: NCollection_Map // (unordered set)
#[derive(Clone, Debug, Default)]
pub struct NcMap<K: std::hash::Hash + Eq + Clone> {
    pub inner: std::collections::HashSet<K>,
}

impl<K: std::hash::Hash + Eq + Clone> NcMap<K> {
    pub fn new() -> Self { Self { inner: std::collections::HashSet::new() } }
    pub fn add(&mut self, key: K) -> bool { self.inner.insert(key) }
    pub fn contains(&self, key: &K) -> bool { self.inner.contains(key) }
    pub fn remove(&mut self, key: &K) -> bool { self.inner.remove(key) }
    pub fn extent(&self) -> usize { self.inner.len() }
    pub fn is_empty(&self) -> bool { self.inner.is_empty() }
    pub fn clear(&mut self) { self.inner.clear(); }
    pub fn iter(&self) -> impl Iterator<Item = &K> { self.inner.iter() }

    pub fn subtract(&self, other: &NcMap<K>) -> NcMap<K> {
        NcMap { inner: self.inner.difference(&other.inner).cloned().collect() }
    }

    pub fn intersection(&self, other: &NcMap<K>) -> NcMap<K> {
        NcMap { inner: self.inner.intersection(&other.inner).cloned().collect() }
    }

    pub fn union(&self, other: &NcMap<K>) -> NcMap<K> {
        NcMap { inner: self.inner.union(&other.inner).cloned().collect() }
    }
}

// occt: NCollection_IndexedDataMap // (key → value, with integer index access)
#[derive(Clone, Debug, Default)]
pub struct NcIndexedDataMap<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
    index: HashMap<String, usize>,
}

impl<K: Clone + std::fmt::Debug + std::hash::Hash + Eq + ToString, V: Clone> NcIndexedDataMap<K, V> {
    pub fn new() -> Self { Self { keys: Vec::new(), values: Vec::new(), index: HashMap::new() } }

    pub fn bind(&mut self, key: K, value: V) -> usize {
        let k = key.to_string();
        if let Some(&idx) = self.index.get(&k) {
            self.values[idx] = value;
            idx + 1
        } else {
            let idx = self.keys.len();
            self.index.insert(k, idx);
            self.keys.push(key);
            self.values.push(value);
            idx + 1
        }
    }

    pub fn find_from_key(&self, key: &K) -> Option<&V> {
        let k = key.to_string();
        self.index.get(&k).map(|&i| &self.values[i])
    }

    pub fn find_index(&self, key: &K) -> Option<usize> {
        self.index.get(&key.to_string()).map(|&i| i + 1)
    }

    pub fn find_key(&self, idx: usize) -> Option<&K> {
        if idx == 0 { return None; }
        self.keys.get(idx - 1)
    }

    pub fn find_from_index(&self, idx: usize) -> Option<&V> {
        if idx == 0 { return None; }
        self.values.get(idx - 1)
    }

    pub fn extent(&self) -> usize { self.keys.len() }
    pub fn is_empty(&self) -> bool { self.keys.is_empty() }
    pub fn contains_key(&self, key: &K) -> bool { self.index.contains_key(&key.to_string()) }

    pub fn remove_last(&mut self) {
        if let Some(k) = self.keys.pop() {
            self.values.pop();
            self.index.remove(&k.to_string());
        }
    }
}

// occt: NCollection_SparseArray // (sparse indexed array)
#[derive(Clone, Debug, Default)]
pub struct NcSparseArray<T> {
    pub data: HashMap<u64, T>,
    pub capacity_hint: usize,
}

impl<T: Clone> NcSparseArray<T> {
    pub fn new() -> Self { Self { data: HashMap::new(), capacity_hint: 0 } }

    pub fn set_value(&mut self, idx: u64, val: T) { self.data.insert(idx, val); }
    pub fn value(&self, idx: u64) -> Option<&T> { self.data.get(&idx) }
    pub fn has_value(&self, idx: u64) -> bool { self.data.contains_key(&idx) }
    pub fn unset_value(&mut self, idx: u64) { self.data.remove(&idx); }
    pub fn extent(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }
    pub fn clear(&mut self) { self.data.clear(); }

    pub fn min_index(&self) -> Option<u64> { self.data.keys().copied().min() }
    pub fn max_index(&self) -> Option<u64> { self.data.keys().copied().max() }
}

// occt: NCollection_UBTree // (Unbalanced Binary Tree for bounding boxes)
#[derive(Clone, Debug)]
pub struct UBTreeNode {
    pub id: u32,
    pub bbox_min: [f64; 3],
    pub bbox_max: [f64; 3],
}

impl UBTreeNode {
    pub fn new(id: u32, mn: [f64; 3], mx: [f64; 3]) -> Self {
        Self { id, bbox_min: mn, bbox_max: mx }
    }

    pub fn overlaps(&self, mn: &[f64; 3], mx: &[f64; 3]) -> bool {
        for i in 0..3 {
            if self.bbox_max[i] < mn[i] || self.bbox_min[i] > mx[i] { return false; }
        }
        true
    }

    pub fn contains_point(&self, pt: &[f64; 3]) -> bool {
        (0..3).all(|i| pt[i] >= self.bbox_min[i] && pt[i] <= self.bbox_max[i])
    }
}

// occt: NCollection_UBTree
#[derive(Clone, Debug, Default)]
pub struct NcUBTree {
    pub nodes: Vec<UBTreeNode>,
}

impl NcUBTree {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, node: UBTreeNode) {
        self.nodes.push(node);
    }

    pub fn select(&self, mn: [f64; 3], mx: [f64; 3]) -> Vec<u32> {
        self.nodes.iter()
            .filter(|n| n.overlaps(&mn, &mx))
            .map(|n| n.id)
            .collect()
    }

    pub fn select_point(&self, pt: [f64; 3]) -> Vec<u32> {
        self.nodes.iter()
            .filter(|n| n.contains_point(&pt))
            .map(|n| n.id)
            .collect()
    }

    pub fn nb_nodes(&self) -> usize { self.nodes.len() }
    pub fn clear(&mut self) { self.nodes.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nc_sequence_ops() {
        let mut seq: NcSequence<i32> = NcSequence::new();
        seq.append(1); seq.append(2); seq.append(3);
        seq.prepend(0);
        assert_eq!(seq.length(), 4);
        assert_eq!(seq.value(0), Some(&0));
        assert_eq!(seq.first(), Some(&0));
        assert_eq!(seq.last(), Some(&3));
        seq.exchange(0, 3);
        assert_eq!(seq.value(0), Some(&3));
    }

    #[test]
    fn nc_array2() {
        let mut arr: NcArray2<f64> = NcArray2::new(1, 3, 1, 3);
        assert_eq!(arr.nb_rows(), 3);
        assert_eq!(arr.nb_cols(), 3);
        arr.set_value(2, 2, 42.0);
        assert!((arr.value(2, 2).unwrap() - 42.0).abs() < 1e-10);
    }

    #[test]
    fn nc_map_set_ops() {
        let mut m1: NcMap<i32> = NcMap::new();
        m1.add(1); m1.add(2); m1.add(3);
        let mut m2: NcMap<i32> = NcMap::new();
        m2.add(2); m2.add(3); m2.add(4);
        assert_eq!(m1.intersection(&m2).extent(), 2);
        assert_eq!(m1.union(&m2).extent(), 4);
        assert_eq!(m1.subtract(&m2).extent(), 1);
    }

    #[test]
    fn nc_indexed_data_map() {
        let mut m: NcIndexedDataMap<String, f64> = NcIndexedDataMap::new();
        let i1 = m.bind("alpha".to_string(), 1.0);
        let i2 = m.bind("beta".to_string(), 2.0);
        assert_eq!(i1, 1);
        assert_eq!(i2, 2);
        assert_eq!(m.find_from_key(&"alpha".to_string()), Some(&1.0));
        assert_eq!(m.find_index(&"beta".to_string()), Some(2));
        assert_eq!(m.find_key(1), Some(&"alpha".to_string()));
    }

    #[test]
    fn nc_sparse_array() {
        let mut arr: NcSparseArray<String> = NcSparseArray::new();
        arr.set_value(100, "hello".to_string());
        arr.set_value(9999, "world".to_string());
        assert!(arr.has_value(100));
        assert!(!arr.has_value(50));
        assert_eq!(arr.extent(), 2);
        assert_eq!(arr.min_index(), Some(100));
    }

    #[test]
    fn ub_tree_select() {
        let mut tree = NcUBTree::new();
        tree.add(UBTreeNode::new(0, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]));
        tree.add(UBTreeNode::new(1, [2.0, 0.0, 0.0], [3.0, 1.0, 1.0]));
        let hits = tree.select([0.5, 0.5, 0.5], [1.5, 1.5, 1.5]);
        assert_eq!(hits, vec![0]);
        let pt_hits = tree.select_point([0.5, 0.5, 0.5]);
        assert_eq!(pt_hits, vec![0]);
    }
}
