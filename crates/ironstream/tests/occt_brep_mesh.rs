// FILE: tests/occt_brep_mesh.rs
extern crate ironstream;

use ironstream::brep_mesh::{
    mesh_face_only, mesh_shape, BRepMesh_DiscretFactory, BRepMesh_IncrementalMesh,
    IMeshData_Parameters, MeshAlgorithm, Poly_Triangle, Poly_Triangulation,
};
use ironstream::gp::Pnt;
use ironstream::top_exp::{
    TopoDS_CompSolid, TopoDS_Compound, TopoDS_Edge, TopoDS_Face, TopoDS_Shell,
    TopoDS_Shape, TopoDS_Solid, TopoDS_Vertex, TopoDS_Wire,
};

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

fn vtx(id: u64, x: f64, y: f64, z: f64) -> TopoDS_Vertex {
    TopoDS_Vertex { id, x, y, z }
}

fn edge2(id: u64, v0: TopoDS_Vertex, v1: TopoDS_Vertex) -> TopoDS_Edge {
    TopoDS_Edge { id, vertices: vec![v0, v1] }
}

fn make_wire(id: u64, edges: Vec<TopoDS_Edge>) -> TopoDS_Wire {
    TopoDS_Wire { id, edges }
}

fn make_face(id: u64, wire: TopoDS_Wire) -> TopoDS_Face {
    TopoDS_Face { id, wires: vec![wire] }
}

fn make_shell(id: u64, faces: Vec<TopoDS_Face>) -> TopoDS_Shell {
    TopoDS_Shell { id, faces }
}

fn make_solid(id: u64, shell: TopoDS_Shell) -> TopoDS_Solid {
    TopoDS_Solid { id, shells: vec![shell] }
}

/// A unit square face lying in the XY plane.
fn unit_square_face(face_id: u64) -> TopoDS_Face {
    make_face(
        face_id,
        make_wire(
            face_id * 100,
            vec![
                edge2(face_id * 100 + 1, vtx(face_id * 100 + 1, 0.0, 0.0, 0.0), vtx(face_id * 100 + 2, 1.0, 0.0, 0.0)),
                edge2(face_id * 100 + 2, vtx(face_id * 100 + 2, 1.0, 0.0, 0.0), vtx(face_id * 100 + 3, 1.0, 1.0, 0.0)),
                edge2(face_id * 100 + 3, vtx(face_id * 100 + 3, 1.0, 1.0, 0.0), vtx(face_id * 100 + 4, 0.0, 1.0, 0.0)),
                edge2(face_id * 100 + 4, vtx(face_id * 100 + 4, 0.0, 1.0, 0.0), vtx(face_id * 100 + 1, 0.0, 0.0, 0.0)),
            ],
        ),
    )
}

/// A right-triangle face.
fn triangle_face(face_id: u64) -> TopoDS_Face {
    make_face(
        face_id,
        make_wire(
            face_id * 100,
            vec![
                edge2(face_id * 100 + 1, vtx(face_id * 100 + 1, 0.0, 0.0, 0.0), vtx(face_id * 100 + 2, 2.0, 0.0, 0.0)),
                edge2(face_id * 100 + 2, vtx(face_id * 100 + 2, 2.0, 0.0, 0.0), vtx(face_id * 100 + 3, 0.0, 2.0, 0.0)),
                edge2(face_id * 100 + 3, vtx(face_id * 100 + 3, 0.0, 2.0, 0.0), vtx(face_id * 100 + 1, 0.0, 0.0, 0.0)),
            ],
        ),
    )
}

/// A pentagon face (5-gon) to exercise the general ear-clip path.
fn pentagon_face(face_id: u64) -> TopoDS_Face {
    use std::f64::consts::TAU;
    let n = 5usize;
    let verts: Vec<TopoDS_Vertex> = (0..n)
        .map(|i| {
            let a = TAU * i as f64 / n as f64;
            vtx(face_id * 100 + i as u64, a.cos(), a.sin(), 0.0)
        })
        .collect();
    let edges: Vec<TopoDS_Edge> = (0..n)
        .map(|i| {
            let j = (i + 1) % n;
            edge2(
                face_id * 100 + i as u64 + 10,
                verts[i].clone(),
                verts[j].clone(),
            )
        })
        .collect();
    make_face(face_id, make_wire(face_id * 100, edges))
}

// ---------------------------------------------------------------------------
// Test 1 – IMeshData_Parameters: absolute vs relative deflection
// ---------------------------------------------------------------------------

#[test]
fn test_params_absolute_deflection_is_unchanged_by_diag() {
    let p = IMeshData_Parameters::new(0.05, 0.4);
    // Absolute mode: diag irrelevant.
    assert!((p.effective_deflection(200.0) - 0.05).abs() < 1e-12);
    assert!((p.effective_deflection(0.01) - 0.05).abs() < 1e-12);
}

