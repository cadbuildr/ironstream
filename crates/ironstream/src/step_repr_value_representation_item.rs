// FILE: step_repr_value_representation_item.rs
// occt: StepRepr_ValueRepresentationItem

/// Placeholder for MeasureValueMember
#[derive(Clone, Debug, PartialEq)]
pub struct MeasureValueMember {
    value: f64,
}

/// Represents a representation item with a measure value component (STEP).
pub struct ValueRepresentationItem {
    name: Option<String>,
    value_component_member: Option<MeasureValueMember>,
}

impl ValueRepresentationItem {
    /// Create a new ValueRepresentationItem
    pub fn new() -> Self {
        ValueRepresentationItem {
            name: None,
            value_component_member: None,
        }
    }

    /// Initialize with name and measure value member
    pub fn init(&mut self, name: String, value_component_member: MeasureValueMember) {
        self.name = Some(name);
        self.value_component_member = Some(value_component_member);
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Set the value component member
    pub fn set_value_component_member(&mut self, member: MeasureValueMember) {
        self.value_component_member = Some(member);
    }

    /// Get the value component member
    pub fn value_component_member(&self) -> Option<&MeasureValueMember> {
        self.value_component_member.as_ref()
    }
}

impl Default for ValueRepresentationItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let item = ValueRepresentationItem::new();
        assert_eq!(item.name(), None);
        assert_eq!(item.value_component_member(), None);
    }

    #[test]
    fn test_init() {
        let mut item = ValueRepresentationItem::new();
        let member = MeasureValueMember { value: 42.0 };
        item.init("ValueItem".to_string(), member.clone());
        assert_eq!(item.name(), Some("ValueItem"));
        assert_eq!(item.value_component_member(), Some(&member));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut item = ValueRepresentationItem::new();
        item.set_name("TestValue".to_string());
        assert_eq!(item.name(), Some("TestValue"));
    }

    #[test]
    fn test_set_and_get_value_member() {
        let mut item = ValueRepresentationItem::new();
        let member = MeasureValueMember { value: 3.14 };
        item.set_value_component_member(member.clone());
        assert_eq!(item.value_component_member(), Some(&member));
    }
}
