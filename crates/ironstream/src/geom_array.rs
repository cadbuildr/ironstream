// FILE: geom_array.rs
// occt-ref: TColgp_Array1OfPnt, TColgp_Array2OfPnt
//       TColgp_Array1OfPnt2d, TColgp_SequenceOfPnt,
//       TColgp_HArray1OfPnt

/// 1D array of 3D points (1-based, like OCCT).
#[derive(Clone, Debug)]
pub struct Array1OfPnt {
    pub lower: usize,
    pub upper: usize,
    data: Vec<[f64; 3]>,
}

impl Array1OfPnt {
    pub fn new(lower: usize, upper: usize) -> Self {
        assert!(upper >= lower, "upper must be >= lower");
        Self { lower, upper, data: vec![[0.0; 3]; upper - lower + 1] }
    }

    pub fn from_vec(lower: usize, data: Vec<[f64; 3]>) -> Self {
        let upper = lower + data.len().saturating_sub(1);
        Self { lower, upper, data }
    }

    pub fn lower(&self) -> usize { self.lower }
    pub fn upper(&self) -> usize { self.upper }
    pub fn length(&self) -> usize { self.upper - self.lower + 1 }
    pub fn is_empty(&self) -> bool { self.length() == 0 }

    pub fn value(&self, i: usize) -> Option<[f64; 3]> {
        if i < self.lower || i > self.upper { None } else { Some(self.data[i - self.lower]) }
    }

    pub fn set_value(&mut self, i: usize, p: [f64; 3]) {
        if i >= self.lower && i <= self.upper { self.data[i - self.lower] = p; }
    }

    pub fn as_slice(&self) -> &[[f64; 3]] { &self.data }

    pub fn init(&mut self, p: [f64; 3]) { self.data.iter_mut().for_each(|x| *x = p); }

    pub fn bounding_box(&self) -> ([f64; 3], [f64; 3]) {
        let mut mn = [f64::INFINITY; 3];
        let mut mx = [f64::NEG_INFINITY; 3];
        for &p in &self.data {
            for k in 0..3 { mn[k] = mn[k].min(p[k]); mx[k] = mx[k].max(p[k]); }
        }
        (mn, mx)
    }
}

/// 2D array of 3D points (row × col, both 1-based).
#[derive(Clone, Debug)]
pub struct Array2OfPnt {
    pub r_lower: usize,
    pub r_upper: usize,
    pub c_lower: usize,
    pub c_upper: usize,
    data: Vec<Vec<[f64; 3]>>,
}

impl Array2OfPnt {
    pub fn new(r1: usize, r2: usize, c1: usize, c2: usize) -> Self {
        assert!(r2 >= r1 && c2 >= c1);
        let rows = r2 - r1 + 1;
        let cols = c2 - c1 + 1;
        Self {
            r_lower: r1, r_upper: r2, c_lower: c1, c_upper: c2,
            data: vec![vec![[0.0; 3]; cols]; rows],
        }
    }

    pub fn nb_rows(&self) -> usize { self.r_upper - self.r_lower + 1 }
    pub fn nb_cols(&self) -> usize { self.c_upper - self.c_lower + 1 }

    pub fn value(&self, r: usize, c: usize) -> Option<[f64; 3]> {
        if r < self.r_lower || r > self.r_upper || c < self.c_lower || c > self.c_upper { return None; }
        Some(self.data[r - self.r_lower][c - self.c_lower])
    }

    pub fn set_value(&mut self, r: usize, c: usize, p: [f64; 3]) {
        if r >= self.r_lower && r <= self.r_upper && c >= self.c_lower && c <= self.c_upper {
            self.data[r - self.r_lower][c - self.c_lower] = p;
        }
    }

    pub fn init(&mut self, p: [f64; 3]) {
        for row in &mut self.data { for x in row { *x = p; } }
    }
}

/// 1D array of 2D points (1-based).
#[derive(Clone, Debug)]
pub struct Array1OfPnt2d {
    pub lower: usize,
    pub upper: usize,
    data: Vec<[f64; 2]>,
}

impl Array1OfPnt2d {
    pub fn new(lower: usize, upper: usize) -> Self {
        assert!(upper >= lower);
        Self { lower, upper, data: vec![[0.0; 2]; upper - lower + 1] }
    }

    pub fn from_vec(lower: usize, data: Vec<[f64; 2]>) -> Self {
        let upper = lower + data.len().saturating_sub(1);
        Self { lower, upper, data }
    }

    pub fn lower(&self) -> usize { self.lower }
    pub fn upper(&self) -> usize { self.upper }
    pub fn length(&self) -> usize { self.upper - self.lower + 1 }

    pub fn value(&self, i: usize) -> Option<[f64; 2]> {
        if i < self.lower || i > self.upper { None } else { Some(self.data[i - self.lower]) }
    }

    pub fn set_value(&mut self, i: usize, p: [f64; 2]) {
        if i >= self.lower && i <= self.upper { self.data[i - self.lower] = p; }
    }

    pub fn as_slice(&self) -> &[[f64; 2]] { &self.data }
}

/// Sequence of 3D points (0-based internally but 1-based API like OCCT).
#[derive(Clone, Debug, Default)]
pub struct SequenceOfPnt {
    data: Vec<[f64; 3]>,
}

