// FILE: tests/occt_brep_tools.rs
extern crate ironstream;

use ironstream::brep_tools::{BRepTools, WireExplorer};
use ironstream::gp::Pnt;
use ironstream::top_exp::{
    TopoDS_Compound, TopoDS_Edge, TopoDS_Face, TopoDS_Shell, TopoDS_Shape,
    TopoDS_Solid, TopoDS_Vertex, TopoDS_Wire,
};

// ---- helpers ----------------------------------------------------------------

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

fn make_solid_shape(id: u64, shell: TopoDS_Shell) -> TopoDS_Shape {
    TopoDS_Shape::Solid(TopoDS_Solid { id, shells: vec![shell] })
}

fn triangle_wire(id: u64) -> TopoDS_Wire {
    make_wire(
        id,
        vec![
            edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
            edge2(2, vtx(2, 1.0, 0.0, 0.0), vtx(3, 0.5, 1.0, 0.0)),
            edge2(3, vtx(3, 0.5, 1.0, 0.0), vtx(1, 0.0, 0.0, 0.0)),
        ],
    )
}

fn unit_square_wire(id: u64) -> TopoDS_Wire {
    make_wire(
        id,
        vec![
            edge2(10, vtx(10, 0.0, 0.0, 0.0), vtx(11, 1.0, 0.0, 0.0)),
            edge2(11, vtx(11, 1.0, 0.0, 0.0), vtx(12, 1.0, 1.0, 0.0)),
            edge2(12, vtx(12, 1.0, 1.0, 0.0), vtx(13, 0.0, 1.0, 0.0)),
            edge2(13, vtx(13, 0.0, 1.0, 0.0), vtx(10, 0.0, 0.0, 0.0)),
        ],
    )
}

// ---- BRepTools::bounding_box ------------------------------------------------

#[test]
fn bounding_box_vertex_shape() {
    let v = TopoDS_Shape::Vertex(vtx(1, 5.0, -3.0, 2.0));
    let bbox = BRepTools::bounding_box(&v);
    assert!(!bbox.is_void());
    let lim = bbox.get_limits();
    assert!((lim.xmin - 5.0).abs() < 1e-12);
    assert!((lim.ymin - (-3.0)).abs() < 1e-12);
    assert!((lim.zmin - 2.0).abs() < 1e-12);
}

#[test]
fn bounding_box_wire_shape() {
    let wire = unit_square_wire(1);
    let shape = TopoDS_Shape::Wire(wire);
    let bbox = BRepTools::bounding_box(&shape);
    let lim = bbox.get_limits();
    assert!((lim.xmin - 0.0).abs() < 1e-12);
    assert!((lim.xmax - 1.0).abs() < 1e-12);
    assert!((lim.ymax - 1.0).abs() < 1e-12);
}

#[test]
fn bounding_box_compound_of_edges() {
    let e1 = TopoDS_Shape::Edge(edge2(1, vtx(1, -2.0, 0.0, 0.0), vtx(2, 0.0, 0.0, 0.0)));
    let e2 = TopoDS_Shape::Edge(edge2(2, vtx(3, 0.0, 0.0, 0.0), vtx(4, 0.0, 3.0, 0.0)));
    let c = TopoDS_Shape::Compound(TopoDS_Compound {
        id: 1,
        shapes: vec![e1, e2],
    });
    let bbox = BRepTools::bounding_box(&c);
    let lim = bbox.get_limits();
    assert!((lim.xmin - (-2.0)).abs() < 1e-12);
    assert!((lim.ymax - 3.0).abs() < 1e-12);
}

#[test]
fn bounding_box_solid_shape() {
    // A minimal solid: one shell, one face, one wire, two edges.
    let wire = make_wire(
        1,
        vec![
            edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 10.0, 0.0, 0.0)),
            edge2(2, vtx(2, 10.0, 0.0, 0.0), vtx(3, 10.0, 10.0, 5.0)),
        ],
    );
    let face = make_face(1, wire);
    let shell = make_shell(1, vec![face]);
    let solid = make_solid_shape(1, shell);
    let bbox = BRepTools::bounding_box(&solid);
    let lim = bbox.get_limits();
    assert!((lim.xmax - 10.0).abs() < 1e-12);
    assert!((lim.ymax - 10.0).abs() < 1e-12);
    assert!((lim.zmax - 5.0).abs() < 1e-12);
}

// ---- BRepTools::outer_wire --------------------------------------------------

