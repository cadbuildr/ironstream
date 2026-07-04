// FILE: step_visual_presented_item_representation.rs
// occt: StepVisual_PresentedItemRepresentation

use std::sync::Arc;

pub struct PresentationRepresentationSelect;
pub struct PresentedItemRef;

pub struct PresentedItemRepresentation {
    presentation: Option<PresentationRepresentationSelect>,
    item: Option<Arc<PresentedItemRef>>,
}

impl PresentedItemRepresentation {
    pub fn new() -> Self {
        PresentedItemRepresentation {
            presentation: None,
            item: None,
        }
    }

    pub fn init(
        &mut self,
        presentation: Option<PresentationRepresentationSelect>,
        item: Option<Arc<PresentedItemRef>>,
    ) {
        self.presentation = presentation;
        self.item = item;
    }

    pub fn set_presentation(&mut self, presentation: Option<PresentationRepresentationSelect>) {
        self.presentation = presentation;
    }

    pub fn presentation(&self) -> Option<&PresentationRepresentationSelect> {
        self.presentation.as_ref()
    }

    pub fn set_item(&mut self, item: Option<Arc<PresentedItemRef>>) {
        self.item = item;
    }

    pub fn item(&self) -> Option<&Arc<PresentedItemRef>> {
        self.item.as_ref()
    }
}

impl Default for PresentedItemRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pir = PresentedItemRepresentation::new();
        assert!(pir.presentation().is_none());
        assert!(pir.item().is_none());
    }

    #[test]
    fn test_set_and_get_presentation() {
        let mut pir = PresentedItemRepresentation::new();
        let presentation = PresentationRepresentationSelect;
        pir.set_presentation(Some(presentation));
        assert!(pir.presentation().is_some());
    }

    #[test]
    fn test_set_and_get_item() {
        let mut pir = PresentedItemRepresentation::new();
        let item = Arc::new(PresentedItemRef);
        pir.set_item(Some(item.clone()));
        assert!(pir.item().is_some());
    }

    #[test]
    fn test_init() {
        let mut pir = PresentedItemRepresentation::new();
        let presentation = PresentationRepresentationSelect;
        let item = Arc::new(PresentedItemRef);
        pir.init(Some(presentation), Some(item.clone()));

        assert!(pir.presentation().is_some());
        assert!(pir.item().is_some());
    }
}
