// FILE: step_visual_style_context_select.rs
// occt: StepVisual_StyleContextSelect

pub struct Representation;
pub struct RepresentationItem;
pub struct PresentationSet;

pub struct StyleContextSelect {
    case: Option<SelectCase>,
    value: Option<Box<dyn std::any::Any>>,
}

#[derive(Clone, Copy)]
enum SelectCase {
    Representation = 1,
    RepresentationItem = 2,
    PresentationSet = 3,
}

impl StyleContextSelect {
    pub fn new() -> Self {
        StyleContextSelect {
            case: None,
            value: None,
        }
    }

    pub fn case_num(&self) -> i32 {
        match self.case {
            Some(SelectCase::Representation) => 1,
            Some(SelectCase::RepresentationItem) => 2,
            Some(SelectCase::PresentationSet) => 3,
            None => 0,
        }
    }

    pub fn representation(&self) -> Option<&Representation> {
        if matches!(self.case, Some(SelectCase::Representation)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<Representation>())
        } else {
            None
        }
    }

    pub fn representation_item(&self) -> Option<&RepresentationItem> {
        if matches!(self.case, Some(SelectCase::RepresentationItem)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<RepresentationItem>())
        } else {
            None
        }
    }

    pub fn presentation_set(&self) -> Option<&PresentationSet> {
        if matches!(self.case, Some(SelectCase::PresentationSet)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<PresentationSet>())
        } else {
            None
        }
    }

    pub fn set_representation(&mut self, repr: Representation) {
        self.case = Some(SelectCase::Representation);
        self.value = Some(Box::new(repr));
    }

    pub fn set_representation_item(&mut self, item: RepresentationItem) {
        self.case = Some(SelectCase::RepresentationItem);
        self.value = Some(Box::new(item));
    }

    pub fn set_presentation_set(&mut self, set: PresentationSet) {
        self.case = Some(SelectCase::PresentationSet);
        self.value = Some(Box::new(set));
    }
}

impl Default for StyleContextSelect {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for StyleContextSelect {
    fn clone(&self) -> Self {
        StyleContextSelect {
            case: self.case,
            value: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let scs = StyleContextSelect::new();
        assert_eq!(scs.case_num(), 0);
        assert!(scs.representation().is_none());
        assert!(scs.representation_item().is_none());
        assert!(scs.presentation_set().is_none());
    }

    #[test]
    fn test_set_representation() {
        let mut scs = StyleContextSelect::new();
        scs.set_representation(Representation);
        assert_eq!(scs.case_num(), 1);
        assert!(scs.representation().is_some());
    }

    #[test]
    fn test_set_representation_item() {
        let mut scs = StyleContextSelect::new();
        scs.set_representation_item(RepresentationItem);
        assert_eq!(scs.case_num(), 2);
        assert!(scs.representation_item().is_some());
    }

    #[test]
    fn test_set_presentation_set() {
        let mut scs = StyleContextSelect::new();
        scs.set_presentation_set(PresentationSet);
        assert_eq!(scs.case_num(), 3);
        assert!(scs.presentation_set().is_some());
    }
}
