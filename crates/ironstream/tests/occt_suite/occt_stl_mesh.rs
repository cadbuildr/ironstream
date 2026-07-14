// FILE: rust/ironstream/crates/ironstream/tests/occt_stl_mesh.rs
//! Integration tests for the `stl_mesh` module — faithful ports of the
//! OCCT `StlMesh` package tests.
//!
//! Covers:
//! - `StlTriangle` construction and accessors
//! - `StlMeshDomain` add / count / index / clear
//! - `StlMesh` multi-domain counts
//! - ASCII STL round-trip (write → parse → compare)
//! - Parser error paths
//! - A multi-triangle tetrahedron round-trip

extern crate ironstream;
use ironstream::stl_mesh::*;

// ─── helpers ────────────────────────────────────────────────────────────────

fn f3(x: f32, y: f32, z: f32) -> [f32; 3] {
    [x, y, z]
}

fn make_triangle(nx: f32, ny: f32, nz: f32) -> StlTriangle {
    StlTriangle::new(f3(nx, ny, nz), f3(0.0, 0.0, 0.0), f3(1.0, 0.0, 0.0), f3(0.0, 1.0, 0.0))
}

fn near_f32(a: f32, b: f32, tol: f32) {
    assert!(
        (a - b).abs() <= tol,
        "expected {a} ≈ {b} (tol {tol})"
    );
}

fn assert_vecs_close(a: [f32; 3], b: [f32; 3], tol: f32) {
    for i in 0..3 {
        near_f32(a[i], b[i], tol);
    }
}

// ─── StlTriangle ────────────────────────────────────────────────────────────

#[test]
fn triangle_new_and_normal() {
    let t = StlTriangle::new(f3(0.0, 0.0, 1.0), f3(0.0, 0.0, 0.0), f3(1.0, 0.0, 0.0), f3(0.0, 1.0, 0.0));
    assert_eq!(t.normal(), [0.0, 0.0, 1.0]);
}

#[test]
fn triangle_vertices() {
    let v0 = f3(1.0, 2.0, 3.0);
    let v1 = f3(4.0, 5.0, 6.0);
    let v2 = f3(7.0, 8.0, 9.0);
    let t = StlTriangle::new(f3(0.0, 1.0, 0.0), v0, v1, v2);
    let (r0, r1, r2) = t.vertices();
    assert_eq!(r0, v0);
    assert_eq!(r1, v1);
    assert_eq!(r2, v2);
}

#[test]
fn triangle_copy() {
    let t = make_triangle(0.0, 0.0, 1.0);
    let t2 = t;
    assert_eq!(t2.normal(), [0.0, 0.0, 1.0]);
}

// ─── StlMeshDomain ──────────────────────────────────────────────────────────

#[test]
fn domain_new_is_empty() {
    let d = StlMeshDomain::new();
    assert_eq!(d.nb_triangles(), 0);
}

#[test]
fn domain_add_and_count() {
    let mut d = StlMeshDomain::new();
    d.add_triangle(make_triangle(0.0, 0.0, 1.0));
    d.add_triangle(make_triangle(1.0, 0.0, 0.0));
    assert_eq!(d.nb_triangles(), 2);
}

#[test]
fn domain_triangle_indexing() {
    let mut d = StlMeshDomain::new();
    let t0 = make_triangle(0.0, 0.0, 1.0);
    let t1 = make_triangle(0.0, 1.0, 0.0);
    d.add_triangle(t0);
    d.add_triangle(t1);
    assert_eq!(d.triangle(0).normal(), [0.0, 0.0, 1.0]);
    assert_eq!(d.triangle(1).normal(), [0.0, 1.0, 0.0]);
}

#[test]
fn domain_clear() {
    let mut d = StlMeshDomain::new();
    d.add_triangle(make_triangle(1.0, 0.0, 0.0));
    d.add_triangle(make_triangle(0.0, 1.0, 0.0));
    assert_eq!(d.nb_triangles(), 2);
    d.clear();
    assert_eq!(d.nb_triangles(), 0);
}

// ─── StlMesh ────────────────────────────────────────────────────────────────

#[test]
fn mesh_name() {
    let m = StlMesh::new("MyPart");
    assert_eq!(m.name(), "MyPart");
}

#[test]
fn mesh_new_has_no_domains() {
    let m = StlMesh::new("empty");
    assert_eq!(m.nb_domains(), 0);
    assert_eq!(m.nb_triangles(), 0);
}

#[test]
fn mesh_add_single_domain() {
    let mut m = StlMesh::new("part");
    let mut d = StlMeshDomain::new();
    d.add_triangle(make_triangle(0.0, 0.0, 1.0));
    m.add_domain(d);
    assert_eq!(m.nb_domains(), 1);
    assert_eq!(m.nb_triangles(), 1);
}

