// FILE: step_visual_annotation_fill_area.rs
// occt: StepVisual_AnnotationFillArea

/// Represents a StepVisual AnnotationFillArea
#[derive(Debug, Clone, Default)]
pub struct StepVisual_AnnotationFillArea {
    name: Option<String>,
}

impl StepVisual_AnnotationFillArea {
    pub fn new() -> Self {
        StepVisual_AnnotationFillArea { name: None }
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
        let afa = StepVisual_AnnotationFillArea::new();
        assert!(afa.name().is_none());
    }
}
