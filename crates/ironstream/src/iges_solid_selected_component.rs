// FILE: iges_solid_selected_component.rs
// occt: IGESSolid_SelectedComponent

/// SelectedComponent entity (Type 182, Form 0) in IGESSolid.
/// Provides a means of selecting one component of a disjoint CSG solid
/// by specifying a Boolean tree entity and a selection point.
#[derive(Debug, Clone)]
pub struct IGESSolidSelectedComponent {
    /// Reference to the Boolean tree entity
    component: Option<String>, // Handle to BooleanTree (simplified as String for Rust port)
    /// Point on/in the selected component
    select_point: [f64; 3],
}

impl IGESSolidSelectedComponent {
    /// Creates a new SelectedComponent with default values.
    pub fn new() -> Self {
        Self {
            component: None,
            select_point: [0.0, 0.0, 0.0],
        }
    }

    /// Initializes the fields of SelectedComponent.
    /// - entity: the Boolean tree entity
    /// - select_point: Point in or on the desired component
    pub fn init(&mut self, entity: Option<String>, select_point: [f64; 3]) {
        self.component = entity;
        self.select_point = select_point;
    }

    /// Returns the Boolean tree entity reference.
    pub fn component(&self) -> Option<&str> {
        self.component.as_deref()
    }

    /// Returns the point on/in the selected component.
    pub fn select_point(&self) -> [f64; 3] {
        self.select_point
    }

    /// Returns the point on/in the selected component after applying transformation.
    /// (In a full implementation, would apply the TransformationMatrix if present)
    pub fn transformed_select_point(&self) -> [f64; 3] {
        // For now, return the same point; a full implementation would apply
        // transformation if one exists
        self.select_point
    }

    /// Sets the Boolean tree component.
    pub fn set_component(&mut self, entity: Option<String>) {
        self.component = entity;
    }

    /// Sets the selection point.
    pub fn set_select_point(&mut self, point: [f64; 3]) {
        self.select_point = point;
    }
}

impl Default for IGESSolidSelectedComponent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_creation() {
        let comp = IGESSolidSelectedComponent::new();
        assert_eq!(comp.component(), None);
        assert_eq!(comp.select_point(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_init() {
        let mut comp = IGESSolidSelectedComponent::new();
        comp.init(Some("BooleanTree_1".to_string()), [1.0, 2.0, 3.0]);

        assert_eq!(comp.component(), Some("BooleanTree_1"));
        assert_eq!(comp.select_point(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_set_component() {
        let mut comp = IGESSolidSelectedComponent::new();
        comp.set_component(Some("Tree_A".to_string()));
        assert_eq!(comp.component(), Some("Tree_A"));
    }

    #[test]
    fn test_set_select_point() {
        let mut comp = IGESSolidSelectedComponent::new();
        comp.set_select_point([5.0, 6.0, 7.0]);
        assert_eq!(comp.select_point(), [5.0, 6.0, 7.0]);
    }

    #[test]
    fn test_transformed_select_point() {
        let mut comp = IGESSolidSelectedComponent::new();
        comp.set_select_point([1.5, 2.5, 3.5]);
        assert_eq!(comp.transformed_select_point(), [1.5, 2.5, 3.5]);
    }

    #[test]
    fn test_roundtrip() {
        let mut comp = IGESSolidSelectedComponent::new();
        comp.init(Some("BoolTree".to_string()), [10.0, 20.0, 30.0]);

        assert_eq!(comp.component(), Some("BoolTree"));
        assert_eq!(comp.select_point(), [10.0, 20.0, 30.0]);
    }
}
