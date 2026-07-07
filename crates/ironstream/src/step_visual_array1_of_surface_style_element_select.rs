// FILE: step_visual_array1_of_surface_style_element_select.rs
// occt: StepVisual_Array1OfSurfaceStyleElementSelect

/// StepVisual_Array1OfSurfaceStyleElementSelect: a 1-based array.
///
/// This is a deprecated OCCT typedef for backward compatibility.
/// OCCT Array1 uses 1-based indexing (Lower/Upper bounds).
#[derive(Debug, Clone)]
pub struct StepVisual_Array1OfSurfaceStyleElementSelect {
    lower: i32,
    upper: i32,
    data: Vec<u64>,
}

impl StepVisual_Array1OfSurfaceStyleElementSelect {
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

    pub fn is_empty(&self) -> bool {
        self.len() <= 0
    }

    pub fn at(&self, idx: i32) -> u64 {
        assert!(idx >= self.lower && idx <= self.upper, "Index out of bounds");
        let offset = (idx - self.lower) as usize;
        self.data[offset]
    }

    pub fn set(&mut self, idx: i32, value: u64) {
        assert!(idx >= self.lower && idx <= self.upper, "Index out of bounds");
        let offset = (idx - self.lower) as usize;
        self.data[offset] = value;
    }

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
    fn test_array1_bounds() {
        let arr = StepVisual_Array1OfSurfaceStyleElementSelect::new(1, 10);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 10);
        assert_eq!(arr.len(), 10);
    }

    #[test]
    fn test_array1_set_and_get() {
        let mut arr = StepVisual_Array1OfSurfaceStyleElementSelect::new(1, 5);
        arr.set(1, 99);
        arr.set(5, 42);
        assert_eq!(arr.at(1), 99);
        assert_eq!(arr.at(5), 42);
    }

    #[test]
    fn test_array1_fill() {
        let mut arr = StepVisual_Array1OfSurfaceStyleElementSelect::new(1, 3);
        arr.fill(77);
        assert_eq!(arr.at(1), 77);
        assert_eq!(arr.at(2), 77);
        assert_eq!(arr.at(3), 77);
    }
}
