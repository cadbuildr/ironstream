// FILE: rust/ironstream/crates/ironstream/src/rw_brep.rs
//! `rw_brep` — BRep read/write parameter and result types, mirroring
//! OpenCascade's `BRepTools` package
//! (`src/ModelingData/TKBRep/BRepTools/`).
//!
//! BRepTools provides utilities for reading and writing BRep shapes, including
//! tessellation control (deflection, angle) and topology counters for
//! validation.  This module provides zero-dependency, pure-Rust stub
//! implementations of the most commonly used types.
//!
//! # OCCT correspondence
//!
//! | OCCT type / function              | IronStream type / function  |
//! |-----------------------------------|-----------------------------|
//! | `BRepTools_Modifier` write params | [`BrepWriteParams`]         |
//! | `BRepTools` read result counts    | [`BrepReadResult`]          |
//! | `BRepTools::Write` header string  | [`write_brep_header`]       |
//! | `BRepTools::Read` header parse    | [`parse_brep_counts`]       |
//! | `BRepTools` namespace             | [`BrepIO`]                  |

// ---------------------------------------------------------------------------
// BrepWriteParams
// ---------------------------------------------------------------------------

/// Parameters controlling how a BRep shape is serialised to a file or stream.
///
/// Mirrors the write-parameter block accepted by `BRepTools::Write` /
/// `BRepTools_Modifier` in OCCT.  Default values match OCCT defaults:
/// no triangulation, absolute deflection 0.001, angle tolerance 0.5 rad.
// occt-ref: BRepTools // write params
#[derive(Debug, Clone)]
pub struct BrepWriteParams {
    /// Whether to include triangulation data in the output.
    // occt: BRepTools_Modifier // ::SetTriangulation
    triangulate: bool,
    /// Whether the deflection value is relative to the bounding-box diagonal
    /// (`true`) or absolute (`false`).
    // occt: BRepTools_Modifier // ::SetRelativeMode
    relative_deflection: bool,
    /// Linear deflection tolerance used when tessellating curved edges/faces.
    // occt: BRepTools_Modifier // ::SetDeflection
    deflection: f64,
    /// Angular deflection tolerance (radians) used when tessellating.
    // occt: BRepTools_Modifier // ::SetAngle
    angle: f64,
}

impl BrepWriteParams {
    /// Create a new [`BrepWriteParams`] with OCCT default values:
    /// `triangulate = false`, `relative_deflection = false`,
    /// `deflection = 0.001`, `angle = 0.5`.
    pub fn new() -> Self {
        Self {
            triangulate: false,
            relative_deflection: false,
            deflection: 0.001,
            angle: 0.5,
        }
    }

    /// Enable or disable triangulation output.
    pub fn set_triangulate(&mut self, triangulate: bool) {
        self.triangulate = triangulate;
    }

    /// Return `true` when triangulation output is enabled.
    pub fn triangulate(&self) -> bool {
        self.triangulate
    }

    /// Enable or disable relative deflection mode.
    pub fn set_relative_deflection(&mut self, relative: bool) {
        self.relative_deflection = relative;
    }

    /// Return `true` when the deflection value is interpreted as relative to
    /// the bounding-box diagonal.
    pub fn relative_deflection(&self) -> bool {
        self.relative_deflection
    }

    /// Set the linear deflection tolerance.
    pub fn set_deflection(&mut self, deflection: f64) {
        self.deflection = deflection;
    }

    /// Return the linear deflection tolerance.
    pub fn deflection(&self) -> f64 {
        self.deflection
    }

    /// Set the angular deflection tolerance in radians.
    pub fn set_angle(&mut self, angle: f64) {
        self.angle = angle;
    }

    /// Return the angular deflection tolerance in radians.
    pub fn angle(&self) -> f64 {
        self.angle
    }
}

