// FILE: rust/ironstream/crates/ironstream/src/rw_gltf.rs
//! `rw_gltf` — glTF 2.0 (GL Transmission Format) I/O framework, mirroring
//! OpenCascade's `RWGltf` package
//! (`src/DataExchange/TKRWMesh/RWGltf/`).
//!
//! glTF is a JSON-based (`.gltf`) or binary (`.glb`) format for 3D assets.
//! Each mesh is described by one or more primitives; each primitive references
//! typed accessor views (positions, normals, indices) stored in binary buffers.
//!
//! This module provides zero-dependency, pure-Rust **stub** implementations of
//! the most commonly used RWGltf types.  No file I/O is performed; callers
//! construct the data model in memory and can serialise it through a separate
//! layer.
//!
//! # OCCT correspondence
//!
//! | OCCT type                         | IronStream type          |
//! |-----------------------------------|--------------------------|
//! | `RWGltf_GltfAccessorLayout`       | [`GltfAccessorLayout`]   |
//! | `RWGltf_GltfPrimitiveMode`        | [`GltfPrimitiveMode`]    |
//! | `RWGltf_GltfLatePrimitiveArray`   | [`GltfMeshPrimitive`]    |
//! | `RWGltf_CafWriter`                | [`RwGltfWriter`]         |

// ---------------------------------------------------------------------------
// GltfAccessorLayout
// ---------------------------------------------------------------------------

/// Layout (number and arrangement of components) of a glTF accessor.
///
/// Matches the `type` field of a glTF accessor object
/// (`"SCALAR"`, `"VEC2"`, …, `"MAT4"`).
// occt: RWGltf_GltfAccessorLayout
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GltfAccessorLayout {
    /// Single scalar value (`"SCALAR"`).
    Scalar,
    /// 2-component vector (`"VEC2"`).
    Vec2,
    /// 3-component vector (`"VEC3"`).
    Vec3,
    /// 4-component vector (`"VEC4"`).
    Vec4,
    /// 2×2 matrix (`"MAT2"`).
    Mat2,
    /// 3×3 matrix (`"MAT3"`).
    Mat3,
    /// 4×4 matrix (`"MAT4"`).
    Mat4,
}

impl GltfAccessorLayout {
    /// Return the canonical glTF JSON string for this layout.
    pub fn as_str(self) -> &'static str {
        match self {
            GltfAccessorLayout::Scalar => "SCALAR",
            GltfAccessorLayout::Vec2   => "VEC2",
            GltfAccessorLayout::Vec3   => "VEC3",
            GltfAccessorLayout::Vec4   => "VEC4",
            GltfAccessorLayout::Mat2   => "MAT2",
            GltfAccessorLayout::Mat3   => "MAT3",
            GltfAccessorLayout::Mat4   => "MAT4",
        }
    }

    /// Number of scalar components per element for this layout.
    pub fn nb_components(self) -> usize {
        match self {
            GltfAccessorLayout::Scalar => 1,
            GltfAccessorLayout::Vec2   => 2,
            GltfAccessorLayout::Vec3   => 3,
            GltfAccessorLayout::Vec4   => 4,
            GltfAccessorLayout::Mat2   => 4,
            GltfAccessorLayout::Mat3   => 9,
            GltfAccessorLayout::Mat4   => 16,
        }
    }
}

impl std::fmt::Display for GltfAccessorLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

// ---------------------------------------------------------------------------
// GltfPrimitiveMode
// ---------------------------------------------------------------------------

/// Rendering mode for a glTF mesh primitive.
///
/// Matches the integer `mode` field of a glTF `mesh.primitive` object
/// (default 4 = `TRIANGLES`).
// occt: RWGltf_GltfPrimitiveMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GltfPrimitiveMode {
    /// Each vertex is an independent point (`mode` = 0).
    Points,
    /// Each pair of vertices forms a line segment (`mode` = 1).
    Lines,
    /// Vertices form a closed loop of line segments (`mode` = 2).
    LineLoop,
    /// Vertices form a connected strip of line segments (`mode` = 3).
    LineStrip,
    /// Every three vertices form an independent triangle (`mode` = 4).
    Triangles,
    /// Vertices form a strip of triangles sharing edges (`mode` = 5).
    TriangleStrip,
    /// Vertices form a fan of triangles around the first vertex (`mode` = 6).
    TriangleFan,
}

impl GltfPrimitiveMode {
    /// Return the numeric `mode` value used in glTF JSON.
    pub fn as_u8(self) -> u8 {
        match self {
            GltfPrimitiveMode::Points        => 0,
            GltfPrimitiveMode::Lines         => 1,
            GltfPrimitiveMode::LineLoop      => 2,
            GltfPrimitiveMode::LineStrip     => 3,
            GltfPrimitiveMode::Triangles     => 4,
            GltfPrimitiveMode::TriangleStrip => 5,
            GltfPrimitiveMode::TriangleFan   => 6,
        }
    }

