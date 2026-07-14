// FILE: tests/occt_poly.rs
//! Integration tests for `ironstream::poly`, mirroring OpenCascade's `Poly`
//! package: `Poly_Triangle`, `Poly_Triangulation`, `Poly_Array1OfTriangle`,
//! and `Poly_Connect`.

extern crate ironstream;

use ironstream::gp::Pnt;
use ironstream::gp2d::Pnt2d;
use ironstream::poly::{Poly_Array1OfTriangle, Poly_Connect, Poly_Triangle, Poly_Triangulation};

const TOL: f64 = 1e-10;

fn near(a: f64, b: f64, label: &str) {
    assert!(
        (a - b).abs() <= TOL,
        "{label}: expected {a} ≈ {b} (diff {})",
        (a - b).abs()
    );
}

fn pnt_near(a: Pnt, b: Pnt, label: &str) {
    near(a.x, b.x, &format!("{label}.x"));
    near(a.y, b.y, &format!("{label}.y"));
    near(a.z, b.z, &format!("{label}.z"));
}

// ── Helper: build a unit-square mesh (two right-isosceles triangles) ─────────

fn unit_square_mesh() -> Poly_Triangulation {
    //   4──3
    //   │ /│
    //   │/ │
    //   1──2
    // nodes: 1=(0,0,0)  2=(1,0,0)  3=(1,1,0)  4=(0,1,0)
    // tris:  1=[1,2,3]  2=[1,3,4]
    let mut mesh = Poly_Triangulation::new(4, 2);
    mesh.set_node(1, Pnt::new(0.0, 0.0, 0.0));
    mesh.set_node(2, Pnt::new(1.0, 0.0, 0.0));
    mesh.set_node(3, Pnt::new(1.0, 1.0, 0.0));
    mesh.set_node(4, Pnt::new(0.0, 1.0, 0.0));
    mesh.set_triangle(1, Poly_Triangle::from_nodes(1, 2, 3));
    mesh.set_triangle(2, Poly_Triangle::from_nodes(1, 3, 4));
    mesh
}

// ── Helper: build a minimal tetrahedron mesh ─────────────────────────────────

fn tetrahedron_mesh() -> Poly_Triangulation {
    // Nodes at the four corners of a regular-ish tetrahedron.
    // Using coordinates:  1=(0,0,0)  2=(1,0,0)  3=(0.5, 1, 0)  4=(0.5, 0.5, 1)
    // Faces (all outward-pointing by right-hand rule not strictly guaranteed,
    // but areas and connectivity are the focus):
    //   T1=[1,2,3]   T2=[1,2,4]   T3=[2,3,4]   T4=[1,3,4]
    let nodes = [
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.5, 1.0, 0.0),
        Pnt::new(0.5, 0.5, 1.0),
    ];
    let tris = [
        Poly_Triangle::from_nodes(1, 2, 3),
        Poly_Triangle::from_nodes(1, 2, 4),
        Poly_Triangle::from_nodes(2, 3, 4),
        Poly_Triangle::from_nodes(1, 3, 4),
    ];
    Poly_Triangulation::from_slices(&nodes, &tris)
}

// ── Poly_Triangle ─────────────────────────────────────────────────────────────

#[test]
fn triangle_from_nodes_roundtrip() {
    let t = Poly_Triangle::from_nodes(5, 10, 15);
    let (a, b, c) = t.get();
    assert_eq!(a, 5, "n1");
    assert_eq!(b, 10, "n2");
    assert_eq!(c, 15, "n3");
    assert!(t.is_valid());
}

#[test]
fn triangle_set_replaces_all_nodes() {
    let mut t = Poly_Triangle::from_nodes(1, 2, 3);
    t.set(7, 8, 9);
    assert_eq!(t.get(), (7, 8, 9));
}

#[test]
fn triangle_set_value_single_node() {
    let mut t = Poly_Triangle::from_nodes(1, 2, 3);
    t.set_value(2, 42);
    assert_eq!(t.value(1), 1);
    assert_eq!(t.value(2), 42);
    assert_eq!(t.value(3), 3);
}

#[test]
fn triangle_default_is_invalid() {
    let t = Poly_Triangle::new();
    assert!(!t.is_valid(), "default triangle should be invalid");
}

