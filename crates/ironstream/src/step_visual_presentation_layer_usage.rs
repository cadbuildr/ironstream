// FILE: step_visual_presentation_layer_usage.rs
// occt: StepVisual_PresentationLayerUsage

use std::sync::Arc;

pub struct PresentationLayerAssignmentRef;
pub struct PresentationRepresentationRef;

pub struct PresentationLayerUsage {
    assignment: Option<Arc<PresentationLayerAssignmentRef>>,
    presentation: Option<Arc<PresentationRepresentationRef>>,
}

impl PresentationLayerUsage {
    pub fn new() -> Self {
        PresentationLayerUsage {
            assignment: None,
            presentation: None,
        }
    }

    pub fn init(
        &mut self,
        assignment: Option<Arc<PresentationLayerAssignmentRef>>,
        presentation: Option<Arc<PresentationRepresentationRef>>,
    ) {
        self.assignment = assignment;
        self.presentation = presentation;
    }

    pub fn set_assignment(&mut self, assignment: Option<Arc<PresentationLayerAssignmentRef>>) {
        self.assignment = assignment;
    }

    pub fn assignment(&self) -> Option<&Arc<PresentationLayerAssignmentRef>> {
        self.assignment.as_ref()
    }

    pub fn set_presentation(&mut self, presentation: Option<Arc<PresentationRepresentationRef>>) {
        self.presentation = presentation;
    }

    pub fn presentation(&self) -> Option<&Arc<PresentationRepresentationRef>> {
        self.presentation.as_ref()
    }
}

impl Default for PresentationLayerUsage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let plu = PresentationLayerUsage::new();
        assert!(plu.assignment().is_none());
        assert!(plu.presentation().is_none());
    }

    #[test]
    fn test_set_and_get_assignment() {
        let mut plu = PresentationLayerUsage::new();
        let assignment = Arc::new(PresentationLayerAssignmentRef);
        plu.set_assignment(Some(assignment.clone()));
        assert!(plu.assignment().is_some());
    }

    #[test]
    fn test_set_and_get_presentation() {
        let mut plu = PresentationLayerUsage::new();
        let presentation = Arc::new(PresentationRepresentationRef);
        plu.set_presentation(Some(presentation.clone()));
        assert!(plu.presentation().is_some());
    }

    #[test]
    fn test_init() {
        let mut plu = PresentationLayerUsage::new();
        let assignment = Arc::new(PresentationLayerAssignmentRef);
        let presentation = Arc::new(PresentationRepresentationRef);
        plu.init(Some(assignment.clone()), Some(presentation.clone()));

        assert!(plu.assignment().is_some());
        assert!(plu.presentation().is_some());
    }
}
