// FILE: tests/occt_brep_check.rs
extern crate ironstream;

use ironstream::brep_check::{
    BRepCheck_Analyzer, BRepCheck_ListOfStatus, BRepCheck_Result, BRepCheck_Status,
    check_status, is_valid,
};
use ironstream::top_exp::{
    TopoDS_Compound, TopoDS_Edge, TopoDS_Face, TopoDS_Shape, TopoDS_Shell, TopoDS_Solid,
    TopoDS_Vertex, TopoDS_Wire,
};

// ---------------------------------------------------------------------------
// Helper constructors
// ---------------------------------------------------------------------------

fn vertex(id: u64, x: f64, y: f64, z: f64) -> TopoDS_Vertex {
    TopoDS_Vertex { id, x, y, z }
}

/// Build a simple valid edge from (0,0,0) to (1,0,0).
fn simple_edge(id: u64) -> TopoDS_Edge {
    TopoDS_Edge {
        id,
        vertices: vec![
            vertex(id * 10, 0.0, 0.0, 0.0),
            vertex(id * 10 + 1, 1.0, 0.0, 0.0),
        ],
    }
}

/// Build a closed triangular wire in the XY-plane.
fn triangle_wire(base_id: u64) -> TopoDS_Wire {
    let p0 = (0.0f64, 0.0f64, 0.0f64);
    let p1 = (1.0f64, 0.0f64, 0.0f64);
    let p2 = (0.5f64, 1.0f64, 0.0f64);
    let bid = base_id * 1000;
    let e0 = TopoDS_Edge {
        id: bid,
        vertices: vec![
            vertex(bid + 0, p0.0, p0.1, p0.2),
            vertex(bid + 1, p1.0, p1.1, p1.2),
        ],
    };
    let e1 = TopoDS_Edge {
        id: bid + 1,
        vertices: vec![
            vertex(bid + 1, p1.0, p1.1, p1.2),
            vertex(bid + 2, p2.0, p2.1, p2.2),
        ],
    };
    let e2 = TopoDS_Edge {
        id: bid + 2,
        vertices: vec![
            vertex(bid + 2, p2.0, p2.1, p2.2),
            vertex(bid + 0, p0.0, p0.1, p0.2),
        ],
    };
    TopoDS_Wire { id: base_id, edges: vec![e0, e1, e2] }
}

/// Build a valid triangular face.
fn triangle_face(id: u64) -> TopoDS_Face {
    TopoDS_Face { id, wires: vec![triangle_wire(id)] }
}

/// Build a 4-face "tetrahedron" shell (the minimum for a closed shell).
fn tetra_shell(id: u64) -> TopoDS_Shell {
    TopoDS_Shell {
        id,
        faces: vec![
            triangle_face(id * 100),
            triangle_face(id * 100 + 1),
            triangle_face(id * 100 + 2),
            triangle_face(id * 100 + 3),
        ],
    }
}

/// Build a valid solid from a tetrahedron shell.
fn valid_solid(id: u64) -> TopoDS_Solid {
    TopoDS_Solid { id, shells: vec![tetra_shell(id)] }
}

// ---------------------------------------------------------------------------
// BRepCheck_Status tests
// ---------------------------------------------------------------------------

#[test]
fn status_no_error_is_ok() {
    assert!(BRepCheck_Status::NoError.is_ok());
}

#[test]
fn status_errors_are_not_ok() {
    let errors = [
        BRepCheck_Status::InvalidVertex,
        BRepCheck_Status::InvalidDegeneratedEdge,
        BRepCheck_Status::InvalidEdge,
        BRepCheck_Status::NotClosed,
        BRepCheck_Status::NotConnected,
        BRepCheck_Status::NoSurface,
        BRepCheck_Status::InvalidFace,
        BRepCheck_Status::OpenShell,
        BRepCheck_Status::InvalidSolid,
    ];
    for e in &errors {
        assert!(!e.is_ok(), "{:?} should not be ok", e);
    }
}

