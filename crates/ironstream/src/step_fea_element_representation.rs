// FILE: step_fea_element_representation.rs
// occt: StepFEA_ElementRepresentation

//! Representation of STEP entity ElementRepresentation in FEA.

use std::rc::Rc;

/// Representation context
#[derive(Debug, Clone)]
pub struct RepresentationContext {
    id: String,
}

impl RepresentationContext {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Representation item
#[derive(Debug, Clone)]
pub struct RepresentationItem {
    name: String,
}

impl RepresentationItem {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Node representation
#[derive(Debug, Clone)]
pub struct NodeRepresentation {
    id: String,
    node_type: String,
}

impl NodeRepresentation {
    pub fn new(id: String, node_type: String) -> Self {
        Self { id, node_type }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn node_type(&self) -> &str {
        &self.node_type
    }
}

/// Base Representation structure
#[derive(Debug, Clone)]
pub struct Representation {
    name: Option<String>,
    items: Vec<Rc<RepresentationItem>>,
    context: Option<Rc<RepresentationContext>>,
}

impl Representation {
    pub fn new() -> Self {
        Self {
            name: None,
            items: Vec::new(),
            context: None,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        items: Vec<Rc<RepresentationItem>>,
        context: Rc<RepresentationContext>,
    ) {
        self.name = Some(name);
        self.items = items;
        self.context = Some(context);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn items(&self) -> &[Rc<RepresentationItem>] {
        &self.items
    }

    pub fn set_items(&mut self, items: Vec<Rc<RepresentationItem>>) {
        self.items = items;
    }

    pub fn context(&self) -> Option<&Rc<RepresentationContext>> {
        self.context.as_ref()
    }

    pub fn set_context(&mut self, context: Rc<RepresentationContext>) {
        self.context = Some(context);
    }
}

impl Default for Representation {
    fn default() -> Self {
        Self::new()
    }
}

/// An ElementRepresentation is a Representation specific to FEA elements
#[derive(Debug, Clone)]
pub struct StepFeaElementRepresentation {
    representation: Representation,
    node_list: Vec<Rc<NodeRepresentation>>,
}

impl StepFeaElementRepresentation {
    /// Create a new ElementRepresentation
    pub fn new() -> Self {
        Self {
            representation: Representation::new(),
            node_list: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        items: Vec<Rc<RepresentationItem>>,
        context: Rc<RepresentationContext>,
        nodes: Vec<Rc<NodeRepresentation>>,
    ) {
        self.representation.init(name, items, context);
        self.node_list = nodes;
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.representation.name()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.representation.set_name(name);
    }

    /// Get the items
    pub fn items(&self) -> &[Rc<RepresentationItem>] {
        self.representation.items()
    }

    /// Set the items
    pub fn set_items(&mut self, items: Vec<Rc<RepresentationItem>>) {
        self.representation.set_items(items);
    }

    /// Get the context
    pub fn context(&self) -> Option<&Rc<RepresentationContext>> {
        self.representation.context()
    }

    /// Set the context
    pub fn set_context(&mut self, context: Rc<RepresentationContext>) {
        self.representation.set_context(context);
    }

    /// Get the node list
    pub fn node_list(&self) -> &[Rc<NodeRepresentation>] {
        &self.node_list
    }

    /// Set the node list
    pub fn set_node_list(&mut self, nodes: Vec<Rc<NodeRepresentation>>) {
        self.node_list = nodes;
    }

    /// Get number of nodes
    pub fn nb_nodes(&self) -> usize {
        self.node_list.len()
    }

    /// Get a specific node by index
    pub fn node_value(&self, index: usize) -> Option<&Rc<NodeRepresentation>> {
        self.node_list.get(index)
    }
}

impl Default for StepFeaElementRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let er = StepFeaElementRepresentation::new();
        assert_eq!(er.name(), None);
        assert_eq!(er.nb_nodes(), 0);
    }

    #[test]
    fn test_node_representation() {
        let node = NodeRepresentation::new("NODE_1".to_string(), "point".to_string());
        assert_eq!(node.id(), "NODE_1");
        assert_eq!(node.node_type(), "point");
    }

    #[test]
    fn test_representation_item() {
        let item = RepresentationItem::new("comp_1".to_string());
        assert_eq!(item.name(), "comp_1");
    }

    #[test]
    fn test_representation_context() {
        let ctx = RepresentationContext::new("CTX_1".to_string());
        assert_eq!(ctx.id(), "CTX_1");
    }

    #[test]
    fn test_init() {
        let mut er = StepFeaElementRepresentation::new();
        let context = Rc::new(RepresentationContext::new("CTX_X".to_string()));
        let node1 = Rc::new(NodeRepresentation::new("N1".to_string(), "pt".to_string()));
        let node2 = Rc::new(NodeRepresentation::new("N2".to_string(), "pt".to_string()));
        let item = Rc::new(RepresentationItem::new("i1".to_string()));
        er.init("element_1".to_string(), vec![item], context, vec![node1, node2]);
        assert_eq!(er.name(), Some("element_1"));
        assert_eq!(er.nb_nodes(), 2);
    }

    #[test]
    fn test_set_node_list() {
        let mut er = StepFeaElementRepresentation::new();
        let node = Rc::new(NodeRepresentation::new("N_X".to_string(), "pt".to_string()));
        er.set_node_list(vec![node]);
        assert_eq!(er.nb_nodes(), 1);
    }

    #[test]
    fn test_node_value() {
        let mut er = StepFeaElementRepresentation::new();
        let node = Rc::new(NodeRepresentation::new("N_Y".to_string(), "pt".to_string()));
        er.set_node_list(vec![node.clone()]);
        assert!(er.node_value(0).is_some());
        assert_eq!(er.node_value(0).unwrap().id(), "N_Y");
    }
}
