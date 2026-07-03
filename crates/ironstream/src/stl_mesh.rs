// FILE: rust/ironstream/crates/ironstream/src/stl_mesh.rs
//! STL mesh types — triangle-soup containers used when reading or writing
//! ASCII / binary STL files.  These types sit one level above the
//! tessellation-and-CSG stack (`mesh.rs`, `bsp.rs`) and are the direct Rust
//! equivalent of the OCCT `StlMesh` package.
//!
//! The three OCCT classes reproduced here are:
//!
//! | OCCT                    | IronStream         |
//! |-------------------------|--------------------|
//! | `StlMesh_TriangleFace`  | [`StlTriangle`]    |
//! | `StlMesh_MeshDomain`    | [`StlMeshDomain`]  |
//! | `StlMesh_Mesh`          | [`StlMesh`]        |
//!
//! Free functions [`write_ascii_stl`] and [`parse_ascii_stl`] handle ASCII STL
//! serialisation and deserialisation with no external dependencies.

// ─── StlTriangle ────────────────────────────────────────────────────────────

/// A single STL triangle: one face normal and three vertex positions, all in
/// single-precision floating point as the STL specification requires.
///
// occt: StlMesh_TriangleFace
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StlTriangle {
    normal: [f32; 3],
    v0: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
}

impl StlTriangle {
    /// Construct a triangle from a face normal and three vertex positions.
    pub fn new(normal: [f32; 3], v0: [f32; 3], v1: [f32; 3], v2: [f32; 3]) -> Self {
        Self { normal, v0, v1, v2 }
    }

    /// Return the face normal.
    pub fn normal(&self) -> [f32; 3] {
        self.normal
    }

    /// Return the three vertex positions `(v0, v1, v2)`.
    pub fn vertices(&self) -> ([f32; 3], [f32; 3], [f32; 3]) {
        (self.v0, self.v1, self.v2)
    }
}

// ─── StlMeshDomain ──────────────────────────────────────────────────────────

/// An ordered collection of [`StlTriangle`]s that together form one coherent
/// surface patch (domain) within an [`StlMesh`].
///
// occt: StlMesh_MeshDomain
#[derive(Clone, Debug, Default)]
pub struct StlMeshDomain {
    triangles: Vec<StlTriangle>,
}

impl StlMeshDomain {
    /// Create an empty domain.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a triangle to this domain.
    pub fn add_triangle(&mut self, t: StlTriangle) {
        self.triangles.push(t);
    }

    /// Return the number of triangles in this domain.
    pub fn nb_triangles(&self) -> usize {
        self.triangles.len()
    }

    /// Return a reference to the triangle at index `i`.
    ///
    /// # Panics
    ///
    /// Panics if `i >= nb_triangles()`.
    pub fn triangle(&self, i: usize) -> &StlTriangle {
        &self.triangles[i]
    }

    /// Remove all triangles from this domain.
    pub fn clear(&mut self) {
        self.triangles.clear();
    }
}

// ─── StlMesh ────────────────────────────────────────────────────────────────

/// A named STL mesh composed of one or more [`StlMeshDomain`]s.
///
/// In OCCT a mesh may be split into domains (one per shell in the original
/// solid), though most STL files contain exactly one domain.
///
// occt: StlMesh_Mesh
#[derive(Clone, Debug)]
pub struct StlMesh {
    name: String,
    domains: Vec<StlMeshDomain>,
}

impl StlMesh {
    /// Create an empty mesh with the given solid name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            domains: Vec::new(),
        }
    }

    /// Return the solid name embedded in the STL header.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Append a domain.
    pub fn add_domain(&mut self, d: StlMeshDomain) {
        self.domains.push(d);
    }

    /// Return the number of domains.
    pub fn nb_domains(&self) -> usize {
        self.domains.len()
    }

    /// Return a reference to domain at index `i`.
    ///
    /// # Panics
    ///
    /// Panics if `i >= nb_domains()`.
    pub fn domain(&self, i: usize) -> &StlMeshDomain {
        &self.domains[i]
    }

    /// Return the total number of triangles across all domains.
    pub fn nb_triangles(&self) -> usize {
        self.domains.iter().map(|d| d.nb_triangles()).sum()
    }
}