// ---------------------------------------------------------------------------
// Test 2 – IMeshData_Parameters: relative mode scales by bbox diagonal
// ---------------------------------------------------------------------------

#[test]
fn test_params_relative_deflection_scales_with_diag() {
    let p = IMeshData_Parameters::with_relative(0.02, 0.5, true);
    let eff = p.effective_deflection(50.0);
    assert!((eff - 1.0).abs() < 1e-9, "expected 0.02 * 50 = 1.0, got {eff}");
}

// ---------------------------------------------------------------------------
// Test 3 – Poly_Triangle validity and indexing
// ---------------------------------------------------------------------------

#[test]
fn test_poly_triangle_basic_properties() {
    let t = Poly_Triangle::new(0, 1, 2);
    assert!(t.is_valid());
    assert_eq!(t.n1, 0);
    assert_eq!(t.n2, 1);
    assert_eq!(t.n3, 2);
    assert_eq!(t.as_array(), [0, 1, 2]);

    let degenerate = Poly_Triangle::new(5, 5, 7);
    assert!(!degenerate.is_valid());
}

// ---------------------------------------------------------------------------
// Test 4 – Poly_Triangulation area computation
// ---------------------------------------------------------------------------

#[test]
fn test_poly_triangulation_area_of_unit_right_triangle() {
    // A right triangle with legs of length 1 along X and Y has area 0.5.
    let nodes = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
    ];
    let tris = vec![Poly_Triangle::new(0, 1, 2)];
    let tri = Poly_Triangulation::from_parts(nodes, tris, 0.1);
    assert!((tri.area() - 0.5).abs() < 1e-9, "area = {}", tri.area());
}

// ---------------------------------------------------------------------------
// Test 5 – mesh_face_only: triangle face yields exactly one triangle
// ---------------------------------------------------------------------------

#[test]
fn test_mesh_face_only_triangle() {
    let face = triangle_face(1);
    let params = IMeshData_Parameters::new(5.0, 0.5); // coarse → no subdivision
    let tri = mesh_face_only(face, &params);
    assert_eq!(tri.nb_triangles(), 1, "triangle face must give 1 triangle");
    assert_eq!(tri.nb_nodes(), 3);
    assert!(tri.triangles[0].is_valid());
}

// ---------------------------------------------------------------------------
// Test 6 – mesh_face_only: square face yields two triangles
// ---------------------------------------------------------------------------

#[test]
fn test_mesh_face_only_square_two_triangles() {
    let face = unit_square_face(1);
    let params = IMeshData_Parameters::new(5.0, 0.5);
    let tri = mesh_face_only(face, &params);
    assert_eq!(tri.nb_triangles(), 2, "quad gives 2 triangles");
    assert!(tri.nb_nodes() >= 4);
}

// ---------------------------------------------------------------------------
// Test 7 – mesh_face_only: pentagon face yields three triangles (n-2)
// ---------------------------------------------------------------------------

#[test]
fn test_mesh_face_only_pentagon_three_triangles() {
    let face = pentagon_face(1);
    let params = IMeshData_Parameters::new(5.0, 0.5);
    let tri = mesh_face_only(face, &params);
    // Ear-clip of a convex polygon with n vertices produces n-2 triangles.
    assert_eq!(tri.nb_triangles(), 3, "pentagon → 3 triangles");
    assert_eq!(tri.nb_nodes(), 5);
}

// ---------------------------------------------------------------------------
// Test 8 – BRepMesh_IncrementalMesh: meshing a shell with multiple faces
// ---------------------------------------------------------------------------

#[test]
fn test_incremental_mesh_multi_face_shell() {
    let f1 = unit_square_face(1);
    let f2 = triangle_face(2);
    let f3 = pentagon_face(3);
    let shell = make_shell(1, vec![f1, f2, f3]);
    let shape = TopoDS_Shape::Shell(shell);

    let mesh = BRepMesh_IncrementalMesh::with_deflection(shape, 5.0);
    assert!(mesh.is_done());
    assert_eq!(mesh.face_triangulations().len(), 3);
    // square → 2, triangle → 1, pentagon → 3 = 6 total
    assert_eq!(mesh.total_triangles(), 6);
}

// ---------------------------------------------------------------------------
// Test 9 – BRepMesh_IncrementalMesh: triangulation_for retrieves by face id
// ---------------------------------------------------------------------------

#[test]
fn test_incremental_mesh_lookup_by_face_id() {
    let f1 = unit_square_face(77);
    let shape = TopoDS_Shape::Face(f1);
    let mesh = BRepMesh_IncrementalMesh::with_deflection(shape, 5.0);

    let found = mesh.triangulation_for(77);
    assert!(found.is_some(), "should find triangulation for face 77");
    assert!(found.unwrap().nb_triangles() > 0);

    let missing = mesh.triangulation_for(999);
    assert!(missing.is_none(), "should not find face 999");
}

