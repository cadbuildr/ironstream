// FILE: step_visual_draughting_callout_element.rs
// occt: StepVisual_DraughtingCalloutElement

/// A draughting callout element in STEP representation.
///
/// This represents an individual element within a draughting callout.
pub struct DraughtingCalloutElement {
    element_id: i32,
    description: String,
}

impl DraughtingCalloutElement {
    /// Creates a new draughting callout element.
    pub fn new(id: i32) -> Self {
        DraughtingCalloutElement {
            element_id: id,
            description: String::new(),
        }
    }

    /// Returns the element ID.
    pub fn element_id(&self) -> i32 {
        self.element_id
    }

    /// Sets the description.
    pub fn set_description(&mut self, desc: String) {
        self.description = desc;
    }

    /// Returns the description.
    pub fn description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draughting_callout_element_new() {
        let elem = DraughtingCalloutElement::new(42);
        assert_eq!(elem.element_id(), 42);
        assert_eq!(elem.description(), "");
    }

    #[test]
    fn test_set_description() {
        let mut elem = DraughtingCalloutElement::new(1);
        elem.set_description("Element Description".to_string());
        assert_eq!(elem.description(), "Element Description");
    }
}
