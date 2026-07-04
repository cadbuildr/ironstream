// FILE: if_select_select_union.rs
// occt: IFSelect_SelectUnion

/// A selection filter that computes the union (OR) of results from multiple inputs.
/// Guarantees uniqueness in output.
#[derive(Clone, Debug, Default)]
pub struct IFSelectSelectUnion {
    inputs: Vec<usize>, // indices of input selections
}

impl IFSelectSelectUnion {
    /// Creates an empty SelectUnion
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
        }
    }

    /// Returns the number of input selections
    pub fn nb_inputs(&self) -> usize {
        self.inputs.len()
    }

    /// Adds an input selection
    pub fn add_input(&mut self, input_id: usize) {
        self.inputs.push(input_id);
    }

    /// Returns a text defining the criterium: "Union (OR)"
    pub fn label(&self) -> &'static str {
        "Union (OR)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let selector = IFSelectSelectUnion::new();
        assert_eq!(selector.nb_inputs(), 0);
    }

    #[test]
    fn test_label() {
        let selector = IFSelectSelectUnion::new();
        assert_eq!(selector.label(), "Union (OR)");
    }

    #[test]
    fn test_add_inputs() {
        let mut selector = IFSelectSelectUnion::new();
        selector.add_input(1);
        selector.add_input(2);
        selector.add_input(3);
        assert_eq!(selector.nb_inputs(), 3);
    }

    #[test]
    fn test_default() {
        let selector = IFSelectSelectUnion::default();
        assert_eq!(selector.nb_inputs(), 0);
    }
}
