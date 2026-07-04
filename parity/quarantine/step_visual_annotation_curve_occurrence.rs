// FILE: step_visual_annotation_curve_occurrence.rs
// occt: StepVisual_AnnotationCurveOccurrence

/// Represents a StepVisual AnnotationCurveOccurrence
#[derive(Debug, Clone, Default)]
pub struct StepVisual_AnnotationCurveOccurrence {
    name: Option<String>,
}

impl StepVisual_AnnotationCurveOccurrence {
    pub fn new() -> Self {
        StepVisual_AnnotationCurveOccurrence { name: None }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let aco = StepVisual_AnnotationCurveOccurrence::new();
        assert!(aco.name().is_none());
    }

    #[test]
    fn test_set_name() {
        let mut aco = StepVisual_AnnotationCurveOccurrence::new();
        aco.set_name("test".to_string());
        assert_eq!(aco.name(), Some("test"));
    }
}
