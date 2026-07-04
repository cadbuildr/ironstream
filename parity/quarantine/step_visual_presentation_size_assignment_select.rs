// FILE: step_visual_presentation_size_assignment_select.rs
// occt: StepVisual_PresentationSizeAssignmentSelect

pub struct PresentationView;
pub struct PresentationArea;
pub struct AreaInSet;

pub struct PresentationSizeAssignmentSelect {
    case: Option<SelectCase>,
    value: Option<Box<dyn std::any::Any>>,
}

enum SelectCase {
    PresentationView = 1,
    PresentationArea = 2,
    AreaInSet = 3,
}

impl PresentationSizeAssignmentSelect {
    pub fn new() -> Self {
        PresentationSizeAssignmentSelect {
            case: None,
            value: None,
        }
    }

    pub fn case_num(&self) -> i32 {
        match &self.case {
            Some(SelectCase::PresentationView) => 1,
            Some(SelectCase::PresentationArea) => 2,
            Some(SelectCase::AreaInSet) => 3,
            None => 0,
        }
    }

    pub fn presentation_view(&self) -> Option<&PresentationView> {
        if matches!(&self.case, Some(SelectCase::PresentationView)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<PresentationView>())
        } else {
            None
        }
    }

    pub fn presentation_area(&self) -> Option<&PresentationArea> {
        if matches!(&self.case, Some(SelectCase::PresentationArea)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<PresentationArea>())
        } else {
            None
        }
    }

    pub fn area_in_set(&self) -> Option<&AreaInSet> {
        if matches!(&self.case, Some(SelectCase::AreaInSet)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<AreaInSet>())
        } else {
            None
        }
    }

    pub fn set_presentation_view(&mut self, view: PresentationView) {
        self.case = Some(SelectCase::PresentationView);
        self.value = Some(Box::new(view));
    }

    pub fn set_presentation_area(&mut self, area: PresentationArea) {
        self.case = Some(SelectCase::PresentationArea);
        self.value = Some(Box::new(area));
    }

    pub fn set_area_in_set(&mut self, area: AreaInSet) {
        self.case = Some(SelectCase::AreaInSet);
        self.value = Some(Box::new(area));
    }
}

impl Default for PresentationSizeAssignmentSelect {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for PresentationSizeAssignmentSelect {
    fn clone(&self) -> Self {
        PresentationSizeAssignmentSelect {
            case: self.case.clone(),
            value: None, // Cannot clone the boxed value without knowing its type
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let psas = PresentationSizeAssignmentSelect::new();
        assert_eq!(psas.case_num(), 0);
        assert!(psas.presentation_view().is_none());
        assert!(psas.presentation_area().is_none());
        assert!(psas.area_in_set().is_none());
    }

    #[test]
    fn test_set_presentation_view() {
        let mut psas = PresentationSizeAssignmentSelect::new();
        psas.set_presentation_view(PresentationView);
        assert_eq!(psas.case_num(), 1);
        assert!(psas.presentation_view().is_some());
        assert!(psas.presentation_area().is_none());
        assert!(psas.area_in_set().is_none());
    }

    #[test]
    fn test_set_presentation_area() {
        let mut psas = PresentationSizeAssignmentSelect::new();
        psas.set_presentation_area(PresentationArea);
        assert_eq!(psas.case_num(), 2);
        assert!(psas.presentation_view().is_none());
        assert!(psas.presentation_area().is_some());
        assert!(psas.area_in_set().is_none());
    }

    #[test]
    fn test_set_area_in_set() {
        let mut psas = PresentationSizeAssignmentSelect::new();
        psas.set_area_in_set(AreaInSet);
        assert_eq!(psas.case_num(), 3);
        assert!(psas.presentation_view().is_none());
        assert!(psas.presentation_area().is_none());
        assert!(psas.area_in_set().is_some());
    }

    #[test]
    fn test_switch_selection() {
        let mut psas = PresentationSizeAssignmentSelect::new();
        psas.set_presentation_view(PresentationView);
        assert_eq!(psas.case_num(), 1);
        psas.set_presentation_area(PresentationArea);
        assert_eq!(psas.case_num(), 2);
    }
}
