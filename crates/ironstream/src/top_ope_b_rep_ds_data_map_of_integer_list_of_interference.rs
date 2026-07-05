// FILE: top_ope_b_rep_ds_data_map_of_integer_list_of_interference.rs
// occt: TopOpeBRepDS_DataMapOfIntegerListOfInterference

use std::collections::HashMap;

/// Interference: Simplified interference data.
#[derive(Clone, Debug)]
pub struct Interference {
    id: usize,
    param: f64,
}

impl Interference {
    pub fn new(id: usize, param: f64) -> Self {
        Interference { id, param }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn param(&self) -> f64 {
        self.param
    }
}

/// ListOfInterference: List of interference objects.
#[derive(Clone, Debug)]
pub struct ListOfInterference {
    items: Vec<Interference>,
}

impl ListOfInterference {
    pub fn new() -> Self {
        ListOfInterference { items: Vec::new() }
    }

    pub fn append(&mut self, item: Interference) {
        self.items.push(item);
    }

    pub fn prepend(&mut self, item: Interference) {
        self.items.insert(0, item);
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn length(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Interference> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Interference> {
        self.items.iter_mut()
    }

    pub fn get(&self, index: usize) -> Option<&Interference> {
        self.items.get(index)
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for ListOfInterference {
    fn default() -> Self {
        Self::new()
    }
}

/// DataMapOfIntegerListOfInterference: Maps integer key to list of interferences.
#[derive(Clone, Debug)]
pub struct DataMapOfIntegerListOfInterference {
    data: HashMap<i32, ListOfInterference>,
}

impl DataMapOfIntegerListOfInterference {
    pub fn new() -> Self {
        DataMapOfIntegerListOfInterference {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, key: i32, list: ListOfInterference) -> bool {
        self.data.insert(key, list).is_none()
    }

    pub fn contains(&self, key: i32) -> bool {
        self.data.contains_key(&key)
    }

    pub fn find(&self, key: i32) -> Option<&ListOfInterference> {
        self.data.get(&key)
    }

    pub fn find_mut(&mut self, key: i32) -> Option<&mut ListOfInterference> {
        self.data.get_mut(&key)
    }

    pub fn remove(&mut self, key: i32) -> bool {
        self.data.remove(&key).is_some()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (i32, &ListOfInterference)> {
        self.data.iter().map(|(k, v)| (*k, v))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (i32, &mut ListOfInterference)> {
        self.data.iter_mut().map(|(k, v)| (*k, v))
    }
}

impl Default for DataMapOfIntegerListOfInterference {
    fn default() -> Self {
        Self::new()
    }
}

/// DataMapIterator: Iterator for the data map.
pub struct DataMapIterator {
    entries: Vec<(i32, ListOfInterference)>,
    index: usize,
}

impl DataMapIterator {
    pub fn new(map: &DataMapOfIntegerListOfInterference) -> Self {
        DataMapIterator {
            entries: map.data.iter().map(|(k, v)| (*k, v.clone())).collect(),
            index: 0,
        }
    }

    pub fn is_more(&self) -> bool {
        self.index < self.entries.len()
    }

    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn key(&self) -> Option<i32> {
        self.entries.get(self.index).map(|(k, _)| *k)
    }

    pub fn value(&self) -> Option<&ListOfInterference> {
        self.entries.get(self.index).map(|(_, v)| v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interference_new() {
        let interf = Interference::new(42, 0.5);
        assert_eq!(interf.id(), 42);
        assert_eq!(interf.param(), 0.5);
    }

    #[test]
    fn test_list_of_interference_append() {
        let mut list = ListOfInterference::new();
        list.append(Interference::new(1, 0.1));
        list.append(Interference::new(2, 0.2));
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_list_of_interference_prepend() {
        let mut list = ListOfInterference::new();
        list.append(Interference::new(2, 0.2));
        list.prepend(Interference::new(1, 0.1));
        assert_eq!(list.get(0).unwrap().id(), 1);
        assert_eq!(list.get(1).unwrap().id(), 2);
    }

    #[test]
    fn test_data_map_bind() {
        let mut map = DataMapOfIntegerListOfInterference::new();
        let mut list = ListOfInterference::new();
        list.append(Interference::new(10, 1.0));

        assert!(map.bind(5, list));
        assert!(map.contains(5));
        assert!(!map.contains(6));
    }

    #[test]
    fn test_data_map_find() {
        let mut map = DataMapOfIntegerListOfInterference::new();
        let mut list = ListOfInterference::new();
        list.append(Interference::new(100, 2.5));
        map.bind(10, list);

        let found = map.find(10).unwrap();
        assert_eq!(found.size(), 1);
        assert_eq!(found.get(0).unwrap().id(), 100);
    }

    #[test]
    fn test_data_map_remove() {
        let mut map = DataMapOfIntegerListOfInterference::new();
        map.bind(7, ListOfInterference::new());
        assert_eq!(map.size(), 1);
        assert!(map.remove(7));
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_data_map_clear() {
        let mut map = DataMapOfIntegerListOfInterference::new();
        map.bind(1, ListOfInterference::new());
        map.bind(2, ListOfInterference::new());
        map.clear();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_data_map_iterator() {
        let mut map = DataMapOfIntegerListOfInterference::new();
        let list1 = ListOfInterference::new();
        let list2 = ListOfInterference::new();
        map.bind(1, list1);
        map.bind(2, list2);

        let mut iter = DataMapIterator::new(&map);
        let mut count = 0;
        while iter.is_more() {
            assert!(iter.key().is_some());
            assert!(iter.value().is_some());
            iter.next();
            count += 1;
        }
        assert_eq!(count, 2);
    }
}