impl Default for BrepWriteParams {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// BrepReadResult
// ---------------------------------------------------------------------------

/// Topology counters and validity flag returned after reading a BRep file.
///
/// Mirrors the information extracted by `BRepTools::Read` in OCCT when it
/// parses a `.brep` file header and topology section.
// occt-ref: BRepTools // read result
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrepReadResult {
    /// Number of vertex sub-shapes found.
    // occt: BRep_Builder // vertex count
    nb_vertices: usize,
    /// Number of edge sub-shapes found.
    // occt: BRep_Builder // edge count
    nb_edges: usize,
    /// Number of face sub-shapes found.
    // occt: BRep_Builder // face count
    nb_faces: usize,
    /// Number of shell sub-shapes found.
    // occt: BRep_Builder // shell count
    nb_shells: usize,
    /// Number of solid sub-shapes found.
    // occt: BRep_Builder // solid count
    nb_solids: usize,
    /// Whether the shape passed a basic validity check after reading.
    // occt-ref: BRepCheck_Analyzer // ::IsValid
    is_valid: bool,
}

impl BrepReadResult {
    /// Create a new [`BrepReadResult`] with all counters at zero and
    /// `is_valid = false`.
    pub fn new() -> Self {
        Self {
            nb_vertices: 0,
            nb_edges: 0,
            nb_faces: 0,
            nb_shells: 0,
            nb_solids: 0,
            is_valid: false,
        }
    }

    /// Set all topology counters at once.
    ///
    /// Arguments: vertices `v`, edges `e`, faces `f`, shells `sh`, solids `so`.
    pub fn set_counts(&mut self, v: usize, e: usize, f: usize, sh: usize, so: usize) {
        self.nb_vertices = v;
        self.nb_edges = e;
        self.nb_faces = f;
        self.nb_shells = sh;
        self.nb_solids = so;
    }

    /// Return `true` when the shape passed validity checks.
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    /// Set the validity flag.
    pub fn set_valid(&mut self, valid: bool) {
        self.is_valid = valid;
    }

    /// Return the number of vertices.
    pub fn nb_vertices(&self) -> usize {
        self.nb_vertices
    }

    /// Return the number of edges.
    pub fn nb_edges(&self) -> usize {
        self.nb_edges
    }

    /// Return the number of faces.
    pub fn nb_faces(&self) -> usize {
        self.nb_faces
    }

    /// Return the number of shells.
    pub fn nb_shells(&self) -> usize {
        self.nb_shells
    }

