// FILE: step_visual_annotation_occurrence.rs
// occt: StepVisual_AnnotationOccurrence

/// Represents a StepVisual AnnotationOccurrence
#[derive(Debug, Clone, Default)]
pub struct StepVisual_AnnotationOccurrence {
    name: Option<String>,
}

impl StepVisual_AnnotationOccurrence {
    pub fn new() -> Self {
        StepVisual_AnnotationOccurrence { name: None }
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
        let ao = StepVisual_AnnotationOccurrence::new();
        assert!(ao.name().is_none());
    }
}