// ---------------------------------------------------------------------------
// Test 10 – BRepMesh_DiscretFactory: algorithm switching and mesh production
// ---------------------------------------------------------------------------

#[test]
fn test_discret_factory_algorithm_switching() {
    let mut factory = BRepMesh_DiscretFactory::get();

    // Default is Standard.
    assert_eq!(factory.algorithm(), MeshAlgorithm::Standard);

    // Switch to Preview.
    let ok = factory.set_default(MeshAlgorithm::Preview);
    assert!(ok);
    assert_eq!(factory.algorithm(), MeshAlgorithm::Preview);

    // Both algorithms produce valid meshes.
    let face = unit_square_face(1);
    let shape = TopoDS_Shape::Face(face);
    let mesh = factory.discret(shape, 0.5, 0.5);
    assert!(mesh.is_done());
    assert!(mesh.total_triangles() > 0);
}

// ---------------------------------------------------------------------------
// Test 11 – mesh_shape free function: compound of faces
// ---------------------------------------------------------------------------

#[test]
fn test_mesh_shape_compound_of_faces() {
    let f1 = TopoDS_Shape::Face(unit_square_face(1));
    let f2 = TopoDS_Shape::Face(triangle_face(2));
    let compound = TopoDS_Shape::Compound(TopoDS_Compound {
        id: 1,
        shapes: vec![f1, f2],
    });

    let mesh = mesh_shape(compound, 5.0);
    assert!(mesh.is_done());
    assert_eq!(mesh.face_triangulations().len(), 2);
    // square → 2, triangle → 1 = 3 total
    assert_eq!(mesh.total_triangles(), 3);
}

// ---------------------------------------------------------------------------
// Test 12 – set_parameters triggers re-mesh with finer parameters
// ---------------------------------------------------------------------------

#[test]
fn test_set_parameters_remeshes() {
    let face = unit_square_face(1);
    let shape = TopoDS_Shape::Face(face);

    // Coarse mesh first.
    let mut mesh =
        BRepMesh_IncrementalMesh::new(shape, IMeshData_Parameters::new(5.0, 0.5));
    let coarse_count = mesh.total_triangles();

    // Fine mesh should produce at least as many triangles (subdivision on edges
    // creates more boundary points → more triangles).
    mesh.set_parameters(IMeshData_Parameters::new(0.05, 0.5));
    let fine_count = mesh.total_triangles();

    assert!(fine_count >= coarse_count, "fine mesh must not have fewer triangles than coarse");
}

// ---------------------------------------------------------------------------
// Test 13 – Solid shape is traversed and all faces are meshed
// ---------------------------------------------------------------------------

#[test]
fn test_incremental_mesh_solid() {
    // Build a fake solid with 3 faces.
    let shell = make_shell(
        1,
        vec![unit_square_face(1), triangle_face(2), unit_square_face(3)],
    );
    let solid = make_solid(1, shell);
    let shape = TopoDS_Shape::Solid(solid);

    let mesh = BRepMesh_IncrementalMesh::with_deflection(shape, 5.0);
    assert!(mesh.is_done());
    assert_eq!(mesh.face_triangulations().len(), 3);
}

// ---------------------------------------------------------------------------
// Test 14 – BRepMesh_DiscretFactory::find_algo recognises known names
// ---------------------------------------------------------------------------

#[test]
fn test_factory_find_algo_case_insensitive() {
    let factory = BRepMesh_DiscretFactory::get();
    assert!(factory.find_algo("Standard"));
    assert!(factory.find_algo("STANDARD"));
    assert!(factory.find_algo("standard"));
    assert!(factory.find_algo("Preview"));
    assert!(factory.find_algo("PREVIEW"));
    assert!(!factory.find_algo("Exact"));
    assert!(!factory.find_algo(""));
}

// ---------------------------------------------------------------------------
// Test 15 – Poly_Triangulation node accessor
// ---------------------------------------------------------------------------

#[test]
fn test_poly_triangulation_node_accessor() {
    let nodes = vec![
        Pnt::new(1.0, 2.0, 3.0),
        Pnt::new(4.0, 5.0, 6.0),
        Pnt::new(7.0, 8.0, 9.0),
    ];
    let tris = vec![Poly_Triangle::new(0, 1, 2)];
    let tri = Poly_Triangulation::from_parts(nodes, tris, 0.1);

    assert!((tri.node(0).x - 1.0).abs() < 1e-12);
    assert!((tri.node(1).y - 5.0).abs() < 1e-12);
    assert!((tri.node(2).z - 9.0).abs() < 1e-12);
    assert_eq!(tri.triangle(0), Poly_Triangle::new(0, 1, 2));
}
