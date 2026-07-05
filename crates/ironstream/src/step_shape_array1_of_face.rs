// FILE: step_shape_array1_of_face.rs
// occt: StepShape_Array1OfFace

use std::vec::Vec;

pub struct StepShapeArray1OfFace {
    data: Vec<Option<String>>,
    lower: usize,
}

impl StepShapeArray1OfFace {
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper { return Self { data: Vec::new(), lower }; }
        Self { data: vec![None; upper - lower + 1], lower }
    }

    pub fn lower(&self) -> usize { self.lower }
    pub fn upper(&self) -> usize { if self.data.is_empty() { self.lower - 1 } else { self.lower + self.data.len() - 1 } }
    pub fn len(&self) -> usize { self.data.len() }

    pub fn value(&self, index: usize) -> Option<&Option<String>> {
        if index < self.lower || index > self.upper() { return None; }
        self.data.get(index - self.lower)
    }

    pub fn set_value(&mut self, index: usize, value: Option<String>) -> bool {
        if index < self.lower || index > self.upper() { return false; }
        if let Some(elem) = self.data.get_mut(index - self.lower) { *elem = value; true } else { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let arr = StepShapeArray1OfFace::new(1, 3);
        assert_eq!(arr.len(), 3);
    }
}
