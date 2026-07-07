// FILE: t_col_geom2d_h_array1_of_bezier_curve.rs
// occt: TColGeom2d_HArray1OfBezierCurve

use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TColGeom2dArray1BezierCurve {
    lower: i32,
    upper: i32,
    data: Vec<u64>,
}

impl TColGeom2dArray1BezierCurve {
    pub fn new(lower: i32, upper: i32) -> Self {
        let size = (upper - lower + 1) as usize;
        Self {
            lower,
            upper,
            data: vec![0; size],
        }
    }

    pub fn lower(&self) -> i32 {
        self.lower
    }

    pub fn upper(&self) -> i32 {
        self.upper
    }

    pub fn len(&self) -> i32 {
        self.upper - self.lower + 1
    }

    pub fn at(&self, idx: i32) -> u64 {
        assert!(idx >= self.lower && idx <= self.upper, "Index out of bounds");
        self.data[(idx - self.lower) as usize]
    }

    pub fn set(&mut self, idx: i32, value: u64) {
        assert!(idx >= self.lower && idx <= self.upper, "Index out of bounds");
        self.data[(idx - self.lower) as usize] = value;
    }
}

pub type TColGeom2d_HArray1OfBezierCurve = Arc<TColGeom2dArray1BezierCurve>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harray_bounds() {
        let arr = Arc::new(TColGeom2dArray1BezierCurve::new(1, 5));
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
    }

    #[test]
    fn test_harray_shared() {
        let arr1 = Arc::new(TColGeom2dArray1BezierCurve::new(1, 3));
        let arr2 = Arc::clone(&arr1);
        assert_eq!(Arc::strong_count(&arr1), 2);
    }
}
