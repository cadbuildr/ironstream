// FILE: crates/ironstream/src/geom_map_tools.rs

// occt: TopTools_IndexedDataMapOfShapeListOfShape
/// An ordered map from string keys to lists of strings, with O(1) lookup by
/// key and O(1) lookup by insertion-order index.  Mirrors the semantics of
/// TopTools_IndexedDataMapOfShapeListOfShape from Open CASCADE Technology.
pub struct IndexedDataMap {
    pub keys: Vec<String>,
    pub values: Vec<Vec<String>>,
    pub index: std::collections::HashMap<String, usize>,
}

impl IndexedDataMap {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
            index: std::collections::HashMap::new(),
        }
    }

    /// Add a key/value pair.  If the key is already present its value is
    /// replaced and the existing 1-based index is returned; otherwise the
    /// key is appended and the new 1-based index is returned.
    pub fn add(&mut self, key: &str, val: Vec<String>) -> usize {
        if let Some(&i) = self.index.get(key) {
            self.values[i - 1] = val;
            i
        } else {
            self.keys.push(key.to_string());
            self.values.push(val);
            let i = self.keys.len(); // 1-based
            self.index.insert(key.to_string(), i);
            i
        }
    }

    /// Return a reference to the value list for the given key, or `None`.
    pub fn find_from_key(&self, key: &str) -> Option<&Vec<String>> {
        let &i = self.index.get(key)?;
        self.values.get(i - 1)
    }

    /// Return a reference to the value list at the given 1-based index, or `None`.
    pub fn find_from_index(&self, i: usize) -> Option<&Vec<String>> {
        if i == 0 {
            return None;
        }
        self.values.get(i - 1)
    }

    /// Return a reference to the key at the given 1-based index, or `None`.
    pub fn find_key(&self, i: usize) -> Option<&String> {
        if i == 0 {
            return None;
        }
        self.keys.get(i - 1)
    }

    /// Number of entries in the map.
    pub fn size(&self) -> usize {
        self.keys.len()
    }

    /// Return `true` when the key is present in the map.
    pub fn contains(&self, key: &str) -> bool {
        self.index.contains_key(key)
    }
}

impl Default for IndexedDataMap {
    fn default() -> Self {
        Self::new()
    }
}

// occt: NCollection_DataMap
/// A generic unordered key-to-value map.  Mirrors the semantics of
/// NCollection_DataMap from Open CASCADE Technology.
pub struct DataMap<K: std::hash::Hash + Eq + Clone, V: Clone> {
    pub map: std::collections::HashMap<K, V>,
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> DataMap<K, V> {
    pub fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
        }
    }

    /// Insert or overwrite the binding for `k`.
    pub fn bind(&mut self, k: K, v: V) {
        self.map.insert(k, v);
    }

    /// Return a reference to the value bound to `k`, or `None`.
    pub fn find(&self, k: &K) -> Option<&V> {
        self.map.get(k)
    }

    /// Return `true` when `k` has a binding in the map.
    pub fn is_bound(&self, k: &K) -> bool {
        self.map.contains_key(k)
    }

    /// Remove the binding for `k`.  Does nothing if `k` is not present.
    pub fn unbind(&mut self, k: &K) {
        self.map.remove(k);
    }

    /// Number of bindings in the map.
    pub fn size(&self) -> usize {
        self.map.len()
    }
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> Default for DataMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
