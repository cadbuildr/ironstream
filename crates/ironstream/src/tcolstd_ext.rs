// FILE: tcolstd_ext.rs
// occt-ref: TColStd_Array1OfReal, TColStd_Array2OfReal
//       TColStd_DataMapOfStringInteger, TColStd_ListOfInteger

use std::collections::HashMap;

// occt-ref: TColStd_Array1OfReal // — 1D array of f64 with arbitrary bounds
#[derive(Clone, Debug)]
pub struct Array1OfReal {
    pub lower: usize,
    pub upper: usize,
    data: Vec<f64>,
}

impl Array1OfReal {
    pub fn new(lower: usize, upper: usize) -> Self {
        assert!(upper >= lower);
        Self { lower, upper, data: vec![0.0; upper - lower + 1] }
    }

    pub fn with_init(lower: usize, upper: usize, v: f64) -> Self {
        let mut a = Self::new(lower, upper);
        a.init(v);
        a
    }

    pub fn from_vec(lower: usize, data: Vec<f64>) -> Self {
        let upper = lower + data.len().saturating_sub(1);
        Self { lower, upper, data }
    }

    pub fn len(&self) -> usize { self.upper - self.lower + 1 }
    pub fn lower(&self) -> usize { self.lower }
    pub fn upper(&self) -> usize { self.upper }
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    pub fn get(&self, i: usize) -> Option<f64> {
        if i < self.lower || i > self.upper { None } else { Some(self.data[i - self.lower]) }
    }

    pub fn set(&mut self, i: usize, v: f64) {
        if i >= self.lower && i <= self.upper { self.data[i - self.lower] = v; }
    }

    pub fn init(&mut self, v: f64) { for x in &mut self.data { *x = v; } }

    pub fn max_value(&self) -> f64 { self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max) }
    pub fn min_value(&self) -> f64 { self.data.iter().cloned().fold(f64::INFINITY, f64::min) }

    pub fn as_slice(&self) -> &[f64] { &self.data }

    pub fn resize(&mut self, new_lower: usize, new_upper: usize, init: f64) {
        let new_len = new_upper - new_lower + 1;
        let mut new_data = vec![init; new_len];
        // copy overlapping range
        let ov_l = self.lower.max(new_lower);
        let ov_u = self.upper.min(new_upper);
        if ov_l <= ov_u {
            for i in ov_l..=ov_u {
                new_data[i - new_lower] = self.data[i - self.lower];
            }
        }
        self.lower = new_lower;
        self.upper = new_upper;
        self.data = new_data;
    }
}

// occt-ref: TColStd_Array2OfReal // — 2D array of f64
#[derive(Clone, Debug)]
pub struct Array2OfReal {
    pub r_lower: usize,
    pub r_upper: usize,
    pub c_lower: usize,
    pub c_upper: usize,
    data: Vec<Vec<f64>>,
}

impl Array2OfReal {
    pub fn new(r1: usize, r2: usize, c1: usize, c2: usize) -> Self {
        assert!(r2 >= r1 && c2 >= c1);
        let rows = r2 - r1 + 1;
        let cols = c2 - c1 + 1;
        Self { r_lower: r1, r_upper: r2, c_lower: c1, c_upper: c2, data: vec![vec![0.0; cols]; rows] }
    }

    pub fn nb_rows(&self) -> usize { self.r_upper - self.r_lower + 1 }
    pub fn nb_cols(&self) -> usize { self.c_upper - self.c_lower + 1 }

    pub fn get(&self, r: usize, c: usize) -> Option<f64> {
        if r < self.r_lower || r > self.r_upper || c < self.c_lower || c > self.c_upper { return None; }
        Some(self.data[r - self.r_lower][c - self.c_lower])
    }

    pub fn set(&mut self, r: usize, c: usize, v: f64) {
        if r >= self.r_lower && r <= self.r_upper && c >= self.c_lower && c <= self.c_upper {
            self.data[r - self.r_lower][c - self.c_lower] = v;
        }
    }

    pub fn init(&mut self, v: f64) { for row in &mut self.data { for x in row { *x = v; } } }
}

// occt: TColStd_DataMapOfStringInteger // — hash map String → i32
#[derive(Clone, Debug, Default)]
pub struct DataMapOfStringInteger {
    pub map: HashMap<String, i32>,
}

impl DataMapOfStringInteger {
    pub fn new() -> Self { Self::default() }

    pub fn bind(&mut self, key: &str, value: i32) { self.map.insert(key.to_owned(), value); }
    pub fn unbind(&mut self, key: &str) -> bool { self.map.remove(key).is_some() }

    pub fn find(&self, key: &str) -> Option<i32> { self.map.get(key).copied() }
    pub fn is_bound(&self, key: &str) -> bool { self.map.contains_key(key) }

