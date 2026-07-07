// FILE: ch_fi_ds_sec_array1.rs
// occt: ChFiDS_SecArray1

//! Deprecated type alias for backward compatibility.
//! Use Vec<ChFiDsCircSection> directly instead.

/// Circular section data for fillet operations.
#[derive(Clone, Debug, PartialEq)]
pub struct ChFiDsCircSection {
    /// Center X coordinate
    pub center_x: f64,
    /// Center Y coordinate
    pub center_y: f64,
    /// Center Z coordinate
    pub center_z: f64,
    /// Radius
    pub radius: f64,
    /// Parameter
    pub param: f64,
}

impl ChFiDsCircSection {
    /// Creates a new circular section.
    pub fn new(center_x: f64, center_y: f64, center_z: f64, radius: f64, param: f64) -> Self {
        ChFiDsCircSection {
            center_x,
            center_y,
            center_z,
            radius,
            param,
        }
    }

    /// Creates a zero-initialized circular section.
    pub fn default_section() -> Self {
        ChFiDsCircSection {
            center_x: 0.0,
            center_y: 0.0,
            center_z: 0.0,
            radius: 0.0,
            param: 0.0,
        }
    }
}

impl Default for ChFiDsCircSection {
    fn default() -> Self {
        Self::default_section()
    }
}

/// Circular section array indexed from 1 to N.
/// Deprecated alias for NCollection_Array1<ChFiDS_CircSection>.
/// Modeled as a vector with 1-based indexing via offset.
pub struct ChFiDsSecArray1 {
    items: Vec<ChFiDsCircSection>,
    lower: usize,
}

impl ChFiDsSecArray1 {
    /// Creates a new array with the given size, indexed from lower to upper (inclusive).
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = upper.saturating_sub(lower) + 1;
        ChFiDsSecArray1 {
            items: vec![ChFiDsCircSection::default(); size],
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
    pub fn get(&self, index: usize) -> Option<&ChFiDsCircSection> {
        if index < self.lower || index > self.upper() {
            return None;
        }
        let offset = index - self.lower;
        self.items.get(offset)
    }

    /// Gets a mutable reference to an element at the given index (1-based).
    pub fn get_mut(&mut self, index: usize) -> Option<&mut ChFiDsCircSection> {
        if index < self.lower || index > self.upper() {
            return None;
        }
        let offset = index - self.lower;
        self.items.get_mut(offset)
    }

    /// Sets an element at the given index (1-based).
    pub fn set(&mut self, index: usize, section: ChFiDsCircSection) -> bool {
        if let Some(item) = self.get_mut(index) {
            *item = section;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circ_section_creation() {
        let sec = ChFiDsCircSection::new(1.0, 2.0, 3.0, 5.0, 0.5);
        assert_eq!(sec.center_x, 1.0);
        assert_eq!(sec.center_y, 2.0);
        assert_eq!(sec.center_z, 3.0);
        assert_eq!(sec.radius, 5.0);
        assert_eq!(sec.param, 0.5);
    }

    #[test]
    fn test_sec_array1_creation() {
        let array = ChFiDsSecArray1::new(1, 10);
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 10);
        assert_eq!(array.len(), 10);
    }

    #[test]
    fn test_sec_array1_set_and_get() {
        let mut array = ChFiDsSecArray1::new(1, 5);
        let sec = ChFiDsCircSection::new(0.0, 0.0, 0.0, 1.0, 0.0);
        assert!(array.set(1, sec.clone()));

        let retrieved = array.get(1).unwrap();
        assert_eq!(retrieved.center_x, 0.0);
        assert_eq!(retrieved.radius, 1.0);
    }

    #[test]
    fn test_sec_array1_bounds_checking() {
        let mut array = ChFiDsSecArray1::new(1, 3);
        let sec = ChFiDsCircSection::new(0.0, 0.0, 0.0, 1.0, 0.0);
        assert!(array.set(1, sec.clone()));
        assert!(array.set(3, sec.clone()));
        assert!(!array.set(0, sec.clone())); // below lower
        assert!(!array.set(4, sec.clone())); // above upper
    }

    #[test]
    fn test_sec_array1_get_out_of_bounds() {
        let array = ChFiDsSecArray1::new(1, 5);
        assert!(array.get(1).is_some());
        assert!(array.get(0).is_none());
        assert!(array.get(6).is_none());
    }

    #[test]
    fn test_sec_array1_get_mut() {
        let mut array = ChFiDsSecArray1::new(1, 5);
        if let Some(sec) = array.get_mut(2) {
            sec.radius = 10.0;
        }

        let retrieved = array.get(2).unwrap();
        assert_eq!(retrieved.radius, 10.0);
    }
}
