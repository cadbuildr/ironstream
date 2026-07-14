use ironstream::mesh_smooth::*;

#[test]
fn meshing_params_defaults() {
    let p = MeshingParams::default();
    assert!((p.linear_deflection - 0.1).abs() < 1e-10);
    assert!(p.adapt_min);
    assert!(p.internal_vertices_mode);
    assert!(p.control_surface_deflection);
}

#[test]
fn brep_mesh_perform_with_shape() {
    let mut m = BrepMeshFastDiscret::new(MeshingParams::default());
    m.set_shape(42);
    m.perform();
    assert!(m.is_done());
    assert!(m.nb_total_nodes() > 0);
    assert!(m.nb_total_triangles() > 0);
    assert_eq!(m.nb_faces_meshed(), 6);
}

#[test]
fn brep_mesh_no_shape_fails() {
    let mut m = BrepMeshFastDiscret::new(MeshingParams::default());
    m.perform(); // shape_id is 0
    assert_eq!(m.status, MeshStatus::Failed);
    assert!(!m.is_done());
}

#[test]
fn brep_mesh_context_perform_all() {
    let mut ctx = BrepMeshContext::new(MeshingParams::default());
    ctx.add_shape(1);
    ctx.add_shape(0); // will fail
    ctx.add_shape(2);
    ctx.perform_all();
    assert_eq!(ctx.nb_done(), 2);
    assert_eq!(ctx.status(1), Some(MeshStatus::Failed));
    assert_eq!(ctx.status(0), Some(MeshStatus::Done));
}

#[test]
fn laplacian_smoother_basic() {
    let mut s = MeshLaplacianSmoother::new();
    let verts = vec![[0.0f32, 0.0, 0.0], [2.0, 0.0, 0.0], [1.0, 1.0, 0.0]];
    let adj = vec![vec![1usize, 2], vec![0, 2], vec![0, 1]];
    s.set_mesh(verts, adj);
    s.set_iterations(5);
    s.smooth();
    assert_eq!(s.vertices().len(), 3);
}

#[test]
fn laplacian_smoother_defaults() {
    let s = MeshLaplacianSmoother::new();
    assert_eq!(s.nb_iterations, 10);
    assert!((s.lambda - 0.5).abs() < 1e-10);
    assert!(s.mu < 0.0, "mu should be negative for Taubin smoothing");
}
