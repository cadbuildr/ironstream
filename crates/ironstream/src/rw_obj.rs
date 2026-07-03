// FILE: rust/ironstream/crates/ironstream/src/rw_obj.rs
//! `rw_obj` — Wavefront OBJ file I/O framework, mirroring OpenCascade's
//! `RWObj` package (`src/DataExchange/TKRWMesh/RWObj/`).
//!
//! OBJ is a plain-text format for 3-D geometry.  Each file may reference one
//! or more named material libraries (`.mtl` files) and groups meshes into
//! named objects.
//!
//! This module provides zero-dependency, pure-Rust **stub** implementations of
//! the most commonly used RWObj types.  No file I/O is performed; callers
//! construct the data model in memory.
//!
//! # OCCT correspondence
//!
//! | OCCT type                | IronStream type    |
//! |--------------------------|--------------------|
//! | `RWObj_Material`         | [`RwObjMaterial`]  |
//! | *(mesh container)*       | [`RwObjMesh`]      |
//! | `RWObj_CafReader`        | [`RwObjScene`]     |

// ---------------------------------------------------------------------------
// RwObjMaterial
// ---------------------------------------------------------------------------

/// Material properties for a single OBJ material block.
///
/// Each material corresponds to one `newmtl` entry in a `.mtl` file and
/// holds the standard Phong lighting parameters together with an optional
/// diffuse texture map path.
// occt: RWObj_Material
#[derive(Debug, Clone)]
pub struct RwObjMaterial {
    /// Material name as declared after `newmtl`.
    name: String,
    /// Ambient colour components `(Ka)` in linear RGB \[0, 1\].
    ambient: [f32; 3],
    /// Diffuse colour components `(Kd)` in linear RGB \[0, 1\].
    diffuse: [f32; 3],
    /// Specular colour components `(Ks)` in linear RGB \[0, 1\].
    specular: [f32; 3],
    /// Specular exponent `(Ns)` — shininess in \[0, 1000\].
    shininess: f32,
    /// Transparency factor `(d)` where 0.0 = fully opaque, 1.0 = invisible.
    transparency: f32,
    /// Path to the diffuse texture map `(map_Kd)`, empty if not set.
    texture_map: String,
}

impl RwObjMaterial {
    /// Create a new material with the given name and neutral default values.
    ///
    /// Defaults: ambient = \[0, 0, 0\], diffuse = \[1, 1, 1\] (white),
    /// specular = \[0, 0, 0\], shininess = 0.0, transparency = 0.0,
    /// texture_map = "".
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ambient: [0.0, 0.0, 0.0],
            diffuse: [1.0, 1.0, 1.0],
            specular: [0.0, 0.0, 0.0],
            shininess: 0.0,
            transparency: 0.0,
            texture_map: String::new(),
        }
    }

    /// Return the material name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the diffuse colour `(Kd)`.
    pub fn set_diffuse(&mut self, rgb: [f32; 3]) {
        self.diffuse = rgb;
    }

    /// Return the diffuse colour `(Kd)`.
    pub fn diffuse(&self) -> [f32; 3] {
        self.diffuse
    }

    /// Set the specular colour `(Ks)`.
    pub fn set_specular(&mut self, rgb: [f32; 3]) {
        self.specular = rgb;
    }

    /// Return the specular colour `(Ks)`.
    pub fn specular(&self) -> [f32; 3] {
        self.specular
    }

    /// Set the shininess exponent `(Ns)`.
    pub fn set_shininess(&mut self, v: f32) {
        self.shininess = v;
    }

    /// Return the shininess exponent `(Ns)`.
    pub fn shininess(&self) -> f32 {
        self.shininess
    }

    /// Set the transparency factor `(d)`.
    pub fn set_transparency(&mut self, v: f32) {
        self.transparency = v;
    }

    /// Return the transparency factor `(d)`.
    pub fn transparency(&self) -> f32 {
        self.transparency
    }

    /// Set the diffuse texture map path `(map_Kd)`.
    pub fn set_texture_map(&mut self, path: &str) {
        self.texture_map.clear();
        self.texture_map.push_str(path);
    }

    /// Return the diffuse texture map path, or an empty string if unset.
    pub fn texture_map(&self) -> &str {
        &self.texture_map
    }
}

impl Default for RwObjMaterial {
    fn default() -> Self {
        Self::new("")
    }
}

// ---------------------------------------------------------------------------
// RwObjMesh
// ---------------------------------------------------------------------------

