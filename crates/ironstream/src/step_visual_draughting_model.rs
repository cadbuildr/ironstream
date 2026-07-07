// FILE: step_visual_draughting_model.rs
// occt: StepVisual_DraughtingModel

/// A draughting model in STEP representation.
///
/// This represents a 2D draughting or technical drawing model.
pub struct DraughtingModel {
    name: String,
    scale: f64,
}

impl DraughtingModel {
    /// Creates a new draughting model.
    pub fn new(name: String) -> Self {
        DraughtingModel {
            name,
            scale: 1.0,
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the scale.
    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }

    /// Returns the scale.
    pub fn scale(&self) -> f64 {
        self.scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draughting_model_new() {
        let model = DraughtingModel::new("Drawing1".to_string());
        assert_eq!(model.name(), "Drawing1");
        assert_eq!(model.scale(), 1.0);
    }

    #[test]
    fn test_set_scale() {
        let mut model = DraughtingModel::new("Model".to_string());
        model.set_scale(0.5);
        assert_eq!(model.scale(), 0.5);
    }
}
