// FILE: step_visual_presentation_size.rs
// occt: StepVisual_PresentationSize

use std::sync::Arc;

pub struct PlanarBox;
pub struct PresentationSizeAssignmentSelect;

pub struct PresentationSize {
    unit: Option<PresentationSizeAssignmentSelect>,
    size: Option<Arc<PlanarBox>>,
}

impl PresentationSize {
    pub fn new() -> Self {
        PresentationSize {
            unit: None,
            size: None,
        }
    }

    pub fn init(
        &mut self,
        unit: Option<PresentationSizeAssignmentSelect>,
        size: Option<Arc<PlanarBox>>,
    ) {
        self.unit = unit;
        self.size = size;
    }

    pub fn set_unit(&mut self, unit: Option<PresentationSizeAssignmentSelect>) {
        self.unit = unit;
    }

    pub fn unit(&self) -> Option<&PresentationSizeAssignmentSelect> {
        self.unit.as_ref()
    }

    pub fn set_size(&mut self, size: Option<Arc<PlanarBox>>) {
        self.size = size;
    }

    pub fn size(&self) -> Option<&Arc<PlanarBox>> {
        self.size.as_ref()
    }
}

impl Default for PresentationSize {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ps = PresentationSize::new();
        assert!(ps.unit().is_none());
        assert!(ps.size().is_none());
    }

    #[test]
    fn test_set_and_get_unit() {
        let mut ps = PresentationSize::new();
        let unit = PresentationSizeAssignmentSelect;
        ps.set_unit(Some(unit));
        assert!(ps.unit().is_some());
    }

    #[test]
    fn test_set_and_get_size() {
        let mut ps = PresentationSize::new();
        let size = Arc::new(PlanarBox);
        ps.set_size(Some(size.clone()));
        assert!(ps.size().is_some());
    }

    #[test]
    fn test_init() {
        let mut ps = PresentationSize::new();
        let unit = PresentationSizeAssignmentSelect;
        let size = Arc::new(PlanarBox);
        ps.init(Some(unit), Some(size.clone()));

        assert!(ps.unit().is_some());
        assert!(ps.size().is_some());
    }
}