#[test]
fn status_description_non_empty_for_all_variants() {
    let all = [
        BRepCheck_Status::NoError,
        BRepCheck_Status::InvalidPointOnCurve,
        BRepCheck_Status::InvalidVertex,
        BRepCheck_Status::InvalidDegeneratedEdge,
        BRepCheck_Status::InvalidEdge,
        BRepCheck_Status::NotClosed,
        BRepCheck_Status::NotConnected,
        BRepCheck_Status::SelfIntersectingWire,
        BRepCheck_Status::NoSurface,
        BRepCheck_Status::InvalidFace,
        BRepCheck_Status::NotPlanarFace,
        BRepCheck_Status::OpenShell,
        BRepCheck_Status::BadOrientationOfSubshape,
        BRepCheck_Status::InvalidSolid,
        BRepCheck_Status::CheckFail,
    ];
    for s in &all {
        assert!(!s.description().is_empty(), "{:?} description must be non-empty", s);
    }
}

#[test]
fn status_variants_are_distinct() {
    assert_ne!(BRepCheck_Status::NoError, BRepCheck_Status::NotClosed);
    assert_ne!(BRepCheck_Status::InvalidVertex, BRepCheck_Status::InvalidEdge);
    assert_ne!(BRepCheck_Status::OpenShell, BRepCheck_Status::InvalidSolid);
}

// ---------------------------------------------------------------------------
// BRepCheck_ListOfStatus tests
// ---------------------------------------------------------------------------

#[test]
fn list_empty_is_valid() {
    let list = BRepCheck_ListOfStatus::new();
    assert!(list.is_empty());
    assert!(list.is_valid());
}

#[test]
fn list_no_error_singleton_is_valid() {
    let list = BRepCheck_ListOfStatus::with_status(BRepCheck_Status::NoError);
    assert!(list.is_valid());
    assert_eq!(list.len(), 1);
    assert_eq!(list.first(), Some(BRepCheck_Status::NoError));
}

#[test]
fn list_error_singleton_is_invalid() {
    let list = BRepCheck_ListOfStatus::with_status(BRepCheck_Status::NotClosed);
    assert!(!list.is_valid());
    assert_eq!(list.first(), Some(BRepCheck_Status::NotClosed));
}

#[test]
fn list_add_multiple_errors() {
    let mut list = BRepCheck_ListOfStatus::new();
    list.add(BRepCheck_Status::NotClosed);
    list.add(BRepCheck_Status::NotConnected);
    assert_eq!(list.len(), 2);
    assert!(!list.is_valid());
    assert!(list.contains(BRepCheck_Status::NotClosed));
    assert!(list.contains(BRepCheck_Status::NotConnected));
}

#[test]
fn list_deduplication() {
    let mut list = BRepCheck_ListOfStatus::new();
    list.add(BRepCheck_Status::OpenShell);
    list.add(BRepCheck_Status::OpenShell);
    list.add(BRepCheck_Status::OpenShell);
    assert_eq!(list.len(), 1);
}

#[test]
fn list_as_slice_matches_iter() {
    let mut list = BRepCheck_ListOfStatus::new();
    list.add(BRepCheck_Status::InvalidFace);
    list.add(BRepCheck_Status::NoSurface);
    let from_slice: Vec<_> = list.as_slice().iter().copied().collect();
    let from_iter: Vec<_> = list.iter().copied().collect();
    assert_eq!(from_slice, from_iter);
}

// ---------------------------------------------------------------------------
// check_status (free function) tests
// ---------------------------------------------------------------------------

#[test]
fn check_status_valid_vertex_returns_no_error() {
    let v = TopoDS_Shape::Vertex(vertex(1, 1.0, 2.0, 3.0));
    assert_eq!(check_status(&v), BRepCheck_Status::NoError);
}

#[test]
fn check_status_nan_vertex_returns_invalid_vertex() {
    let v = TopoDS_Shape::Vertex(vertex(2, f64::NAN, 0.0, 0.0));
    assert_eq!(check_status(&v), BRepCheck_Status::InvalidVertex);
}

#[test]
fn check_status_inf_vertex_returns_invalid_vertex() {
    let v = TopoDS_Shape::Vertex(vertex(3, 0.0, f64::NEG_INFINITY, 0.0));
    assert_eq!(check_status(&v), BRepCheck_Status::InvalidVertex);
}

#[test]
fn check_status_valid_edge_returns_no_error() {
    let e = TopoDS_Shape::Edge(simple_edge(1));
    assert_eq!(check_status(&e), BRepCheck_Status::NoError);
}

