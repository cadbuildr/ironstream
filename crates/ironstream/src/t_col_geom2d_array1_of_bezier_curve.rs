// FILE: t_col_geom2d_array1_of_bezier_curve.rs
// occt: TColGeom2d_Array1OfBezierCurve

/// TColGeom2d_Array1OfBezierCurve: a 1-based array of 2D Bezier curve handles.
#[derive(Debug, Clone)]
pub struct TColGeom2d_Array1OfBezierCurve {
    lower: i32,
    upper: i32,
    data: Vec<u64>,
}

impl TColGeom2d_Array1OfBezierCurve {
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

    pub fn is_empty(&self) -> bool {
        self.len() <= 0
    }

    pub fn at(&self, idx: i32) -> u64 {
        assert!(idx >= self.lower && idx <= self.upper, "Index out of bounds");
        self.data[(idx - self.lower) as usize]
    }

    pub fn set(&mut self, idx: i32, value: u64) {
        assert!(idx >= self.lower && idx <= self.upper, "Index out of bounds");
        self.data[(idx - self.lower) as usize] = value;
    }

    pub fn fill(&mut self, value: u64) {
        for elem in &mut self.data {
            *elem = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_bounds() {
        let arr = TColGeom2d_Array1OfBezierCurve::new(1, 10);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 10);
    }

    #[test]
    fn test_array_at_and_set() {
        let mut arr = TColGeom2d_Array1OfBezierCurve::new(1, 5);
        arr.set(3, 77);
        assert_eq!(arr.at(3), 77);
    }
}
