// FILE: t_colgp_array1_of_vec2d.rs
// occt: TColgp_Array1OfVec2d

/// A 2D vector (gp_Vec2d in OCCT).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2d {
    /// X component
    pub x: f64,
    /// Y component
    pub y: f64,
}

impl Vec2d {
    /// Creates a 2D vector.
    pub fn new(x: f64, y: f64) -> Self {
        Vec2d { x, y }
    }

    /// Returns the magnitude (length) of the vector.
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

/// 1-based Array1 of 2D vectors (gp_Vec2d), mirroring NCollection_Array1 semantics.
#[derive(Debug, Clone)]
pub struct TColgpArray1OfVec2d {
    lower: usize,
    upper: usize,
    data: Vec<Vec2d>,
}

impl TColgpArray1OfVec2d {
    /// Creates an array with bounds [lower, upper].
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper {
            panic!("Lower bound {} exceeds upper bound {}", lower, upper);
        }
        let size = upper - lower + 1;
        TColgpArray1OfVec2d {
            lower,
            upper,
            data: vec![Vec2d { x: 0.0, y: 0.0 }; size],
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
    pub fn get(&self, idx: usize) -> Vec2d {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        self.data[idx - self.lower]
    }

    /// Sets the element at the given index (within bounds).
    pub fn set(&mut self, idx: usize, value: Vec2d) {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        self.data[idx - self.lower] = value;
    }

    /// Gets a reference to the element at the given index.
    pub fn at(&self, idx: usize) -> &Vec2d {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        &self.data[idx - self.lower]
    }

    /// Gets a mutable reference to the element at the given index.
    pub fn at_mut(&mut self, idx: usize) -> &mut Vec2d {
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
    fn test_vec2d_magnitude() {
        let v = Vec2d::new(3.0, 4.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_array_creation_with_bounds() {
        let arr = TColgpArray1OfVec2d::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn test_array_set_and_get() {
        let mut arr = TColgpArray1OfVec2d::new(1, 3);
        let vec = Vec2d::new(7.0, 9.0);
        arr.set(2, vec);

        assert_eq!(arr.get(2), vec);
    }

    #[test]
    fn test_array_at_mutable() {
        let mut arr = TColgpArray1OfVec2d::new(1, 2);
        arr.at_mut(1).x = 11.0;
        arr.at_mut(1).y = 13.0;

        let retrieved = arr.get(1);
        assert_eq!(retrieved.x, 11.0);
        assert_eq!(retrieved.y, 13.0);
    }

    #[test]
    fn test_array_default_initialization() {
        let arr = TColgpArray1OfVec2d::new(10, 12);
        for i in 10..=12 {
            let v = arr.get(i);
            assert_eq!(v.x, 0.0);
            assert_eq!(v.y, 0.0);
        }
    }

    #[test]
    #[should_panic]
    fn test_array_get_out_of_bounds() {
        let arr = TColgpArray1OfVec2d::new(5, 10);
        let _ = arr.get(11);
    }

    #[test]
    #[should_panic]
    fn test_array_invalid_bounds() {
        let _ = TColgpArray1OfVec2d::new(20, 10);
    }
}
