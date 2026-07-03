// FILE: loc_ope_glued_shape.rs
// occt: LocOpe_GluedShape

/// Container for shapes after glueing operation.
pub struct LocOpeGluedShape {
    shape_id: usize,
}

impl LocOpeGluedShape {
    /// Creates a new glued shape container.
    pub fn new() -> Self {
        LocOpeGluedShape { shape_id: 0 }
    }

    /// Sets the glued shape.
    pub fn set_shape(&mut self, _shape_id: usize) {
        // Set shape
    }

    /// Returns the glued shape.
    pub fn shape(&self) -> usize {
        self.shape_id
    }
}

impl Default for LocOpeGluedShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glued_shape_creation() {
        let glued = LocOpeGluedShape::new();
        assert_eq!(glued.shape(), 0);
    }
}
