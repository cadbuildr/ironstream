// FILE: step_visual_annotation_plane.rs
// occt: StepVisual_AnnotationPlane

/// Represents a StepVisual AnnotationPlane
#[derive(Debug, Clone, Default)]
pub struct StepVisual_AnnotationPlane {
    name: Option<String>,
}

impl StepVisual_AnnotationPlane {
    pub fn new() -> Self {
        StepVisual_AnnotationPlane { name: None }
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
        let ap = StepVisual_AnnotationPlane::new();
        assert!(ap.name().is_none());
    }
}
