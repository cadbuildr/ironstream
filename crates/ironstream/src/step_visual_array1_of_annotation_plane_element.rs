// FILE: step_visual_array1_of_annotation_plane_element.rs
// occt: StepVisual_Array1OfAnnotationPlaneElement

/// StepVisual_Array1OfAnnotationPlaneElement: a 1-based array of AnnotationPlaneElement.
///
/// This is a deprecated OCCT typedef for backward compatibility.
/// OCCT Array1 uses 1-based indexing (Lower/Upper bounds), which we model
/// with a backing Vec plus offset logic to preserve the indexing semantics.
#[derive(Debug, Clone)]
pub struct StepVisual_Array1OfAnnotationPlaneElement {
    lower: i32,
    upper: i32,
    data: Vec<u64>, // element IDs (placeholders for actual objects)
}

impl StepVisual_Array1OfAnnotationPlaneElement {
    /// Create a new 1-based array with bounds [lower, upper].
    pub fn new(lower: i32, upper: i32) -> Self {
        let size = (upper - lower + 1) as usize;
        Self {
            lower,
            upper,
            data: vec![0; size],
        }
    }

    /// Return the lower bound (usually 1).
    pub fn lower(&self) -> i32 {
        self.lower
    }

    /// Return the upper bound.
    pub fn upper(&self) -> i32 {
        self.upper
    }

    /// Return the length of the array.
    pub fn len(&self) -> i32 {
        self.upper - self.lower + 1
    }

    /// Check if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.len() <= 0
    }

    /// Access element at 1-based index (panics if out of bounds).
    pub fn at(&self, idx: i32) -> u64 {
        assert!(idx >= self.lower && idx <= self.upper, "Index out of bounds");
        let offset = (idx - self.lower) as usize;
        self.data[offset]
    }

    /// Set element at 1-based index (panics if out of bounds).
    pub fn set(&mut self, idx: i32, value: u64) {
        assert!(idx >= self.lower && idx <= self.upper, "Index out of bounds");
        let offset = (idx - self.lower) as usize;
        self.data[offset] = value;
    }

    /// Get a slice of the underlying data (0-based).
    pub fn as_slice(&self) -> &[u64] {
        &self.data
    }

    /// Get a mutable slice of the underlying data (0-based).
    pub fn as_mut_slice(&mut self) -> &mut [u64] {
        &mut self.data
    }

    /// Iterate over (1-based index, value) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (i32, u64)> + '_ {
        self.data.iter().enumerate().map(move |(i, &v)| {
            (self.lower + i as i32, v)
        })
    }

    /// Initialize all elements to a given value.
    pub fn fill(&mut self, value: u64) {
        for elem in &mut self.data {
            *elem = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array1_new_and_bounds() {
        let arr = StepVisual_Array1OfAnnotationPlaneElement::new(1, 10);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 10);
        assert_eq!(arr.len(), 10);
        assert!(!arr.is_empty());
    }

    #[test]
    fn test_array1_custom_bounds() {
        let arr = StepVisual_Array1OfAnnotationPlaneElement::new(5, 15);
        assert_eq!(arr.lower(), 5);
        assert_eq!(arr.upper(), 15);
        assert_eq!(arr.len(), 11);
    }

    #[test]
    fn test_array1_at_and_set() {
        let mut arr = StepVisual_Array1OfAnnotationPlaneElement::new(1, 5);

        arr.set(1, 100);
        arr.set(3, 300);
        arr.set(5, 500);

        assert_eq!(arr.at(1), 100);
        assert_eq!(arr.at(3), 300);
        assert_eq!(arr.at(5), 500);

        // Unset indices remain 0
        assert_eq!(arr.at(2), 0);
        assert_eq!(arr.at(4), 0);
    }

    #[test]
    #[should_panic]
    fn test_array1_out_of_bounds_low() {
        let arr = StepVisual_Array1OfAnnotationPlaneElement::new(1, 10);
        let _ = arr.at(0); // Below lower bound
    }

    #[test]
    #[should_panic]
    fn test_array1_out_of_bounds_high() {
        let arr = StepVisual_Array1OfAnnotationPlaneElement::new(1, 10);
        let _ = arr.at(11); // Above upper bound
    }

    #[test]
    fn test_array1_fill() {
        let mut arr = StepVisual_Array1OfAnnotationPlaneElement::new(1, 5);
        arr.fill(42);

        for i in 1..=5 {
            assert_eq!(arr.at(i), 42);
        }
    }

    #[test]
    fn test_array1_iterator() {
        let mut arr = StepVisual_Array1OfAnnotationPlaneElement::new(1, 3);
        arr.set(1, 10);
        arr.set(2, 20);
        arr.set(3, 30);

        let collected: Vec<_> = arr.iter().collect();
        assert_eq!(collected.len(), 3);
        assert_eq!(collected[0], (1, 10));
        assert_eq!(collected[1], (2, 20));
        assert_eq!(collected[2], (3, 30));
    }

    #[test]
    fn test_array1_empty() {
        let arr = StepVisual_Array1OfAnnotationPlaneElement::new(1, 0);
        assert!(arr.is_empty());
        assert_eq!(arr.len(), 0);
    }
}
