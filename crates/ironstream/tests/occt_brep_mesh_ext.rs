extern crate ironstream;
use ironstream::brep_mesh_ext::*;

#[test]
fn mesh_params_presets() {
    let fine = MeshParams::fine();
    assert!(fine.linear_deflection < 0.1);
    let coarse = MeshParams::coarse();
    assert!(coarse.linear_deflection > 0.1);
}

#[test]
fn mesh_params_builder() {
    let p = MeshParams::default().with_relative(true).with_parallel(true);
    assert!(p.relative);
    assert!(p.in_parallel);
}

#[test]
fn incremental_mesh_perform() {
    let mut m = IncrementalMesh::new(MeshParams::fine());
    assert!(!m.is_done());
    m.perform(5);
    assert!(m.is_done());
    assert_eq!(m.nb_faces_meshed, 5);
    assert_eq!(m.get_status_flags(), 0);
}

#[test]
fn fast_discret_totals() {
    let mut f = FastDiscret::new(MeshParams::default());
    f.add_face(0, 50, 40);
    f.add_face(1, 100, 80);
    assert_eq!(f.nb_faces(), 2);
    assert_eq!(f.total_nodes(), 150);
    assert_eq!(f.total_triangles(), 120);
    assert!(f.max_deflection() > 0.0);
}

#[test]
fn edge_discretizer_arc() {
    let mut d = EdgeDiscretizer::new(0.05, 0.2);
    d.discretize_arc([0.0,0.0,0.0], 1.0, 0.0, std::f64::consts::PI);
    assert!(d.nb_points() >= 4);
    // First point should be at angle 0: (1,0,0)
    assert!((d.points[0][0] - 1.0).abs() < 1e-10);
}
