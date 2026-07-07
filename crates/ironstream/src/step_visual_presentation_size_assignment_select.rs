// FILE: step_visual_presentation_size_assignment_select.rs
// occt: StepVisual_PresentationSizeAssignmentSelect

#[derive(Clone)]
pub struct PresentationView;
#[derive(Clone)]
pub struct PresentationArea;
#[derive(Clone)]
pub struct AreaInSet;

/// Select type over PresentationView / PresentationArea / AreaInSet
/// (OCCT CaseNum: 1, 2, 3; 0 when unset).
#[derive(Clone)]
pub struct PresentationSizeAssignmentSelect {
    case: Option<SelectCase>,
}

#[derive(Clone)]
enum SelectCase {
    PresentationView(PresentationView),
    PresentationArea(PresentationArea),
    AreaInSet(AreaInSet),
}

impl PresentationSizeAssignmentSelect {
    pub fn new() -> Self {
        PresentationSizeAssignmentSelect { case: None }
    }

    pub fn case_num(&self) -> i32 {
        match &self.case {
            Some(SelectCase::PresentationView(_)) => 1,
            Some(SelectCase::PresentationArea(_)) => 2,
            Some(SelectCase::AreaInSet(_)) => 3,
            None => 0,
        }
    }

    pub fn presentation_view(&self) -> Option<&PresentationView> {
        match &self.case {
            Some(SelectCase::PresentationView(v)) => Some(v),
            _ => None,
        }
    }

    pub fn presentation_area(&self) -> Option<&PresentationArea> {
        match &self.case {
            Some(SelectCase::PresentationArea(a)) => Some(a),
            _ => None,
        }
    }

    pub fn area_in_set(&self) -> Option<&AreaInSet> {
        match &self.case {
            Some(SelectCase::AreaInSet(a)) => Some(a),
            _ => None,
        }
    }

    pub fn set_presentation_view(&mut self, view: PresentationView) {
        self.case = Some(SelectCase::PresentationView(view));
    }

    pub fn set_presentation_area(&mut self, area: PresentationArea) {
        self.case = Some(SelectCase::PresentationArea(area));
    }

    pub fn set_area_in_set(&mut self, area: AreaInSet) {
        self.case = Some(SelectCase::AreaInSet(area));
    }
}

impl Default for PresentationSizeAssignmentSelect {
    fn default() -> Self {
        Self::new()
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
