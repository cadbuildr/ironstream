// FILE: step_visual_tessellated_shape_representation_with_accuracy_parameters.rs
// occt: StepVisual_TessellatedShapeRepresentationWithAccuracyParameters

pub struct TessellatedShapeRepresentationWithAccuracyParameters {
    accuracy: f64,
}

impl TessellatedShapeRepresentationWithAccuracyParameters {
    pub fn new() -> Self {
        TessellatedShapeRepresentationWithAccuracyParameters {
            accuracy: 0.0,
        }
    }

    pub fn accuracy(&self) -> f64 {
        self.accuracy
    }

    pub fn set_accuracy(&mut self, value: f64) {
        self.accuracy = value;
    }
}

impl Default for TessellatedShapeRepresentationWithAccuracyParameters {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tsrwap = TessellatedShapeRepresentationWithAccuracyParameters::new();
        assert_eq!(tsrwap.accuracy(), 0.0);
    }

    #[test]
    fn test_set_and_get_accuracy() {
        let mut tsrwap = TessellatedShapeRepresentationWithAccuracyParameters::new();
        tsrwap.set_accuracy(0.001);
        assert_eq!(tsrwap.accuracy(), 0.001);
    }

    #[test]
    fn test_default() {
        let tsrwap = TessellatedShapeRepresentationWithAccuracyParameters::default();
        assert_eq!(tsrwap.accuracy(), 0.0);
    }
}