impl SequenceOfPnt {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, p: [f64; 3]) { self.data.push(p); }
    pub fn prepend(&mut self, p: [f64; 3]) { self.data.insert(0, p); }

    pub fn value(&self, i: usize) -> Option<[f64; 3]> {
        if i == 0 { None } else { self.data.get(i - 1).copied() }
    }

    pub fn set_value(&mut self, i: usize, p: [f64; 3]) {
        if i >= 1 && i <= self.data.len() { self.data[i - 1] = p; }
    }

    pub fn remove(&mut self, i: usize) {
        if i >= 1 && i <= self.data.len() { self.data.remove(i - 1); }
    }

    pub fn insert_before(&mut self, i: usize, p: [f64; 3]) {
        if i >= 1 && i <= self.data.len() + 1 { self.data.insert(i - 1, p); }
    }

    pub fn first(&self) -> Option<[f64; 3]> { self.data.first().copied() }
    pub fn last(&self) -> Option<[f64; 3]> { self.data.last().copied() }
    pub fn length(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }
    pub fn clear(&mut self) { self.data.clear(); }

    pub fn as_slice(&self) -> &[[f64; 3]] { &self.data }

    pub fn centroid(&self) -> Option<[f64; 3]> {
        if self.data.is_empty() { return None; }
        let n = self.data.len() as f64;
        let mut s = [0.0f64; 3];
        for &p in &self.data { for k in 0..3 { s[k] += p[k]; } }
        Some([s[0]/n, s[1]/n, s[2]/n])
    }
}

/// Sequence of 2D points.
#[derive(Clone, Debug, Default)]
pub struct SequenceOfPnt2d {
    data: Vec<[f64; 2]>,
}

impl SequenceOfPnt2d {
    pub fn new() -> Self { Self::default() }

    pub fn append(&mut self, p: [f64; 2]) { self.data.push(p); }
    pub fn prepend(&mut self, p: [f64; 2]) { self.data.insert(0, p); }
    pub fn value(&self, i: usize) -> Option<[f64; 2]> {
        if i == 0 { None } else { self.data.get(i - 1).copied() }
    }
    pub fn length(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }
    pub fn clear(&mut self) { self.data.clear(); }
    pub fn as_slice(&self) -> &[[f64; 2]] { &self.data }
}

/// Handle-based 1D array (OCCTHArray1OfPnt stub).
#[derive(Clone, Debug)]
pub struct HArray1OfPnt {
    pub inner: Array1OfPnt,
}

impl HArray1OfPnt {
    pub fn new(lower: usize, upper: usize) -> Self {
        Self { inner: Array1OfPnt::new(lower, upper) }
    }

    pub fn array(&self) -> &Array1OfPnt { &self.inner }
    pub fn array_mut(&mut self) -> &mut Array1OfPnt { &mut self.inner }
    pub fn value(&self, i: usize) -> Option<[f64; 3]> { self.inner.value(i) }
    pub fn set_value(&mut self, i: usize, p: [f64; 3]) { self.inner.set_value(i, p); }
    pub fn length(&self) -> usize { self.inner.length() }
    pub fn lower(&self) -> usize { self.inner.lower() }
    pub fn upper(&self) -> usize { self.inner.upper() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array1_pnt_basic() {
        let mut a = Array1OfPnt::new(1, 4);
        a.set_value(1, [1.0, 0.0, 0.0]);
        a.set_value(4, [4.0, 0.0, 0.0]);
        assert_eq!(a.value(1), Some([1.0, 0.0, 0.0]));
        assert_eq!(a.value(4), Some([4.0, 0.0, 0.0]));
        assert_eq!(a.value(0), None);
        assert_eq!(a.length(), 4);
    }

    #[test]
    fn array2_pnt_basic() {
        let mut a = Array2OfPnt::new(1, 3, 1, 3);
        a.set_value(2, 2, [5.0, 5.0, 0.0]);
        assert_eq!(a.value(2, 2), Some([5.0, 5.0, 0.0]));
        assert_eq!(a.nb_rows(), 3);
        assert_eq!(a.nb_cols(), 3);
        assert_eq!(a.value(0, 1), None);
    }

    #[test]
    fn array1_pnt2d_basic() {
        let mut a = Array1OfPnt2d::new(1, 3);
        a.set_value(2, [3.0, 4.0]);
        assert_eq!(a.value(2), Some([3.0, 4.0]));
        assert_eq!(a.value(0), None);
        assert_eq!(a.length(), 3);
    }

    #[test]
    fn sequence_pnt_1based() {
        let mut s = SequenceOfPnt::new();
        s.append([1.0, 0.0, 0.0]);
        s.append([2.0, 0.0, 0.0]);
        s.prepend([0.0, 0.0, 0.0]);
        assert_eq!(s.length(), 3);
        assert_eq!(s.value(1), Some([0.0, 0.0, 0.0]));
        assert_eq!(s.value(0), None);
        s.remove(1);
        assert_eq!(s.first(), Some([1.0, 0.0, 0.0]));
    }

    #[test]
    fn sequence_pnt_centroid() {
        let mut s = SequenceOfPnt::new();
        s.append([0.0, 0.0, 0.0]);
        s.append([2.0, 0.0, 0.0]);
        let c = s.centroid().unwrap();
        assert!((c[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn harray1_pnt_wrapper() {
        let mut h = HArray1OfPnt::new(1, 5);
        h.set_value(3, [7.0, 8.0, 9.0]);
        assert_eq!(h.value(3), Some([7.0, 8.0, 9.0]));
        assert_eq!(h.length(), 5);
        assert_eq!(h.lower(), 1);
        assert_eq!(h.upper(), 5);
    }
}