#[test]
#[should_panic]
fn triangle_set_value_out_of_range_panics() {
    let mut t = Poly_Triangle::from_nodes(1, 2, 3);
    t.set_value(4, 1); // index 4 is invalid
}

// ── Poly_Array1OfTriangle ─────────────────────────────────────────────────────

#[test]
fn array1_construction_and_bounds() {
    let arr = Poly_Array1OfTriangle::new(6);
    assert_eq!(arr.lower(), 1);
    assert_eq!(arr.upper(), 6);
    assert_eq!(arr.length(), 6);
    assert!(!arr.is_empty());
}

#[test]
fn array1_set_get_and_iter() {
    let mut arr = Poly_Array1OfTriangle::new(3);
    arr.set_value(1, Poly_Triangle::from_nodes(1, 2, 3));
    arr.set_value(2, Poly_Triangle::from_nodes(4, 5, 6));
    arr.set_value(3, Poly_Triangle::from_nodes(7, 8, 9));

    assert_eq!(arr.value(1).get(), (1, 2, 3));
    assert_eq!(arr.value(3).get(), (7, 8, 9));

    let all: Vec<Poly_Triangle> = arr.iter().cloned().collect();
    assert_eq!(all.len(), 3);
    assert_eq!(all[1].get(), (4, 5, 6));
}

#[test]
fn array1_with_explicit_bounds() {
    let mut arr = Poly_Array1OfTriangle::with_bounds(3, 7);
    assert_eq!(arr.lower(), 3);
    assert_eq!(arr.upper(), 7);
    assert_eq!(arr.length(), 5);
    arr.set_value(5, Poly_Triangle::from_nodes(10, 20, 30));
    assert_eq!(arr.value(5).get(), (10, 20, 30));
}

#[test]
fn array1_iter_indexed_reports_correct_indices() {
    let mut arr = Poly_Array1OfTriangle::with_bounds(2, 4);
    arr.set_value(2, Poly_Triangle::from_nodes(1, 2, 3));
    arr.set_value(3, Poly_Triangle::from_nodes(4, 5, 6));
    arr.set_value(4, Poly_Triangle::from_nodes(7, 8, 9));

    let pairs: Vec<(usize, Poly_Triangle)> = arr.iter_indexed().collect();
    assert_eq!(pairs[0].0, 2);
    assert_eq!(pairs[2].0, 4);
}

// ── Poly_Triangulation ────────────────────────────────────────────────────────

#[test]
fn triangulation_node_and_triangle_counts() {
    let mesh = unit_square_mesh();
    assert_eq!(mesh.nb_nodes(), 4, "nb_nodes");
    assert_eq!(mesh.nb_triangles(), 2, "nb_triangles");
}

#[test]
fn triangulation_node_positions() {
    let mesh = unit_square_mesh();
    pnt_near(mesh.node(1), Pnt::new(0.0, 0.0, 0.0), "node 1");
    pnt_near(mesh.node(3), Pnt::new(1.0, 1.0, 0.0), "node 3");
}

#[test]
fn triangulation_total_area_unit_square() {
    let mesh = unit_square_mesh();
    near(mesh.total_area(), 1.0, "total area");
}

#[test]
fn triangulation_triangle_normal_z_up() {
    let mesh = unit_square_mesh();
    for ti in 1..=2 {
        let n = mesh.triangle_normal(ti);
        near(n.z, 1.0, &format!("tri {ti} normal.z"));
        near(n.x, 0.0, &format!("tri {ti} normal.x"));
        near(n.y, 0.0, &format!("tri {ti} normal.y"));
    }
}

#[test]
fn triangulation_triangle_centroid() {
    let mesh = unit_square_mesh();
    // triangle 1 = (0,0,0), (1,0,0), (1,1,0) → centroid = (2/3, 1/3, 0)
    let c1 = mesh.triangle_centroid(1);
    near(c1.x, 2.0 / 3.0, "tri1 centroid.x");
    near(c1.y, 1.0 / 3.0, "tri1 centroid.y");
    near(c1.z, 0.0, "tri1 centroid.z");
}