// ─── ASCII STL writer ────────────────────────────────────────────────────────

/// Serialise `mesh` to an ASCII STL string.
///
/// The output follows the standard grammar:
///
/// ```text
/// solid <name>
///   facet normal <nx> <ny> <nz>
///     outer loop
///       vertex <x> <y> <z>
///       vertex <x> <y> <z>
///       vertex <x> <y> <z>
///     endloop
///   endfacet
/// endsolid <name>
/// ```
///
/// All floating-point values are written with full round-trip precision using
/// Rust's default `f32` `Display` formatter.
pub fn write_ascii_stl(mesh: &StlMesh) -> String {
    let mut out = String::new();
    let name = mesh.name();
    out.push_str(&format!("solid {name}\n"));
    for d in 0..mesh.nb_domains() {
        let domain = mesh.domain(d);
        for i in 0..domain.nb_triangles() {
            let tri = domain.triangle(i);
            let [nx, ny, nz] = tri.normal();
            let (v0, v1, v2) = tri.vertices();
            out.push_str(&format!(
                "  facet normal {nx} {ny} {nz}\n    outer loop\n"
            ));
            for v in [v0, v1, v2] {
                out.push_str(&format!("      vertex {} {} {}\n", v[0], v[1], v[2]));
            }
            out.push_str("    endloop\n  endfacet\n");
        }
    }
    out.push_str(&format!("endsolid {name}\n"));
    out
}

// ─── ASCII STL parser ────────────────────────────────────────────────────────

/// Parse an ASCII STL string into an [`StlMesh`].
///
/// Returns `Err(String)` with a human-readable message on any parse failure.
/// All triangles are placed into a single [`StlMeshDomain`].
///
/// The parser is deliberately lenient about extra whitespace and case, matching
/// the behaviour of most real-world STL readers.
pub fn parse_ascii_stl(data: &str) -> Result<StlMesh, String> {
    let mut tokens = data.split_ascii_whitespace().peekable();

    // ── "solid [name]" ──
    match tokens.next() {
        Some(t) if t.eq_ignore_ascii_case("solid") => {}
        Some(t) => return Err(format!("expected 'solid', got '{t}'")),
        None => return Err("empty STL input".to_owned()),
    }
    // Everything on the rest of the `solid` line is the name; since we are
    // tokenising on whitespace we rebuild it from the peeked tokens until we
    // hit "facet" or "endsolid".
    let mut name_parts: Vec<&str> = Vec::new();
    while let Some(&next) = tokens.peek() {
        if next.eq_ignore_ascii_case("facet") || next.eq_ignore_ascii_case("endsolid") {
            break;
        }
        name_parts.push(tokens.next().unwrap());
    }
    let name = name_parts.join(" ");

    let mut mesh = StlMesh::new(&name);
    let mut domain = StlMeshDomain::new();

    // ── triangles ──
    loop {
        match tokens.next().as_deref() {
            Some(t) if t.eq_ignore_ascii_case("endsolid") => {
                // consume optional trailing name tokens
                break;
            }
            Some(t) if t.eq_ignore_ascii_case("facet") => {
                // expect "normal nx ny nz"
                expect_keyword(&mut tokens, "normal")?;
                let nx = next_f32(&mut tokens, "normal x")?;
                let ny = next_f32(&mut tokens, "normal y")?;
                let nz = next_f32(&mut tokens, "normal z")?;

                // "outer loop"
                expect_keyword(&mut tokens, "outer")?;
                expect_keyword(&mut tokens, "loop")?;

                let v0 = parse_vertex(&mut tokens)?;
                let v1 = parse_vertex(&mut tokens)?;
                let v2 = parse_vertex(&mut tokens)?;

                // "endloop"
                expect_keyword(&mut tokens, "endloop")?;
                // "endfacet"
                expect_keyword(&mut tokens, "endfacet")?;

                domain.add_triangle(StlTriangle::new([nx, ny, nz], v0, v1, v2));
            }
            Some(t) => return Err(format!("unexpected token '{t}'")),
            None => return Err("unexpected end of input before 'endsolid'".to_owned()),
        }
    }

    mesh.add_domain(domain);
    Ok(mesh)
}

