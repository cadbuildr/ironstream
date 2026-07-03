// FILE: meshvs_mesh.rs
// occt: MeshVS_Mesh, MeshVS_DataSource, MeshVS_MeshEntityOwner, MeshVS_Drawer

/// Node information in a mesh visualization dataset.
#[derive(Clone, Debug)]
pub struct MeshNode {
    pub id: u64,
    pub coords: [f64; 3],
}

/// Element (face/volume) in a mesh.
#[derive(Clone, Debug)]
pub struct MeshElement {
    pub id: u64,
    pub node_ids: Vec<u64>,
    pub element_type: MeshElementType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeshElementType {
    Node,
    Edge,
    Face,
    Volume,
}

impl MeshElement {
    pub fn new(id: u64, node_ids: Vec<u64>, element_type: MeshElementType) -> Self {
        Self { id, node_ids, element_type }
    }

    pub fn nb_nodes(&self) -> usize { self.node_ids.len() }
}

// occt: MeshVS_DataSource
#[derive(Clone, Debug, Default)]
pub struct MeshDataSource {
    pub nodes: Vec<MeshNode>,
    pub elements: Vec<MeshElement>,
}

impl MeshDataSource {
    pub fn new() -> Self { Self::default() }

    pub fn add_node(&mut self, id: u64, coords: [f64; 3]) {
        self.nodes.push(MeshNode { id, coords });
    }

    pub fn add_element(&mut self, elem: MeshElement) { self.elements.push(elem); }

    pub fn nb_nodes(&self) -> usize { self.nodes.len() }
    pub fn nb_elements(&self) -> usize { self.elements.len() }

    pub fn find_node(&self, id: u64) -> Option<&MeshNode> { self.nodes.iter().find(|n| n.id == id) }
    pub fn find_element(&self, id: u64) -> Option<&MeshElement> { self.elements.iter().find(|e| e.id == id) }

    pub fn elements_of_type(&self, t: MeshElementType) -> Vec<&MeshElement> {
        self.elements.iter().filter(|e| e.element_type == t).collect()
    }

    pub fn bounding_box(&self) -> Option<([f64; 3], [f64; 3])> {
        if self.nodes.is_empty() { return None; }
        let mut mn = [f64::INFINITY; 3]; let mut mx = [f64::NEG_INFINITY; 3];
        for n in &self.nodes {
            for i in 0..3 { mn[i] = mn[i].min(n.coords[i]); mx[i] = mx[i].max(n.coords[i]); }
        }
        Some((mn, mx))
    }
}

// occt: MeshVS_Drawer — visual properties for mesh display
#[derive(Clone, Debug)]
pub struct MeshDrawer {
    pub color: [f32; 4],
    pub line_width: f32,
    pub point_size: f32,
    pub display_nodes: bool,
    pub display_shrink: bool,
    pub shrink_coeff: f64,
    pub back_face_culling: bool,
}

impl MeshDrawer {
    pub fn new() -> Self {
        Self {
            color: [0.5, 0.5, 0.5, 1.0],
            line_width: 1.0,
            point_size: 3.0,
            display_nodes: false,
            display_shrink: false,
            shrink_coeff: 0.8,
            back_face_culling: true,
        }
    }

    pub fn with_color(mut self, r: f32, g: f32, b: f32) -> Self { self.color = [r, g, b, 1.0]; self }
    pub fn with_line_width(mut self, w: f32) -> Self { self.line_width = w; self }
}

impl Default for MeshDrawer {
    fn default() -> Self { Self::new() }
}

// occt: MeshVS_Mesh
#[derive(Clone, Debug, Default)]
pub struct MeshVsMesh {
    pub data_source: MeshDataSource,
    pub drawer: MeshDrawer,
    pub name: String,
    pub is_visible: bool,
}

impl MeshVsMesh {
    pub fn new(name: impl Into<String>, data_source: MeshDataSource) -> Self {
        Self { data_source, drawer: MeshDrawer::new(), name: name.into(), is_visible: true }
    }

    pub fn nb_selected_nodes(&self) -> usize { 0 }
    pub fn nb_nodes(&self) -> usize { self.data_source.nb_nodes() }
    pub fn nb_elements(&self) -> usize { self.data_source.nb_elements() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_source_nodes_elements() {
        let mut ds = MeshDataSource::new();
        ds.add_node(1, [0.0,0.0,0.0]);
        ds.add_node(2, [1.0,0.0,0.0]);
        ds.add_node(3, [0.5,1.0,0.0]);
        ds.add_element(MeshElement::new(1, vec![1,2,3], MeshElementType::Face));
        assert_eq!(ds.nb_nodes(), 3);
        assert_eq!(ds.nb_elements(), 1);
        assert_eq!(ds.elements_of_type(MeshElementType::Face).len(), 1);
    }

    #[test]
    fn data_source_bbox() {
        let mut ds = MeshDataSource::new();
        ds.add_node(1, [0.0,0.0,0.0]);
        ds.add_node(2, [3.0,4.0,5.0]);
        let (mn, mx) = ds.bounding_box().unwrap();
        assert!((mx[2] - 5.0).abs() < 1e-10);
        assert!(mn[0].abs() < 1e-10);
    }

    #[test]
    fn mesh_vs_mesh() {
        let ds = MeshDataSource::new();
        let m = MeshVsMesh::new("test_mesh", ds);
        assert!(m.is_visible);
        assert_eq!(m.nb_nodes(), 0);
    }

    #[test]
    fn mesh_drawer_defaults() {
        let d = MeshDrawer::new();
        assert!(d.back_face_culling);
        assert!((d.shrink_coeff - 0.8).abs() < 1e-10);
    }
}
