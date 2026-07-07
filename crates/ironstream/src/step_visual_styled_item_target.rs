// FILE: step_visual_styled_item_target.rs
// occt: StepVisual_StyledItemTarget

pub struct GeometricRepresentationItem;
pub struct MappedItem;
pub struct Representation;
pub struct TopologicalRepresentationItem;

pub struct StyledItemTarget {
    case: Option<SelectCase>,
    value: Option<Box<dyn std::any::Any>>,
}

#[derive(Clone, Copy)]
enum SelectCase {
    GeometricRepresentationItem = 1,
    MappedItem = 2,
    Representation = 3,
    TopologicalRepresentationItem = 4,
}

impl StyledItemTarget {
    pub fn new() -> Self {
        StyledItemTarget {
            case: None,
            value: None,
        }
    }

    pub fn case_num(&self) -> i32 {
        match self.case {
            Some(SelectCase::GeometricRepresentationItem) => 1,
            Some(SelectCase::MappedItem) => 2,
            Some(SelectCase::Representation) => 3,
            Some(SelectCase::TopologicalRepresentationItem) => 4,
            None => 0,
        }
    }

    pub fn geometric_representation_item(&self) -> Option<&GeometricRepresentationItem> {
        if matches!(self.case, Some(SelectCase::GeometricRepresentationItem)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<GeometricRepresentationItem>())
        } else {
            None
        }
    }

    pub fn mapped_item(&self) -> Option<&MappedItem> {
        if matches!(self.case, Some(SelectCase::MappedItem)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<MappedItem>())
        } else {
            None
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

    pub fn topological_representation_item(&self) -> Option<&TopologicalRepresentationItem> {
        if matches!(self.case, Some(SelectCase::TopologicalRepresentationItem)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<TopologicalRepresentationItem>())
        } else {
            None
        }
    }

    pub fn set_geometric_representation_item(&mut self, item: GeometricRepresentationItem) {
        self.case = Some(SelectCase::GeometricRepresentationItem);
        self.value = Some(Box::new(item));
    }

    pub fn set_mapped_item(&mut self, item: MappedItem) {
        self.case = Some(SelectCase::MappedItem);
        self.value = Some(Box::new(item));
    }

    pub fn set_representation(&mut self, repr: Representation) {
        self.case = Some(SelectCase::Representation);
        self.value = Some(Box::new(repr));
    }

    pub fn set_topological_representation_item(&mut self, item: TopologicalRepresentationItem) {
        self.case = Some(SelectCase::TopologicalRepresentationItem);
        self.value = Some(Box::new(item));
    }
}

impl Default for StyledItemTarget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sit = StyledItemTarget::new();
        assert_eq!(sit.case_num(), 0);
        assert!(sit.geometric_representation_item().is_none());
        assert!(sit.mapped_item().is_none());
        assert!(sit.representation().is_none());
        assert!(sit.topological_representation_item().is_none());
    }

    #[test]
    fn test_set_geometric_representation_item() {
        let mut sit = StyledItemTarget::new();
        sit.set_geometric_representation_item(GeometricRepresentationItem);
        assert_eq!(sit.case_num(), 1);
        assert!(sit.geometric_representation_item().is_some());
    }

    #[test]
    fn test_set_mapped_item() {
        let mut sit = StyledItemTarget::new();
        sit.set_mapped_item(MappedItem);
        assert_eq!(sit.case_num(), 2);
        assert!(sit.mapped_item().is_some());
    }

    #[test]
    fn test_set_representation() {
        let mut sit = StyledItemTarget::new();
        sit.set_representation(Representation);
        assert_eq!(sit.case_num(), 3);
        assert!(sit.representation().is_some());
    }

    #[test]
    fn test_set_topological_representation_item() {
        let mut sit = StyledItemTarget::new();
        sit.set_topological_representation_item(TopologicalRepresentationItem);
        assert_eq!(sit.case_num(), 4);
        assert!(sit.topological_representation_item().is_some());
    }
}