    /// Return the number of solids.
    pub fn nb_solids(&self) -> usize {
        self.nb_solids
    }
}

impl Default for BrepReadResult {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// write_brep_header
// ---------------------------------------------------------------------------

/// Produce a BRep header string encoding the write parameters.
///
/// The output has the form:
/// ```text
/// CASCADE Topology V1, (c) Matra-Datavision
/// deflection=<deflection> angle=<angle> triangulate=<0|1> relative=<0|1>
/// ```
///
/// The first line is the standard OCCT BRep file magic string; the second
/// encodes the parameter values so that a reader can reconstruct them.
// occt-ref: BRepTools // ::Write — file header section
pub fn write_brep_header(params: &BrepWriteParams) -> String {
    format!(
        "CASCADE Topology V1, (c) Matra-Datavision\ndeflection={deflection} angle={angle} triangulate={tri} relative={rel}\nV={v} E={e} F={f} SH={sh} SO={so}",
        deflection = params.deflection(),
        angle      = params.angle(),
        tri        = if params.triangulate() { 1 } else { 0 },
        rel        = if params.relative_deflection() { 1 } else { 0 },
        v  = 0,
        e  = 0,
        f  = 0,
        sh = 0,
        so = 0,
    )
}

// ---------------------------------------------------------------------------
// parse_brep_counts
// ---------------------------------------------------------------------------

/// Parse a BRep header string and extract topology counters.
///
/// Looks for a line containing `V=<n> E=<n> F=<n> SH=<n> SO=<n>` (in that
/// order, space-separated) and returns a [`BrepReadResult`] populated with
/// those counts.  Returns `None` when no such line is found or when any
/// counter fails to parse.
///
/// The validity flag is set to `true` when all five counters are present and
/// at least one of vertices, edges, or faces is non-zero.
// occt-ref: BRepTools // ::Read — header parse / topology count extraction
pub fn parse_brep_counts(header: &str) -> Option<BrepReadResult> {
    for line in header.lines() {
        // Require all five tokens on the same line.
        if let (Some(v), Some(e), Some(f), Some(sh), Some(so)) = (
            parse_tag(line, "V="),
            parse_tag(line, "E="),
            parse_tag(line, "F="),
            parse_tag(line, "SH="),
            parse_tag(line, "SO="),
        ) {
            let mut result = BrepReadResult::new();
            result.set_counts(v, e, f, sh, so);
            // Consider the shape valid when there is at least one topological
            // entity at the vertex, edge, or face level.
            result.set_valid(v > 0 || e > 0 || f > 0);
            return Some(result);
        }
    }
    None
}

/// Extract the `usize` value after `tag` within `s`.
///
/// Matches `<tag><digits>` where the digits end at a space, end-of-string, or
/// any non-digit character that is not part of the number.
fn parse_tag(s: &str, tag: &str) -> Option<usize> {
    let start = s.find(tag)? + tag.len();
    let rest = &s[start..];
    let end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    if end == 0 {
        return None;
    }
    rest[..end].parse::<usize>().ok()
}

// ---------------------------------------------------------------------------
// BrepIO
// ---------------------------------------------------------------------------

/// High-level BRep I/O facade, mirroring the `BRepTools` namespace in OCCT.
///
/// Holds a set of [`BrepWriteParams`] and exposes convenience methods for
/// generating BRep file headers and parsing them back into
/// [`BrepReadResult`] values.
// occt-ref: BRepTools // namespace
#[derive(Debug, Clone)]
pub struct BrepIO {
    /// Write parameters used when generating BRep headers.
    // occt-ref: BRepTools // write-parameter block
    pub write_params: BrepWriteParams,
}

impl BrepIO {
    /// Create a new [`BrepIO`] with default write parameters.
    pub fn new() -> Self {
        Self {
            write_params: BrepWriteParams::new(),
        }
    }

    /// Generate a BRep file header string from the current write parameters.
    ///
    /// Delegates to [`write_brep_header`].
    pub fn write_header(&self) -> String {
        write_brep_header(&self.write_params)
    }

    /// Parse a BRep header string and return topology counters.
    ///
    /// Delegates to [`parse_brep_counts`].
    pub fn read_header(&self, s: &str) -> Option<BrepReadResult> {
        parse_brep_counts(s)
    }
}

impl Default for BrepIO {
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

    // --- BrepWriteParams ---

    #[test]
    fn write_params_new_defaults() {
        let p = BrepWriteParams::new();
        assert!(!p.triangulate());
        assert!(!p.relative_deflection());
        assert_eq!(p.deflection(), 0.001);
        assert_eq!(p.angle(), 0.5);
    }

    #[test]
    fn write_params_set_triangulate() {
        let mut p = BrepWriteParams::new();
        p.set_triangulate(true);
        assert!(p.triangulate());
        p.set_triangulate(false);
        assert!(!p.triangulate());
    }

    #[test]
    fn write_params_set_deflection() {
        let mut p = BrepWriteParams::new();
        p.set_deflection(0.05);
        assert_eq!(p.deflection(), 0.05);
    }

    #[test]
    fn write_params_set_angle() {
        let mut p = BrepWriteParams::new();
        p.set_angle(1.0);
        assert_eq!(p.angle(), 1.0);
    }

    #[test]
    fn write_params_set_relative_deflection() {
        let mut p = BrepWriteParams::new();
        p.set_relative_deflection(true);
        assert!(p.relative_deflection());
    }

    #[test]
    fn write_params_default_equals_new() {
        let a = BrepWriteParams::new();
        let b = BrepWriteParams::default();
        assert_eq!(a.deflection(), b.deflection());
        assert_eq!(a.angle(), b.angle());
        assert_eq!(a.triangulate(), b.triangulate());
        assert_eq!(a.relative_deflection(), b.relative_deflection());
    }

