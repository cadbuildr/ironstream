// FILE: topo_shape_tools.rs
//! Shape classification and traversal utilities, mirroring the OCCT
//! `BRepTools` and `TopExp_Explorer` oriented surface of the topology API.
//!
//! # Key types
//!
//! * [`ShapeType`]  — enumeration of all topological shape kinds.
//! * [`ShapeInfo`]  — lightweight descriptor carrying a shape's type and name.
//! * [`ShapeTools`] — static helpers for I/O and geometric queries on shapes.
//!
//! # Free functions
//!
//! * [`shape_type_name`] — human-readable name for a [`ShapeType`] variant.
//! * [`map_shapes`]      — collect all sub-shapes of a given type under a root.

use std::fmt;

// ---------------------------------------------------------------------------
// ShapeType
// ---------------------------------------------------------------------------

/// Topological shape kinds, mirroring `TopAbs_ShapeEnum` in OCCT.
///
/// The ordering follows the OCCT convention: from lowest dimensionality
/// (`Vertex`) up through the highest (`Compound`).
// occt: BRepTools
// occt: TopExp
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShapeType {
    /// A zero-dimensional point in space.
    // occt: TopAbs_VERTEX
    Vertex,
    /// A one-dimensional curve bounded by two vertices.
    // occt: TopAbs_EDGE
    Edge,
    /// A closed or open sequence of connected edges.
    // occt: TopAbs_WIRE
    Wire,
    /// A bounded region of a surface.
    // occt: TopAbs_FACE
    Face,
    /// A connected set of faces.
    // occt: TopAbs_SHELL
    Shell,
    /// A region of space bounded by one or more shells.
    // occt: TopAbs_SOLID
    Solid,
    /// A connected set of solids sharing faces.
    // occt: TopAbs_COMPSOLID
    CompSolid,
    /// An arbitrary collection of shapes.
    // occt: TopAbs_COMPOUND
    Compound,
}

impl fmt::Display for ShapeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(shape_type_name(self))
    }
}

// ---------------------------------------------------------------------------
// ShapeInfo
// ---------------------------------------------------------------------------

/// Lightweight descriptor for a topological shape.
///
/// Mirrors the information surface exposed by `TopoDS_Shape` in OCCT when
/// queried through `BRepTools` or `TopExp_Explorer`.
// occt: BRepTools
// occt: TopExp
#[derive(Clone, Debug, PartialEq)]
pub struct ShapeInfo {
    /// The topological kind of this shape.
    pub shape_type: ShapeType,
    /// A human-readable label or identifier for the shape.
    pub name: String,
    /// `true` when the shape carries no geometry (analogous to
    /// `TopoDS_Shape::IsNull()` returning `true` in OCCT).
    pub is_null: bool,
}

impl ShapeInfo {
    /// Construct a non-null [`ShapeInfo`] with the given type and name.
    ///
    /// Mirrors the typical pattern of building a `TopoDS_Shape` and attaching
    /// metadata through `TopExp` utilities.
    // occt: BRepTools
    pub fn new(t: ShapeType, name: &str) -> Self {
        Self {
            shape_type: t,
            name: name.to_string(),
            is_null: false,
        }
    }

    /// Return `true` when the shape forms a topologically closed entity.
    ///
    /// A shape is considered closed when it is a [`ShapeType::Wire`] (closed
    /// loop), [`ShapeType::Shell`] (closed surface), or any higher-dimensional
    /// shape (`Solid`, `CompSolid`, `Compound`) that is inherently bounded.
    ///
    /// Mirrors `BRep_Builder`/`BRepCheck` closure tests in OCCT.
    // occt: BRepTools
    pub fn is_closed(&self) -> bool {
        matches!(
            self.shape_type,
            ShapeType::Wire
                | ShapeType::Shell
                | ShapeType::Solid
                | ShapeType::CompSolid
                | ShapeType::Compound
        )
    }
}

