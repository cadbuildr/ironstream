// FILE: step_visual_annotation_fill_area_occurrence.rs
// occt: StepVisual_AnnotationFillAreaOccurrence

/// Represents a StepVisual AnnotationFillAreaOccurrence
#[derive(Debug, Clone, Default)]
pub struct StepVisual_AnnotationFillAreaOccurrence {
    name: Option<String>,
}

impl StepVisual_AnnotationFillAreaOccurrence {
    pub fn new() -> Self {
        StepVisual_AnnotationFillAreaOccurrence { name: None }
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
        let afao = StepVisual_AnnotationFillAreaOccurrence::new();
        assert!(afao.name().is_none());
    }
}