    /// Parse a numeric `mode` value into a [`GltfPrimitiveMode`].
    /// Returns `None` for unrecognised values.
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(GltfPrimitiveMode::Points),
            1 => Some(GltfPrimitiveMode::Lines),
            2 => Some(GltfPrimitiveMode::LineLoop),
            3 => Some(GltfPrimitiveMode::LineStrip),
            4 => Some(GltfPrimitiveMode::Triangles),
            5 => Some(GltfPrimitiveMode::TriangleStrip),
            6 => Some(GltfPrimitiveMode::TriangleFan),
            _ => None,
        }
    }
}

impl Default for GltfPrimitiveMode {
    fn default() -> Self {
        GltfPrimitiveMode::Triangles
    }
}

impl std::fmt::Display for GltfPrimitiveMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            GltfPrimitiveMode::Points        => "POINTS",
            GltfPrimitiveMode::Lines         => "LINES",
            GltfPrimitiveMode::LineLoop      => "LINE_LOOP",
            GltfPrimitiveMode::LineStrip     => "LINE_STRIP",
            GltfPrimitiveMode::Triangles     => "TRIANGLES",
            GltfPrimitiveMode::TriangleStrip => "TRIANGLE_STRIP",
            GltfPrimitiveMode::TriangleFan   => "TRIANGLE_FAN",
        };
        f.write_str(name)
    }
}

// ---------------------------------------------------------------------------
// GltfMeshPrimitive
// ---------------------------------------------------------------------------

/// A single mesh primitive — the fundamental drawing unit in a glTF mesh.
///
/// Each primitive has a rendering mode, a set of accessor indices referencing
/// typed buffer views, and an optional name.  Accessor indices are 0-based
/// indices into the `accessors` array of the glTF JSON document; `u32::MAX`
/// is used as a sentinel for "not set".
// occt: RWGltf_GltfLatePrimitiveArray
#[derive(Debug, Clone)]
pub struct GltfMeshPrimitive {
    /// Human-readable name for this primitive (may be empty).
    name: String,
    /// Rendering topology for this primitive.
    mode: GltfPrimitiveMode,
    /// Index of the accessor supplying element indices (`indices` attribute).
    /// `u32::MAX` when no index accessor has been assigned.
    index_accessor: u32,
    /// Index of the accessor supplying vertex positions (`POSITION` attribute).
    /// `u32::MAX` when not assigned.
    position_accessor: u32,
    /// Index of the accessor supplying vertex normals (`NORMAL` attribute).
    /// `u32::MAX` when not assigned.
    normal_accessor: u32,
}

impl GltfMeshPrimitive {
    /// Sentinel value meaning "accessor not set".
    pub const UNSET: u32 = u32::MAX;

    /// Create a new primitive with the given name and rendering mode.
    /// All accessor indices start as [`Self::UNSET`].
    pub fn new(name: impl Into<String>, mode: GltfPrimitiveMode) -> Self {
        Self {
            name: name.into(),
            mode,
            index_accessor:    Self::UNSET,
            position_accessor: Self::UNSET,
            normal_accessor:   Self::UNSET,
        }
    }

    /// Return the primitive's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the primitive's rendering mode.
    pub fn mode(&self) -> GltfPrimitiveMode {
        self.mode
    }

    /// Set the accessor index for element indices.
    pub fn set_index_accessor(&mut self, idx: u32) {
        self.index_accessor = idx;
    }

    /// Return the accessor index for element indices, or [`Self::UNSET`] if
    /// none has been assigned.
    pub fn index_accessor(&self) -> u32 {
        self.index_accessor
    }

    /// Set the accessor index for vertex positions.
    pub fn set_position_accessor(&mut self, idx: u32) {
        self.position_accessor = idx;
    }

    /// Return the accessor index for vertex positions, or [`Self::UNSET`].
    pub fn position_accessor(&self) -> u32 {
        self.position_accessor
    }

    /// Set the accessor index for vertex normals.
    pub fn set_normal_accessor(&mut self, idx: u32) {
        self.normal_accessor = idx;
    }

    /// Return the accessor index for vertex normals, or [`Self::UNSET`].
    pub fn normal_accessor(&self) -> u32 {
        self.normal_accessor
    }

    /// Return `true` when the index accessor has been assigned.
    pub fn has_index_accessor(&self) -> bool {
        self.index_accessor != Self::UNSET
    }

    /// Return `true` when the position accessor has been assigned.
    pub fn has_position_accessor(&self) -> bool {
        self.position_accessor != Self::UNSET
    }

    /// Return `true` when the normal accessor has been assigned.
    pub fn has_normal_accessor(&self) -> bool {
        self.normal_accessor != Self::UNSET
    }
}