impl fmt::Display for ShapeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ShapeInfo {{ type: {}, name: {:?}, is_null: {} }}",
            self.shape_type, self.name, self.is_null
        )
    }
}

// ---------------------------------------------------------------------------
// ShapeTools
// ---------------------------------------------------------------------------

/// Static utility methods for working with shapes, mirroring the combined
/// surface of `BRepTools` (I/O and geometric queries) and `TopExp` (traversal
/// helpers) from OCCT.
///
/// All methods in this stub operate on opaque string tokens representing shape
/// identifiers.  A full implementation would accept `TopoDS_Shape` references.
// occt: BRepTools
// occt: TopExp
pub struct ShapeTools;

impl ShapeTools {
    /// Return the outer wire label of a face.
    ///
    /// Mirrors `BRepTools::OuterWire(face)`.  The returned string is the
    /// canonical label of the outer bounding wire.  When the face label is
    /// empty the function returns an empty string indicating a null shape.
    // occt: BRepTools
    pub fn outer_wire(face: &str) -> String {
        if face.is_empty() {
            return String::new();
        }
        format!("{face}::OuterWire")
    }

    /// Serialise a shape to a raw byte buffer.
    ///
    /// Mirrors the `BRepTools::Write` / `BinTools::Write` family in OCCT.
    /// The stub encodes the UTF-8 bytes of the shape label preceded by a
    /// 4-byte little-endian length prefix so callers can round-trip through
    /// [`ShapeTools::read_shape`].
    // occt: BRepTools
    pub fn write_shape(shape: &str) -> Vec<u8> {
        let bytes = shape.as_bytes();
        let len = bytes.len() as u32;
        let mut out = Vec::with_capacity(4 + bytes.len());
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(bytes);
        out
    }

    /// Deserialise a shape from a raw byte buffer produced by [`write_shape`].
    ///
    /// Mirrors `BRepTools::Read` / `BinTools::Read`.  Returns an empty string
    /// if the buffer is malformed or too short.
    // occt: BRepTools
    pub fn read_shape(data: &[u8]) -> String {
        if data.len() < 4 {
            return String::new();
        }
        let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        let payload = &data[4..];
        if payload.len() < len {
            return String::new();
        }
        String::from_utf8(payload[..len].to_vec()).unwrap_or_default()
    }

    /// Compute the axis-aligned bounding box of a shape.
    ///
    /// Returns a pair `(min_corner, max_corner)` each as `[x, y, z]`.
    ///
    /// Mirrors `BRepBndLib::Add` / `BRepTools` bounding-box helpers.  The
    /// stub derives deterministic pseudo-bounds from the hash of the shape
    /// label so that callers can exercise the API without real geometry.
    // occt: BRepTools
    pub fn bounds(shape: &str) -> ([f64; 3], [f64; 3]) {
        // Derive a stable seed from the shape label so that tests are
        // reproducible.  A production implementation would walk the topology
        // and accumulate vertex coordinates via BRepBndLib::Add.
        let seed = simple_hash(shape);
        let half: f64 = 1.0 + (seed % 1000) as f64 * 0.001;
        let min = [-half, -half, -half];
        let max = [half, half, half];
        (min, max)
    }
}

// ---------------------------------------------------------------------------
// Free functions
// ---------------------------------------------------------------------------

/// Return the canonical OCCT-style name for a [`ShapeType`] variant.
///
/// Mirrors `TopAbs::ShapeTypeToString` from OCCT.
// occt: TopExp
pub fn shape_type_name(t: &ShapeType) -> &'static str {
    match t {
        ShapeType::Vertex => "Vertex",
        ShapeType::Edge => "Edge",
        ShapeType::Wire => "Wire",
        ShapeType::Face => "Face",
        ShapeType::Shell => "Shell",
        ShapeType::Solid => "Solid",
        ShapeType::CompSolid => "CompSolid",
        ShapeType::Compound => "Compound",
    }
}

