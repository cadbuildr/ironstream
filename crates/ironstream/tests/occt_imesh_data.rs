// FILE: tests/occt_imesh_data.rs
extern crate ironstream;

use ironstream::gp::Pnt;
use ironstream::imesh_data::{
    EdgeOrientation, FaceStatus, IMeshData_Curve, IMeshData_Edge, IMeshData_Face,
    IMeshData_Parameters,
};

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

/// Build a unit-square face triangulation (two triangles).
fn unit_square_face_data() -> IMeshData_Face {
    let nodes = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(1.0, 1.0, 0.0),
        Pnt::new(0.0, 1.0, 0.0),
    ];
    let triangles = vec![(0, 1, 2), (0, 2, 3)];
    let mut face = IMeshData_Face::new(1);
    face.set_triangulation(nodes, triangles, 0.1);
    face
}

/// Build a straight-line curve from (0,0,0) to (3,0,0) with 4 samples.
fn straight_curve() -> IMeshData_Curve {
    let params = vec![0.0, 1.0, 2.0, 3.0];
    let points = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
        Pnt::new(3.0, 0.0, 0.0),
    ];
    IMeshData_Curve::from_samples(params, points)
}

// ---------------------------------------------------------------------------
// IMeshData_Parameters integration tests
// ---------------------------------------------------------------------------

#[test]
fn params_new_stores_deflections() {
    let p = IMeshData_Parameters::new(0.05, 0.2);
    assert!((p.linear_deflection - 0.05).abs() < 1e-12);
    assert!((p.angular_deflection - 0.2).abs() < 1e-12);
}

#[test]
fn params_with_relative_sets_flag() {
    let p = IMeshData_Parameters::with_relative(0.02, 0.1, true);
    assert!(p.relative);
    // Effective deflection in relative mode scales with the bounding-box diagonal.
    let eff = p.effective_deflection(200.0);
    assert!((eff - 4.0).abs() < 1e-10); // 0.02 * 200 == 4.0
}

#[test]
fn params_is_coarse_threshold() {
    let very_coarse = IMeshData_Parameters::new(1.0, 2.0);
    assert!(very_coarse.is_coarse());

    let fine = IMeshData_Parameters::new(0.001, 0.01);
    assert!(!fine.is_coarse());
}

#[test]
fn params_clamped_limits_deflections() {
    let p = IMeshData_Parameters::new(0.5, 1.5).clamped(0.3, 1.0);
    assert!(p.linear_deflection <= 0.3 + 1e-12);
    assert!(p.angular_deflection <= 1.0 + 1e-12);
}

#[test]
fn params_default_is_reasonable() {
    let p = IMeshData_Parameters::default();
    // Default values must give a non-coarse but not extremely fine mesh.
    assert!(p.linear_deflection > 0.0);
    assert!(p.angular_deflection > 0.0);
    assert!(!p.in_parallel);
    assert!(!p.mesh_in_parallel);
}

// ---------------------------------------------------------------------------
// IMeshData_Curve integration tests
// ---------------------------------------------------------------------------

#[test]
fn curve_from_samples_roundtrip() {
    let c = straight_curve();
    assert_eq!(c.nb_points(), 4);
    assert!((c.parameter(0) - 0.0).abs() < 1e-12);
    assert!((c.parameter(3) - 3.0).abs() < 1e-12);
    assert!((c.point(2).x - 2.0).abs() < 1e-12);
}

#[test]
fn curve_length_along_axis() {
    let c = straight_curve();
    // 3 unit-length segments → total length 3.
    assert!((c.length() - 3.0).abs() < 1e-10);
}

#[test]
fn curve_param_range_values() {
    let c = straight_curve();
    let (t0, t1) = c.param_range();
    assert!((t0 - 0.0).abs() < 1e-12);
    assert!((t1 - 3.0).abs() < 1e-12);
}

#[test]
fn curve_subdivide_refines_coarse_segment() {
    // One long segment; subdivide with tight tolerance.
    let mut c = IMeshData_Curve::new();
    c.add_point(0.0, Pnt::new(0.0, 0.0, 0.0));
    c.add_point(10.0, Pnt::new(10.0, 0.0, 0.0));

    let refined = c.subdivided(2.0);
    // 10-unit chord / 2.0 max_chord → ≥ 6 points (endpoints + 4 inner).
    assert!(refined.nb_points() >= 6, "nb_points={}", refined.nb_points());
    // Total length must still be 10.
    assert!((refined.length() - 10.0).abs() < 1e-10);
}