// ---------------------------------------------------------------------------
// RwGltfWriter
// ---------------------------------------------------------------------------

/// glTF file writer stub.
///
/// Accumulates [`GltfMeshPrimitive`] instances and tracks whether the output
/// format is binary glTF (`.glb`) or plain JSON glTF (`.gltf`).  Serialisation
/// to bytes is left to a higher-level layer; this type manages only the
/// in-memory model.
// occt: RWGltf_CafWriter
#[derive(Debug)]
pub struct RwGltfWriter {
    /// Number of mesh primitives that have been added.
    nb_meshes: usize,
    /// `true` for binary glTF (`.glb`), `false` for JSON glTF (`.gltf`).
    is_binary: bool,
    /// Accumulated mesh primitives in insertion order.
    primitives: Vec<GltfMeshPrimitive>,
}

impl RwGltfWriter {
    /// Create a new writer.
    ///
    /// Set `is_binary` to `true` to target binary glTF (`.glb`) output, or
    /// `false` to target plain JSON glTF (`.gltf`) output.
    pub fn new(is_binary: bool) -> Self {
        Self {
            nb_meshes: 0,
            is_binary,
            primitives: Vec::new(),
        }
    }

    /// Set or clear the binary output flag.
    pub fn set_binary(&mut self, binary: bool) {
        self.is_binary = binary;
    }

    /// Return `true` when binary glTF (`.glb`) output is selected.
    pub fn is_binary(&self) -> bool {
        self.is_binary
    }

    /// Return the number of mesh primitives that have been added.
    pub fn nb_meshes(&self) -> usize {
        self.nb_meshes
    }

    /// Add a [`GltfMeshPrimitive`] to the writer.
    ///
    /// Each call increments [`nb_meshes`](Self::nb_meshes) by one.
    pub fn add_mesh(&mut self, prim: GltfMeshPrimitive) {
        self.primitives.push(prim);
        self.nb_meshes += 1;
    }

    /// Return a slice of all accumulated primitives.
    pub fn primitives(&self) -> &[GltfMeshPrimitive] {
        &self.primitives
    }

    /// Return the primitive at the given 0-based index, or `None` if out of
    /// range.
    pub fn primitive(&self, idx: usize) -> Option<&GltfMeshPrimitive> {
        self.primitives.get(idx)
    }

    /// Remove all accumulated primitives and reset the mesh counter.
    pub fn clear(&mut self) {
        self.primitives.clear();
        self.nb_meshes = 0;
    }
}

