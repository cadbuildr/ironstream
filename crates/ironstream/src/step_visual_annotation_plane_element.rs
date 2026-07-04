// FILE: step_visual_annotation_plane_element.rs
// occt: StepVisual_AnnotationPlaneElement

/// Represents a StepVisual AnnotationPlaneElement
#[derive(Debug, Clone, Default)]
pub struct StepVisual_AnnotationPlaneElement {
    name: Option<String>,
}

impl StepVisual_AnnotationPlaneElement {
    pub fn new() -> Self {
        StepVisual_AnnotationPlaneElement { name: None }
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
        let ape = StepVisual_AnnotationPlaneElement::new();
        assert!(ape.name().is_none());
    }
}
