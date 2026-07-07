// FILE: top_ope_b_rep_ds_h_array1_of_data_map_of_integer_list_of_interference.rs
// occt: TopOpeBRepDS_HArray1OfDataMapOfIntegerListOfInterference

use std::collections::HashMap;
use std::sync::Arc;

/// Interference: Basic interference structure.
#[derive(Clone, Debug)]
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
}

impl Default for ListOfInterference {
    fn default() -> Self {
        Self::new()
    }
}

/// DataMapOfIntegerListOfInterference: Maps integer to list of interferences.
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

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for DataMapOfIntegerListOfInterference {
    fn default() -> Self {
        Self::new()
    }
}

/// HArray1: Handle-based 1-based array (shared ownership via Arc).
///
/// H prefix means "Handle" in OCCT - shared reference-counted container.
#[derive(Clone)]
pub struct HArray1OfDataMapOfIntegerListOfInterference {
    inner: Arc<Array1Content>,
}

struct Array1Content {
    data: Vec<DataMapOfIntegerListOfInterference>,
    lower: usize,
}

impl HArray1OfDataMapOfIntegerListOfInterference {
    /// Creates a new H-array from lower to upper (inclusive, 1-based).
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower == 0 {
            panic!("OCCT arrays use 1-based indexing");
        }
        let size = upper - lower + 1;
        HArray1OfDataMapOfIntegerListOfInterference {
            inner: Arc::new(Array1Content {
                data: (0..size)
                    .map(|_| DataMapOfIntegerListOfInterference::new())
                    .collect(),
                lower,
            }),
        }
    }

    pub fn lower(&self) -> usize {
        self.inner.lower
    }

    pub fn upper(&self) -> usize {
        self.inner.lower + self.inner.data.len() - 1
    }

    pub fn length(&self) -> usize {
        self.inner.data.len()
    }

    /// Gets value at 1-based index (requires interior mutability).
    /// Returns a cloned copy.
    pub fn value(&self, index_1based: usize) -> Option<DataMapOfIntegerListOfInterference> {
        if index_1based < self.lower() {
            None
        } else {
            self.inner
                .data
                .get(index_1based - self.lower())
                .cloned()
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &DataMapOfIntegerListOfInterference> {
        self.inner.data.iter()
    }
}

impl std::fmt::Debug for HArray1OfDataMapOfIntegerListOfInterference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HArray1OfDataMapOfIntegerListOfInterference")
            .field("lower", &self.lower())
            .field("upper", &self.upper())
            .field("length", &self.length())
            .finish()
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
    fn test_data_map_of_integer_list_of_interference() {
        let mut map = DataMapOfIntegerListOfInterference::new();
        let mut list = ListOfInterference::new();
        list.append(Interference::new(10));
        map.bind(5, list);
        assert!(map.contains(5));
    }

    #[test]
    fn test_h_array1_new() {
        let arr = HArray1OfDataMapOfIntegerListOfInterference::new(1, 10);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 10);
        assert_eq!(arr.length(), 10);
    }

    #[test]
    fn test_h_array1_custom_bounds() {
        let arr = HArray1OfDataMapOfIntegerListOfInterference::new(5, 15);
        assert_eq!(arr.lower(), 5);
        assert_eq!(arr.upper(), 15);
        assert_eq!(arr.length(), 11);
    }

    #[test]
    fn test_h_array1_value() {
        let arr = HArray1OfDataMapOfIntegerListOfInterference::new(1, 3);
        let val = arr.value(1);
        assert!(val.is_some());
        assert_eq!(val.unwrap().size(), 0);
    }

    #[test]
    fn test_h_array1_clone() {
        let arr1 = HArray1OfDataMapOfIntegerListOfInterference::new(1, 5);
        let arr2 = arr1.clone();
        assert_eq!(arr1.lower(), arr2.lower());
        assert_eq!(arr1.upper(), arr2.upper());
    }

    #[test]
    #[should_panic]
    fn test_h_array1_zero_lower_panic() {
        let _ = HArray1OfDataMapOfIntegerListOfInterference::new(0, 5);
    }
}