    // --- BrepReadResult ---

    #[test]
    fn read_result_new_all_zero_invalid() {
        let r = BrepReadResult::new();
        assert_eq!(r.nb_vertices(), 0);
        assert_eq!(r.nb_edges(), 0);
        assert_eq!(r.nb_faces(), 0);
        assert_eq!(r.nb_shells(), 0);
        assert_eq!(r.nb_solids(), 0);
        assert!(!r.is_valid());
    }

    #[test]
    fn read_result_set_counts() {
        let mut r = BrepReadResult::new();
        r.set_counts(8, 12, 6, 1, 1);
        assert_eq!(r.nb_vertices(), 8);
        assert_eq!(r.nb_edges(), 12);
        assert_eq!(r.nb_faces(), 6);
        assert_eq!(r.nb_shells(), 1);
        assert_eq!(r.nb_solids(), 1);
    }

    #[test]
    fn read_result_set_valid() {
        let mut r = BrepReadResult::new();
        r.set_valid(true);
        assert!(r.is_valid());
        r.set_valid(false);
        assert!(!r.is_valid());
    }

    #[test]
    fn read_result_default_equals_new() {
        let a = BrepReadResult::new();
        let b = BrepReadResult::default();
        assert_eq!(a, b);
    }

    // --- write_brep_header ---

    #[test]
    fn write_brep_header_contains_magic() {
        let p = BrepWriteParams::new();
        let h = write_brep_header(&p);
        assert!(h.contains("CASCADE Topology V1"));
    }

    #[test]
    fn write_brep_header_contains_deflection() {
        let mut p = BrepWriteParams::new();
        p.set_deflection(0.042);
        let h = write_brep_header(&p);
        assert!(h.contains("deflection=0.042"), "header: {h}");
    }

    #[test]
    fn write_brep_header_contains_angle() {
        let mut p = BrepWriteParams::new();
        p.set_angle(0.3);
        let h = write_brep_header(&p);
        assert!(h.contains("angle=0.3"), "header: {h}");
    }

    #[test]
    fn write_brep_header_triangulate_flag() {
        let mut p = BrepWriteParams::new();
        p.set_triangulate(true);
        let h = write_brep_header(&p);
        assert!(h.contains("triangulate=1"), "header: {h}");

        p.set_triangulate(false);
        let h2 = write_brep_header(&p);
        assert!(h2.contains("triangulate=0"), "header: {h2}");
    }

    #[test]
    fn write_brep_header_relative_flag() {
        let mut p = BrepWriteParams::new();
        p.set_relative_deflection(true);
        let h = write_brep_header(&p);
        assert!(h.contains("relative=1"), "header: {h}");

        p.set_relative_deflection(false);
        let h2 = write_brep_header(&p);
        assert!(h2.contains("relative=0"), "header: {h2}");
    }

    #[test]
    fn write_brep_header_contains_count_line() {
        let p = BrepWriteParams::new();
        let h = write_brep_header(&p);
        assert!(h.contains("V=0 E=0 F=0 SH=0 SO=0"), "header: {h}");
    }

    // --- parse_brep_counts ---

    #[test]
    fn parse_brep_counts_basic() {
        let header = "CASCADE Topology V1, (c) Matra-Datavision\ndeflection=0.001 angle=0.5 triangulate=0 relative=0\nV=8 E=12 F=6 SH=1 SO=1";
        let r = parse_brep_counts(header).expect("should parse");
        assert_eq!(r.nb_vertices(), 8);
        assert_eq!(r.nb_edges(), 12);
        assert_eq!(r.nb_faces(), 6);
        assert_eq!(r.nb_shells(), 1);
        assert_eq!(r.nb_solids(), 1);
        assert!(r.is_valid());
    }

