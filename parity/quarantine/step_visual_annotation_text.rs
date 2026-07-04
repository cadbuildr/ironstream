// FILE: step_visual_annotation_text.rs
// occt: StepVisual_AnnotationText

/// Represents a StepVisual AnnotationText
#[derive(Debug, Clone, Default)]
pub struct StepVisual_AnnotationText {
    name: Option<String>,
}

impl StepVisual_AnnotationText {
    pub fn new() -> Self {
        StepVisual_AnnotationText { name: None }
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
        let at = StepVisual_AnnotationText::new();
        assert!(at.name().is_none());
    }
}
