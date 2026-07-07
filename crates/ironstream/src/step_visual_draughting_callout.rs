// FILE: step_visual_draughting_callout.rs
// occt: StepVisual_DraughtingCallout

/// A draughting callout in STEP representation.
///
/// This represents a text callout or note in draughting.
pub struct DraughtingCallout {
    name: String,
    elements: Vec<String>,
}

impl DraughtingCallout {
    /// Creates a new draughting callout.
    pub fn new(name: String) -> Self {
        DraughtingCallout {
            name,
            elements: Vec::new(),
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Adds an element to the callout.
    pub fn add_element(&mut self, element: String) {
        self.elements.push(element);
    }

    /// Returns the elements.
    pub fn elements(&self) -> &[String] {
        &self.elements
    }

    /// Sets the elements.
    pub fn set_elements(&mut self, elements: Vec<String>) {
        self.elements = elements;
    }

    /// Returns the number of elements.
    pub fn nb_elements(&self) -> usize {
        self.elements.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draughting_callout_new() {
        let callout = DraughtingCallout::new("Callout1".to_string());
        assert_eq!(callout.name(), "Callout1");
        assert_eq!(callout.nb_elements(), 0);
    }

    #[test]
    fn test_add_element() {
        let mut callout = DraughtingCallout::new("MyCallout".to_string());
        callout.add_element("Text1".to_string());
        callout.add_element("Text2".to_string());
        assert_eq!(callout.nb_elements(), 2);
    }

    #[test]
    fn test_set_elements() {
        let mut callout = DraughtingCallout::new("Callout".to_string());
        let elements = vec!["E1".to_string(), "E2".to_string()];
        callout.set_elements(elements);
        assert_eq!(callout.nb_elements(), 2);
    }
}
