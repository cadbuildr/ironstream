// FILE: vrml_scene.rs
// occt: VrmlData_Scene, VrmlData_Group, VrmlData_ShapeNode,
//       VrmlData_Appearance, VrmlData_Material

/// VRML material (diffuse + emissive + specular + shininess + transparency).
/// occt: VrmlData_Material
#[derive(Clone, Debug)]
pub struct VrmlMaterial {
    pub name: String,
    pub diffuse_color: [f32; 3],
    pub emissive_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub shininess: f32,
    pub transparency: f32,
    pub ambient_intensity: f32,
}

impl Default for VrmlMaterial {
    fn default() -> Self {
        Self {
            name: String::new(),
            diffuse_color: [0.8, 0.8, 0.8],
            emissive_color: [0.0, 0.0, 0.0],
            specular_color: [0.0, 0.0, 0.0],
            shininess: 0.2,
            transparency: 0.0,
            ambient_intensity: 0.2,
        }
    }
}

impl VrmlMaterial {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), ..Self::default() }
    }

    pub fn with_diffuse(mut self, r: f32, g: f32, b: f32) -> Self {
        self.diffuse_color = [r, g, b];
        self
    }

    pub fn set_transparency(&mut self, t: f32) { self.transparency = t.clamp(0.0, 1.0); }
    pub fn set_shininess(&mut self, s: f32) { self.shininess = s.clamp(0.0, 1.0); }
    pub fn is_transparent(&self) -> bool { self.transparency > 0.0 }

    pub fn to_vrml_string(&self) -> String {
        format!(
            "Material {{ diffuseColor {} {} {} shininess {} transparency {} }}",
            self.diffuse_color[0], self.diffuse_color[1], self.diffuse_color[2],
            self.shininess, self.transparency
        )
    }
}

/// VRML appearance node: material + texture references.
/// occt: VrmlData_Appearance
#[derive(Clone, Debug, Default)]
pub struct VrmlAppearance {
    pub name: String,
    pub material: Option<VrmlMaterial>,
    pub texture_id: Option<u32>,
}

impl VrmlAppearance {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), ..Self::default() }
    }

    pub fn set_material(&mut self, m: VrmlMaterial) { self.material = Some(m); }
    pub fn set_texture(&mut self, id: u32) { self.texture_id = Some(id); }
    pub fn has_material(&self) -> bool { self.material.is_some() }
    pub fn has_texture(&self) -> bool { self.texture_id.is_some() }
}

/// VRML geometry reference (mesh or implicit shape).
#[derive(Clone, Debug)]
pub struct VrmlGeometry {
    pub geometry_id: u32,
    pub geometry_type: String,  // "IndexedFaceSet", "IndexedLineSet", "Box", etc.
}

impl VrmlGeometry {
    pub fn new(geometry_id: u32, geometry_type: impl Into<String>) -> Self {
        Self { geometry_id, geometry_type: geometry_type.into() }
    }
}

/// VRML Shape node: geometry + appearance.
/// occt: VrmlData_ShapeNode
#[derive(Clone, Debug, Default)]
pub struct VrmlShapeNode {
    pub node_id: u32,
    pub name: String,
    pub geometry: Option<VrmlGeometry>,
    pub appearance: Option<VrmlAppearance>,
}

impl VrmlShapeNode {
    pub fn new(node_id: u32, name: impl Into<String>) -> Self {
        Self { node_id, name: name.into(), ..Self::default() }
    }

    pub fn set_geometry(&mut self, g: VrmlGeometry) { self.geometry = Some(g); }
    pub fn set_appearance(&mut self, a: VrmlAppearance) { self.appearance = Some(a); }
    pub fn has_geometry(&self) -> bool { self.geometry.is_some() }
    pub fn has_appearance(&self) -> bool { self.appearance.is_some() }
}

/// Transform applied to a group.
#[derive(Clone, Debug)]
pub struct VrmlTransform {
    pub translation: [f32; 3],
    pub rotation: [f32; 4],  // axis-angle (x,y,z,angle)
    pub scale: [f32; 3],
    pub scale_orientation: [f32; 4],
    pub center: [f32; 3],
}

impl Default for VrmlTransform {
    fn default() -> Self {
        Self {
            translation: [0.0; 3],
            rotation: [0.0, 0.0, 1.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            scale_orientation: [0.0, 0.0, 1.0, 0.0],
            center: [0.0; 3],
        }
    }
}

impl VrmlTransform {
    pub fn translate(mut self, x: f32, y: f32, z: f32) -> Self {
        self.translation = [x, y, z];
        self
    }

    pub fn scale_uniform(mut self, s: f32) -> Self {
        self.scale = [s, s, s];
        self
    }