#[test]
fn check_status_degenerate_edge_returns_degenerate() {
    let e = TopoDS_Shape::Edge(TopoDS_Edge {
        id: 99,
        vertices: vec![
            vertex(1, 5.0, 5.0, 5.0),
            vertex(2, 5.0, 5.0, 5.0), // same position
        ],
    });
    assert_eq!(check_status(&e), BRepCheck_Status::InvalidDegeneratedEdge);
}

#[test]
fn check_status_valid_wire_returns_no_error() {
    let w = TopoDS_Shape::Wire(triangle_wire(1));
    assert_eq!(check_status(&w), BRepCheck_Status::NoError);
}

#[test]
fn check_status_open_wire_returns_not_closed() {
    // Two edges that don't close.
    let e0 = TopoDS_Edge {
        id: 1,
        vertices: vec![vertex(1, 0.0, 0.0, 0.0), vertex(2, 1.0, 0.0, 0.0)],
    };
    let e1 = TopoDS_Edge {
        id: 2,
        vertices: vec![vertex(2, 1.0, 0.0, 0.0), vertex(3, 2.0, 0.0, 0.0)],
    };
    let w = TopoDS_Shape::Wire(TopoDS_Wire { id: 5, edges: vec![e0, e1] });
    assert_eq!(check_status(&w), BRepCheck_Status::NotClosed);
}

#[test]
fn check_status_valid_face_returns_no_error() {
    let f = TopoDS_Shape::Face(triangle_face(1));
    assert_eq!(check_status(&f), BRepCheck_Status::NoError);
}

#[test]
fn check_status_empty_face_returns_no_surface() {
    let f = TopoDS_Shape::Face(TopoDS_Face { id: 1, wires: vec![] });
    assert_eq!(check_status(&f), BRepCheck_Status::NoSurface);
}

// ---------------------------------------------------------------------------
// is_valid (free function) tests
// ---------------------------------------------------------------------------

#[test]
fn is_valid_good_vertex() {
    let v = TopoDS_Shape::Vertex(vertex(10, 0.0, 1.0, 2.0));
    assert!(is_valid(&v));
}

#[test]
fn is_valid_bad_vertex_returns_false() {
    let v = TopoDS_Shape::Vertex(vertex(11, f64::NAN, 0.0, 0.0));
    assert!(!is_valid(&v));
}

#[test]
fn is_valid_good_face() {
    let f = TopoDS_Shape::Face(triangle_face(99));
    assert!(is_valid(&f));
}

#[test]
fn is_valid_good_solid() {
    let s = TopoDS_Shape::Solid(valid_solid(1));
    assert!(is_valid(&s));
}

#[test]
fn is_valid_empty_solid_returns_false() {
    let s = TopoDS_Shape::Solid(TopoDS_Solid { id: 1, shells: vec![] });
    assert!(!is_valid(&s));
}

// ---------------------------------------------------------------------------
// BRepCheck_Analyzer tests
// ---------------------------------------------------------------------------

#[test]
fn analyzer_valid_vertex_is_valid() {
    let v = TopoDS_Shape::Vertex(vertex(1, 0.0, 0.0, 0.0));
    let a = BRepCheck_Analyzer::new(v);
    assert!(a.is_valid());
    assert_eq!(a.error_count(), 0);
}

#[test]
fn analyzer_invalid_vertex_error_count_nonzero() {
    let v = TopoDS_Shape::Vertex(vertex(2, f64::NAN, 0.0, 0.0));
    let a = BRepCheck_Analyzer::new(v);
    assert!(!a.is_valid());
    assert!(a.error_count() > 0);
}

#[test]
fn analyzer_result_for_root_returns_some() {
    let e = TopoDS_Shape::Edge(simple_edge(3));
    let a = BRepCheck_Analyzer::new(e.clone());
    let r = a.result_for(&e);
    assert!(r.is_some());
}

#[test]
fn analyzer_result_for_valid_edge_is_no_error() {
    let e = TopoDS_Shape::Edge(simple_edge(4));
    let a = BRepCheck_Analyzer::new(e.clone());
    let r = a.result_for(&e).unwrap();
    assert!(r.is_valid());
    assert_eq!(r.status(), BRepCheck_Status::NoError);
}

