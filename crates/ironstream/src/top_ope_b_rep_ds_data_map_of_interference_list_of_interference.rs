// FILE: top_ope_b_rep_ds_data_map_of_interference_list_of_interference.rs
// occt: TopOpeBRepDS_DataMapOfInterferenceListOfInterference

use std::collections::HashMap;

/// Interference: Interference data structure.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Interference {
    id: usize,
}

impl Interference {
    pub fn new(id: usize) -> Self {
        Interference { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// ListOfInterference: List of interferences.
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

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Interference> {
        self.items.iter()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Default for ListOfInterference {
    fn default() -> Self {
        Self::new()
    }
}

/// DataMapOfInterferenceListOfInterference: Maps Interference to ListOfInterference.
#[derive(Clone, Debug)]
pub struct DataMapOfInterferenceListOfInterference {
    data: HashMap<Interference, ListOfInterference>,
}

impl DataMapOfInterferenceListOfInterference {
    pub fn new() -> Self {
        DataMapOfInterferenceListOfInterference {
            data: HashMap::new(),
        }
    }

    pub fn bind(&mut self, key: Interference, list: ListOfInterference) -> bool {
        self.data.insert(key, list).is_none()
    }

    pub fn contains(&self, key: &Interference) -> bool {
        self.data.contains_key(key)
    }

    pub fn find(&self, key: &Interference) -> Option<&ListOfInterference> {
        self.data.get(key)
    }

    pub fn find_mut(&mut self, key: &Interference) -> Option<&mut ListOfInterference> {
        self.data.get_mut(key)
    }

    pub fn remove(&mut self, key: &Interference) -> bool {
        self.data.remove(key).is_some()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Interference, &ListOfInterference)> {
        self.data.iter()
    }
}

impl Default for DataMapOfInterferenceListOfInterference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interference_new() {
        let interf = Interference::new(42);
        assert_eq!(interf.id(), 42);
    }

    #[test]
    fn test_list_of_interference() {
        let mut list = ListOfInterference::new();
        list.append(Interference::new(1));
        list.append(Interference::new(2));
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_data_map_bind() {
        let mut map = DataMapOfInterferenceListOfInterference::new();
        let key = Interference::new(5);
        let list = ListOfInterference::new();

        assert!(map.bind(key.clone(), list));
        assert!(!map.bind(key, ListOfInterference::new()));
    }

    #[test]
    fn test_data_map_find() {
        let mut map = DataMapOfInterferenceListOfInterference::new();
        let key = Interference::new(3);
        let mut list = ListOfInterference::new();
        list.append(Interference::new(100));
        map.bind(key.clone(), list);

        let found = map.find(&key).unwrap();
        assert_eq!(found.size(), 1);
    }

    #[test]
    fn test_data_map_remove() {
        let mut map = DataMapOfInterferenceListOfInterference::new();
        let key = Interference::new(7);
        map.bind(key.clone(), ListOfInterference::new());

        assert_eq!(map.size(), 1);
        assert!(map.remove(&key));
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_data_map_clear() {
        let mut map = DataMapOfInterferenceListOfInterference::new();
        map.bind(Interference::new(1), ListOfInterference::new());
        map.bind(Interference::new(2), ListOfInterference::new());
        map.clear();
        assert_eq!(map.size(), 0);
    }
}