/// An OBJ mesh container holding one named group of geometry.
///
/// Vertices, normals and texture coordinates are stored as flat `Vec`s.
/// Triangle indices reference positions in those arrays using 0-based indices.
// occt: RWObj mesh container
#[derive(Debug, Clone)]
pub struct RwObjMesh {
    /// Object/group name.
    name: String,
    /// Vertex positions (`v` statements).
    vertices: Vec<[f32; 3]>,
    /// Vertex normals (`vn` statements).
    normals: Vec<[f32; 3]>,
    /// Texture coordinates (`vt` statements).
    tex_coords: Vec<[f32; 2]>,
    /// Triangle index triples referencing positions in [`vertices`].
    indices: Vec<[u32; 3]>,
    /// Name of the material applied to this mesh (empty if none).
    material_name: String,
}

impl RwObjMesh {
    /// Create a new, empty mesh with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            vertices: Vec::new(),
            normals: Vec::new(),
            tex_coords: Vec::new(),
            indices: Vec::new(),
            material_name: String::new(),
        }
    }

    /// Return the mesh name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Append a vertex position and return its 0-based index.
    pub fn add_vertex(&mut self, v: [f32; 3]) -> usize {
        let idx = self.vertices.len();
        self.vertices.push(v);
        idx
    }

    /// Append a vertex normal and return its 0-based index.
    pub fn add_normal(&mut self, n: [f32; 3]) -> usize {
        let idx = self.normals.len();
        self.normals.push(n);
        idx
    }

    /// Append a texture coordinate and return its 0-based index.
    pub fn add_tex_coord(&mut self, uv: [f32; 2]) -> usize {
        let idx = self.tex_coords.len();
        self.tex_coords.push(uv);
        idx
    }

    /// Append a triangle defined by three vertex indices.
    pub fn add_triangle(&mut self, tri: [u32; 3]) {
        self.indices.push(tri);
    }

    /// Return the number of vertices.
    pub fn nb_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Return the number of triangles.
    pub fn nb_triangles(&self) -> usize {
        self.indices.len()
    }

    /// Set the material name for this mesh.
    pub fn set_material(&mut self, name: &str) {
        self.material_name.clear();
        self.material_name.push_str(name);
    }

    /// Return the material name, or an empty string if no material is set.
    pub fn material_name(&self) -> &str {
        &self.material_name
    }

    /// Return a slice of all vertex positions.
    pub fn vertices(&self) -> &[[f32; 3]] {
        &self.vertices
    }

    /// Return a slice of all vertex normals.
    pub fn normals(&self) -> &[[f32; 3]] {
        &self.normals
    }

    /// Return a slice of all texture coordinates.
    pub fn tex_coords(&self) -> &[[f32; 2]] {
        &self.tex_coords
    }

    /// Return a slice of all triangle index triples.
    pub fn indices(&self) -> &[[u32; 3]] {
        &self.indices
    }
}

// ---------------------------------------------------------------------------
// RwObjScene
// ---------------------------------------------------------------------------

/// Top-level OBJ scene container.
///
/// Accumulates [`RwObjMesh`] objects and [`RwObjMaterial`] definitions that
/// together describe a complete OBJ asset.  Corresponds broadly to the reader
/// result produced by `RWObj_CafReader` after ingesting an OBJ file.
// occt: RWObj_CafReader scene stub
#[derive(Debug)]
pub struct RwObjScene {
    /// All meshes in insertion order.
    meshes: Vec<RwObjMesh>,
    /// All materials in insertion order.
    materials: Vec<RwObjMaterial>,
}

impl RwObjScene {
    /// Create an empty scene.
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            materials: Vec::new(),
        }
    }

    /// Append a mesh to the scene.
    pub fn add_mesh(&mut self, m: RwObjMesh) {
        self.meshes.push(m);
    }

    /// Append a material to the scene.
    pub fn add_material(&mut self, m: RwObjMaterial) {
        self.materials.push(m);
    }

    /// Return the number of meshes.
    pub fn nb_meshes(&self) -> usize {
        self.meshes.len()
    }

    /// Return the number of materials.
    pub fn nb_materials(&self) -> usize {
        self.materials.len()
    }

    /// Return a reference to the mesh at 0-based index `i`.
    ///
    /// # Panics
    ///
    /// Panics if `i >= nb_meshes()`.
    pub fn mesh(&self, i: usize) -> &RwObjMesh {
        &self.meshes[i]
    }

    /// Return a reference to the material at 0-based index `i`.
    ///
    /// # Panics
    ///
    /// Panics if `i >= nb_materials()`.
    pub fn material(&self, i: usize) -> &RwObjMaterial {
        &self.materials[i]
    }
}

impl Default for RwObjScene {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- RwObjMaterial ---