#[test]
fn mesh_nb_triangles_sums_all_domains() {
    let mut m = StlMesh::new("multi");
    for count in [3_usize, 5, 7] {
        let mut d = StlMeshDomain::new();
        for _ in 0..count {
            d.add_triangle(make_triangle(0.0, 0.0, 1.0));
        }
        m.add_domain(d);
    }
    assert_eq!(m.nb_domains(), 3);
    assert_eq!(m.nb_triangles(), 15);
}

#[test]
fn mesh_domain_indexing() {
    let mut m = StlMesh::new("idx");
    let mut d0 = StlMeshDomain::new();
    d0.add_triangle(make_triangle(1.0, 0.0, 0.0));
    let mut d1 = StlMeshDomain::new();
    d1.add_triangle(make_triangle(0.0, 1.0, 0.0));
    d1.add_triangle(make_triangle(0.0, 0.0, 1.0));
    m.add_domain(d0);
    m.add_domain(d1);
    assert_eq!(m.domain(0).nb_triangles(), 1);
    assert_eq!(m.domain(1).nb_triangles(), 2);
}

// ─── ASCII STL write ─────────────────────────────────────────────────────────

#[test]
fn write_contains_solid_name() {
    let mut m = StlMesh::new("Widget");
    let mut d = StlMeshDomain::new();
    d.add_triangle(make_triangle(0.0, 0.0, 1.0));
    m.add_domain(d);
    let stl = write_ascii_stl(&m);
    assert!(stl.starts_with("solid Widget\n"), "got: {stl}");
    assert!(stl.trim_end().ends_with("endsolid Widget"), "got: {stl}");
}

#[test]
fn write_contains_facet_keywords() {
    let mut m = StlMesh::new("kw");
    let mut d = StlMeshDomain::new();
    d.add_triangle(StlTriangle::new(f3(0.0, 0.0, 1.0), f3(0.0, 0.0, 0.0), f3(1.0, 0.0, 0.0), f3(0.0, 1.0, 0.0)));
    m.add_domain(d);
    let stl = write_ascii_stl(&m);
    assert!(stl.contains("facet normal"));
    assert!(stl.contains("outer loop"));
    assert!(stl.contains("vertex"));
    assert!(stl.contains("endloop"));
    assert!(stl.contains("endfacet"));
}

#[test]
fn write_empty_mesh_has_no_facets() {
    let m = StlMesh::new("empty");
    let stl = write_ascii_stl(&m);
    assert!(!stl.contains("facet"));
    assert!(stl.contains("solid empty"));
    assert!(stl.contains("endsolid empty"));
}

// ─── ASCII STL round-trip ────────────────────────────────────────────────────

#[test]
fn round_trip_single_triangle() {
    let mut m = StlMesh::new("single");
    let mut d = StlMeshDomain::new();
    let tri = StlTriangle::new(
        f3(0.0, 0.0, 1.0),
        f3(0.0, 0.0, 0.0),
        f3(1.0, 0.0, 0.0),
        f3(0.0, 1.0, 0.0),
    );
    d.add_triangle(tri);
    m.add_domain(d);

    let stl = write_ascii_stl(&m);
    let parsed = parse_ascii_stl(&stl).expect("round-trip parse failed");

    assert_eq!(parsed.name(), "single");
    assert_eq!(parsed.nb_domains(), 1);
    assert_eq!(parsed.nb_triangles(), 1);

    let orig = m.domain(0).triangle(0);
    let got = parsed.domain(0).triangle(0);
    assert_vecs_close(got.normal(), orig.normal(), 1e-6);
    let (ov0, ov1, ov2) = orig.vertices();
    let (gv0, gv1, gv2) = got.vertices();
    assert_vecs_close(gv0, ov0, 1e-6);
    assert_vecs_close(gv1, ov1, 1e-6);
    assert_vecs_close(gv2, ov2, 1e-6);
}

