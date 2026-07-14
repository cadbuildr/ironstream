// FILE: occt_collect.rs
// occt-ref: NCollection_Array1, NCollection_Array2, NCollection_List, NCollection_Sequence

// occt-ref: NCollection_Array1
/// Fixed-size 1-D array with arbitrary lower bound (OCCT style).
#[derive(Clone, Debug)]
pub struct Array1<T: Clone> {
    data: Vec<T>,
    lower: i32,
}

impl<T: Clone + Default> Array1<T> {
    pub fn new(lower: i32, upper: i32) -> Self {
        let size = (upper - lower + 1).max(0) as usize;
        Self { data: vec![T::default(); size], lower }
    }

    pub fn size(&self) -> usize { self.data.len() }
    pub fn lower(&self) -> i32 { self.lower }
    pub fn upper(&self) -> i32 { self.lower + self.data.len() as i32 - 1 }

    pub fn get(&self, i: i32) -> Option<&T> {
        let idx = (i - self.lower) as usize;
        self.data.get(idx)
    }

    pub fn set(&mut self, i: i32, val: T) {
        let idx = (i - self.lower) as usize;
        if idx < self.data.len() { self.data[idx] = val; }
    }

    pub fn init(&mut self, val: T) {
        for e in &mut self.data { *e = val.clone(); }
    }
}

// occt-ref: NCollection_Array2
/// Fixed-size 2-D array.
#[derive(Clone, Debug)]
pub struct Array2<T: Clone> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
    row_lower: i32,
    col_lower: i32,
}

impl<T: Clone + Default> Array2<T> {
    pub fn new(row_lower: i32, row_upper: i32, col_lower: i32, col_upper: i32) -> Self {
        let rows = (row_upper - row_lower + 1).max(0) as usize;
        let cols = (col_upper - col_lower + 1).max(0) as usize;
        Self { data: vec![T::default(); rows * cols], rows, cols, row_lower, col_lower }
    }

    pub fn nb_rows(&self) -> usize { self.rows }
    pub fn nb_cols(&self) -> usize { self.cols }

    pub fn get(&self, r: i32, c: i32) -> Option<&T> {
        let ri = (r - self.row_lower) as usize;
        let ci = (c - self.col_lower) as usize;
        if ri < self.rows && ci < self.cols {
            self.data.get(ri * self.cols + ci)
        } else { None }
    }

    pub fn set(&mut self, r: i32, c: i32, val: T) {
        let ri = (r - self.row_lower) as usize;
        let ci = (c - self.col_lower) as usize;
        if ri < self.rows && ci < self.cols {
            let idx = ri * self.cols + ci;
            self.data[idx] = val;
        }
    }
}

// occt-ref: NCollection_List
/// Singly-linked list used in OCCT collection APIs.
#[derive(Clone, Debug)]
pub struct List<T: Clone> {
    items: Vec<T>,
}

impl<T: Clone> Default for List<T> {
    fn default() -> Self { Self { items: Vec::new() } }
}

impl<T: Clone> List<T> {
    pub fn new() -> Self { Self::default() }
    pub fn append(&mut self, val: T) { self.items.push(val); }
    pub fn prepend(&mut self, val: T) { self.items.insert(0, val); }
    pub fn size(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn first(&self) -> Option<&T> { self.items.first() }
    pub fn last(&self) -> Option<&T> { self.items.last() }
    pub fn clear(&mut self) { self.items.clear(); }
    pub fn iter(&self) -> impl Iterator<Item = &T> { self.items.iter() }

    pub fn remove_first(&mut self) {
        if !self.items.is_empty() { self.items.remove(0); }
    }
}

// occt-ref: NCollection_Sequence
/// Indexed sequence (1-based) backed by a Vec.
#[derive(Clone, Debug)]
pub struct Sequence<T: Clone> {
    items: Vec<T>,
}

impl<T: Clone> Default for Sequence<T> {
    fn default() -> Self { Self { items: Vec::new() } }
}

impl<T: Clone> Sequence<T> {
    pub fn new() -> Self { Self::default() }
    pub fn append(&mut self, val: T) { self.items.push(val); }
    pub fn prepend(&mut self, val: T) { self.items.insert(0, val); }
    pub fn size(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn clear(&mut self) { self.items.clear(); }

    pub fn value(&self, i: usize) -> Option<&T> {
        self.items.get(i.saturating_sub(1))
    }

    pub fn set_value(&mut self, i: usize, val: T) {
        let idx = i.saturating_sub(1);
        if idx < self.items.len() { self.items[idx] = val; }
    }

    pub fn remove(&mut self, i: usize) {
        let idx = i.saturating_sub(1);
        if idx < self.items.len() { self.items.remove(idx); }
    }

    pub fn insert(&mut self, before: usize, val: T) {
        let idx = before.saturating_sub(1).min(self.items.len());
        self.items.insert(idx, val);
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> { self.items.iter() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array1_get_set() {
        let mut a: Array1<f64> = Array1::new(1, 5);
        assert_eq!(a.size(), 5);
        a.set(3, 3.14);
        assert!((a.get(3).unwrap() - 3.14).abs() < 1e-10);
    }

    #[test]
    fn array2_get_set() {
        let mut a: Array2<i32> = Array2::new(1, 3, 1, 4);
        a.set(2, 3, 42);
        assert_eq!(a.get(2, 3), Some(&42));
        assert_eq!(a.nb_rows(), 3);
        assert_eq!(a.nb_cols(), 4);
    }

    #[test]
    fn list_ops() {
        let mut l: List<i32> = List::new();
        l.append(10);
        l.append(20);
        l.prepend(5);
        assert_eq!(l.size(), 3);
        assert_eq!(l.first(), Some(&5));
    }

    #[test]
    fn sequence_value_set() {
        let mut s: Sequence<&str> = Sequence::new();
        s.append("a");
        s.append("b");
        s.append("c");
        assert_eq!(s.value(2), Some(&"b"));
        s.set_value(2, "B");
        assert_eq!(s.value(2), Some(&"B"));
    }

    #[test]
    fn sequence_remove() {
        let mut s: Sequence<u32> = Sequence::new();
        s.append(1); s.append(2); s.append(3);
        s.remove(2);
        assert_eq!(s.size(), 2);
        assert_eq!(s.value(1), Some(&1));
        assert_eq!(s.value(2), Some(&3));
    }
}
