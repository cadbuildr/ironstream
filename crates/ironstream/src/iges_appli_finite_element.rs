// FILE: iges_appli_finite_element.rs
// occt: IGESAppli_FiniteElement

/// Represents a finite element entity in FEA.
///
/// IGES Type 136
/// Stores information about element topology and nodes.
#[derive(Clone, Debug)]
pub struct IgesAppliFiniteElement {
    element_type: i32,
    nb_nodes: i32,
    node_ids: Vec<i32>,
}

impl IgesAppliFiniteElement {
    /// Creates a new FiniteElement entity.
    pub fn new() -> Self {
        Self {
            element_type: 0,
            nb_nodes: 0,
            node_ids: Vec::new(),
        }
    }

    /// Initializes with element type and node identifiers.
    pub fn init(&mut self, elem_type: i32, nodes: Vec<i32>) {
        self.element_type = elem_type;
        self.nb_nodes = nodes.len() as i32;
        self.node_ids = nodes;
    }

    /// Returns the element type code.
    pub fn element_type(&self) -> i32 {
        self.element_type
    }

    /// Returns the number of nodes.
    pub fn nb_nodes(&self) -> i32 {
        self.nb_nodes
    }

    /// Returns the node IDs.
    pub fn node_ids(&self) -> &[i32] {
        &self.node_ids
    }
}

impl Default for IgesAppliFiniteElement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let elem = IgesAppliFiniteElement::new();
        assert_eq!(elem.element_type(), 0);
        assert_eq!(elem.nb_nodes(), 0);
        assert!(elem.node_ids().is_empty());
    }

    #[test]
    fn test_init() {
        let mut elem = IgesAppliFiniteElement::new();
        elem.init(1, vec![10, 11, 12, 13]);

        assert_eq!(elem.element_type(), 1);
        assert_eq!(elem.nb_nodes(), 4);
        assert_eq!(elem.node_ids(), &[10, 11, 12, 13]);
    }

    #[test]
    fn test_clone() {
        let mut elem1 = IgesAppliFiniteElement::new();
        elem1.init(2, vec![1, 2, 3]);

        let elem2 = elem1.clone();
        assert_eq!(elem2.element_type(), 2);
        assert_eq!(elem2.nb_nodes(), 3);
        assert_eq!(elem2.node_ids(), &[1, 2, 3]);
    }
}
