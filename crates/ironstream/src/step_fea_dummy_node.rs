// FILE: step_fea_dummy_node.rs
// occt: StepFEA_DummyNode

//! Representation of STEP entity DummyNode in FEA (Finite Element Analysis).

use std::rc::Rc;

/// Placeholder for FEA model reference
#[derive(Debug, Clone)]
pub struct FeaModel {
    id: String,
}

impl FeaModel {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

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

/// Base Representation structure
#[derive(Debug, Clone)]
pub struct Representation {
    name: Option<String>,
    items: Vec<RepresentationItem>,
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
        items: Vec<RepresentationItem>,
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

    pub fn items(&self) -> &[RepresentationItem] {
        &self.items
    }

    pub fn set_items(&mut self, items: Vec<RepresentationItem>) {
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

/// A NodeRepresentation is a Representation specific to FEA nodes
#[derive(Debug, Clone)]
pub struct NodeRepresentation {
    representation: Representation,
    model_ref: Option<Rc<FeaModel>>,
}

impl NodeRepresentation {
    pub fn new() -> Self {
        Self {
            representation: Representation::new(),
            model_ref: None,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        items: Vec<RepresentationItem>,
        context: Rc<RepresentationContext>,
        model_ref: Rc<FeaModel>,
    ) {
        self.representation.init(name, items, context);
        self.model_ref = Some(model_ref);
    }

    pub fn name(&self) -> Option<&str> {
        self.representation.name()
    }

    pub fn set_name(&mut self, name: String) {
        self.representation.set_name(name);
    }

    pub fn model_ref(&self) -> Option<&Rc<FeaModel>> {
        self.model_ref.as_ref()
    }

    pub fn set_model_ref(&mut self, model: Rc<FeaModel>) {
        self.model_ref = Some(model);
    }
}

impl Default for NodeRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

/// A DummyNode is a NodeRepresentation that acts as a placeholder node
#[derive(Debug, Clone)]
pub struct StepFeaDummyNode {
    node_representation: NodeRepresentation,
}

impl StepFeaDummyNode {
    /// Create a new DummyNode
    pub fn new() -> Self {
        Self {
            node_representation: NodeRepresentation::new(),
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        items: Vec<RepresentationItem>,
        context: Rc<RepresentationContext>,
        model_ref: Rc<FeaModel>,
    ) {
        self.node_representation.init(name, items, context, model_ref);
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.node_representation.name()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.node_representation.set_name(name);
    }

    /// Get the model reference
    pub fn model_ref(&self) -> Option<&Rc<FeaModel>> {
        self.node_representation.model_ref()
    }

    /// Set the model reference
    pub fn set_model_ref(&mut self, model: Rc<FeaModel>) {
        self.node_representation.set_model_ref(model);
    }
}

impl Default for StepFeaDummyNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_model() {
        let model = FeaModel::new("FEA_MODEL_1".to_string());
        assert_eq!(model.id(), "FEA_MODEL_1");
    }

    #[test]
    fn test_new() {
        let node = StepFeaDummyNode::new();
        assert_eq!(node.name(), None);
        assert!(node.model_ref().is_none());
    }

    #[test]
    fn test_init() {
        let mut node = StepFeaDummyNode::new();
        let context = Rc::new(RepresentationContext::new("CTX_1".to_string()));
        let model = Rc::new(FeaModel::new("MODEL_1".to_string()));
        let items = vec![RepresentationItem::new("item1".to_string())];
        node.init("node_1".to_string(), items, context, model);
        assert_eq!(node.name(), Some("node_1"));
        assert!(node.model_ref().is_some());
    }

    #[test]
    fn test_set_model_ref() {
        let mut node = StepFeaDummyNode::new();
        let model = Rc::new(FeaModel::new("MODEL_X".to_string()));
        node.set_model_ref(model);
        assert!(node.model_ref().is_some());
    }

    #[test]
    fn test_representation_item() {
        let item = RepresentationItem::new("comp_1".to_string());
        assert_eq!(item.name(), "comp_1");
    }
}