    #[test]
    fn parse_brep_counts_all_zero_not_valid() {
        let header = "V=0 E=0 F=0 SH=0 SO=0";
        let r = parse_brep_counts(header).expect("should parse");
        assert_eq!(r.nb_vertices(), 0);
        assert!(!r.is_valid());
    }

    #[test]
    fn parse_brep_counts_returns_none_on_missing_line() {
        let header = "CASCADE Topology V1\nno counts here";
        assert!(parse_brep_counts(header).is_none());
    }

    #[test]
    fn parse_brep_counts_roundtrip_via_write_header() {
        let mut p = BrepWriteParams::new();
        p.set_deflection(0.01);
        p.set_angle(0.25);
        // write_brep_header always emits V=0..SO=0; still parseable.
        let h = write_brep_header(&p);
        let r = parse_brep_counts(&h).expect("roundtrip should parse");
        assert_eq!(r.nb_vertices(), 0);
        assert_eq!(r.nb_solids(), 0);
    }

    #[test]
    fn parse_brep_counts_partial_missing_so() {
        // Missing SO= — should return None.
        let header = "V=4 E=6 F=4 SH=1";
        assert!(parse_brep_counts(header).is_none());
    }

    #[test]
    fn parse_brep_counts_valid_when_vertices_nonzero() {
        let header = "V=1 E=0 F=0 SH=0 SO=0";
        let r = parse_brep_counts(header).expect("should parse");
        assert!(r.is_valid());
    }

    #[test]
    fn parse_brep_counts_valid_when_edges_nonzero() {
        let header = "V=0 E=3 F=0 SH=0 SO=0";
        let r = parse_brep_counts(header).expect("should parse");
        assert!(r.is_valid());
    }

    #[test]
    fn parse_brep_counts_valid_when_faces_nonzero() {
        let header = "V=0 E=0 F=1 SH=0 SO=0";
        let r = parse_brep_counts(header).expect("should parse");
        assert!(r.is_valid());
    }

    // --- BrepIO ---

    #[test]
    fn brep_io_new_default_params() {
        let io = BrepIO::new();
        assert_eq!(io.write_params.deflection(), 0.001);
        assert_eq!(io.write_params.angle(), 0.5);
        assert!(!io.write_params.triangulate());
    }

    #[test]
    fn brep_io_write_header_contains_magic() {
        let io = BrepIO::new();
        assert!(io.write_header().contains("CASCADE Topology V1"));
    }

    #[test]
    fn brep_io_read_header_parses_counts() {
        let io = BrepIO::new();
        let s = "V=3 E=3 F=1 SH=1 SO=0";
        let r = io.read_header(s).expect("should parse");
        assert_eq!(r.nb_vertices(), 3);
        assert_eq!(r.nb_edges(), 3);
        assert_eq!(r.nb_faces(), 1);
        assert_eq!(r.nb_shells(), 1);
        assert_eq!(r.nb_solids(), 0);
    }

    #[test]
    fn brep_io_read_header_none_on_empty() {
        let io = BrepIO::new();
        assert!(io.read_header("").is_none());
    }

    #[test]
    fn brep_io_write_then_read_roundtrip() {
        let mut io = BrepIO::new();
        io.write_params.set_deflection(0.005);
        io.write_params.set_angle(0.8);
        let header = io.write_header();
        // The header always contains V=0..SO=0 counts; reading back succeeds.
        let r = io.read_header(&header).expect("roundtrip parse");
        assert_eq!(r.nb_vertices(), 0);
    }

    #[test]
    fn brep_io_default_equals_new() {
        let a = BrepIO::new();
        let b = BrepIO::default();
        assert_eq!(a.write_params.deflection(), b.write_params.deflection());
    }

    #[test]
    fn brep_io_modify_params_via_public_field() {
        let mut io = BrepIO::new();
        io.write_params.set_triangulate(true);
        assert!(io.write_header().contains("triangulate=1"));
    }

    #[test]
    fn brep_io_debug_available() {
        let io = BrepIO::new();
        let _ = format!("{:?}", io);
    }
}