#[test]
fn round_trip_tetrahedron() {
    // Build a tetrahedron (4 faces) manually.
    let a = f3(0.0, 0.0, 0.0);
    let b = f3(1.0, 0.0, 0.0);
    let c = f3(0.5, 1.0, 0.0);
    let d_pt = f3(0.5, 0.5, 1.0);

    let faces: [(([f32; 3], [f32; 3], [f32; 3]), [f32; 3]); 4] = [
        ((a, c, b), f3(0.0, 0.0, -1.0)),
        ((a, b, d_pt), f3(0.0, -1.0, 0.0)),
        ((b, c, d_pt), f3(1.0, 0.0, 0.0)),
        ((a, d_pt, c), f3(-1.0, 0.0, 0.0)),
    ];

    let mut mesh = StlMesh::new("tetrahedron");
    let mut domain = StlMeshDomain::new();
    for ((p0, p1, p2), n) in faces {
        domain.add_triangle(StlTriangle::new(n, p0, p1, p2));
    }
    mesh.add_domain(domain);

    let stl = write_ascii_stl(&mesh);
    let parsed = parse_ascii_stl(&stl).expect("tetrahedron round-trip failed");

    assert_eq!(parsed.name(), "tetrahedron");
    assert_eq!(parsed.nb_triangles(), 4);

    for i in 0..4 {
        let orig = mesh.domain(0).triangle(i);
        let got = parsed.domain(0).triangle(i);
        assert_vecs_close(got.normal(), orig.normal(), 1e-6);
        let (ov0, ov1, ov2) = orig.vertices();
        let (gv0, gv1, gv2) = got.vertices();
        assert_vecs_close(gv0, ov0, 1e-6);
        assert_vecs_close(gv1, ov1, 1e-6);
        assert_vecs_close(gv2, ov2, 1e-6);
    }
}

#[test]
fn round_trip_preserves_fractional_coordinates() {
    let mut m = StlMesh::new("frac");
    let mut d = StlMeshDomain::new();
    d.add_triangle(StlTriangle::new(
        f3(0.57735026, 0.57735026, 0.57735026),
        f3(0.1, 0.2, 0.3),
        f3(1.5, -0.75, 0.0),
        f3(-0.5, 0.5, 2.25),
    ));
    m.add_domain(d);
    let stl = write_ascii_stl(&m);
    let parsed = parse_ascii_stl(&stl).unwrap();
    let orig = m.domain(0).triangle(0);
    let got = parsed.domain(0).triangle(0);
    assert_vecs_close(got.normal(), orig.normal(), 1e-5);
    let (ov0, ov1, ov2) = orig.vertices();
    let (gv0, gv1, gv2) = got.vertices();
    assert_vecs_close(gv0, ov0, 1e-5);
    assert_vecs_close(gv1, ov1, 1e-5);
    assert_vecs_close(gv2, ov2, 1e-5);
}

// ─── Parser error paths ──────────────────────────────────────────────────────

#[test]
fn parse_empty_input_is_error() {
    assert!(parse_ascii_stl("").is_err());
}

#[test]
fn parse_missing_solid_keyword_is_error() {
    assert!(parse_ascii_stl("notsolid name\nendsolid name\n").is_err());
}

#[test]
fn parse_missing_endsolid_is_error() {
    let bad = "solid part\n  facet normal 0 0 1\n    outer loop\n      vertex 0 0 0\n      vertex 1 0 0\n      vertex 0 1 0\n    endloop\n  endfacet\n";
    assert!(parse_ascii_stl(bad).is_err());
}

#[test]
fn parse_bad_float_is_error() {
    let bad = "solid part\n  facet normal abc 0 1\n    outer loop\n      vertex 0 0 0\n      vertex 1 0 0\n      vertex 0 1 0\n    endloop\n  endfacet\nendsolid part\n";
    assert!(parse_ascii_stl(bad).is_err());
}

#[test]
fn parse_handwritten_ascii_stl() {
    let src = "\
solid cube_face
  facet normal 0 0 1
    outer loop
      vertex 0 0 1
      vertex 1 0 1
      vertex 1 1 1
    endloop
  endfacet
  facet normal 0 0 1
    outer loop
      vertex 0 0 1
      vertex 1 1 1
      vertex 0 1 1
    endloop
  endfacet
endsolid cube_face
";
    let m = parse_ascii_stl(src).expect("failed to parse handwritten STL");
    assert_eq!(m.name(), "cube_face");
    assert_eq!(m.nb_triangles(), 2);
    assert_eq!(m.domain(0).triangle(0).normal(), [0.0, 0.0, 1.0]);
    let (v0, _, _) = m.domain(0).triangle(1).vertices();
    assert_eq!(v0, [0.0, 0.0, 1.0]);
}

#[test]
fn parse_solid_with_no_name() {
    // Some exporters omit the solid name entirely.
    let src = "\
solid
  facet normal 0 0 1
    outer loop
      vertex 0 0 0
      vertex 1 0 0
      vertex 0 1 0
    endloop
  endfacet
endsolid
";
    let m = parse_ascii_stl(src).expect("no-name parse failed");
    assert_eq!(m.name(), "");
    assert_eq!(m.nb_triangles(), 1);
}