    #[test]
    fn material_new_defaults() {
        let m = RwObjMaterial::new("Mat1");
        assert_eq!(m.name(), "Mat1");
        assert_eq!(m.diffuse(), [1.0, 1.0, 1.0]);
        assert_eq!(m.specular(), [0.0, 0.0, 0.0]);
        assert_eq!(m.shininess(), 0.0);
        assert_eq!(m.transparency(), 0.0);
        assert_eq!(m.texture_map(), "");
    }

    #[test]
    fn material_set_diffuse() {
        let mut m = RwObjMaterial::new("red");
        m.set_diffuse([1.0, 0.0, 0.0]);
        assert_eq!(m.diffuse(), [1.0, 0.0, 0.0]);
    }

    #[test]
    fn material_set_specular() {
        let mut m = RwObjMaterial::new("shiny");
        m.set_specular([0.5, 0.5, 0.5]);
        assert_eq!(m.specular(), [0.5, 0.5, 0.5]);
    }

    #[test]
    fn material_set_shininess() {
        let mut m = RwObjMaterial::new("glossy");
        m.set_shininess(128.0);
        assert_eq!(m.shininess(), 128.0);
    }

    #[test]
    fn material_set_transparency() {
        let mut m = RwObjMaterial::new("glass");
        m.set_transparency(0.5);
        assert_eq!(m.transparency(), 0.5);
    }

    #[test]
    fn material_set_texture_map() {
        let mut m = RwObjMaterial::new("textured");
        m.set_texture_map("textures/wood.png");
        assert_eq!(m.texture_map(), "textures/wood.png");
    }

    #[test]
    fn material_set_texture_map_overwrite() {
        let mut m = RwObjMaterial::new("t");
        m.set_texture_map("first.png");
        m.set_texture_map("second.png");
        assert_eq!(m.texture_map(), "second.png");
    }

    #[test]
    fn material_default() {
        let m = RwObjMaterial::default();
        assert_eq!(m.name(), "");
        assert_eq!(m.diffuse(), [1.0, 1.0, 1.0]);
    }

    // --- RwObjMesh ---

    #[test]
    fn mesh_new_is_empty() {
        let m = RwObjMesh::new("MyMesh");
        assert_eq!(m.name(), "MyMesh");
        assert_eq!(m.nb_vertices(), 0);
        assert_eq!(m.nb_triangles(), 0);
        assert_eq!(m.material_name(), "");
        assert!(m.vertices().is_empty());
        assert!(m.normals().is_empty());
        assert!(m.tex_coords().is_empty());
        assert!(m.indices().is_empty());
    }

    #[test]
    fn mesh_add_vertex_increments_count() {
        let mut m = RwObjMesh::new("v");
        let i0 = m.add_vertex([0.0, 0.0, 0.0]);
        let i1 = m.add_vertex([1.0, 0.0, 0.0]);
        let i2 = m.add_vertex([0.0, 1.0, 0.0]);
        assert_eq!(i0, 0);
        assert_eq!(i1, 1);
        assert_eq!(i2, 2);
        assert_eq!(m.nb_vertices(), 3);
    }

    #[test]
    fn mesh_add_normal_increments_count() {
        let mut m = RwObjMesh::new("n");
        let i = m.add_normal([0.0, 0.0, 1.0]);
        assert_eq!(i, 0);
        assert_eq!(m.normals().len(), 1);
        assert_eq!(m.normals()[0], [0.0, 0.0, 1.0]);
    }

    #[test]
    fn mesh_add_tex_coord_increments_count() {
        let mut m = RwObjMesh::new("uv");
        let i = m.add_tex_coord([0.5, 0.25]);
        assert_eq!(i, 0);
        assert_eq!(m.tex_coords().len(), 1);
        assert_eq!(m.tex_coords()[0], [0.5, 0.25]);
    }

    #[test]
    fn mesh_add_triangle_increments_count() {
        let mut m = RwObjMesh::new("tri");
        m.add_vertex([0.0, 0.0, 0.0]);
        m.add_vertex([1.0, 0.0, 0.0]);
        m.add_vertex([0.0, 1.0, 0.0]);
        m.add_triangle([0, 1, 2]);
        assert_eq!(m.nb_triangles(), 1);
        assert_eq!(m.indices()[0], [0, 1, 2]);
    }

    #[test]
    fn mesh_multiple_triangles() {
        let mut m = RwObjMesh::new("quad");
        m.add_vertex([0.0, 0.0, 0.0]);
        m.add_vertex([1.0, 0.0, 0.0]);
        m.add_vertex([1.0, 1.0, 0.0]);
        m.add_vertex([0.0, 1.0, 0.0]);
        m.add_triangle([0, 1, 2]);
        m.add_triangle([0, 2, 3]);
        assert_eq!(m.nb_triangles(), 2);
        assert_eq!(m.nb_vertices(), 4);
    }