#[test]
fn outer_wire_of_face_with_hole() {
    let outer = unit_square_wire(1);
    let hole = triangle_wire(2);
    let face = TopoDS_Face {
        id: 1,
        wires: vec![outer.clone(), hole],
    };
    let result = BRepTools::outer_wire(&face).unwrap();
    assert_eq!(result.id, 1, "outer wire should be the first wire (id=1)");
}

#[test]
fn outer_wire_empty_face_is_none() {
    let face = TopoDS_Face { id: 99, wires: vec![] };
    assert!(BRepTools::outer_wire(&face).is_none());
}

// ---- BRepTools::uv_bounds ---------------------------------------------------

#[test]
fn uv_bounds_unit_square() {
    let origin = Pnt::new(0.0, 0.0, 0.0);
    let x_dir = Pnt::new(1.0, 0.0, 0.0);
    let y_dir = Pnt::new(0.0, 1.0, 0.0);
    let wire = unit_square_wire(1);
    let (u_min, u_max, v_min, v_max) =
        BRepTools::uv_bounds(&wire, origin, x_dir, y_dir).unwrap();
    assert!((u_min - 0.0).abs() < 1e-12);
    assert!((u_max - 1.0).abs() < 1e-12);
    assert!((v_min - 0.0).abs() < 1e-12);
    assert!((v_max - 1.0).abs() < 1e-12);
}

#[test]
fn uv_bounds_offset_origin() {
    // Wire at (5,5,0) → (6,5,0) → (6,6,0), origin at (5,5,0).
    let origin = Pnt::new(5.0, 5.0, 0.0);
    let x_dir = Pnt::new(1.0, 0.0, 0.0);
    let y_dir = Pnt::new(0.0, 1.0, 0.0);
    let wire = make_wire(
        1,
        vec![
            edge2(1, vtx(1, 5.0, 5.0, 0.0), vtx(2, 6.0, 5.0, 0.0)),
            edge2(2, vtx(2, 6.0, 5.0, 0.0), vtx(3, 6.0, 6.0, 0.0)),
        ],
    );
    let (u_min, u_max, v_min, v_max) =
        BRepTools::uv_bounds(&wire, origin, x_dir, y_dir).unwrap();
    assert!((u_min - 0.0).abs() < 1e-12);
    assert!((u_max - 1.0).abs() < 1e-12);
    assert!((v_min - 0.0).abs() < 1e-12);
    assert!((v_max - 1.0).abs() < 1e-12);
}

// ---- BRepTools::compare -----------------------------------------------------

#[test]
fn compare_same_edge() {
    let e = TopoDS_Shape::Edge(edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 1.0, 1.0)));
    assert!(BRepTools::compare(&e, &e, 1e-7));
}

#[test]
fn compare_different_y_vertex() {
    let e1 = TopoDS_Shape::Edge(edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 0.0, 1.0, 0.0)));
    let e2 = TopoDS_Shape::Edge(edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 0.0, 2.0, 0.0)));
    assert!(!BRepTools::compare(&e1, &e2, 1e-7));
}

// ---- BRepTools::nb_edges / nb_vertices / is_closed --------------------------

#[test]
fn nb_edges_and_is_closed_triangle() {
    let wire = triangle_wire(1);
    assert_eq!(BRepTools::nb_edges(&wire), 3);
    assert!(BRepTools::is_closed(&wire, 1e-7));
}

#[test]
fn nb_vertices_shared_in_triangle() {
    let wire = triangle_wire(1);
    // Vertices 1, 2, 3 — each shared between two edges → 3 distinct ids.
    assert_eq!(BRepTools::nb_vertices(&wire), 3);
}

#[test]
fn is_closed_open_chain_is_false() {
    let wire = make_wire(
        1,
        vec![
            edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
            edge2(2, vtx(2, 1.0, 0.0, 0.0), vtx(3, 2.0, 0.0, 0.0)),
        ],
    );
    assert!(!BRepTools::is_closed(&wire, 1e-7));
}

// ---- BRepTools::clean_wire --------------------------------------------------

#[test]
fn clean_wire_removes_degenerate() {
    let wire = make_wire(
        1,
        vec![
            edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
            edge2(2, vtx(3, 2.0, 0.0, 0.0), vtx(4, 2.0, 0.0, 0.0)), // degenerate
            edge2(3, vtx(5, 3.0, 0.0, 0.0), vtx(6, 4.0, 0.0, 0.0)),
        ],
    );
    let cleaned = BRepTools::clean_wire(&wire, 1e-7);
    assert_eq!(cleaned.edges.len(), 2, "degenerate edge should be removed");
    // Preserved ids should be 1 and 3.
    assert_eq!(cleaned.edges[0].id, 1);
    assert_eq!(cleaned.edges[1].id, 3);
}

