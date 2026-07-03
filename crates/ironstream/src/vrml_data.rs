// FILE: rust/ironstream/crates/ironstream/src/vrml_data.rs

// occt: VrmlData node type — enum Shape, Appearance, Material, IndexedFaceSet,
//       Coordinate, Normal, TextureCoordinate, Group, Transform, Unknown
/// Mirrors the node-type discriminants from the OCCT VrmlData package.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrmlDataNodeType {
    Shape,
    Appearance,
    Material,
    IndexedFaceSet,
    Coordinate,
    Normal,
    TextureCoordinate,
    Group,
    Transform,
    Unknown,
}

// occt: VrmlData_Material — PBR-adjacent surface appearance descriptor
/// Stores the surface-appearance fields from a VRML Material node.
#[derive(Debug, Clone)]
pub struct VrmlDataMaterial {
    name: String,
    diffuse_color: [f32; 3],
    specular_color: [f32; 3],
    emissive_color: [f32; 3],
    ambient_intensity: f32,
    shininess: f32,
    transparency: f32,
}

impl VrmlDataMaterial {
    /// Create a new material with default (black/zero) colour fields.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            diffuse_color: [0.0; 3],
            specular_color: [0.0; 3],
            emissive_color: [0.0; 3],
            ambient_intensity: 0.2,
            shininess: 0.2,
            transparency: 0.0,
        }
    }

    /// Return the node name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the diffuse (base) colour as an RGB triple in [0, 1].
    pub fn set_diffuse(&mut self, color: [f32; 3]) {
        self.diffuse_color = color;
    }

    /// Return the diffuse colour.
    pub fn diffuse(&self) -> [f32; 3] {
        self.diffuse_color
    }

    /// Set the specular colour as an RGB triple in [0, 1].
    pub fn set_specular(&mut self, color: [f32; 3]) {
        self.specular_color = color;
    }

    /// Return the specular colour.
    pub fn specular(&self) -> [f32; 3] {
        self.specular_color
    }

    /// Set the emissive colour as an RGB triple in [0, 1].
    pub fn set_emissive(&mut self, color: [f32; 3]) {
        self.emissive_color = color;
    }

    /// Return the emissive colour.
    pub fn emissive(&self) -> [f32; 3] {
        self.emissive_color
    }

    /// Set ambient intensity in [0, 1].
    pub fn set_ambient_intensity(&mut self, v: f32) {
        self.ambient_intensity = v;
    }

    /// Return ambient intensity.
    pub fn ambient_intensity(&self) -> f32 {
        self.ambient_intensity
    }

    /// Set transparency in [0, 1] where 1 is fully transparent.
    pub fn set_transparency(&mut self, v: f32) {
        self.transparency = v;
    }

    /// Return transparency.
    pub fn transparency(&self) -> f32 {
        self.transparency
    }

    /// Set shininess (Phong exponent proxy) in [0, 1].
    pub fn set_shininess(&mut self, v: f32) {
        self.shininess = v;
    }

    /// Return shininess.
    pub fn shininess(&self) -> f32 {
        self.shininess
    }
}

// occt: VrmlData_IndexedFaceSet stub — mesh geometry
/// Holds the per-face index soup produced by a VRML IndexedFaceSet node.
#[derive(Debug, Clone)]
pub struct VrmlDataIndexedFaceSet {
    pub coords: Vec<[f32; 3]>,
    pub coord_indices: Vec<Vec<usize>>,
    pub normals: Vec<[f32; 3]>,
    pub normal_per_vertex: bool,
}

impl VrmlDataIndexedFaceSet {
    /// Create an empty face set.
    pub fn new() -> Self {
        Self {
            coords: Vec::new(),
            coord_indices: Vec::new(),
            normals: Vec::new(),
            normal_per_vertex: true,
        }
    }

    /// Append a 3-D coordinate to the coordinate pool.
    pub fn add_coord(&mut self, pt: [f32; 3]) {
        self.coords.push(pt);
    }

    /// Append one polygonal face given as a list of coord indices.
    pub fn add_face(&mut self, indices: Vec<usize>) {
        self.coord_indices.push(indices);
    }

    /// Return the number of coordinate points.
    pub fn nb_coords(&self) -> usize {
        self.coords.len()
    }

    /// Return the number of faces.
    pub fn nb_faces(&self) -> usize {
        self.coord_indices.len()
    }

    /// Return the index slice for face `i`.
    ///
    /// # Panics
    /// Panics if `i >= nb_faces()`.
    pub fn face(&self, i: usize) -> &[usize] {
        &self.coord_indices[i]
    }
}

impl Default for VrmlDataIndexedFaceSet {
    fn default() -> Self {
        Self::new()
    }
}

// occt: VrmlData_Scene — VRML scene container
/// Top-level VRML scene holding face sets and materials.
#[derive(Debug, Clone)]
pub struct VrmlDataScene {
    pub name: String,
    pub face_sets: Vec<VrmlDataIndexedFaceSet>,
    pub materials: Vec<VrmlDataMaterial>,
}

impl VrmlDataScene {
    /// Create a new, empty scene with the given name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            face_sets: Vec::new(),
            materials: Vec::new(),
        }
    }

    /// Append a face set to the scene.
    pub fn add_face_set(&mut self, fs: VrmlDataIndexedFaceSet) {
        self.face_sets.push(fs);
    }

    /// Append a material to the scene.
    pub fn add_material(&mut self, m: VrmlDataMaterial) {
        self.materials.push(m);
    }

    /// Return the number of face sets.
    pub fn nb_face_sets(&self) -> usize {
        self.face_sets.len()
    }

    /// Return the number of materials.
    pub fn nb_materials(&self) -> usize {
        self.materials.len()
    }
}
