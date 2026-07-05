// FILE: b_rep_adaptor_h_array1_of_curve.rs
// occt: BRepAdaptor_HArray1OfCurve

use std::sync::Arc;

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

/// 1D array of BRepAdaptor_Curve.
#[derive(Debug, Clone)]
struct Array1OfCurve {
    data: Vec<Curve>,
    lower: usize,              // OCCT uses 1-based indexing
}

impl Array1OfCurve {
    /// Creates an array with the given bounds (1-based indexing).
    fn new(lower: usize, upper: usize) -> Self {
        let size = if upper >= lower {
            upper - lower + 1
        } else {
            0
        };
        Array1OfCurve {
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
    fn lower(&self) -> usize {
        self.lower
    }

    /// Returns the upper bound.
    fn upper(&self) -> usize {
        self.lower + self.data.len().saturating_sub(1)
    }

    /// Returns the length of the array.
    fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the array is empty.
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Accesses an element by 1-based index.
    fn get(&self, index: usize) -> Option<&Curve> {
        if index < self.lower {
            return None;
        }
        self.data.get(index - self.lower)
    }

    /// Mutably accesses an element by 1-based index.
    fn get_mut(&mut self, index: usize) -> Option<&mut Curve> {
        if index < self.lower {
            return None;
        }
        self.data.get_mut(index - self.lower)
    }

    /// Sets an element by 1-based index.
    fn set(&mut self, index: usize, value: Curve) -> bool {
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
    fn iter(&self) -> impl Iterator<Item = &Curve> {
        self.data.iter()
    }
}

/// Deprecated type alias: Handle to Array1 of BRepAdaptor_Curve.
/// In OCCT, this is a reference-counted handle. We use Arc for the same semantic.
pub type BrepAdaptorHArray1OfCurve = Arc<Array1OfCurve>;

/// Constructor function for creating a handle to a new array.
pub fn new_h_array1_of_curve(lower: usize, upper: usize) -> BrepAdaptorHArray1OfCurve {
    Arc::new(Array1OfCurve::new(lower, upper))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let array = Array1OfCurve::new(1, 5);
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 5);
        assert_eq!(array.len(), 5);
    }

    #[test]
    fn test_handle_creation() {
        let handle = new_h_array1_of_curve(1, 3);
        assert_eq!(handle.lower(), 1);
        assert_eq!(handle.upper(), 3);
    }

    #[test]
    fn test_array_get_set() {
        let mut array = Array1OfCurve::new(1, 3);
        let curve = Curve::new(42, 0.0, 1.0);
        assert!(array.set(1, curve));
        assert_eq!(array.get(1).unwrap().curve_id, 42);
    }

    #[test]
    fn test_array_1_based_indexing() {
        let mut array = Array1OfCurve::new(1, 3);
        let curve = Curve::new(10, 0.5, 2.5);
        assert!(array.set(2, curve));
        assert_eq!(array.get(2).unwrap().first, 0.5);
    }

    #[test]
    fn test_array_out_of_bounds() {
        let array = Array1OfCurve::new(1, 3);
        assert!(array.get(0).is_none());
        assert!(array.get(5).is_none());
    }

    #[test]
    fn test_handle_ref_counting() {
        let handle1 = new_h_array1_of_curve(1, 2);
        let handle2 = handle1.clone();
        assert_eq!(Arc::strong_count(&handle1), 2);
        assert_eq!(Arc::strong_count(&handle2), 2);
    }

    #[test]
    fn test_handle_access() {
        let mut array = Array1OfCurve::new(1, 2);
        let curve = Curve::new(5, 1.0, 2.0);
        array.set(1, curve);
        let handle = Arc::new(array);
        assert_eq!(handle.get(1).unwrap().curve_id, 5);
    }

    #[test]
    fn test_array_multiple_elements() {
        let mut array = Array1OfCurve::new(1, 4);
        for i in 1..=4 {
            let curve = Curve::new(i * 10, i as f64, (i + 1) as f64);
            assert!(array.set(i, curve));
        }
        assert_eq!(array.get(1).unwrap().curve_id, 10);
        assert_eq!(array.get(4).unwrap().curve_id, 40);
    }

    #[test]
    fn test_array_iterator() {
        let mut array = Array1OfCurve::new(1, 3);
        for i in 1..=3 {
            let curve = Curve::new(i, 0.0, 1.0);
            let _ = array.set(i, curve);
        }
        let count = array.iter().count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_array_empty() {
        let array = Array1OfCurve::new(1, 0);
        assert!(array.is_empty());
        assert_eq!(array.len(), 0);
    }
}
