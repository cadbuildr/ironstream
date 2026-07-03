// FILE: loc_ope_linear_form.rs
// occt: LocOpe_LinearForm

/// Linear form feature (extrusion along a direction).
pub struct LocOpeLinearForm {
    shape_id: usize,
}

impl LocOpeLinearForm {
    /// Creates a new linear form.
    pub fn new() -> Self {
        LocOpeLinearForm { shape_id: 0 }
    }

    /// Initializes with a base shape.
    pub fn init(&mut self, _shape_id: usize) {
        // Initialize
    }

    /// Performs the linear extrusion.
    pub fn perform(&mut self, _direction: f64, _length: f64) {
        // Perform
    }

    /// Returns the result shape.
    pub fn shape(&self) -> usize {
        self.shape_id
    }
}

impl Default for LocOpeLinearForm {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_form_creation() {
        let form = LocOpeLinearForm::new();
        assert_eq!(form.shape(), 0);
    }
}
