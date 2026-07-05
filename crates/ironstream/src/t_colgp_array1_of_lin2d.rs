// FILE: t_colgp_array1_of_lin2d.rs
// occt: TColgp_Array1OfLin2d

/// A 2D line representation (gp_Lin2d in OCCT).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lin2d {
    /// Point X on line
    pub px: f64,
    /// Point Y on line
    pub py: f64,
    /// Direction X (normalized)
    pub dx: f64,
    /// Direction Y (normalized)
    pub dy: f64,
}

impl Lin2d {
    /// Creates a 2D line from a point and a direction.
    pub fn new(px: f64, py: f64, dx: f64, dy: f64) -> Self {
        let mag = (dx * dx + dy * dy).sqrt();
        if mag == 0.0 {
            panic!("Cannot create line from zero direction vector");
        }
        Lin2d {
            px,
            py,
            dx: dx / mag,
            dy: dy / mag,
        }
    }
}

/// 1-based Array1 of 2D lines (gp_Lin2d), mirroring NCollection_Array1 semantics.
#[derive(Debug, Clone)]
pub struct TColgpArray1OfLin2d {
    lower: usize,
    upper: usize,
    data: Vec<Lin2d>,
}

impl TColgpArray1OfLin2d {
    /// Creates an array with bounds [lower, upper].
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper {
            panic!("Lower bound {} exceeds upper bound {}", lower, upper);
        }
        let size = upper - lower + 1;
        TColgpArray1OfLin2d {
            lower,
            upper,
            data: vec![Lin2d { px: 0.0, py: 0.0, dx: 1.0, dy: 0.0 }; size],
        }
    }

    /// Returns the lower bound.
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Returns the upper bound.
    pub fn upper(&self) -> usize {
        self.upper
    }

    /// Returns the length of the array.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Gets the element at the given index (within bounds).
    pub fn get(&self, idx: usize) -> Lin2d {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        self.data[idx - self.lower]
    }

    /// Sets the element at the given index (within bounds).
    pub fn set(&mut self, idx: usize, value: Lin2d) {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        self.data[idx - self.lower] = value;
    }

    /// Gets a reference to the element at the given index.
    pub fn at(&self, idx: usize) -> &Lin2d {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        &self.data[idx - self.lower]
    }

    /// Gets a mutable reference to the element at the given index.
    pub fn at_mut(&mut self, idx: usize) -> &mut Lin2d {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        &mut self.data[idx - self.lower]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lin2d_direction_normalization() {
        let l = Lin2d::new(1.0, 2.0, 3.0, 4.0);
        let mag = (l.dx * l.dx + l.dy * l.dy).sqrt();
        assert!((mag - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_array_creation_with_bounds() {
        let arr = TColgpArray1OfLin2d::new(1, 4);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 4);
        assert_eq!(arr.len(), 4);
    }

    #[test]
    fn test_array_set_and_get() {
        let mut arr = TColgpArray1OfLin2d::new(0, 2);
        let line = Lin2d::new(5.0, 6.0, 1.0, 0.0);
        arr.set(1, line);

        let retrieved = arr.get(1);
        assert_eq!(retrieved.px, 5.0);
        assert_eq!(retrieved.py, 6.0);
    }

    #[test]
    fn test_array_at_mutable() {
        let mut arr = TColgpArray1OfLin2d::new(1, 2);
        arr.at_mut(1).px = 10.0;
        arr.at_mut(1).py = 20.0;

        let retrieved = arr.get(1);
        assert_eq!(retrieved.px, 10.0);
        assert_eq!(retrieved.py, 20.0);
    }

    #[test]
    fn test_array_default_initialization() {
        let arr = TColgpArray1OfLin2d::new(5, 7);
        for i in 5..=7 {
            let l = arr.get(i);
            assert_eq!(l.dx, 1.0);
            assert_eq!(l.dy, 0.0);
        }
    }

    #[test]
    #[should_panic]
    fn test_array_get_out_of_bounds() {
        let arr = TColgpArray1OfLin2d::new(3, 8);
        let _ = arr.get(2);
    }

    #[test]
    #[should_panic]
    fn test_lin2d_zero_direction() {
        let _ = Lin2d::new(0.0, 0.0, 0.0, 0.0);
    }
}
