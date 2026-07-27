use ironstream::blend_func::*;

#[test]
fn blend_status() {
    assert!(BlendStatus::Ok.is_done());
    assert!(!BlendStatus::NotDone.is_done());
}

#[test]
fn blend_point_basic() {
    let p = BlendPoint::new(0.5, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.5, 0.5], [0.3, 0.7]);
    assert!((p.parameter() - 0.5).abs() < 1e-12);
    assert!(!p.is_tangency());
}

#[test]
fn brep_blend_line_ops() {
    let mut l = BrepBlendLine::new();
    l.append(BlendPoint::new(0.0, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0], [0.0, 0.0]));
    l.append(BlendPoint::new(1.0, [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0], [1.0, 0.0]));
    assert_eq!(l.nb_points(), 2);
    assert!(l.point(0).is_none());
    assert!(l.point(1).is_some());
}

#[test]
fn blend_func_evol_rad() {
    let mut f = BlendFuncEvolRad::new(1, 2, 3);
    f.set_radius(0.5);
    assert!((f.initial_radius() - 0.5).abs() < 1e-12);
    assert_eq!(f.nb_equations(), 4);
}

#[test]
fn point_on_rst() {
    let p = PointOnRst::new(5, 0.3);
    assert_eq!(p.arc_id(), 5);
    assert!((p.parameter_on_arc() - 0.3).abs() < 1e-12);
}
