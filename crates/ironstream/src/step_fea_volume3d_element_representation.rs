// FILE: step_fea_volume3d_element_representation.rs
// occt: StepFEA_Volume3dElementRepresentation

//! Representation of STEP entity Volume3dElementRepresentation in FEA.

use std::rc::Rc;

/// 3D FEA model
#[derive(Debug, Clone)]
pub struct FeaModel3d {
    id: String,
}

impl FeaModel3d {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Volume 3D element descriptor
#[derive(Debug, Clone)]
pub struct Volume3dElementDescriptor {
    element_type: String,
}

impl Volume3dElementDescriptor {
    pub fn new(element_type: String) -> Self {
        Self { element_type }
    }

    pub fn element_type(&self) -> &str {
        &self.element_type
    }
}

/// Element material properties
#[derive(Debug, Clone)]
pub struct ElementMaterial {
    material_id: String,
    material_name: String,
}

impl ElementMaterial {
    pub fn new(material_id: String, material_name: String) -> Self {
        Self {
            material_id,
            material_name,
        }
    }

    pub fn material_id(&self) -> &str {
        &self.material_id
    }

    pub fn material_name(&self) -> &str {
        &self.material_name
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

/// Node representation
#[derive(Debug, Clone)]
pub struct NodeRepresentation {
    id: String,
}

impl NodeRepresentation {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Base ElementRepresentation
#[derive(Debug, Clone)]
pub struct ElementRepresentation {
    name: Option<String>,
    items: Vec<Rc<RepresentationItem>>,
    context: Option<Rc<RepresentationContext>>,
    node_list: Vec<Rc<NodeRepresentation>>,
}

impl ElementRepresentation {
    pub fn new() -> Self {
        Self {
            name: None,
            items: Vec::new(),
            context: None,
            node_list: Vec::new(),
        }
    }

    pub fn init(
        &mut self,
        name: String,
        items: Vec<Rc<RepresentationItem>>,
        context: Rc<RepresentationContext>,
        nodes: Vec<Rc<NodeRepresentation>>,
    ) {
        self.name = Some(name);
        self.items = items;
        self.context = Some(context);
        self.node_list = nodes;
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn node_list(&self) -> &[Rc<NodeRepresentation>] {
        &self.node_list
    }
}

impl Default for ElementRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

/// Volume3dElementRepresentation extends ElementRepresentation with 3D-specific properties
#[derive(Debug, Clone)]
pub struct StepFeaVolume3dElementRepresentation {
    element_representation: ElementRepresentation,
    model_ref: Option<Rc<FeaModel3d>>,
    element_descriptor: Option<Rc<Volume3dElementDescriptor>>,
    material: Option<Rc<ElementMaterial>>,
}

impl StepFeaVolume3dElementRepresentation {
    /// Create a new Volume3dElementRepresentation
    pub fn new() -> Self {
        Self {
            element_representation: ElementRepresentation::new(),
            model_ref: None,
            element_descriptor: None,
            material: None,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        items: Vec<Rc<RepresentationItem>>,
        context: Rc<RepresentationContext>,
        nodes: Vec<Rc<NodeRepresentation>>,
        model_ref: Rc<FeaModel3d>,
        descriptor: Rc<Volume3dElementDescriptor>,
        material: Rc<ElementMaterial>,
    ) {
        self.element_representation.init(name, items, context, nodes);
        self.model_ref = Some(model_ref);
        self.element_descriptor = Some(descriptor);
        self.material = Some(material);
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.element_representation.name()
    }

    /// Get the model reference
    pub fn model_ref(&self) -> Option<&Rc<FeaModel3d>> {
        self.model_ref.as_ref()
    }

    /// Set the model reference
    pub fn set_model_ref(&mut self, model: Rc<FeaModel3d>) {
        self.model_ref = Some(model);
    }

    /// Get the element descriptor
    pub fn element_descriptor(&self) -> Option<&Rc<Volume3dElementDescriptor>> {
        self.element_descriptor.as_ref()
    }

    /// Set the element descriptor
    pub fn set_element_descriptor(&mut self, descriptor: Rc<Volume3dElementDescriptor>) {
        self.element_descriptor = Some(descriptor);
    }

    /// Get the material
    pub fn material(&self) -> Option<&Rc<ElementMaterial>> {
        self.material.as_ref()
    }

    /// Set the material
    pub fn set_material(&mut self, material: Rc<ElementMaterial>) {
        self.material = Some(material);
    }

    /// Get the node list
    pub fn node_list(&self) -> &[Rc<NodeRepresentation>] {
        self.element_representation.node_list()
    }
}

impl Default for StepFeaVolume3dElementRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v3d = StepFeaVolume3dElementRepresentation::new();
        assert_eq!(v3d.name(), None);
        assert!(v3d.model_ref().is_none());
    }

    #[test]
    fn test_fea_model3d() {
        let model = FeaModel3d::new("MODEL_3D".to_string());
        assert_eq!(model.id(), "MODEL_3D");
    }

    #[test]
    fn test_volume3d_element_descriptor() {
        let desc = Volume3dElementDescriptor::new("tetrahedron".to_string());
        assert_eq!(desc.element_type(), "tetrahedron");
    }

    #[test]
    fn test_element_material() {
        let mat = ElementMaterial::new("MAT_1".to_string(), "Steel".to_string());
        assert_eq!(mat.material_id(), "MAT_1");
        assert_eq!(mat.material_name(), "Steel");
    }

    #[test]
    fn test_init() {
        let mut v3d = StepFeaVolume3dElementRepresentation::new();
        let context = Rc::new(RepresentationContext::new("CTX".to_string()));
        let model = Rc::new(FeaModel3d::new("M3D".to_string()));
        let desc = Rc::new(Volume3dElementDescriptor::new("hex".to_string()));
        let mat = Rc::new(ElementMaterial::new("MAT".to_string(), "Aluminum".to_string()));
        let node = Rc::new(NodeRepresentation::new("N1".to_string()));
        let item = Rc::new(RepresentationItem::new("i1".to_string()));
        v3d.init(
            "vol_elem".to_string(),
            vec![item],
            context,
            vec![node],
            model,
            desc,
            mat,
        );
        assert_eq!(v3d.name(), Some("vol_elem"));
        assert!(v3d.model_ref().is_some());
        assert!(v3d.element_descriptor().is_some());
        assert!(v3d.material().is_some());
    }

    #[test]
    fn test_set_model_ref() {
        let mut v3d = StepFeaVolume3dElementRepresentation::new();
        let model = Rc::new(FeaModel3d::new("M_X".to_string()));
        v3d.set_model_ref(model);
        assert!(v3d.model_ref().is_some());
    }

    #[test]
    fn test_set_material() {
        let mut v3d = StepFeaVolume3dElementRepresentation::new();
        let mat = Rc::new(ElementMaterial::new("M_ID".to_string(), "Titanium".to_string()));
        v3d.set_material(mat);
        assert!(v3d.material().is_some());
    }
}
