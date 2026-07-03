// FILE: geom_array2.rs
// occt: TColgp_Array2OfPnt, TColgp_Array1OfPnt, TColgp_Array1OfVec, TColgp_Array2OfVec

/// 1-D array of 3-D points (1-based indexing, arbitrary lower bound).
#[derive(Clone, Debug)]
pub struct Array1OfPnt {
    data: Vec<[f64; 3]>,
    lower: i32,
}

impl Array1OfPnt {
    pub fn new(lower: i32, upper: i32) -> Self {
        let size = (upper - lower + 1).max(0) as usize;
        Self { data: vec![[0.0; 3]; size], lower }
    }

    pub fn from_vec(pts: Vec<[f64; 3]>) -> Self {
        let n = pts.len() as i32;
        Self { data: pts, lower: 1 }
    }

    pub fn size(&self) -> usize { self.data.len() }
    pub fn lower(&self) -> i32 { self.lower }
    pub fn upper(&self) -> i32 { self.lower + self.data.len() as i32 - 1 }

    pub fn value(&self, i: i32) -> Option<[f64; 3]> {
        let idx = (i - self.lower) as usize;
        self.data.get(idx).copied()
    }

    pub fn set_value(&mut self, i: i32, p: [f64; 3]) {
        let idx = (i - self.lower) as usize;
        if idx < self.data.len() { self.data[idx] = p; }
    }

    pub fn iter(&self) -> impl Iterator<Item = &[f64; 3]> { self.data.iter() }
}

/// 2-D array of 3-D points (1-based indexing).
#[derive(Clone, Debug)]
pub struct Array2OfPnt {
    data: Vec<[f64; 3]>,
    rows: usize,
    cols: usize,
    row_lower: i32,
    col_lower: i32,
}

impl Array2OfPnt {
    pub fn new(row_lower: i32, row_upper: i32, col_lower: i32, col_upper: i32) -> Self {
        let rows = (row_upper - row_lower + 1).max(0) as usize;
        let cols = (col_upper - col_lower + 1).max(0) as usize;
        Self { data: vec![[0.0; 3]; rows * cols], rows, cols, row_lower, col_lower }
    }

    pub fn nb_rows(&self) -> usize { self.rows }
    pub fn nb_cols(&self) -> usize { self.cols }
    pub fn size(&self) -> usize { self.rows * self.cols }

    pub fn value(&self, r: i32, c: i32) -> Option<[f64; 3]> {
        let ri = (r - self.row_lower) as usize;
        let ci = (c - self.col_lower) as usize;
        if ri < self.rows && ci < self.cols { self.data.get(ri * self.cols + ci).copied() }
        else { None }
    }

    pub fn set_value(&mut self, r: i32, c: i32, p: [f64; 3]) {
        let ri = (r - self.row_lower) as usize;
        let ci = (c - self.col_lower) as usize;
        if ri < self.rows && ci < self.cols {
            let idx = ri * self.cols + ci;
            self.data[idx] = p;
        }
    }

    pub fn row(&self, r: i32) -> Vec<[f64; 3]> {
        let ri = (r - self.row_lower) as usize;
        if ri >= self.rows { return Vec::new(); }
        (0..self.cols).map(|c| self.data[ri * self.cols + c]).collect()
    }

    pub fn col(&self, c: i32) -> Vec<[f64; 3]> {
        let ci = (c - self.col_lower) as usize;
        if ci >= self.cols { return Vec::new(); }
        (0..self.rows).map(|r| self.data[r * self.cols + ci]).collect()
    }
}

/// 1-D array of 3-D vectors.
#[derive(Clone, Debug)]
pub struct Array1OfVec {
    data: Vec<[f64; 3]>,
    lower: i32,
}

impl Array1OfVec {
    pub fn new(lower: i32, upper: i32) -> Self {
        let size = (upper - lower + 1).max(0) as usize;
        Self { data: vec![[0.0; 3]; size], lower }
    }

    pub fn size(&self) -> usize { self.data.len() }
    pub fn value(&self, i: i32) -> Option<[f64; 3]> {
        self.data.get((i - self.lower) as usize).copied()
    }
    pub fn set_value(&mut self, i: i32, v: [f64; 3]) {
        let idx = (i - self.lower) as usize;
        if idx < self.data.len() { self.data[idx] = v; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array1_pnt() {
        let mut a = Array1OfPnt::new(1, 4);
        assert_eq!(a.size(), 4);
        a.set_value(2, [1.0, 2.0, 3.0]);
        assert_eq!(a.value(2), Some([1.0, 2.0, 3.0]));
    }

    #[test]
    fn array1_pnt_from_vec() {
        let a = Array1OfPnt::from_vec(vec![[0.0;3],[1.0,0.0,0.0]]);
        assert_eq!(a.size(), 2);
        assert_eq!(a.lower(), 1);
    }

    #[test]
    fn array2_pnt() {
        let mut a = Array2OfPnt::new(1, 3, 1, 4);
        a.set_value(2, 3, [5.0, 6.0, 7.0]);
        assert_eq!(a.value(2, 3), Some([5.0, 6.0, 7.0]));
        assert_eq!(a.nb_rows(), 3);
        assert_eq!(a.nb_cols(), 4);
    }

    #[test]
    fn array2_row_col() {
        let mut a = Array2OfPnt::new(1, 2, 1, 3);
        a.set_value(1, 1, [1.0,0.0,0.0]);
        a.set_value(1, 2, [2.0,0.0,0.0]);
        a.set_value(1, 3, [3.0,0.0,0.0]);
        let row = a.row(1);
        assert_eq!(row.len(), 3);
        assert_eq!(row[0], [1.0,0.0,0.0]);
    }

    #[test]
    fn array1_vec() {
        let mut a = Array1OfVec::new(0, 2);
        a.set_value(1, [0.0, 1.0, 0.0]);
        assert_eq!(a.value(1), Some([0.0, 1.0, 0.0]));
    }
}
