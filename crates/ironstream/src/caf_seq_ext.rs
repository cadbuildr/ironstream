// FILE: caf_seq_ext.rs
// occt: TDataStd_NamedData, TDataStd_ListOfExtendedString,
//       TDataStd_IntPackedMap, TDataStd_ReferenceArray

/// Named data attribute — stores heterogeneous named values in a label.
/// occt: TDataStd_NamedData
#[derive(Clone, Debug, Default)]
pub struct TDataStdNamedData {
    pub integers: Vec<(String, i32)>,
    pub reals: Vec<(String, f64)>,
    pub strings: Vec<(String, String)>,
    pub bytes: Vec<(String, Vec<u8>)>,
    pub arrays_of_ints: Vec<(String, Vec<i32>)>,
    pub arrays_of_reals: Vec<(String, Vec<f64>)>,
}

impl TDataStdNamedData {
    pub fn new() -> Self { Self::default() }

    pub fn set_integer(&mut self, key: &str, v: i32) {
        if let Some(e) = self.integers.iter_mut().find(|(k,_)| k == key) { e.1 = v; }
        else { self.integers.push((key.to_string(), v)); }
    }

    pub fn get_integer(&self, key: &str) -> Option<i32> {
        self.integers.iter().find(|(k,_)| k == key).map(|(_,v)| *v)
    }

    pub fn has_integer(&self, key: &str) -> bool { self.get_integer(key).is_some() }

    pub fn set_real(&mut self, key: &str, v: f64) {
        if let Some(e) = self.reals.iter_mut().find(|(k,_)| k == key) { e.1 = v; }
        else { self.reals.push((key.to_string(), v)); }
    }

    pub fn get_real(&self, key: &str) -> Option<f64> {
        self.reals.iter().find(|(k,_)| k == key).map(|(_,v)| *v)
    }

    pub fn set_string(&mut self, key: &str, v: &str) {
        if let Some(e) = self.strings.iter_mut().find(|(k,_)| k == key) { e.1 = v.to_string(); }
        else { self.strings.push((key.to_string(), v.to_string())); }
    }

    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.strings.iter().find(|(k,_)| k == key).map(|(_,v)| v.as_str())
    }

    pub fn set_array_of_ints(&mut self, key: &str, arr: Vec<i32>) {
        if let Some(e) = self.arrays_of_ints.iter_mut().find(|(k,_)| k == key) { e.1 = arr; }
        else { self.arrays_of_ints.push((key.to_string(), arr)); }
    }

    pub fn get_array_of_ints(&self, key: &str) -> Option<&[i32]> {
        self.arrays_of_ints.iter().find(|(k,_)| k == key).map(|(_,v)| v.as_slice())
    }

    pub fn nb_integers(&self) -> usize { self.integers.len() }
    pub fn nb_reals(&self) -> usize { self.reals.len() }
    pub fn nb_strings(&self) -> usize { self.strings.len() }

    pub fn remove(&mut self, key: &str) {
        self.integers.retain(|(k,_)| k != key);
        self.reals.retain(|(k,_)| k != key);
        self.strings.retain(|(k,_)| k != key);
        self.arrays_of_ints.retain(|(k,_)| k != key);
        self.arrays_of_reals.retain(|(k,_)| k != key);
    }

    pub fn all_integer_keys(&self) -> Vec<&str> {
        self.integers.iter().map(|(k,_)| k.as_str()).collect()
    }
}

/// Integer set stored compactly.
/// occt: TDataStd_IntPackedMap
#[derive(Clone, Debug, Default)]
pub struct TDataStdIntPackedMap {
    pub values: Vec<i32>,
}