#[test]
fn curve_empty_has_zero_length_and_empty_range() {
    let c = IMeshData_Curve::new();
    assert!(c.is_empty());
    assert!((c.length() - 0.0).abs() < 1e-12);
    let (t0, t1) = c.param_range();
    assert!((t0 - 0.0).abs() < 1e-12);
    assert!((t1 - 0.0).abs() < 1e-12);
}

// ---------------------------------------------------------------------------
// IMeshData_Edge integration tests
// ---------------------------------------------------------------------------

#[test]
fn edge_stores_orientation_and_curves() {
    let mut edge = IMeshData_Edge::new(10, EdgeOrientation::Forward);
    assert_eq!(edge.orientation, EdgeOrientation::Forward);
    edge.add_curve(straight_curve());
    assert_eq!(edge.nb_curves(), 1);
    assert!(edge.has_geometry());
}

#[test]
fn edge_length_delegates_to_first_curve() {
    let mut edge = IMeshData_Edge::new(11, EdgeOrientation::Reversed);
    edge.add_curve(straight_curve());
    assert!((edge.length() - 3.0).abs() < 1e-10);
}

#[test]
fn edge_pcurve_stored_and_retrieved() {
    let mut edge = IMeshData_Edge::new(12, EdgeOrientation::Forward);
    let uv: Vec<(f64, f64)> = vec![(0.0, 0.0), (0.5, 0.0), (1.0, 0.0)];
    edge.add_pcurve(uv.clone());
    assert_eq!(edge.nb_pcurves(), 1);
    assert_eq!(edge.pcurve(0), uv.as_slice());
}

#[test]
fn edge_no_geometry_when_empty() {
    let edge = IMeshData_Edge::new(13, EdgeOrientation::Forward);
    assert!(!edge.has_geometry());
    assert_eq!(edge.nb_curves(), 0);
}

// ---------------------------------------------------------------------------
// IMeshData_Face integration tests
// ---------------------------------------------------------------------------

#[test]
fn face_unit_square_area_is_one() {
    let face = unit_square_face_data();
    assert_eq!(face.status, FaceStatus::Ok);
    assert!(face.is_meshed());
    assert!((face.area() - 1.0).abs() < 1e-10);
}

#[test]
fn face_nb_nodes_and_triangles() {
    let face = unit_square_face_data();
    assert_eq!(face.nb_nodes(), 4);
    assert_eq!(face.nb_triangles(), 2);
}

#[test]
fn face_bounding_box_unit_square() {
    let face = unit_square_face_data();
    let (mn, mx) = face.bounding_box();
    assert!((mn.x - 0.0).abs() < 1e-12);
    assert!((mn.y - 0.0).abs() < 1e-12);
    assert!((mx.x - 1.0).abs() < 1e-12);
    assert!((mx.y - 1.0).abs() < 1e-12);
    // Z is flat, so min == max == 0.
    assert!((mn.z - 0.0).abs() < 1e-12);
    assert!((mx.z - 0.0).abs() < 1e-12);
}

#[test]
fn face_validate_triangulation_valid_returns_empty() {
    let face = unit_square_face_data();
    assert!(face.validate_triangulation().is_empty());
}

#[test]
fn face_validate_triangulation_catches_degenerate_indices() {
    let nodes = vec![
        Pnt::new(0.0, 0.0, 0.0),
        Pnt::new(1.0, 0.0, 0.0),
        Pnt::new(2.0, 0.0, 0.0),
    ];
    // Second triangle has two identical indices → degenerate.
    let triangles = vec![(0, 1, 2), (0, 0, 2)];
    let mut face = IMeshData_Face::new(20);
    face.set_triangulation(nodes, triangles, 0.1);
    let bad = face.validate_triangulation();
    assert_eq!(bad, vec![1usize]);
}

#[test]
fn face_add_and_retrieve_edges() {
    let mut face = IMeshData_Face::new(30);
    let mut edge = IMeshData_Edge::new(100, EdgeOrientation::Forward);
    edge.add_curve(straight_curve());
    face.add_edge(edge);
    assert_eq!(face.nb_edges(), 1);
    assert!((face.edge(0).length() - 3.0).abs() < 1e-10);
}

#[test]
fn face_status_not_meshed_initially() {
    let face = IMeshData_Face::new(0);
    assert_eq!(face.status, FaceStatus::NotMeshed);
    assert!(!face.is_meshed());
}

#[test]
fn face_set_triangulation_updates_deflection() {
    let mut face = IMeshData_Face::new(50);
    face.set_triangulation(
        vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(0.0, 1.0, 0.0),
        ],
        vec![(0, 1, 2)],
        0.025,
    );
    assert!((face.deflection - 0.025).abs() < 1e-12);
    assert_eq!(face.status, FaceStatus::Ok);
}