    pub fn size(&self) -> usize { self.map.len() }
    pub fn is_empty(&self) -> bool { self.map.is_empty() }
    pub fn clear(&mut self) { self.map.clear(); }

    pub fn keys(&self) -> Vec<&str> { self.map.keys().map(|k| k.as_str()).collect() }
    pub fn values(&self) -> Vec<i32> { self.map.values().copied().collect() }
}

// occt: TColStd_ListOfInteger // — singly-linked list of integers (stub: Vec-backed)
#[derive(Clone, Debug, Default)]
pub struct ListOfInteger {
    data: Vec<i32>,
}

impl ListOfInteger {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, v: i32) { self.data.push(v); }
    pub fn prepend(&mut self, v: i32) { self.data.insert(0, v); }
    pub fn remove_first(&mut self) { if !self.data.is_empty() { self.data.remove(0); } }
    pub fn first(&self) -> Option<i32> { self.data.first().copied() }
    pub fn last(&self) -> Option<i32> { self.data.last().copied() }
    pub fn size(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }
    pub fn clear(&mut self) { self.data.clear(); }
    pub fn contains(&self, v: i32) -> bool { self.data.contains(&v) }
    pub fn remove_value(&mut self, v: i32) -> bool {
        if let Some(pos) = self.data.iter().position(|&x| x == v) {
            self.data.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn as_slice(&self) -> &[i32] { &self.data }
}

// occt-ref: TColStd_SequenceOfReal // — sequence (ordered resizeable list) of f64
#[derive(Clone, Debug, Default)]
pub struct SequenceOfReal {
    data: Vec<f64>,
}

impl SequenceOfReal {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, v: f64) { self.data.push(v); }
    pub fn prepend(&mut self, v: f64) { self.data.insert(0, v); }
    pub fn insert_before(&mut self, i: usize, v: f64) { if i <= self.data.len() { self.data.insert(i, v); } }
    pub fn remove(&mut self, i: usize) { if i < self.data.len() { self.data.remove(i); } }
    pub fn value(&self, i: usize) -> Option<f64> { self.data.get(i).copied() }
    pub fn set_value(&mut self, i: usize, v: f64) { if let Some(x) = self.data.get_mut(i) { *x = v; } }
    pub fn first(&self) -> Option<f64> { self.data.first().copied() }
    pub fn last(&self) -> Option<f64> { self.data.last().copied() }
    pub fn length(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }
    pub fn clear(&mut self) { self.data.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array1_real_bounds() {
        let mut a = Array1OfReal::new(1, 5);
        a.set(1, 1.0);
        a.set(5, 5.0);
        assert_eq!(a.get(1), Some(1.0));
        assert_eq!(a.get(5), Some(5.0));
        assert!(a.get(0).is_none());
        assert_eq!(a.len(), 5);
    }

    #[test]
    fn array1_real_min_max() {
        let a = Array1OfReal::from_vec(0, vec![3.0, 1.0, 4.0, 1.0, 5.0]);
        assert!((a.max_value() - 5.0).abs() < 1e-10);
        assert!((a.min_value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn array1_real_resize() {
        let mut a = Array1OfReal::new(1, 3);
        a.set(1, 10.0); a.set(2, 20.0); a.set(3, 30.0);
        a.resize(1, 5, 0.0);
        assert_eq!(a.get(1), Some(10.0));
        assert_eq!(a.get(4), Some(0.0));
        assert_eq!(a.len(), 5);
    }

    #[test]
    fn array2_real() {
        let mut a = Array2OfReal::new(1, 3, 1, 3);
        a.set(2, 2, 9.0);
        assert_eq!(a.get(2, 2), Some(9.0));
        assert_eq!(a.nb_rows(), 3);
        assert_eq!(a.nb_cols(), 3);
    }

    #[test]
    fn data_map_string_integer() {
        let mut m = DataMapOfStringInteger::new();
        m.bind("alpha", 1);
        m.bind("beta", 2);
        assert_eq!(m.find("alpha"), Some(1));
        assert!(m.is_bound("beta"));
        m.unbind("alpha");
        assert!(!m.is_bound("alpha"));
        assert_eq!(m.size(), 1);
    }

    #[test]
    fn list_of_integer() {
        let mut l = ListOfInteger::new();
        l.append(10);
        l.append(20);
        l.prepend(5);
        assert_eq!(l.first(), Some(5));
        assert_eq!(l.size(), 3);
        l.remove_first();
        assert_eq!(l.first(), Some(10));
        assert!(l.contains(20));
        l.remove_value(20);
        assert!(!l.contains(20));
    }

    #[test]
    fn sequence_of_real() {
        let mut s = SequenceOfReal::new();
        s.append(1.0);
        s.append(2.0);
        s.prepend(0.0);
        assert_eq!(s.length(), 3);
        assert_eq!(s.value(0), Some(0.0));
        s.remove(0);
        assert_eq!(s.first(), Some(1.0));
    }
}
