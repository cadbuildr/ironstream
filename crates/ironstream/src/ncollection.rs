// FILE: ncollection.rs

use std::collections::HashMap;
use std::collections::VecDeque;

// occt: NCollection_List
pub struct NCollectionList<T> {
    data: VecDeque<T>,
}

impl<T> NCollectionList<T> {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    pub fn append(&mut self, v: T) {
        self.data.push_back(v);
    }

    pub fn prepend(&mut self, v: T) {
        self.data.push_front(v);
    }

    pub fn first(&self) -> Option<&T> {
        self.data.front()
    }

    pub fn last(&self) -> Option<&T> {
        self.data.back()
    }

    pub fn remove_first(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
}

impl<T> Default for NCollectionList<T> {
    fn default() -> Self {
        Self::new()
    }
}

// occt-ref: NCollection_Sequence
pub struct NCollectionSequence<T> {
    data: Vec<T>,
}

impl<T> NCollectionSequence<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn append(&mut self, v: T) {
        self.data.push(v);
    }

    pub fn prepend(&mut self, v: T) {
        self.data.insert(0, v);
    }

    pub fn value(&self, idx: usize) -> Option<&T> {
        if idx == 0 {
            return None;
        }
        self.data.get(idx - 1)
    }

    pub fn set_value(&mut self, idx: usize, v: T) {
        if idx == 0 || idx > self.data.len() {
            panic!("NCollectionSequence::set_value: index out of bounds");
        }
        self.data[idx - 1] = v;
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn first(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn last(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn remove(&mut self, idx: usize) {
        if idx == 0 || idx > self.data.len() {
            panic!("NCollectionSequence::remove: index out of bounds");
        }
        self.data.remove(idx - 1);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl<T> Default for NCollectionSequence<T> {
    fn default() -> Self {
        Self::new()
    }
}

// occt: NCollection_DataMap
pub struct NCollectionMap<K: std::hash::Hash + Eq, V> {
    data: HashMap<K, V>,
}

impl<K: std::hash::Hash + Eq, V> NCollectionMap<K, V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, key: K, value: V) -> bool {
        let is_new = !self.data.contains_key(&key);
        self.data.insert(key, value);
        is_new
    }

    pub fn find(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    pub fn unbind(&mut self, key: &K) -> bool {
        self.data.remove(key).is_some()
    }

    pub fn extent(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl<K: std::hash::Hash + Eq, V> Default for NCollectionMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// occt: NCollection_Array1
pub struct NCollectionArray1<T: Clone + Default> {
    data: Vec<T>,
    lower: i32,
    upper: i32,
}

impl<T: Clone + Default> NCollectionArray1<T> {
    pub fn new(lower: i32, upper: i32) -> Self {
        let size = if upper >= lower {
            (upper - lower + 1) as usize
        } else {
            0
        };
        Self {
            data: vec![T::default(); size],
            lower,
            upper,
        }
    }

    fn to_index(&self, idx: i32) -> Option<usize> {
        if idx < self.lower || idx > self.upper {
            None
        } else {
            Some((idx - self.lower) as usize)
        }
    }

    pub fn value(&self, idx: i32) -> Option<&T> {
        self.to_index(idx).map(|i| &self.data[i])
    }

    pub fn set_value(&mut self, idx: i32, v: T) {
        match self.to_index(idx) {
            Some(i) => self.data[i] = v,
            None => panic!("NCollectionArray1::set_value: index out of bounds"),
        }
    }

    pub fn lower(&self) -> i32 {
        self.lower
    }

    pub fn upper(&self) -> i32 {
        self.upper
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn init(&mut self, v: T) {
        for elem in &mut self.data {
            *elem = v.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_new_is_empty() {
        let list: NCollectionList<i32> = NCollectionList::new();
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn list_append_and_first_last() {
        let mut list = NCollectionList::new();
        list.append(10);
        list.append(20);
        list.append(30);
        assert_eq!(list.first(), Some(&10));
        assert_eq!(list.last(), Some(&30));
        assert_eq!(list.size(), 3);
    }

    #[test]
    fn list_prepend() {
        let mut list = NCollectionList::new();
        list.append(2);
        list.prepend(1);
        assert_eq!(list.first(), Some(&1));
        assert_eq!(list.last(), Some(&2));
    }

    #[test]
    fn list_remove_first() {
        let mut list = NCollectionList::new();
        list.append(5);
        list.append(6);
        let removed = list.remove_first();
        assert_eq!(removed, Some(5));
        assert_eq!(list.first(), Some(&6));
        assert_eq!(list.size(), 1);
    }

    #[test]
    fn list_clear() {
        let mut list = NCollectionList::new();
        list.append(1);
        list.append(2);
        list.clear();
        assert!(list.is_empty());
    }

    #[test]
    fn list_iter() {
        let mut list = NCollectionList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        let collected: Vec<&i32> = list.iter().collect();
        assert_eq!(collected, vec![&1, &2, &3]);
    }

    #[test]
    fn sequence_value_1based() {
        let mut seq = NCollectionSequence::new();
        seq.append(10);
        seq.append(20);
        seq.append(30);
        assert_eq!(seq.value(1), Some(&10));
        assert_eq!(seq.value(2), Some(&20));
        assert_eq!(seq.value(3), Some(&30));
        assert_eq!(seq.value(0), None);
        assert_eq!(seq.value(4), None);
    }

    #[test]
    fn sequence_set_value() {
        let mut seq = NCollectionSequence::new();
        seq.append(1);
        seq.append(2);
        seq.set_value(1, 99);
        assert_eq!(seq.value(1), Some(&99));
        assert_eq!(seq.value(2), Some(&2));
    }

    #[test]
    fn sequence_prepend() {
        let mut seq = NCollectionSequence::new();
        seq.append(2);
        seq.prepend(1);
        assert_eq!(seq.value(1), Some(&1));
        assert_eq!(seq.value(2), Some(&2));
        assert_eq!(seq.length(), 2);
    }

    #[test]
    fn sequence_remove() {
        let mut seq = NCollectionSequence::new();
        seq.append(10);
        seq.append(20);
        seq.append(30);
        seq.remove(2);
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.value(1), Some(&10));
        assert_eq!(seq.value(2), Some(&30));
    }

    #[test]
    fn sequence_first_last() {
        let mut seq = NCollectionSequence::new();
        seq.append(7);
        seq.append(8);
        seq.append(9);
        assert_eq!(seq.first(), Some(&7));
        assert_eq!(seq.last(), Some(&9));
    }

    #[test]
    fn map_bind_and_find() {
        let mut map = NCollectionMap::new();
        let is_new = map.bind("key".to_string(), 42);
        assert!(is_new);
        assert_eq!(map.find(&"key".to_string()), Some(&42));
        assert_eq!(map.extent(), 1);
    }

    #[test]
    fn map_bind_replace_returns_false() {
        let mut map = NCollectionMap::new();
        map.bind(1u32, 10);
        let is_new = map.bind(1u32, 20);
        assert!(!is_new);
        assert_eq!(map.find(&1u32), Some(&20));
    }

    #[test]
    fn map_unbind() {
        let mut map = NCollectionMap::new();
        map.bind(1, "a");
        assert!(map.unbind(&1));
        assert!(!map.contains(&1));
        assert!(!map.unbind(&1));
    }

    #[test]
    fn map_clear() {
        let mut map: NCollectionMap<i32, i32> = NCollectionMap::new();
        map.bind(1, 1);
        map.bind(2, 2);
        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn array1_new_and_value() {
        let arr: NCollectionArray1<i32> = NCollectionArray1::new(1, 3);
        assert_eq!(arr.size(), 3);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 3);
        assert_eq!(arr.value(1), Some(&0));
        assert_eq!(arr.value(3), Some(&0));
        assert_eq!(arr.value(0), None);
        assert_eq!(arr.value(4), None);
    }

    #[test]
    fn array1_set_value_and_init() {
        let mut arr: NCollectionArray1<f64> = NCollectionArray1::new(0, 2);
        arr.set_value(0, 1.0);
        arr.set_value(1, 2.0);
        arr.set_value(2, 3.0);
        assert_eq!(arr.value(0), Some(&1.0));
        assert_eq!(arr.value(1), Some(&2.0));
        assert_eq!(arr.value(2), Some(&3.0));
        arr.init(7.0);
        assert_eq!(arr.value(0), Some(&7.0));
        assert_eq!(arr.value(2), Some(&7.0));
    }

    #[test]
    fn array1_nonstandard_bounds() {
        let mut arr: NCollectionArray1<u8> = NCollectionArray1::new(5, 8);
        assert_eq!(arr.size(), 4);
        arr.set_value(5, 11);
        arr.set_value(8, 22);
        assert_eq!(arr.value(5), Some(&11));
        assert_eq!(arr.value(8), Some(&22));
        assert_eq!(arr.value(4), None);
        assert_eq!(arr.value(9), None);
    }
}