impl TDataStdIntPackedMap {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, v: i32) -> bool {
        if !self.values.contains(&v) { self.values.push(v); true } else { false }
    }

    pub fn remove(&mut self, v: i32) -> bool {
        if let Some(pos) = self.values.iter().position(|&x| x == v) {
            self.values.remove(pos); true
        } else { false }
    }

    pub fn contains(&self, v: i32) -> bool { self.values.contains(&v) }
    pub fn nb_values(&self) -> usize { self.values.len() }
    pub fn is_empty(&self) -> bool { self.values.is_empty() }

    pub fn intersect(&self, other: &Self) -> Self {
        let values = self.values.iter().filter(|v| other.contains(**v)).cloned().collect();
        Self { values }
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut values = self.values.clone();
        for &v in &other.values { if !values.contains(&v) { values.push(v); } }
        Self { values }
    }
}

/// Array of label references.
/// occt: TDataStd_ReferenceArray
#[derive(Clone, Debug)]
pub struct TDataStdReferenceArray {
    pub lower: i32,
    pub upper: i32,
    pub references: Vec<Option<u32>>,    // label id references
}

impl TDataStdReferenceArray {
    pub fn new(lower: i32, upper: i32) -> Self {
        let n = (upper - lower + 1).max(0) as usize;
        Self { lower, upper, references: vec![None; n] }
    }

    fn idx(&self, i: i32) -> Option<usize> {
        if i < self.lower || i > self.upper { return None; }
        Some((i - self.lower) as usize)
    }

    pub fn set(&mut self, i: i32, label_id: u32) -> bool {
        if let Some(idx) = self.idx(i) { self.references[idx] = Some(label_id); true } else { false }
    }

    pub fn get(&self, i: i32) -> Option<u32> {
        self.idx(i).and_then(|idx| self.references[idx])
    }

    pub fn nb_references(&self) -> usize { self.references.len() }
    pub fn lower(&self) -> i32 { self.lower }
    pub fn upper(&self) -> i32 { self.upper }
    pub fn nb_set(&self) -> usize { self.references.iter().filter(|r| r.is_some()).count() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_data_set_get() {
        let mut d = TDataStdNamedData::new();
        d.set_integer("count", 42);
        d.set_real("scale", 3.14);
        d.set_string("name", "part");
        assert_eq!(d.get_integer("count"), Some(42));
        assert!((d.get_real("scale").unwrap() - 3.14).abs() < 1e-10);
        assert_eq!(d.get_string("name"), Some("part"));
    }

    #[test]
    fn named_data_override() {
        let mut d = TDataStdNamedData::new();
        d.set_integer("x", 1);
        d.set_integer("x", 2);
        assert_eq!(d.get_integer("x"), Some(2));
        assert_eq!(d.nb_integers(), 1);
    }

    #[test]
    fn named_data_remove() {
        let mut d = TDataStdNamedData::new();
        d.set_integer("a", 1);
        d.set_real("a", 2.0);
        d.remove("a");
        assert!(!d.has_integer("a"));
    }

    #[test]
    fn int_packed_map_ops() {
        let mut m = TDataStdIntPackedMap::new();
        assert!(m.add(10));
        assert!(m.add(20));
        assert!(!m.add(10)); // duplicate
        assert_eq!(m.nb_values(), 2);
        assert!(m.contains(10));
        assert!(m.remove(10));
        assert!(!m.contains(10));
    }

    #[test]
    fn int_packed_map_set_ops() {
        let mut a = TDataStdIntPackedMap::new();
        a.add(1); a.add(2); a.add(3);
        let mut b = TDataStdIntPackedMap::new();
        b.add(2); b.add(3); b.add(4);
        let inter = a.intersect(&b);
        assert_eq!(inter.nb_values(), 2);
        let un = a.union(&b);
        assert_eq!(un.nb_values(), 4);
    }

    #[test]
    fn reference_array_set_get() {
        let mut arr = TDataStdReferenceArray::new(1, 3);
        arr.set(1, 100);
        arr.set(3, 300);
        assert_eq!(arr.get(1), Some(100));
        assert_eq!(arr.get(2), None);
        assert_eq!(arr.get(3), Some(300));
        assert_eq!(arr.nb_set(), 2);
        assert!(arr.get(0).is_none()); // out of bounds
    }
}
