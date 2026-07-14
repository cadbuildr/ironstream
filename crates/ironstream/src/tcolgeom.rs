// FILE: tcolgeom.rs
// occt: TColgp_Array1OfDir
// occt-ref: TColgp_Array1OfVec, TColgp_Array1OfPnt2d
//       TColStd_Array1OfInteger, TColStd_SequenceOfInteger,
//       TColStd_Array1OfReal

/// 1D array of 3D directions (unit vectors).
/// occt: TColgp_Array1OfDir
#[derive(Clone, Debug)]
pub struct TColgpArray1OfDir {
    pub lower: usize,
    pub upper: usize,
    data: Vec<[f64; 3]>,
}

impl TColgpArray1OfDir {
    pub fn new(lower: usize, upper: usize) -> Self {
        let n = if upper >= lower { upper - lower + 1 } else { 0 };
        Self { lower, upper, data: vec![[0.0, 0.0, 1.0]; n] }
    }

    pub fn value(&self, i: usize) -> Option<[f64; 3]> {
        if i < self.lower || i > self.upper { return None; }
        Some(self.data[i - self.lower])
    }

    pub fn set_value(&mut self, i: usize, v: [f64; 3]) {
        if i < self.lower || i > self.upper { return; }
        let len = (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt();
        let v = if len > 1e-14 { [v[0]/len, v[1]/len, v[2]/len] } else { [0.0, 0.0, 1.0] };
        self.data[i - self.lower] = v;
    }

    pub fn length(&self) -> usize { self.data.len() }
    pub fn lower(&self) -> usize { self.lower }
    pub fn upper(&self) -> usize { self.upper }
}

/// 1D array of 3D vectors (not necessarily unit).
// occt-ref: TColgp_Array1OfVec
#[derive(Clone, Debug)]
pub struct TColgpArray1OfVec {
    pub lower: usize,
    pub upper: usize,
    data: Vec<[f64; 3]>,
}

impl TColgpArray1OfVec {
    pub fn new(lower: usize, upper: usize) -> Self {
        let n = if upper >= lower { upper - lower + 1 } else { 0 };
        Self { lower, upper, data: vec![[0.0; 3]; n] }
    }

    pub fn value(&self, i: usize) -> Option<[f64; 3]> {
        if i < self.lower || i > self.upper { return None; }
        Some(self.data[i - self.lower])
    }

    pub fn set_value(&mut self, i: usize, v: [f64; 3]) {
        if i < self.lower || i > self.upper { return; }
        self.data[i - self.lower] = v;
    }

    pub fn length(&self) -> usize { self.data.len() }
    pub fn lower(&self) -> usize { self.lower }
    pub fn upper(&self) -> usize { self.upper }
}

/// 1D array of integers.
// occt-ref: TColStd_Array1OfInteger
#[derive(Clone, Debug)]
pub struct TColStdArray1OfInt {
    pub lower: usize,
    pub upper: usize,
    data: Vec<i32>,
}

impl TColStdArray1OfInt {
    pub fn new(lower: usize, upper: usize) -> Self {
        let n = if upper >= lower { upper - lower + 1 } else { 0 };
        Self { lower, upper, data: vec![0; n] }
    }

    pub fn value(&self, i: usize) -> Option<i32> {
        if i < self.lower || i > self.upper { return None; }
        Some(self.data[i - self.lower])
    }

    pub fn set_value(&mut self, i: usize, v: i32) {
        if i < self.lower || i > self.upper { return; }
        self.data[i - self.lower] = v;
    }

    pub fn length(&self) -> usize { self.data.len() }
    pub fn lower(&self) -> usize { self.lower }
    pub fn upper(&self) -> usize { self.upper }
    pub fn init(&mut self, v: i32) { for x in &mut self.data { *x = v; } }