#[test]
fn analyzer_checked_count_solid_is_positive() {
    let s = TopoDS_Shape::Solid(valid_solid(2));
    let a = BRepCheck_Analyzer::new(s);
    // Solid → shells → faces → wires → edges → vertices
    assert!(a.checked_count() > 1);
}

#[test]
fn analyzer_all_results_matches_checked_count() {
    let s = TopoDS_Shape::Solid(valid_solid(3));
    let a = BRepCheck_Analyzer::new(s);
    let from_iter = a.all_results().count();
    assert_eq!(from_iter, a.checked_count());
}

#[test]
fn analyzer_invalid_results_empty_for_valid_shape() {
    let f = TopoDS_Shape::Face(triangle_face(7));
    let a = BRepCheck_Analyzer::new(f);
    assert!(a.invalid_results().is_empty());
}

#[test]
fn analyzer_is_valid_shape_for_sub_vertex() {
    let w = TopoDS_Shape::Wire(triangle_wire(5));
    let a = BRepCheck_Analyzer::new(w.clone());
    assert!(a.is_valid_shape(&w));
}

#[test]
fn analyzer_shape_returns_root() {
    let v = TopoDS_Shape::Vertex(vertex(99, 1.0, 2.0, 3.0));
    let a = BRepCheck_Analyzer::new(v.clone());
    // The analyzer stores the root shape; shape() returns it.
    assert!(matches!(a.shape(), TopoDS_Shape::Vertex(_)));
}

#[test]
fn analyzer_compound_of_valid_shapes_is_valid() {
    let e1 = TopoDS_Shape::Edge(simple_edge(10));
    let e2 = TopoDS_Shape::Edge(simple_edge(11));
    let compound = TopoDS_Shape::Compound(TopoDS_Compound {
        id: 1,
        shapes: vec![e1, e2],
    });
    let a = BRepCheck_Analyzer::new(compound);
    assert!(a.is_valid());
}

#[test]
fn analyzer_compound_with_bad_vertex_is_invalid() {
    let bad_v = TopoDS_Shape::Vertex(vertex(999, f64::INFINITY, 0.0, 0.0));
    let compound = TopoDS_Shape::Compound(TopoDS_Compound {
        id: 2,
        shapes: vec![bad_v],
    });
    let a = BRepCheck_Analyzer::new(compound);
    assert!(!a.is_valid());
}

// ---------------------------------------------------------------------------
// BRepCheck_Result tests
// ---------------------------------------------------------------------------

#[test]
fn result_new_no_error_is_valid() {
    let v = TopoDS_Shape::Vertex(vertex(1, 0.0, 0.0, 0.0));
    let r = BRepCheck_Result::new(v);
    assert!(r.is_valid());
    assert_eq!(r.status(), BRepCheck_Status::NoError);
}

#[test]
fn result_with_error_status_is_invalid() {
    let v = TopoDS_Shape::Vertex(vertex(2, 0.0, 0.0, 0.0));
    let r = BRepCheck_Result::with_status(v, BRepCheck_Status::InvalidVertex);
    assert!(!r.is_valid());
    assert_eq!(r.status(), BRepCheck_Status::InvalidVertex);
}

#[test]
fn result_add_status_transitions_from_no_error() {
    let v = TopoDS_Shape::Vertex(vertex(3, 0.0, 0.0, 0.0));
    let mut r = BRepCheck_Result::new(v);
    assert!(r.is_valid());
    r.add_status(BRepCheck_Status::OpenShell);
    assert!(!r.is_valid());
    assert_eq!(r.status(), BRepCheck_Status::OpenShell);
}

#[test]
fn result_status_list_accessible() {
    let v = TopoDS_Shape::Vertex(vertex(4, 0.0, 0.0, 0.0));
    let r = BRepCheck_Result::new(v);
    assert!(!r.status_list().is_empty());
    assert_eq!(r.status_list().first(), Some(BRepCheck_Status::NoError));
}

#[test]
fn result_shape_ref_matches_input() {
    let v = TopoDS_Shape::Vertex(vertex(5, 1.0, 2.0, 3.0));
    let r = BRepCheck_Result::new(v.clone());
    assert!(matches!(r.shape(), TopoDS_Shape::Vertex(_)));
}