    pub fn is_identity(&self) -> bool {
        self.translation.iter().all(|&v| v.abs() < 1e-7) &&
        self.rotation[3].abs() < 1e-7 &&
        self.scale.iter().all(|&v| (v - 1.0).abs() < 1e-7)
    }
}

/// VRML Group node: a collection of child nodes with a transform.
/// occt: VrmlData_Group
#[derive(Clone, Debug, Default)]
pub struct VrmlGroup {
    pub node_id: u32,
    pub name: String,
    pub transform: VrmlTransform,
    pub children: Vec<VrmlNodeRef>,
}

/// A reference to a VRML node (group or shape).
#[derive(Clone, Debug)]
pub enum VrmlNodeRef {
    Shape(VrmlShapeNode),
    Group(Box<VrmlGroup>),
}

impl VrmlNodeRef {
    pub fn node_id(&self) -> u32 {
        match self {
            Self::Shape(s) => s.node_id,
            Self::Group(g) => g.node_id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Shape(s) => &s.name,
            Self::Group(g) => &g.name,
        }
    }
}

impl VrmlGroup {
    pub fn new(node_id: u32, name: impl Into<String>) -> Self {
        Self { node_id, name: name.into(), ..Self::default() }
    }

    pub fn add_child(&mut self, child: VrmlNodeRef) { self.children.push(child); }
    pub fn nb_children(&self) -> usize { self.children.len() }

    pub fn set_transform(&mut self, t: VrmlTransform) { self.transform = t; }

    pub fn find_child(&self, name: &str) -> Option<&VrmlNodeRef> {
        self.children.iter().find(|c| c.name() == name)
    }
}

/// VRML 2.0 scene: root nodes + metadata.
/// occt: VrmlData_Scene
#[derive(Clone, Debug, Default)]
pub struct VrmlScene {
    pub version: String,
    pub roots: Vec<VrmlNodeRef>,
    pub nb_shape_nodes: u32,
    pub nb_group_nodes: u32,
    pub triangulation_deflection: f64,
}

impl VrmlScene {
    pub fn new() -> Self {
        Self {
            version: "2.0".into(),
            triangulation_deflection: 0.1,
            ..Self::default()
        }
    }

    pub fn add_root(&mut self, node: VrmlNodeRef) {
        match &node {
            VrmlNodeRef::Shape(_) => self.nb_shape_nodes += 1,
            VrmlNodeRef::Group(_) => self.nb_group_nodes += 1,
        }
        self.roots.push(node);
    }

    pub fn nb_roots(&self) -> usize { self.roots.len() }

    pub fn find_root(&self, name: &str) -> Option<&VrmlNodeRef> {
        self.roots.iter().find(|r| r.name() == name)
    }

    pub fn set_deflection(&mut self, d: f64) { self.triangulation_deflection = d.max(0.0); }

    pub fn header_string(&self) -> String {
        format!("#VRML V{} utf8\n# Generated by IronStream", self.version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn material_string() {
        let m = VrmlMaterial::new("steel").with_diffuse(0.5, 0.5, 0.6);
        assert!(!m.is_transparent());
        let s = m.to_vrml_string();
        assert!(s.contains("Material"));
        assert!(s.contains("0.5"));
    }

    #[test]
    fn appearance_material() {
        let mut a = VrmlAppearance::new("App");
        assert!(!a.has_material());
        a.set_material(VrmlMaterial::default());
        assert!(a.has_material());
        a.set_texture(3);
        assert!(a.has_texture());
    }

    #[test]
    fn group_children() {
        let mut g = VrmlGroup::new(1, "Root");
        g.add_child(VrmlNodeRef::Shape(VrmlShapeNode::new(2, "Part")));
        g.add_child(VrmlNodeRef::Shape(VrmlShapeNode::new(3, "Cover")));
        assert_eq!(g.nb_children(), 2);
        assert!(g.find_child("Part").is_some());
        assert!(g.find_child("Missing").is_none());
    }

    #[test]
    fn scene_roots() {
        let mut scene = VrmlScene::new();
        scene.add_root(VrmlNodeRef::Shape(VrmlShapeNode::new(1, "S1")));
        scene.add_root(VrmlNodeRef::Group(Box::new(VrmlGroup::new(2, "G1"))));
        assert_eq!(scene.nb_roots(), 2);
        assert_eq!(scene.nb_shape_nodes, 1);
        assert_eq!(scene.nb_group_nodes, 1);
        assert!(scene.header_string().contains("#VRML"));
    }

    #[test]
    fn transform_identity() {
        let t = VrmlTransform::default();
        assert!(t.is_identity());
        let t2 = VrmlTransform::default().translate(1.0, 0.0, 0.0);
        assert!(!t2.is_identity());
        let t3 = VrmlTransform::default().scale_uniform(2.0);
        assert!(!t3.is_identity());
    }
}