    pub fn sum(&self) -> i64 { self.data.iter().map(|&x| x as i64).sum() }
}

/// 1D array of real values.
// occt-ref: TColStd_Array1OfReal
#[derive(Clone, Debug)]
pub struct TColStdArray1OfReal {
    pub lower: usize,
    pub upper: usize,
    data: Vec<f64>,
}

impl TColStdArray1OfReal {
    pub fn new(lower: usize, upper: usize) -> Self {
        let n = if upper >= lower { upper - lower + 1 } else { 0 };
        Self { lower, upper, data: vec![0.0; n] }
    }

    pub fn value(&self, i: usize) -> Option<f64> {
        if i < self.lower || i > self.upper { return None; }
        Some(self.data[i - self.lower])
    }

    pub fn set_value(&mut self, i: usize, v: f64) {
        if i < self.lower || i > self.upper { return; }
        self.data[i - self.lower] = v;
    }

    pub fn length(&self) -> usize { self.data.len() }
    pub fn lower(&self) -> usize { self.lower }
    pub fn upper(&self) -> usize { self.upper }
    pub fn init(&mut self, v: f64) { for x in &mut self.data { *x = v; } }

    pub fn min(&self) -> Option<f64> { self.data.iter().cloned().reduce(f64::min) }
    pub fn max(&self) -> Option<f64> { self.data.iter().cloned().reduce(f64::max) }
}

/// Sequence (growable list) of integers.
// occt-ref: TColStd_SequenceOfInteger
#[derive(Clone, Debug, Default)]
pub struct TColStdSeqOfInt {
    data: Vec<i32>,
}

impl TColStdSeqOfInt {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, v: i32) { self.data.push(v); }
    pub fn prepend(&mut self, v: i32) { self.data.insert(0, v); }
    pub fn value(&self, i: usize) -> Option<i32> {
        if i == 0 { return None; }
        self.data.get(i - 1).copied()
    }
    pub fn set_value(&mut self, i: usize, v: i32) {
        if i >= 1 && i <= self.data.len() { self.data[i-1] = v; }
    }
    pub fn remove(&mut self, i: usize) {
        if i >= 1 && i <= self.data.len() { self.data.remove(i-1); }
    }
    pub fn length(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }
    pub fn clear(&mut self) { self.data.clear(); }
    pub fn first(&self) -> Option<i32> { self.data.first().copied() }
    pub fn last(&self) -> Option<i32> { self.data.last().copied() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_of_dir_normalized() {
        let mut a = TColgpArray1OfDir::new(1, 3);
        a.set_value(1, [3.0, 0.0, 0.0]);
        let v = a.value(1).unwrap();
        assert!((v[0] - 1.0).abs() < 1e-10);
        assert!(a.value(0).is_none());
        assert!(a.value(4).is_none());
    }

    #[test]
    fn array_of_vec_basic() {
        let mut a = TColgpArray1OfVec::new(1, 3);
        a.set_value(2, [1.0, 2.0, 3.0]);
        assert_eq!(a.value(2), Some([1.0, 2.0, 3.0]));
        assert_eq!(a.length(), 3);
    }

    #[test]
    fn array_of_int_init_sum() {
        let mut a = TColStdArray1OfInt::new(1, 5);
        a.init(3);
        assert_eq!(a.sum(), 15);
        a.set_value(3, 10);
        assert_eq!(a.value(3), Some(10));
    }

    #[test]
    fn array_of_real_min_max() {
        let mut a = TColStdArray1OfReal::new(1, 4);
        a.set_value(1, 3.0);
        a.set_value(2, 1.0);
        a.set_value(3, 5.0);
        a.set_value(4, 2.0);
        assert!((a.min().unwrap() - 1.0).abs() < 1e-10);
        assert!((a.max().unwrap() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn seq_of_int_append_value() {
        let mut s = TColStdSeqOfInt::new();
        s.append(10);
        s.append(20);
        s.prepend(5);
        assert_eq!(s.length(), 3);
        assert_eq!(s.first(), Some(5));
        assert_eq!(s.last(), Some(20));
        assert_eq!(s.value(2), Some(10));
        s.remove(1);
        assert_eq!(s.first(), Some(10));
    }
}
