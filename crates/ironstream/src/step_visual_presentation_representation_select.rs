// FILE: step_visual_presentation_representation_select.rs
// occt: StepVisual_PresentationRepresentationSelect

pub struct PresentationRepresentation;
pub struct PresentationSet;

pub struct PresentationRepresentationSelect {
    case: Option<SelectCase>,
    value: Option<Box<dyn std::any::Any>>,
}

enum SelectCase {
    PresentationRepresentation = 1,
    PresentationSet = 2,
}

impl PresentationRepresentationSelect {
    pub fn new() -> Self {
        PresentationRepresentationSelect {
            case: None,
            value: None,
        }
    }

    pub fn case_num(&self) -> i32 {
        match &self.case {
            Some(SelectCase::PresentationRepresentation) => 1,
            Some(SelectCase::PresentationSet) => 2,
            None => 0,
        }
    }

    pub fn presentation_representation(&self) -> Option<&PresentationRepresentation> {
        if matches!(&self.case, Some(SelectCase::PresentationRepresentation)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<PresentationRepresentation>())
        } else {
            None
        }
    }

    pub fn presentation_set(&self) -> Option<&PresentationSet> {
        if matches!(&self.case, Some(SelectCase::PresentationSet)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<PresentationSet>())
        } else {
            None
        }
    }

    pub fn set_presentation_representation(&mut self, repr: PresentationRepresentation) {
        self.case = Some(SelectCase::PresentationRepresentation);
        self.value = Some(Box::new(repr));
    }

    pub fn set_presentation_set(&mut self, set: PresentationSet) {
        self.case = Some(SelectCase::PresentationSet);
        self.value = Some(Box::new(set));
    }
}

impl Default for PresentationRepresentationSelect {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let prs = PresentationRepresentationSelect::new();
        assert_eq!(prs.case_num(), 0);
        assert!(prs.presentation_representation().is_none());
        assert!(prs.presentation_set().is_none());
    }

    #[test]
    fn test_set_presentation_representation() {
        let mut prs = PresentationRepresentationSelect::new();
        prs.set_presentation_representation(PresentationRepresentation);
        assert_eq!(prs.case_num(), 1);
        assert!(prs.presentation_representation().is_some());
        assert!(prs.presentation_set().is_none());
    }

    #[test]
    fn test_set_presentation_set() {
        let mut prs = PresentationRepresentationSelect::new();
        prs.set_presentation_set(PresentationSet);
        assert_eq!(prs.case_num(), 2);
        assert!(prs.presentation_representation().is_none());
        assert!(prs.presentation_set().is_some());
    }

    #[test]
    fn test_switch_selection() {
        let mut prs = PresentationRepresentationSelect::new();
        prs.set_presentation_representation(PresentationRepresentation);
        assert_eq!(prs.case_num(), 1);
        prs.set_presentation_set(PresentationSet);
        assert_eq!(prs.case_num(), 2);
        assert!(prs.presentation_representation().is_none());
        assert!(prs.presentation_set().is_some());
    }
}
