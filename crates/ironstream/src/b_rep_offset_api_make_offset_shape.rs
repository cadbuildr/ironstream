// FILE: b_rep_offset_api_make_offset_shape.rs
// occt: BRepOffsetAPI_MakeOffsetShape

/// Describes functions to build a shell out of a shape.
/// The result is an unlooped shape parallel to the source shape.
#[derive(Debug, Clone)]
pub struct BRepOffsetApiMakeOffsetShape {
    /// Whether the operation was successful
    is_done: bool,
    /// Offset distance
    offset: f64,
}

impl BRepOffsetApiMakeOffsetShape {
    /// Create a new MakeOffsetShape builder with offset distance
    pub fn new(offset: f64) -> Self {
        Self {
            is_done: false,
            offset,
        }
    }

    /// Check if the operation succeeded
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Mark operation as done
    pub fn set_done(&mut self) {
        self.is_done = true;
    }

    /// Get offset distance
    pub fn offset(&self) -> f64 {
        self.offset
    }
}

impl Default for BRepOffsetApiMakeOffsetShape {
    fn default() -> Self {
        Self::new(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let offset = BRepOffsetApiMakeOffsetShape::new(2.5);
        assert!(!offset.is_done());
        assert_eq!(offset.offset(), 2.5);
    }

    #[test]
    fn test_set_done() {
        let mut offset = BRepOffsetApiMakeOffsetShape::new(1.0);
        offset.set_done();
        assert!(offset.is_done());
    }

    #[test]
    fn test_default() {
        let offset = BRepOffsetApiMakeOffsetShape::default();
        assert_eq!(offset.offset(), 0.0);
    }
}
