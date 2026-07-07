// FILE: step_geom_h_array1_of_cartesian_point.rs
// occt: StepGeom_HArray1OfCartesianPoint

use std::vec::Vec;

/// Deprecated typedef alias for handle-based Array1<StepGeom_CartesianPoint>.
pub struct StepGeomHArray1OfCartesianPoint {
    data: Vec<Option<String>>,
    lower: usize,
}

impl StepGeomHArray1OfCartesianPoint {
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper {
            return Self { data: Vec::new(), lower };
        }
        let size = upper - lower + 1;
        Self { data: vec![None; size], lower }
    }

    pub fn lower(&self) -> usize { self.lower }
    pub fn upper(&self) -> usize { if self.data.is_empty() { self.lower - 1 } else { self.lower + self.data.len() - 1 } }
    pub fn len(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }

    pub fn value(&self, index: usize) -> Option<&Option<String>> {
        if index < self.lower || index > self.upper() { return None; }
        self.data.get(index - self.lower)
    }

    pub fn set_value(&mut self, index: usize, value: Option<String>) -> bool {
        if index < self.lower || index > self.upper() { return false; }
        if let Some(elem) = self.data.get_mut(index - self.lower) {
            *elem = value; true
        } else { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let arr = StepGeomHArray1OfCartesianPoint::new(1, 5);
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn test_indexing() {
        let mut arr = StepGeomHArray1OfCartesianPoint::new(1, 3);
        arr.set_value(1, Some("p1".to_string()));
        assert_eq!(arr.value(1), Some(&Some("p1".to_string())));
    }
}
