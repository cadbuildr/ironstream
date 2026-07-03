// FILE: tcolstd_array.rs
// occt: TColStd_Array1OfInteger, TColStd_Array1OfReal, TColStd_HArray1OfInteger

/// Array of integers (1-indexed, OCCT style).
/// occt: TColStd_Array1OfInteger
#[derive(Clone, Debug)]
pub struct TColStdArray1OfInteger {
    pub lower: usize,
    pub upper: usize,
    pub data: Vec<i32>,
}

impl TColStdArray1OfInteger {
    pub fn new(lower: usize, upper: usize) -> Self {
        Self {
            lower,
            upper,
            data: vec![0; upper - lower + 1],
        }
    }

    pub fn set(&mut self, index: usize, value: i32) {
        if index >= self.lower && index <= self.upper {
            self.data[index - self.lower] = value;
        }
    }

    pub fn value(&self, index: usize) -> Option<i32> {
        if index >= self.lower && index <= self.upper {
            Some(self.data[index - self.lower])
        } else {
            None
        }
    }

    pub fn length(&self) -> usize { self.upper - self.lower + 1 }
    pub fn lower_bound(&self) -> usize { self.lower }
    pub fn upper_bound(&self) -> usize { self.upper }
}

/// Array of real numbers (1-indexed, OCCT style).
/// occt: TColStd_Array1OfReal
#[derive(Clone, Debug)]
pub struct TColStdArray1OfReal {
    pub lower: usize,
    pub upper: usize,
    pub data: Vec<f64>,
}

impl TColStdArray1OfReal {
    pub fn new(lower: usize, upper: usize) -> Self {
        Self {
            lower,
            upper,
            data: vec![0.0; upper - lower + 1],
        }
    }

    pub fn set(&mut self, index: usize, value: f64) {
        if index >= self.lower && index <= self.upper {
            self.data[index - self.lower] = value;
        }
    }

    pub fn value(&self, index: usize) -> Option<f64> {
        if index >= self.lower && index <= self.upper {
            Some(self.data[index - self.lower])
        } else {
            None
        }
    }

    pub fn length(&self) -> usize { self.upper - self.lower + 1 }
    pub fn lower_bound(&self) -> usize { self.lower }
    pub fn upper_bound(&self) -> usize { self.upper }
}

/// Handle to integer array (reference-counted).
/// occt: TColStd_HArray1OfInteger
#[derive(Clone, Debug)]
pub struct TColStdHArray1OfInteger {
    pub array: TColStdArray1OfInteger,
}

impl TColStdHArray1OfInteger {
    pub fn new(lower: usize, upper: usize) -> Self {
        Self {
            array: TColStdArray1OfInteger::new(lower, upper),
        }
    }

    pub fn change_array(&mut self) -> &mut TColStdArray1OfInteger {
        &mut self.array
    }

    pub fn array_ref(&self) -> &TColStdArray1OfInteger {
        &self.array
    }
}

/// Handle to real array (reference-counted).
/// occt: TColStd_HArray1OfReal
#[derive(Clone, Debug)]
pub struct TColStdHArray1OfReal {
    pub array: TColStdArray1OfReal,
}

impl TColStdHArray1OfReal {
    pub fn new(lower: usize, upper: usize) -> Self {
        Self {
            array: TColStdArray1OfReal::new(lower, upper),
        }
    }

    pub fn change_array(&mut self) -> &mut TColStdArray1OfReal {
        &mut self.array
    }

    pub fn array_ref(&self) -> &TColStdArray1OfReal {
        &self.array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tcolstd_array1_of_integer() {
        let mut arr = TColStdArray1OfInteger::new(1, 5);
        arr.set(1, 10);
        arr.set(3, 30);
        assert_eq!(arr.value(1), Some(10));
        assert_eq!(arr.value(3), Some(30));
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn tcolstd_array1_of_real() {
        let mut arr = TColStdArray1OfReal::new(1, 3);
        arr.set(1, 1.5);
        arr.set(2, 2.5);
        assert!(arr.value(1).is_some());
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 3);
    }

    #[test]
    fn tcolstd_harray1_of_integer() {
        let mut h = TColStdHArray1OfInteger::new(1, 4);
        h.change_array().set(1, 100);
        assert_eq!(h.array_ref().value(1), Some(100));
    }

    #[test]
    fn tcolstd_harray1_of_real() {
        let mut h = TColStdHArray1OfReal::new(0, 2);
        h.change_array().set(0, 3.14);
        assert!((h.array_ref().value(0).unwrap() - 3.14).abs() < 0.01);
    }

    #[test]
    fn array_bounds() {
        let arr = TColStdArray1OfInteger::new(5, 10);
        assert_eq!(arr.lower_bound(), 5);
        assert_eq!(arr.upper_bound(), 10);
        assert_eq!(arr.length(), 6);
    }
}
