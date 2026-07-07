// FILE: b_rep_adaptor_array1_of_curve.rs
// occt: BRepAdaptor_Array1OfCurve

/// Simple curve adaptor for B-rep curves.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Curve {
    curve_id: usize,           // Curve identifier
    first: f64,                // Start parameter
    last: f64,                 // End parameter
}

impl Curve {
    fn new(curve_id: usize, first: f64, last: f64) -> Self {
        Curve {
            curve_id,
            first,
            last,
        }
    }
}

/// Deprecated type alias: Array1 of BRepAdaptor_Curve.
/// This is a vector-based 1D array with 1-based indexing simulation.
pub struct BrepAdaptorArray1OfCurve {
    data: Vec<Curve>,
    lower: usize,              // OCCT uses 1-based indexing
}

impl BrepAdaptorArray1OfCurve {
    /// Creates an array with the given bounds (1-based indexing).
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower {
            upper - lower + 1
        } else {
            0
        };
        BrepAdaptorArray1OfCurve {
            data: vec![
                Curve {
                    curve_id: 0,
                    first: 0.0,
                    last: 0.0
                };
                size
            ],
            lower,
        }
    }

    /// Returns the lower bound.
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Returns the upper bound.
    pub fn upper(&self) -> usize {
        self.lower + self.data.len().saturating_sub(1)
    }

    /// Returns the length of the array.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Accesses an element by 1-based index.
    pub fn get(&self, index: usize) -> Option<&Curve> {
        if index < self.lower {
            return None;
        }
        self.data.get(index - self.lower)
    }

    /// Mutably accesses an element by 1-based index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Curve> {
        if index < self.lower {
            return None;
        }
        self.data.get_mut(index - self.lower)
    }

    /// Sets an element by 1-based index.
    pub fn set(&mut self, index: usize, value: Curve) -> bool {
        if index < self.lower {
            return false;
        }
        if let Some(elem) = self.data.get_mut(index - self.lower) {
            *elem = value;
            return true;
        }
        false
    }

    /// Returns an iterator.
    pub fn iter(&self) -> impl Iterator<Item = &Curve> {
        self.data.iter()
    }

    /// Returns a mutable iterator.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Curve> {
        self.data.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let array = BrepAdaptorArray1OfCurve::new(1, 5);
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 5);
        assert_eq!(array.len(), 5);
    }

    #[test]
    fn test_array_get_set() {
        let mut array = BrepAdaptorArray1OfCurve::new(1, 3);
        let curve = Curve::new(42, 0.0, 1.0);
        assert!(array.set(1, curve));
        assert_eq!(array.get(1).unwrap().curve_id, 42);
    }

    #[test]
    fn test_array_1_based_indexing() {
        let mut array = BrepAdaptorArray1OfCurve::new(1, 3);
        let curve = Curve::new(10, 0.5, 2.5);
        assert!(array.set(2, curve));
        assert_eq!(array.get(2).unwrap().first, 0.5);
    }

    #[test]
    fn test_array_out_of_bounds() {
        let array = BrepAdaptorArray1OfCurve::new(1, 3);
        assert!(array.get(0).is_none());  // Below lower
        assert!(array.get(5).is_none());  // Above upper
    }

    #[test]
    fn test_array_multiple_elements() {
        let mut array = BrepAdaptorArray1OfCurve::new(1, 4);
        for i in 1..=4 {
            let curve = Curve::new(i * 10, i as f64, (i + 1) as f64);
            assert!(array.set(i, curve));
        }
        assert_eq!(array.get(1).unwrap().curve_id, 10);
        assert_eq!(array.get(4).unwrap().curve_id, 40);
    }

    #[test]
    fn test_array_empty() {
        let array = BrepAdaptorArray1OfCurve::new(1, 0);
        assert!(array.is_empty());
        assert_eq!(array.len(), 0);
    }

    #[test]
    fn test_array_iterator() {
        let mut array = BrepAdaptorArray1OfCurve::new(1, 3);
        for i in 1..=3 {
            let curve = Curve::new(i, 0.0, 1.0);
            let _ = array.set(i, curve);
        }
        let count = array.iter().count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_array_iter_mut() {
        let mut array = BrepAdaptorArray1OfCurve::new(1, 2);
        let c1 = Curve::new(1, 0.0, 1.0);
        let c2 = Curve::new(2, 1.0, 2.0);
        array.set(1, c1);
        array.set(2, c2);

        for curve in array.iter_mut() {
            curve.curve_id += 100;
        }
        assert_eq!(array.get(1).unwrap().curve_id, 101);
        assert_eq!(array.get(2).unwrap().curve_id, 102);
    }
}
