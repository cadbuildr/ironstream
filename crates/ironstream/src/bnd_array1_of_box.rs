// FILE: bnd_array1_of_box.rs
// occt: Bnd_Array1OfBox

//! Deprecated type alias for backward compatibility.
//! Use Vec<Bnd_Box> directly instead.

/// Bounding box array indexed from 1 to N.
/// Deprecated alias for NCollection_Array1<Bnd_Box>.
/// Modeled as a vector with 1-based indexing via offset.
pub struct BndArray1OfBox {
    items: Vec<BndBoxItem>,
    lower: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct BndBoxItem {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    z_min: f64,
    z_max: f64,
}

impl BndArray1OfBox {
    /// Creates a new array with the given size, indexed from lower to upper (inclusive).
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = upper.saturating_sub(lower) + 1;
        BndArray1OfBox {
            items: vec![BndBoxItem::default(); size],
            lower,
        }
    }

    /// Returns the lower index bound.
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Returns the upper index bound.
    pub fn upper(&self) -> usize {
        if self.items.is_empty() {
            self.lower
        } else {
            self.lower + self.items.len() - 1
        }
    }

    /// Returns the length of the array.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Gets a reference to an element at the given index (1-based).
    pub fn get(&self, index: usize) -> Option<(f64, f64, f64, f64, f64, f64)> {
        if index < self.lower || index > self.upper() {
            return None;
        }
        let offset = index - self.lower;
        self.items.get(offset).map(|item| {
            (item.x_min, item.x_max, item.y_min, item.y_max, item.z_min, item.z_max)
        })
    }

    /// Sets an element at the given index (1-based).
    pub fn set(&mut self, index: usize, x_min: f64, x_max: f64, y_min: f64, y_max: f64, z_min: f64, z_max: f64) -> bool {
        if index < self.lower || index > self.upper() {
            return false;
        }
        let offset = index - self.lower;
        if let Some(item) = self.items.get_mut(offset) {
            *item = BndBoxItem { x_min, x_max, y_min, y_max, z_min, z_max };
            true
        } else {
            false
        }
    }
}

impl Default for BndBoxItem {
    fn default() -> Self {
        BndBoxItem {
            x_min: f64::INFINITY,
            x_max: f64::NEG_INFINITY,
            y_min: f64::INFINITY,
            y_max: f64::NEG_INFINITY,
            z_min: f64::INFINITY,
            z_max: f64::NEG_INFINITY,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let array = BndArray1OfBox::new(1, 10);
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 10);
        assert_eq!(array.len(), 10);
    }

    #[test]
    fn test_array_set_and_get() {
        let mut array = BndArray1OfBox::new(1, 5);
        assert!(array.set(1, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0));

        let (x_min, x_max, y_min, y_max, z_min, z_max) = array.get(1).unwrap();
        assert_eq!(x_min, 0.0);
        assert_eq!(x_max, 1.0);
        assert_eq!(y_min, 0.0);
        assert_eq!(y_max, 1.0);
        assert_eq!(z_min, 0.0);
        assert_eq!(z_max, 1.0);
    }

    #[test]
    fn test_array_bounds_checking() {
        let mut array = BndArray1OfBox::new(1, 3);
        assert!(array.set(1, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0));
        assert!(array.set(3, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0));
        assert!(!array.set(0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0)); // below lower
        assert!(!array.set(4, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0)); // above upper
    }

    #[test]
    fn test_array_get_out_of_bounds() {
        let array = BndArray1OfBox::new(1, 5);
        assert!(array.get(1).is_some());
        assert!(array.get(0).is_none());
        assert!(array.get(6).is_none());
    }
}