    #[test]
    fn mesh_set_material() {
        let mut m = RwObjMesh::new("box");
        m.set_material("Metal");
        assert_eq!(m.material_name(), "Metal");
    }

    #[test]
    fn mesh_set_material_overwrite() {
        let mut m = RwObjMesh::new("box");
        m.set_material("Metal");
        m.set_material("Plastic");
        assert_eq!(m.material_name(), "Plastic");
    }

    #[test]
    fn mesh_vertices_slice_matches() {
        let mut m = RwObjMesh::new("s");
        m.add_vertex([1.0, 2.0, 3.0]);
        m.add_vertex([4.0, 5.0, 6.0]);
        assert_eq!(m.vertices(), &[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
    }

    // --- RwObjScene ---

    #[test]
    fn scene_new_is_empty() {
        let s = RwObjScene::new();
        assert_eq!(s.nb_meshes(), 0);
        assert_eq!(s.nb_materials(), 0);
    }

    #[test]
    fn scene_default_is_empty() {
        let s = RwObjScene::default();
        assert_eq!(s.nb_meshes(), 0);
        assert_eq!(s.nb_materials(), 0);
    }

    #[test]
    fn scene_add_mesh_increments_count() {
        let mut s = RwObjScene::new();
        s.add_mesh(RwObjMesh::new("A"));
        s.add_mesh(RwObjMesh::new("B"));
        assert_eq!(s.nb_meshes(), 2);
    }

    #[test]
    fn scene_add_material_increments_count() {
        let mut s = RwObjScene::new();
        s.add_material(RwObjMaterial::new("M1"));
        s.add_material(RwObjMaterial::new("M2"));
        assert_eq!(s.nb_materials(), 2);
    }

    #[test]
    fn scene_mesh_by_index() {
        let mut s = RwObjScene::new();
        s.add_mesh(RwObjMesh::new("first"));
        s.add_mesh(RwObjMesh::new("second"));
        assert_eq!(s.mesh(0).name(), "first");
        assert_eq!(s.mesh(1).name(), "second");
    }

    #[test]
    fn scene_material_by_index() {
        let mut s = RwObjScene::new();
        s.add_material(RwObjMaterial::new("red"));
        s.add_material(RwObjMaterial::new("blue"));
        assert_eq!(s.material(0).name(), "red");
        assert_eq!(s.material(1).name(), "blue");
    }

    #[test]
    #[should_panic]
    fn scene_mesh_out_of_bounds_panics() {
        let s = RwObjScene::new();
        let _ = s.mesh(0);
    }

    #[test]
    #[should_panic]
    fn scene_material_out_of_bounds_panics() {
        let s = RwObjScene::new();
        let _ = s.material(0);
    }

    #[test]
    fn scene_full_round_trip() {
        let mut scene = RwObjScene::new();

        let mut mat = RwObjMaterial::new("Wood");
        mat.set_diffuse([0.6, 0.4, 0.2]);
        mat.set_specular([0.1, 0.1, 0.1]);
        mat.set_shininess(32.0);
        mat.set_transparency(0.0);
        mat.set_texture_map("textures/wood.png");
        scene.add_material(mat);

        let mut mesh = RwObjMesh::new("Plank");
        mesh.add_vertex([0.0, 0.0, 0.0]);
        mesh.add_vertex([1.0, 0.0, 0.0]);
        mesh.add_vertex([1.0, 0.0, 1.0]);
        mesh.add_vertex([0.0, 0.0, 1.0]);
        mesh.add_normal([0.0, 1.0, 0.0]);
        mesh.add_tex_coord([0.0, 0.0]);
        mesh.add_tex_coord([1.0, 0.0]);
        mesh.add_tex_coord([1.0, 1.0]);
        mesh.add_tex_coord([0.0, 1.0]);
        mesh.add_triangle([0, 1, 2]);
        mesh.add_triangle([0, 2, 3]);
        mesh.set_material("Wood");
        scene.add_mesh(mesh);

        assert_eq!(scene.nb_meshes(), 1);
        assert_eq!(scene.nb_materials(), 1);

        let m = scene.mesh(0);
        assert_eq!(m.name(), "Plank");
        assert_eq!(m.nb_vertices(), 4);
        assert_eq!(m.nb_triangles(), 2);
        assert_eq!(m.normals().len(), 1);
        assert_eq!(m.tex_coords().len(), 4);
        assert_eq!(m.material_name(), "Wood");

        let mat = scene.material(0);
        assert_eq!(mat.name(), "Wood");
        assert_eq!(mat.diffuse(), [0.6, 0.4, 0.2]);
        assert_eq!(mat.shininess(), 32.0);
        assert_eq!(mat.texture_map(), "textures/wood.png");
    }
}