/// Collect labels of all sub-shapes of `shape_type` reachable from `root`.
///
/// Mirrors a `TopExp_Explorer` loop:
/// ```text
/// for (TopExp_Explorer ex(root, wanted_type); ex.More(); ex.Next()) { … }
/// ```
///
/// The stub synthesises sub-shape labels from the root label and the
/// requested type so that callers can exercise traversal logic without
/// real topology data.
// occt: TopExp
pub fn map_shapes(root: &str, shape_type: ShapeType) -> Vec<String> {
    if root.is_empty() {
        return Vec::new();
    }
    let kind = shape_type_name(&shape_type);
    // Deterministically produce a small list so tests can assert on counts.
    let count = 1 + (simple_hash(root) % 4) as usize;
    (0..count)
        .map(|i| format!("{root}::{kind}[{i}]"))
        .collect()
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// A minimal, allocation-free hash of a string, used to derive deterministic
/// pseudo-geometry without any external crates.
fn simple_hash(s: &str) -> u64 {
    // FNV-1a 64-bit
    let mut h: u64 = 14695981039346656037;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_type_names_round_trip() {
        let cases = [
            (ShapeType::Vertex, "Vertex"),
            (ShapeType::Edge, "Edge"),
            (ShapeType::Wire, "Wire"),
            (ShapeType::Face, "Face"),
            (ShapeType::Shell, "Shell"),
            (ShapeType::Solid, "Solid"),
            (ShapeType::CompSolid, "CompSolid"),
            (ShapeType::Compound, "Compound"),
        ];
        for (t, expected) in cases {
            assert_eq!(shape_type_name(&t), expected);
            assert_eq!(format!("{t}"), expected);
        }
    }

    #[test]
    fn shape_info_new_is_not_null() {
        let info = ShapeInfo::new(ShapeType::Face, "my_face");
        assert_eq!(info.shape_type, ShapeType::Face);
        assert_eq!(info.name, "my_face");
        assert!(!info.is_null);
    }

    #[test]
    fn shape_info_is_closed() {
        assert!(!ShapeInfo::new(ShapeType::Vertex, "v").is_closed());
        assert!(!ShapeInfo::new(ShapeType::Edge, "e").is_closed());
        assert!(ShapeInfo::new(ShapeType::Wire, "w").is_closed());
        assert!(!ShapeInfo::new(ShapeType::Face, "f").is_closed());
        assert!(ShapeInfo::new(ShapeType::Shell, "sh").is_closed());
        assert!(ShapeInfo::new(ShapeType::Solid, "so").is_closed());
        assert!(ShapeInfo::new(ShapeType::CompSolid, "cs").is_closed());
        assert!(ShapeInfo::new(ShapeType::Compound, "co").is_closed());
    }

    #[test]
    fn outer_wire_appends_suffix() {
        assert_eq!(ShapeTools::outer_wire("face_1"), "face_1::OuterWire");
        assert_eq!(ShapeTools::outer_wire(""), "");
    }

    #[test]
    fn write_read_round_trip() {
        let label = "my_solid";
        let bytes = ShapeTools::write_shape(label);
        assert_eq!(ShapeTools::read_shape(&bytes), label);
    }

    #[test]
    fn read_shape_malformed_returns_empty() {
        assert_eq!(ShapeTools::read_shape(&[]), "");
        assert_eq!(ShapeTools::read_shape(&[0, 0, 0]), "");
        // Length field claims more bytes than available.
        assert_eq!(ShapeTools::read_shape(&[255, 0, 0, 0, 0x41]), "");
    }

    #[test]
    fn bounds_min_less_than_max() {
        let (min, max) = ShapeTools::bounds("cube");
        for i in 0..3 {
            assert!(min[i] < max[i]);
        }
    }

    #[test]
    fn map_shapes_empty_root() {
        assert!(map_shapes("", ShapeType::Edge).is_empty());
    }

    #[test]
    fn map_shapes_nonempty() {
        let shapes = map_shapes("body", ShapeType::Face);
        assert!(!shapes.is_empty());
        for s in &shapes {
            assert!(s.starts_with("body::Face["));
        }
    }
}
