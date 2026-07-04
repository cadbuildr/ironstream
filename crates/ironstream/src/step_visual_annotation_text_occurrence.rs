// FILE: step_visual_annotation_text_occurrence.rs
// occt: StepVisual_AnnotationTextOccurrence

/// Represents a StepVisual AnnotationTextOccurrence
#[derive(Debug, Clone, Default)]
pub struct StepVisual_AnnotationTextOccurrence {
    name: Option<String>,
}

impl StepVisual_AnnotationTextOccurrence {
    pub fn new() -> Self {
        StepVisual_AnnotationTextOccurrence { name: None }
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
        let ato = StepVisual_AnnotationTextOccurrence::new();
        assert!(ato.name().is_none());
    }
}