impl Default for RwGltfWriter {
    fn default() -> Self {
        Self::new(false)
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- GltfAccessorLayout ---

    #[test]
    fn accessor_layout_as_str() {
        assert_eq!(GltfAccessorLayout::Scalar.as_str(), "SCALAR");
        assert_eq!(GltfAccessorLayout::Vec3.as_str(),   "VEC3");
        assert_eq!(GltfAccessorLayout::Mat4.as_str(),   "MAT4");
    }

    #[test]
    fn accessor_layout_nb_components() {
        assert_eq!(GltfAccessorLayout::Scalar.nb_components(), 1);
        assert_eq!(GltfAccessorLayout::Vec2.nb_components(),   2);
        assert_eq!(GltfAccessorLayout::Vec3.nb_components(),   3);
        assert_eq!(GltfAccessorLayout::Vec4.nb_components(),   4);
        assert_eq!(GltfAccessorLayout::Mat2.nb_components(),   4);
        assert_eq!(GltfAccessorLayout::Mat3.nb_components(),   9);
        assert_eq!(GltfAccessorLayout::Mat4.nb_components(),   16);
    }

    #[test]
    fn accessor_layout_display() {
        assert_eq!(GltfAccessorLayout::Vec2.to_string(), "VEC2");
    }

    // --- GltfPrimitiveMode ---

    #[test]
    fn primitive_mode_as_u8_roundtrip() {
        for v in 0u8..=6 {
            let mode = GltfPrimitiveMode::from_u8(v).expect("valid mode");
            assert_eq!(mode.as_u8(), v);
        }
    }

    #[test]
    fn primitive_mode_from_u8_invalid() {
        assert!(GltfPrimitiveMode::from_u8(7).is_none());
        assert!(GltfPrimitiveMode::from_u8(255).is_none());
    }

    #[test]
    fn primitive_mode_default_is_triangles() {
        assert_eq!(GltfPrimitiveMode::default(), GltfPrimitiveMode::Triangles);
        assert_eq!(GltfPrimitiveMode::default().as_u8(), 4);
    }

    #[test]
    fn primitive_mode_display() {
        assert_eq!(GltfPrimitiveMode::Triangles.to_string(),     "TRIANGLES");
        assert_eq!(GltfPrimitiveMode::TriangleStrip.to_string(), "TRIANGLE_STRIP");
        assert_eq!(GltfPrimitiveMode::LineLoop.to_string(),      "LINE_LOOP");
    }

    // --- GltfMeshPrimitive ---

    #[test]
    fn mesh_primitive_new_accessors_are_unset() {
        let prim = GltfMeshPrimitive::new("mesh0", GltfPrimitiveMode::Triangles);
        assert_eq!(prim.name(), "mesh0");
        assert_eq!(prim.mode(), GltfPrimitiveMode::Triangles);
        assert_eq!(prim.index_accessor(),    GltfMeshPrimitive::UNSET);
        assert_eq!(prim.position_accessor(), GltfMeshPrimitive::UNSET);
        assert_eq!(prim.normal_accessor(),   GltfMeshPrimitive::UNSET);
        assert!(!prim.has_index_accessor());
        assert!(!prim.has_position_accessor());
        assert!(!prim.has_normal_accessor());
    }

    #[test]
    fn mesh_primitive_set_accessors() {
        let mut prim = GltfMeshPrimitive::new("tri", GltfPrimitiveMode::Triangles);
        prim.set_index_accessor(0);
        prim.set_position_accessor(1);
        prim.set_normal_accessor(2);
        assert_eq!(prim.index_accessor(),    0);
        assert_eq!(prim.position_accessor(), 1);
        assert_eq!(prim.normal_accessor(),   2);
        assert!(prim.has_index_accessor());
        assert!(prim.has_position_accessor());
        assert!(prim.has_normal_accessor());
    }

    #[test]
    fn mesh_primitive_name_and_mode() {
        let prim = GltfMeshPrimitive::new("points_prim", GltfPrimitiveMode::Points);
        assert_eq!(prim.name(), "points_prim");
        assert_eq!(prim.mode(), GltfPrimitiveMode::Points);
        assert_eq!(prim.mode().as_u8(), 0);
    }

    // --- RwGltfWriter ---

    #[test]
    fn writer_new_is_empty() {
        let w = RwGltfWriter::new(false);
        assert!(!w.is_binary());
        assert_eq!(w.nb_meshes(), 0);
        assert!(w.primitives().is_empty());
    }

    #[test]
    fn writer_new_binary() {
        let w = RwGltfWriter::new(true);
        assert!(w.is_binary());
    }

    #[test]
    fn writer_set_binary() {
        let mut w = RwGltfWriter::new(false);
        assert!(!w.is_binary());
        w.set_binary(true);
        assert!(w.is_binary());
        w.set_binary(false);
        assert!(!w.is_binary());
    }

    #[test]
    fn writer_add_mesh_increments_count() {
        let mut w = RwGltfWriter::new(false);
        w.add_mesh(GltfMeshPrimitive::new("a", GltfPrimitiveMode::Triangles));
        assert_eq!(w.nb_meshes(), 1);
        w.add_mesh(GltfMeshPrimitive::new("b", GltfPrimitiveMode::Lines));
        assert_eq!(w.nb_meshes(), 2);
    }

    #[test]
    fn writer_primitive_by_index() {
        let mut w = RwGltfWriter::new(false);
        w.add_mesh(GltfMeshPrimitive::new("first",  GltfPrimitiveMode::Triangles));
        w.add_mesh(GltfMeshPrimitive::new("second", GltfPrimitiveMode::Points));
        assert_eq!(w.primitive(0).unwrap().name(), "first");
        assert_eq!(w.primitive(1).unwrap().name(), "second");
        assert!(w.primitive(2).is_none());
    }

    #[test]
    fn writer_clear_resets_state() {
        let mut w = RwGltfWriter::new(true);
        w.add_mesh(GltfMeshPrimitive::new("x", GltfPrimitiveMode::Triangles));
        assert_eq!(w.nb_meshes(), 1);
        w.clear();
        assert_eq!(w.nb_meshes(), 0);
        assert!(w.primitives().is_empty());
        // is_binary flag is preserved across clear
        assert!(w.is_binary());
    }

    #[test]
    fn writer_default_is_json() {
        let w = RwGltfWriter::default();
        assert!(!w.is_binary());
        assert_eq!(w.nb_meshes(), 0);
    }

    #[test]
    fn writer_add_mesh_with_accessors() {
        let mut w = RwGltfWriter::new(false);
        let mut prim = GltfMeshPrimitive::new("indexed_tri", GltfPrimitiveMode::Triangles);
        prim.set_index_accessor(0);
        prim.set_position_accessor(1);
        prim.set_normal_accessor(2);
        w.add_mesh(prim);
        assert_eq!(w.nb_meshes(), 1);
        let stored = w.primitive(0).unwrap();
        assert_eq!(stored.index_accessor(),    0);
        assert_eq!(stored.position_accessor(), 1);
        assert_eq!(stored.normal_accessor(),   2);
    }
}
