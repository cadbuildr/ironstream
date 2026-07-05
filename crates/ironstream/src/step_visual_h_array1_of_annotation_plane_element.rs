// FILE: step_visual_h_array1_of_annotation_plane_element.rs
// occt: StepVisual_HArray1OfAnnotationPlaneElement

use std::sync::Arc;

/// StepVisual_Array1OfAnnotationPlaneElement (backing array, same as the non-H variant).
#[derive(Debug, Clone)]
pub struct StepVisualArray1AnnotationPlaneElement {
    lower: i32,
    upper: i32,
    data: Vec<u64>,
}

impl StepVisualArray1AnnotationPlaneElement {
    pub fn new(lower: i32, upper: i32) -> Self {
        let size = (upper - lower + 1) as usize;
        Self {
            lower,
            upper,
            data: vec![0; size],
        }
    }

    pub fn lower(&self) -> i32 {
        self.lower
    }

    pub fn upper(&self) -> i32 {
        self.upper
    }

    pub fn len(&self) -> i32 {
        self.upper - self.lower + 1
    }

    pub fn at(&self, idx: i32) -> u64 {
        assert!(idx >= self.lower && idx <= self.upper, "Index out of bounds");
        self.data[(idx - self.lower) as usize]
    }

    pub fn set(&mut self, idx: i32, value: u64) {
        assert!(idx >= self.lower && idx <= self.upper, "Index out of bounds");
        self.data[(idx - self.lower) as usize] = value;
    }
}

/// StepVisual_HArray1OfAnnotationPlaneElement: a handle-based 1-based array.
///
/// This is a deprecated OCCT typedef for backward compatibility.
/// In Rust, we model this as an Arc-wrapped array for shared ownership.
pub type StepVisual_HArray1OfAnnotationPlaneElement =
    Arc<StepVisualArray1AnnotationPlaneElement>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harray1_creation() {
        let arr = Arc::new(StepVisualArray1AnnotationPlaneElement::new(1, 5));
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn test_harray1_shared_ownership() {
        let arr1 = Arc::new(StepVisualArray1AnnotationPlaneElement::new(1, 3));
        let arr2 = Arc::clone(&arr1);
        assert_eq!(Arc::strong_count(&arr1), 2);
        assert_eq!(arr1.at(1), arr2.at(1));
    }

    #[test]
    fn test_harray1_set_and_get() {
        let mut arr = StepVisualArray1AnnotationPlaneElement::new(1, 5);
        arr.set(2, 42);
        assert_eq!(arr.at(2), 42);
    }
}
