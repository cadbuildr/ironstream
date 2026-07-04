// FILE: if_select_select_intersection.rs
// occt: IFSelect_SelectIntersection

/// A selection filter that computes the intersection (AND) of results from multiple input selections.
/// Uniqueness is guaranteed in the output.
#[derive(Clone, Debug, Default)]
pub struct IFSelectSelectIntersection {
    // Would hold a Vec of input selections, but without the full selection hierarchy,
    // we keep this as a structural placeholder
    inputs: Vec<usize>, // indices of input selections (simplified model)
}

impl IFSelectSelectIntersection {
    /// Creates an empty SelectIntersection
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
        }
    }

    /// Returns the number of input selections
    pub fn nb_inputs(&self) -> usize {
        self.inputs.len()
    }

    /// Adds an input selection (simplified interface)
    pub fn add_input(&mut self, input_id: usize) {
        self.inputs.push(input_id);
    }

    /// Returns a text defining the criterium: "Intersection (AND)"
    pub fn label(&self) -> &'static str {
        "Intersection (AND)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let selector = IFSelectSelectIntersection::new();
        assert_eq!(selector.nb_inputs(), 0);
    }

    #[test]
    fn test_label() {
        let selector = IFSelectSelectIntersection::new();
        assert_eq!(selector.label(), "Intersection (AND)");
    }

    #[test]
    fn test_add_inputs() {
        let mut selector = IFSelectSelectIntersection::new();
        selector.add_input(1);
        selector.add_input(2);
        assert_eq!(selector.nb_inputs(), 2);
    }
}
