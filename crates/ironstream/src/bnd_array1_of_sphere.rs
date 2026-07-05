// FILE: bnd_array1_of_sphere.rs
// occt: Bnd_Array1OfSphere

//! Deprecated type alias for backward compatibility.
//! Use Vec<Bnd_Sphere> directly instead.

/// Bounding sphere array indexed from 1 to N.
/// Deprecated alias for NCollection_Array1<Bnd_Sphere>.
/// Modeled as a vector with 1-based indexing via offset.
pub struct BndArray1OfSphere {
    items: Vec<BndSphereItem>,
    lower: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct BndSphereItem {
    center_x: f64,
    center_y: f64,
    center_z: f64,
    radius: f64,
}

impl BndArray1OfSphere {
    /// Creates a new sphere array with the given size, indexed from lower to upper (inclusive).
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = upper.saturating_sub(lower) + 1;
        BndArray1OfSphere {
            items: vec![BndSphereItem::default(); size],
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
    pub fn get(&self, index: usize) -> Option<(f64, f64, f64, f64)> {
        if index < self.lower || index > self.upper() {
            return None;
        }
        let offset = index - self.lower;
        self.items.get(offset).map(|item| {
            (item.center_x, item.center_y, item.center_z, item.radius)
        })
    }

    /// Sets an element at the given index (1-based).
    pub fn set(&mut self, index: usize, center_x: f64, center_y: f64, center_z: f64, radius: f64) -> bool {
        if index < self.lower || index > self.upper() {
            return false;
        }
        let offset = index - self.lower;
        if let Some(item) = self.items.get_mut(offset) {
            *item = BndSphereItem { center_x, center_y, center_z, radius };
            true
        } else {
            false
        }
    }
}

impl Default for BndSphereItem {
    fn default() -> Self {
        BndSphereItem {
            center_x: 0.0,
            center_y: 0.0,
            center_z: 0.0,
            radius: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_array_creation() {
        let array = BndArray1OfSphere::new(1, 10);
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 10);
        assert_eq!(array.len(), 10);
    }

    #[test]
    fn test_sphere_array_set_and_get() {
        let mut array = BndArray1OfSphere::new(1, 5);
        assert!(array.set(1, 0.0, 0.0, 0.0, 5.0));

        let (cx, cy, cz, r) = array.get(1).unwrap();
        assert_eq!(cx, 0.0);
        assert_eq!(cy, 0.0);
        assert_eq!(cz, 0.0);
        assert_eq!(r, 5.0);
    }

    #[test]
    fn test_sphere_array_bounds_checking() {
        let mut array = BndArray1OfSphere::new(1, 3);
        assert!(array.set(1, 0.0, 0.0, 0.0, 1.0));
        assert!(array.set(3, 0.0, 0.0, 0.0, 1.0));
        assert!(!array.set(0, 0.0, 0.0, 0.0, 1.0)); // below lower
        assert!(!array.set(4, 0.0, 0.0, 0.0, 1.0)); // above upper
    }

    #[test]
    fn test_sphere_array_get_out_of_bounds() {
        let array = BndArray1OfSphere::new(1, 5);
        assert!(array.get(1).is_some());
        assert!(array.get(0).is_none());
        assert!(array.get(6).is_none());
    }
}
