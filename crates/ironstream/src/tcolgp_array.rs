// FILE: tcolgp_array.rs
// occt-ref: TColgp_Array1OfPnt, TColgp_Array1OfVec, TColgp_Array1OfPnt2d

/// Array of 3D points (1-indexed).
// occt-ref: TColgp_Array1OfPnt
#[derive(Clone, Debug)]
pub struct TColgpArray1OfPnt {
    pub lower: usize,
    pub upper: usize,
    pub data: Vec<[f64; 3]>,
}

impl TColgpArray1OfPnt {
    pub fn new(lower: usize, upper: usize) -> Self {
        Self {
            lower,
            upper,
            data: vec![[0.0, 0.0, 0.0]; upper - lower + 1],
        }
    }

    pub fn set(&mut self, index: usize, pnt: [f64; 3]) {
        if index >= self.lower && index <= self.upper {
            self.data[index - self.lower] = pnt;
        }
    }

    pub fn value(&self, index: usize) -> Option<[f64; 3]> {
        if index >= self.lower && index <= self.upper {
            Some(self.data[index - self.lower])
        } else {
            None
        }
    }

    pub fn length(&self) -> usize { self.upper - self.lower + 1 }
}

/// Array of 3D vectors (1-indexed).
// occt-ref: TColgp_Array1OfVec
#[derive(Clone, Debug)]
pub struct TColgpArray1OfVec {
    pub lower: usize,
    pub upper: usize,
    pub data: Vec<[f64; 3]>,
}

impl TColgpArray1OfVec {
    pub fn new(lower: usize, upper: usize) -> Self {
        Self {
            lower,
            upper,
            data: vec![[0.0, 0.0, 0.0]; upper - lower + 1],
        }
    }

    pub fn set(&mut self, index: usize, vec: [f64; 3]) {
        if index >= self.lower && index <= self.upper {
            self.data[index - self.lower] = vec;
        }
    }

    pub fn value(&self, index: usize) -> Option<[f64; 3]> {
        if index >= self.lower && index <= self.upper {
            Some(self.data[index - self.lower])
        } else {
            None
        }
    }

    pub fn length(&self) -> usize { self.upper - self.lower + 1 }
}

/// Array of 2D points (1-indexed).
// occt-ref: TColgp_Array1OfPnt2d
#[derive(Clone, Debug)]
pub struct TColgpArray1OfPnt2d {
    pub lower: usize,
    pub upper: usize,
    pub data: Vec<[f64; 2]>,
}

impl TColgpArray1OfPnt2d {
    pub fn new(lower: usize, upper: usize) -> Self {
        Self {
            lower,
            upper,
            data: vec![[0.0, 0.0]; upper - lower + 1],
        }
    }

    pub fn set(&mut self, index: usize, pnt: [f64; 2]) {
        if index >= self.lower && index <= self.upper {
            self.data[index - self.lower] = pnt;
        }
    }

    pub fn value(&self, index: usize) -> Option<[f64; 2]> {
        if index >= self.lower && index <= self.upper {
            Some(self.data[index - self.lower])
        } else {
            None
        }
    }

    pub fn length(&self) -> usize { self.upper - self.lower + 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tcolgp_array1_of_pnt() {
        let mut arr = TColgpArray1OfPnt::new(1, 3);
        arr.set(1, [1.0, 0.0, 0.0]);
        arr.set(2, [0.0, 1.0, 0.0]);
        assert_eq!(arr.value(1), Some([1.0, 0.0, 0.0]));
        assert_eq!(arr.length(), 3);
    }

    #[test]
    fn tcolgp_array1_of_vec() {
        let mut arr = TColgpArray1OfVec::new(0, 2);
        arr.set(0, [1.0, 0.0, 0.0]);
        arr.set(1, [0.0, 1.0, 0.0]);
        arr.set(2, [0.0, 0.0, 1.0]);
        assert_eq!(arr.value(1), Some([0.0, 1.0, 0.0]));
    }

    #[test]
    fn tcolgp_array1_of_pnt2d() {
        let mut arr = TColgpArray1OfPnt2d::new(1, 2);
        arr.set(1, [1.5, 2.5]);
        assert_eq!(arr.value(1), Some([1.5, 2.5]));
    }

    #[test]
    fn array_indexing() {
        let mut arr = TColgpArray1OfPnt::new(5, 8);
        arr.set(5, [1.0, 2.0, 3.0]);
        arr.set(8, [4.0, 5.0, 6.0]);
        assert!(arr.value(4).is_none());
        assert!(arr.value(5).is_some());
        assert!(arr.value(8).is_some());
        assert!(arr.value(9).is_none());
    }

    #[test]
    fn pnt2d_array() {
        let mut arr = TColgpArray1OfPnt2d::new(1, 5);
        for i in 1..=5 {
            arr.set(i, [i as f64, i as f64 * 2.0]);
        }
        assert_eq!(arr.length(), 5);
    }
}