// ─── parser helpers ─────────────────────────────────────────────────────────

fn expect_keyword<'a>(
    tokens: &mut impl Iterator<Item = &'a str>,
    expected: &str,
) -> Result<(), String> {
    match tokens.next() {
        Some(t) if t.eq_ignore_ascii_case(expected) => Ok(()),
        Some(t) => Err(format!("expected '{expected}', got '{t}'")),
        None => Err(format!("expected '{expected}', got end of input")),
    }
}

fn next_f32<'a>(
    tokens: &mut impl Iterator<Item = &'a str>,
    label: &str,
) -> Result<f32, String> {
    match tokens.next() {
        Some(t) => t
            .parse::<f32>()
            .map_err(|e| format!("invalid f32 for {label}: {e}")),
        None => Err(format!("expected f32 for {label}, got end of input")),
    }
}

fn parse_vertex<'a>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<[f32; 3], String> {
    expect_keyword(tokens, "vertex")?;
    let x = next_f32(tokens, "vertex x")?;
    let y = next_f32(tokens, "vertex y")?;
    let z = next_f32(tokens, "vertex z")?;
    Ok([x, y, z])
}

// ─── unit tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_mesh() -> StlMesh {
        let mut mesh = StlMesh::new("test");
        let mut domain = StlMeshDomain::new();
        domain.add_triangle(StlTriangle::new(
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
        ));
        mesh.add_domain(domain);
        mesh
    }

    #[test]
    fn triangle_accessors() {
        let t = StlTriangle::new([1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert_eq!(t.normal(), [1.0, 0.0, 0.0]);
        let (v0, v1, v2) = t.vertices();
        assert_eq!(v0, [0.0, 0.0, 0.0]);
        assert_eq!(v1, [1.0, 0.0, 0.0]);
        assert_eq!(v2, [0.0, 1.0, 0.0]);
    }

    #[test]
    fn domain_counts() {
        let mut d = StlMeshDomain::new();
        assert_eq!(d.nb_triangles(), 0);
        d.add_triangle(StlTriangle::new([0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
        assert_eq!(d.nb_triangles(), 1);
        d.clear();
        assert_eq!(d.nb_triangles(), 0);
    }

    #[test]
    fn mesh_nb_triangles_sums_domains() {
        let mut mesh = StlMesh::new("multi");
        let mut d1 = StlMeshDomain::new();
        d1.add_triangle(StlTriangle::new([0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
        let mut d2 = StlMeshDomain::new();
        d2.add_triangle(StlTriangle::new([0.0, 0.0, -1.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]));
        d2.add_triangle(StlTriangle::new([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0, 1.0]));
        mesh.add_domain(d1);
        mesh.add_domain(d2);
        assert_eq!(mesh.nb_domains(), 2);
        assert_eq!(mesh.nb_triangles(), 3);
    }

    #[test]
    fn write_then_parse_round_trip() {
        let mesh = simple_mesh();
        let stl = write_ascii_stl(&mesh);
        let parsed = parse_ascii_stl(&stl).expect("parse failed");
        assert_eq!(parsed.name(), mesh.name());
        assert_eq!(parsed.nb_domains(), 1);
        assert_eq!(parsed.nb_triangles(), mesh.nb_triangles());
        let orig_tri = mesh.domain(0).triangle(0);
        let parsed_tri = parsed.domain(0).triangle(0);
        assert_eq!(orig_tri.normal(), parsed_tri.normal());
        assert_eq!(orig_tri.vertices(), parsed_tri.vertices());
    }
}
