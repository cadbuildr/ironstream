// FILE: t_colgp_array1_of_xy.rs
// occt: TColgp_Array1OfXY

/// A 2D coordinate pair (gp_XY in OCCT).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XY {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}

impl XY {
    /// Creates a 2D coordinate.
    pub fn new(x: f64, y: f64) -> Self {
        XY { x, y }
    }
}

/// 1-based Array1 of 2D coordinates (gp_XY), mirroring NCollection_Array1 semantics.
#[derive(Debug, Clone)]
pub struct TColgpArray1OfXY {
    lower: usize,
    upper: usize,
    data: Vec<XY>,
}

impl TColgpArray1OfXY {
    /// Creates an array with bounds [lower, upper].
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper {
            panic!("Lower bound {} exceeds upper bound {}", lower, upper);
        }
        let size = upper - lower + 1;
        TColgpArray1OfXY {
            lower,
            upper,
            data: vec![XY { x: 0.0, y: 0.0 }; size],
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
    pub fn get(&self, idx: usize) -> XY {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        self.data[idx - self.lower]
    }

    /// Sets the element at the given index (within bounds).
    pub fn set(&mut self, idx: usize, value: XY) {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        self.data[idx - self.lower] = value;
    }

    /// Gets a reference to the element at the given index.
    pub fn at(&self, idx: usize) -> &XY {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        &self.data[idx - self.lower]
    }

    /// Gets a mutable reference to the element at the given index.
    pub fn at_mut(&mut self, idx: usize) -> &mut XY {
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
    fn test_xy_creation() {
        let xy = XY::new(3.5, 7.2);
        assert_eq!(xy.x, 3.5);
        assert_eq!(xy.y, 7.2);
    }

    #[test]
    fn test_array_creation_with_bounds() {
        let arr = TColgpArray1OfXY::new(1, 4);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 4);
        assert_eq!(arr.len(), 4);
    }

    #[test]
    fn test_array_set_and_get() {
        let mut arr = TColgpArray1OfXY::new(1, 3);
        let xy = XY::new(2.5, 4.5);
        arr.set(2, xy);

        assert_eq!(arr.get(2), xy);
    }

    #[test]
    fn test_array_at_mutable() {
        let mut arr = TColgpArray1OfXY::new(0, 2);
        arr.at_mut(1).x = 15.0;
        arr.at_mut(1).y = 25.0;

        let retrieved = arr.get(1);
        assert_eq!(retrieved.x, 15.0);
        assert_eq!(retrieved.y, 25.0);
    }

    #[test]
    fn test_array_default_initialization() {
        let arr = TColgpArray1OfXY::new(5, 8);
        for i in 5..=8 {
            let xy = arr.get(i);
            assert_eq!(xy.x, 0.0);
            assert_eq!(xy.y, 0.0);
        }
    }

    #[test]
    #[should_panic]
    fn test_array_get_out_of_bounds_low() {
        let arr = TColgpArray1OfXY::new(10, 15);
        let _ = arr.get(9);
    }

    #[test]
    #[should_panic]
    fn test_array_invalid_bounds() {
        let _ = TColgpArray1OfXY::new(5, 3);
    }
}
