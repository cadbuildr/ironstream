use ironstream::shape_alg::*;

#[test]
fn restriction_parameters_defaults() {
    let p = ShapeCustomRestrictionParameters::default();
    assert_eq!(p.gmax_degree, 15);
    assert_eq!(p.gmax_seg, 10000);
    assert!(!p.convert_bsp_curve);
    assert!(!p.convert_conical);
    assert!(!p.convert_toroidal);
    assert!(!p.convert_extrusion);
}

#[test]
fn algo_container_analyze() {
    let a = ShapeAlgoAlgoContainer::new();
    let r = a.analyze(4);
    assert_eq!(r.nb_faces, 4);
    assert_eq!(r.nb_edges, 12);
    assert!(r.is_closed);
    assert!(r.is_manifold);
}

#[test]
fn shape_algo_tool_is_solid_and_merge() {
    assert!(ShapeAlgoTool::is_solid(4));
    assert!(!ShapeAlgoTool::is_solid(3));
    assert_eq!(ShapeAlgoTool::merge(3, 7), 8);
    assert_eq!(ShapeAlgoTool::merge(10, 2), 11);
}

#[test]
fn shape_algo_tool_split_to_faces() {
    let faces = ShapeAlgoTool::split_to_faces(3);
    assert_eq!(faces, vec![1, 2, 3]);
    let empty = ShapeAlgoTool::split_to_faces(0);
    assert!(empty.is_empty());
}

#[test]
fn custom_surface_convert_to_bspline() {
    let mut s = ShapeCustomSurface::new(5);
    assert!(!s.is_done());
    s.convert_to_bspline();
    assert!(s.is_done());
    assert_eq!(s.result(), Some(1005)); // id + 1000
}

#[test]
fn shape_custom_bspline_restriction_and_scaler() {
    let sc = ShapeCustom::new();
    let params = ShapeCustomRestrictionParameters::default();
    assert_eq!(sc.bspline_restriction(10, &params), 11);

    let su = ShapeScaler::uniform(2.0);
    assert!(su.is_uniform());
    let p = su.apply_point([1.0, 2.0, 3.0]);
    assert!((p[0] - 2.0).abs() < 1e-14);
    assert!((p[2] - 6.0).abs() < 1e-14);

    let sn = ShapeScaler::non_uniform(1.0, 2.0, 3.0);
    assert!(!sn.is_uniform());
    let q = sn.apply_point([1.0, 1.0, 1.0]);
    assert!((q[1] - 2.0).abs() < 1e-14);
    assert!((q[2] - 3.0).abs() < 1e-14);
}