#[test]
fn clean_wire_no_degenerate_unchanged() {
    let wire = unit_square_wire(1);
    let cleaned = BRepTools::clean_wire(&wire, 1e-7);
    assert_eq!(cleaned.edges.len(), 4);
}

// ---- BRepTools::wire_perimeter / wire_centroid ------------------------------

#[test]
fn wire_perimeter_3_4_5_triangle() {
    // Right triangle: legs 3 and 4, hypotenuse 5.
    let wire = make_wire(
        1,
        vec![
            edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 3.0, 0.0, 0.0)),
            edge2(2, vtx(2, 3.0, 0.0, 0.0), vtx(3, 3.0, 4.0, 0.0)),
            edge2(3, vtx(3, 3.0, 4.0, 0.0), vtx(1, 0.0, 0.0, 0.0)),
        ],
    );
    let perimeter = BRepTools::wire_perimeter(&wire);
    assert!((perimeter - 12.0).abs() < 1e-10, "perimeter={}", perimeter);
}

#[test]
fn wire_centroid_unit_square() {
    let wire = unit_square_wire(1);
    let c = BRepTools::wire_centroid(&wire).unwrap();
    // The unit square corner vertices average to (0.5, 0.5, 0), but each
    // vertex appears in two edges so the raw average of all vertex entries is
    // still (0.5, 0.5, 0).
    assert!((c.x - 0.5).abs() < 0.1, "cx={}", c.x);
    assert!((c.y - 0.5).abs() < 0.1, "cy={}", c.y);
    assert!(c.z.abs() < 1e-10, "cz={}", c.z);
}

// ---- WireExplorer -----------------------------------------------------------

#[test]
fn wire_explorer_traverses_triangle() {
    let wire = triangle_wire(1);
    let mut ex = WireExplorer::new();
    ex.init_wire(&wire);

    let mut count = 0;
    while ex.more() {
        let _ = ex.current_edge();
        count += 1;
        ex.next();
    }
    assert_eq!(count, 3);
}

#[test]
fn wire_explorer_current_vertex_id() {
    let wire = make_wire(
        1,
        vec![edge2(1, vtx(77, 1.0, 2.0, 3.0), vtx(78, 4.0, 5.0, 6.0))],
    );
    let mut ex = WireExplorer::new();
    ex.init_wire(&wire);
    assert!(ex.more());
    let v = ex.current_vertex().unwrap();
    assert_eq!(v.id, 77);
}

#[test]
fn wire_explorer_reset_allows_second_traversal() {
    let wire = unit_square_wire(1);
    let mut ex = WireExplorer::new();
    ex.init_wire(&wire);

    // First pass.
    let mut first_pass = 0;
    while ex.more() {
        first_pass += 1;
        ex.next();
    }

    // Reset and second pass.
    ex.reset();
    let mut second_pass = 0;
    while ex.more() {
        second_pass += 1;
        ex.next();
    }

    assert_eq!(first_pass, 4);
    assert_eq!(second_pass, 4);
}

#[test]
fn wire_explorer_collect_remaining_after_skip() {
    let wire = make_wire(
        1,
        vec![
            edge2(1, vtx(1, 0.0, 0.0, 0.0), vtx(2, 1.0, 0.0, 0.0)),
            edge2(2, vtx(2, 1.0, 0.0, 0.0), vtx(3, 2.0, 0.0, 0.0)),
            edge2(3, vtx(3, 2.0, 0.0, 0.0), vtx(4, 3.0, 0.0, 0.0)),
        ],
    );
    let mut ex = WireExplorer::new();
    ex.init_wire(&wire);
    // Skip first edge.
    ex.next();
    let rest = ex.collect_remaining();
    assert_eq!(rest.len(), 2);
    assert_eq!(rest[0].id, 2);
    assert_eq!(rest[1].id, 3);
}

#[test]
fn wire_explorer_init_from_face_shape() {
    let wire = triangle_wire(1);
    let face = TopoDS_Shape::Face(make_face(1, wire));
    let mut ex = WireExplorer::new();
    ex.init(&face);
    assert_eq!(ex.nb_edges(), 3);
    assert!(ex.more());
}

#[test]
fn wire_explorer_empty_wire_more_is_false() {
    let wire = make_wire(1, vec![]);
    let mut ex = WireExplorer::new();
    ex.init_wire(&wire);
    assert!(!ex.more());
}