#[test]
fn triangulation_bounding_box_unit_square() {
    let mesh = unit_square_mesh();
    let (lo, hi) = mesh.bounding_box();
    near(lo.x, 0.0, "bbox lo.x");
    near(lo.y, 0.0, "bbox lo.y");
    near(hi.x, 1.0, "bbox hi.x");
    near(hi.y, 1.0, "bbox hi.y");
}

#[test]
fn triangulation_uv_nodes_round_trip() {
    let mut mesh = unit_square_mesh();
    mesh.add_uvs();
    assert!(mesh.has_uvs());
    mesh.set_uv_node(2, Pnt2d::new(1.0, 0.0));
    mesh.set_uv_node(4, Pnt2d::new(0.0, 1.0));
    near(mesh.uv_node(2).x, 1.0, "uv node 2 u");
    near(mesh.uv_node(2).y, 0.0, "uv node 2 v");
    near(mesh.uv_node(4).x, 0.0, "uv node 4 u");
    near(mesh.uv_node(4).y, 1.0, "uv node 4 v");
}

#[test]
fn triangulation_per_node_normals() {
    let mut mesh = unit_square_mesh();
    mesh.add_normals();
    assert!(mesh.has_normals());
    let z = Pnt::new(0.0, 0.0, 1.0);
    for i in 1..=4 {
        mesh.set_normal(i, z);
    }
    for i in 1..=4 {
        let n = mesh.normal(i);
        near(n.z, 1.0, &format!("normal[{i}].z"));
    }
}

#[test]
fn triangulation_deflection_set_get() {
    let mut mesh = unit_square_mesh();
    assert_eq!(mesh.deflection(), 0.0);
    mesh.set_deflection(0.01);
    near(mesh.deflection(), 0.01, "deflection");
}

// ── Poly_Connect ──────────────────────────────────────────────────────────────

#[test]
fn connect_node_fan_degrees() {
    let mesh = unit_square_mesh();
    let conn = Poly_Connect::new(&mesh);
    // Interior diagonal nodes 1 and 3 share both triangles
    assert_eq!(conn.degree(1), 2, "node 1 degree");
    assert_eq!(conn.degree(3), 2, "node 3 degree");
    // Corner nodes 2 and 4 appear in exactly one triangle each
    assert_eq!(conn.degree(2), 1, "node 2 degree");
    assert_eq!(conn.degree(4), 1, "node 4 degree");
}

#[test]
fn connect_triangle_adjacency_reciprocal() {
    let mesh = unit_square_mesh();
    let conn = Poly_Connect::new(&mesh);
    let n1 = conn.neighbours(1);
    let n2 = conn.neighbours(2);
    // Triangles 1 and 2 must be mutual neighbours
    assert!(n1.contains(&2), "tri 1 neighbours should contain tri 2: {:?}", n1);
    assert!(n2.contains(&1), "tri 2 neighbours should contain tri 1: {:?}", n2);
}

#[test]
fn connect_boundary_edges_unit_square() {
    let mesh = unit_square_mesh();
    let conn = Poly_Connect::new(&mesh);
    // 2 triangles, each with 3 edges; 1 shared edge → 4 boundary edges total
    let b1 = conn.boundary_edge_count(1);
    let b2 = conn.boundary_edge_count(2);
    assert_eq!(b1 + b2, 4, "total boundary edges should be 4");
}

#[test]
fn connect_averaged_normal_flat() {
    let mesh = unit_square_mesh();
    let conn = Poly_Connect::new(&mesh);
    // Flat mesh in XY → every vertex averaged normal should point +Z
    for v in 1..=4 {
        let n = conn.averaged_normal(v, &mesh);
        near(n.z, 1.0, &format!("avg normal node {v} .z"));
        near(n.x, 0.0, &format!("avg normal node {v} .x"));
    }
}

#[test]
fn connect_tetrahedron_all_edges_interior() {
    // A closed tetrahedron: every edge is shared by exactly two faces,
    // so no triangle should have any boundary edges.
    let mesh = tetrahedron_mesh();
    let conn = Poly_Connect::new(&mesh);
    for ti in 1..=4 {
        assert_eq!(
            conn.boundary_edge_count(ti),
            0,
            "tetrahedron tri {ti} should have 0 boundary edges"
        );
    }
}
