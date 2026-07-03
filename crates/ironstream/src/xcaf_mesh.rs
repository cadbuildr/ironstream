// FILE: xcaf_mesh.rs
// occt: XCAFDoc_VisMaterial, XCAFDoc_VisMaterialPBR, XCAFPrs_DocumentNode,
//       XCAFPrs_DocumentExplorer, RWMesh_MaterialMap

/// PBR material parameters.
#[derive(Clone, Debug)]
pub struct VisMaterialPBR {
    pub base_color: [f32; 4],          // RGBA
    pub metallic_roughness: [f32; 2],  // [metallic, roughness]
    pub emissive_factor: [f32; 3],
    pub alpha_cutoff: f32,
    pub double_sided: bool,
    pub base_color_texture: Option<String>,
    pub metallic_roughness_texture: Option<String>,
    pub normal_texture: Option<String>,
    pub emissive_texture: Option<String>,
}

impl VisMaterialPBR {
    pub fn new() -> Self {
        Self {
            base_color: [1.0, 1.0, 1.0, 1.0],
            metallic_roughness: [0.0, 1.0],
            emissive_factor: [0.0; 3],
            alpha_cutoff: 0.5,
            double_sided: false,
            base_color_texture: None,
            metallic_roughness_texture: None,
            normal_texture: None,
            emissive_texture: None,
        }
    }

    pub fn with_base_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.base_color = [r, g, b, a]; self
    }
    pub fn with_metallic(mut self, m: f32) -> Self { self.metallic_roughness[0] = m; self }
    pub fn with_roughness(mut self, r: f32) -> Self { self.metallic_roughness[1] = r; self }
    pub fn is_transparent(&self) -> bool { self.base_color[3] < 1.0 }
}

impl Default for VisMaterialPBR {
    fn default() -> Self { Self::new() }
}

// occt: XCAFDoc_VisMaterial
#[derive(Clone, Debug, Default)]
pub struct VisMaterial {
    pub name: String,
    pub pbr: Option<VisMaterialPBR>,
    pub common_transparency: f32,
    pub common_shininess: f32,
    pub is_defined: bool,
}

impl VisMaterial {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), is_defined: true, ..Default::default() }
    }

    pub fn with_pbr(mut self, pbr: VisMaterialPBR) -> Self { self.pbr = Some(pbr); self }
    pub fn has_pbr(&self) -> bool { self.pbr.is_some() }

    pub fn base_color(&self) -> [f32; 4] {
        self.pbr.as_ref().map(|p| p.base_color).unwrap_or([1.0;4])
    }
}

// occt: XCAFPrs_DocumentNode
#[derive(Clone, Debug)]
pub struct DocumentNode {
    pub id: u64,
    pub parent: Option<u64>,
    pub children: Vec<u64>,
    pub material: Option<VisMaterial>,
    pub transform: [[f64; 4]; 4],  // 4x4 column-major
    pub mesh_id: Option<u64>,
    pub name: String,
}

impl DocumentNode {
    pub fn new(id: u64, name: impl Into<String>) -> Self {
        Self {
            id,
            parent: None,
            children: Vec::new(),
            material: None,
            transform: identity4(),
            mesh_id: None,
            name: name.into(),
        }
    }

    pub fn is_leaf(&self) -> bool { self.children.is_empty() }
    pub fn has_mesh(&self) -> bool { self.mesh_id.is_some() }
    pub fn has_material(&self) -> bool { self.material.is_some() }
}

// occt: XCAFPrs_DocumentExplorer
#[derive(Clone, Debug, Default)]
pub struct DocumentExplorer {
    pub nodes: Vec<DocumentNode>,
    pub roots: Vec<u64>,
}

impl DocumentExplorer {
    pub fn new() -> Self { Self::default() }

    pub fn add_node(&mut self, node: DocumentNode) {
        if node.parent.is_none() { self.roots.push(node.id); }
        self.nodes.push(node);
    }

    pub fn find(&self, id: u64) -> Option<&DocumentNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn nb_nodes(&self) -> usize { self.nodes.len() }
    pub fn nb_roots(&self) -> usize { self.roots.len() }

    pub fn leaves(&self) -> Vec<&DocumentNode> {
        self.nodes.iter().filter(|n| n.is_leaf()).collect()
    }

    pub fn nodes_with_mesh(&self) -> Vec<&DocumentNode> {
        self.nodes.iter().filter(|n| n.has_mesh()).collect()
    }
}

// occt: RWMesh_MaterialMap
#[derive(Clone, Debug, Default)]
pub struct MaterialMap {
    pub materials: Vec<VisMaterial>,
}

impl MaterialMap {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, mat: VisMaterial) { self.materials.push(mat); }

    pub fn find(&self, name: &str) -> Option<&VisMaterial> {
        self.materials.iter().find(|m| m.name == name)
    }

    pub fn nb_materials(&self) -> usize { self.materials.len() }

    pub fn transparent_count(&self) -> usize {
        self.materials.iter().filter(|m| m.common_transparency > 0.0
            || m.pbr.as_ref().map(|p| p.is_transparent()).unwrap_or(false)).count()
    }
}

fn identity4() -> [[f64; 4]; 4] {
    [[1.0,0.0,0.0,0.0],[0.0,1.0,0.0,0.0],[0.0,0.0,1.0,0.0],[0.0,0.0,0.0,1.0]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vis_material_pbr() {
        let pbr = VisMaterialPBR::new().with_base_color(1.0,0.0,0.0,0.5).with_metallic(0.8);
        assert!(pbr.is_transparent());
        assert!((pbr.metallic_roughness[0] - 0.8).abs() < 1e-6);
    }

    #[test]
    fn vis_material_base_color() {
        let m = VisMaterial::new("Red").with_pbr(VisMaterialPBR::new().with_base_color(1.0,0.0,0.0,1.0));
        assert!(m.has_pbr());
        assert!((m.base_color()[0] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn document_explorer() {
        let mut ex = DocumentExplorer::new();
        let mut root = DocumentNode::new(1, "root");
        root.children.push(2);
        ex.add_node(root);
        let mut leaf = DocumentNode::new(2, "leaf");
        leaf.parent = Some(1);
        leaf.mesh_id = Some(10);
        ex.add_node(leaf);
        assert_eq!(ex.nb_roots(), 1);
        assert_eq!(ex.leaves().len(), 1);
        assert_eq!(ex.nodes_with_mesh().len(), 1);
    }

    #[test]
    fn material_map() {
        let mut mm = MaterialMap::new();
        mm.add(VisMaterial::new("mat1"));
        mm.add(VisMaterial::new("mat2"));
        assert_eq!(mm.nb_materials(), 2);
        assert!(mm.find("mat1").is_some());
    }
}
