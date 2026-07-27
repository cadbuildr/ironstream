// FILE: top_ope_b_rep_ds_array1_of_data_map_of_integer_list_of_interference.rs
// occt: TopOpeBRepDS_Array1OfDataMapOfIntegerListOfInterference
// occt-ref: TopOpeBRepDS_DataMapOfIntegerListOfInterference, TopOpeBRepDS_Interference

use std::collections::HashMap;

/// Interference: Simplified interference data structure.
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

/// ListOfInterference: Simple list of interference objects.
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

    pub fn iter(&self) -> impl Iterator<Item = (i32, &ListOfInterference)> {
        self.data.iter().map(|(k, v)| (*k, v))
    }
}

impl Default for DataMapOfIntegerListOfInterference {
    fn default() -> Self {
        Self::new()
    }
}

/// Array1OfDataMapOfIntegerListOfInterference: OCCT 1-based array container.
///
/// Deprecated typedef for:
/// NCollection_Array1<TopOpeBRepDS_DataMapOfIntegerListOfInterference>
#[derive(Clone, Debug)]
pub struct Array1OfDataMapOfIntegerListOfInterference {
    data: Vec<DataMapOfIntegerListOfInterference>,
    lower: usize,
}

impl Array1OfDataMapOfIntegerListOfInterference {
    /// Creates a 1-based array with given size.
    pub fn new(size: usize) -> Self {
        Array1OfDataMapOfIntegerListOfInterference {
            data: (0..size)
                .map(|_| DataMapOfIntegerListOfInterference::new())
                .collect(),
            lower: 1,
        }
    }

    /// Creates a 1-based array from lower to upper (inclusive).
    pub fn new_from_bounds(lower: usize, upper: usize) -> Self {
        if lower == 0 {
            panic!("OCCT Array1 uses 1-based indexing; lower must be >= 1");
        }
        let size = upper - lower + 1;
        Array1OfDataMapOfIntegerListOfInterference {
            data: (0..size)
                .map(|_| DataMapOfIntegerListOfInterference::new())
                .collect(),
            lower,
        }
    }

    /// Returns the lower bound (typically 1).
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Returns the upper bound (1-based).
    pub fn upper(&self) -> usize {
        self.lower + self.data.len() - 1
    }

    /// Returns the length of the array.
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Gets value at 1-based index.
    pub fn value(&self, index_1based: usize) -> Option<&DataMapOfIntegerListOfInterference> {
        if index_1based < self.lower {
            None
        } else {
            self.data.get(index_1based - self.lower)
        }
    }

    /// Gets mutable value at 1-based index.
    pub fn value_mut(
        &mut self,
        index_1based: usize,
    ) -> Option<&mut DataMapOfIntegerListOfInterference> {
        if index_1based < self.lower {
            None
        } else {
            self.data.get_mut(index_1based - self.lower)
        }
    }

    /// Sets value at 1-based index.
    pub fn set_value(&mut self, index_1based: usize, value: DataMapOfIntegerListOfInterference) {
        if index_1based < self.lower {
            panic!("Index {} is out of bounds (lower bound is {})", index_1based, self.lower);
        }
        let idx = index_1based - self.lower;
        if idx >= self.data.len() {
            panic!("Index {} is out of bounds (upper bound is {})", index_1based, self.upper());
        }
        self.data[idx] = value;
    }

    pub fn iter(&self) -> impl Iterator<Item = &DataMapOfIntegerListOfInterference> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut DataMapOfIntegerListOfInterference> {
        self.data.iter_mut()
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
    fn test_list_of_interference() {
        let mut list = ListOfInterference::new();
        list.append(Interference::new(1, 0.1));
        list.append(Interference::new(2, 0.2));
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_data_map_of_integer_list_of_interference() {
        let mut map = DataMapOfIntegerListOfInterference::new();
        let mut list = ListOfInterference::new();
        list.append(Interference::new(10, 1.0));

        assert!(map.bind(5, list));
        assert!(map.contains(5));
        assert!(!map.contains(6));
    }

    #[test]
    fn test_array1_new() {
        let arr = Array1OfDataMapOfIntegerListOfInterference::new(5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn test_array1_from_bounds() {
        let arr = Array1OfDataMapOfIntegerListOfInterference::new_from_bounds(1, 10);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 10);
        assert_eq!(arr.length(), 10);
    }

    #[test]
    fn test_array1_from_bounds_custom() {
        let arr = Array1OfDataMapOfIntegerListOfInterference::new_from_bounds(5, 15);
        assert_eq!(arr.lower(), 5);
        assert_eq!(arr.upper(), 15);
        assert_eq!(arr.length(), 11);
    }

    #[test]
    fn test_array1_value() {
        let mut arr = Array1OfDataMapOfIntegerListOfInterference::new(3);
        let map = DataMapOfIntegerListOfInterference::new();
        arr.set_value(1, map);

        assert!(arr.value(1).is_some());
        assert!(arr.value(2).is_some());
        assert!(arr.value(4).is_none());
        assert!(arr.value(10).is_none());
        assert!(arr.value(0).is_none());
    }

    #[test]
    fn test_array1_set_and_get() {
        let mut arr = Array1OfDataMapOfIntegerListOfInterference::new(2);
        let mut map1 = DataMapOfIntegerListOfInterference::new();
        let mut list1 = ListOfInterference::new();
        list1.append(Interference::new(100, 1.0));
        map1.bind(1, list1);

        arr.set_value(1, map1);
        let retrieved = arr.value(1).unwrap();
        assert!(retrieved.contains(1));
    }

    #[test]
    #[should_panic]
    fn test_array1_zero_lower_bound_panic() {
        let _ = Array1OfDataMapOfIntegerListOfInterference::new_from_bounds(0, 5);
    }

    #[test]
    #[should_panic]
    fn test_array1_out_of_bounds_panic() {
        // OCCT NCollection_Array1 raises Standard_OutOfRange uniformly for any
        // out-of-bounds index. In this port the Option-returning value() maps
        // that exception to None (as test_array1_value already asserts for
        // value(4)), while set_value() keeps OCCT's raising behavior as a panic.
        let mut arr = Array1OfDataMapOfIntegerListOfInterference::new(3);
        arr.set_value(10, DataMapOfIntegerListOfInterference::new());
    }
}
