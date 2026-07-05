// FILE: t_colgp_array1_of_circ2d.rs
// occt: TColgp_Array1OfCirc2d

/// A simple 2D circle representation (gp_Circ2d in OCCT).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circ2d {
    /// Center X
    pub cx: f64,
    /// Center Y
    pub cy: f64,
    /// Radius
    pub radius: f64,
}

impl Circ2d {
    pub fn new(cx: f64, cy: f64, radius: f64) -> Self {
        Circ2d { cx, cy, radius }
    }
}

/// 1-based Array1 of 2D circles (gp_Circ2d), mirroring NCollection_Array1 semantics.
#[derive(Debug, Clone)]
pub struct TColgpArray1OfCirc2d {
    lower: usize,
    upper: usize,
    data: Vec<Circ2d>,
}

impl TColgpArray1OfCirc2d {
    /// Creates an array with bounds [lower, upper].
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper {
            panic!("Lower bound {} exceeds upper bound {}", lower, upper);
        }
        let size = upper - lower + 1;
        TColgpArray1OfCirc2d {
            lower,
            upper,
            data: vec![Circ2d { cx: 0.0, cy: 0.0, radius: 0.0 }; size],
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
    pub fn get(&self, idx: usize) -> Circ2d {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        self.data[idx - self.lower]
    }

    /// Sets the element at the given index (within bounds).
    pub fn set(&mut self, idx: usize, value: Circ2d) {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        self.data[idx - self.lower] = value;
    }

    /// Gets a reference to the element at the given index.
    pub fn at(&self, idx: usize) -> &Circ2d {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        &self.data[idx - self.lower]
    }

    /// Gets a mutable reference to the element at the given index.
    pub fn at_mut(&mut self, idx: usize) -> &mut Circ2d {
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
    fn test_array_creation_with_bounds() {
        let arr = TColgpArray1OfCirc2d::new(1, 3);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 3);
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn test_array_set_and_get() {
        let mut arr = TColgpArray1OfCirc2d::new(1, 3);
        let circ = Circ2d::new(5.0, 10.0, 2.5);
        arr.set(2, circ);

        assert_eq!(arr.get(2), circ);
    }

    #[test]
    fn test_array_at_mutable() {
        let mut arr = TColgpArray1OfCirc2d::new(1, 2);
        arr.at_mut(1).cx = 3.5;
        arr.at_mut(1).cy = 4.5;
        arr.at_mut(1).radius = 1.0;

        let c = arr.get(1);
        assert_eq!(c.cx, 3.5);
        assert_eq!(c.cy, 4.5);
        assert_eq!(c.radius, 1.0);
    }

    #[test]
    fn test_array_bounds_checking() {
        let arr = TColgpArray1OfCirc2d::new(10, 15);
        assert_eq!(arr.lower(), 10);
        assert_eq!(arr.upper(), 15);
        assert_eq!(arr.len(), 6);
    }

    #[test]
    #[should_panic]
    fn test_array_get_out_of_bounds_low() {
        let arr = TColgpArray1OfCirc2d::new(5, 10);
        let _ = arr.get(4);
    }

    #[test]
    #[should_panic]
    fn test_array_get_out_of_bounds_high() {
        let arr = TColgpArray1OfCirc2d::new(5, 10);
        let _ = arr.get(11);
    }

    #[test]
    #[should_panic]
    fn test_array_invalid_bounds() {
        let _ = TColgpArray1OfCirc2d::new(10, 5);
    }
}
