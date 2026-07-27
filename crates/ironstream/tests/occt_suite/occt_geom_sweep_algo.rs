// FILE: tests/occt_geom_sweep_algo.rs
extern crate ironstream;
use ironstream::geom_sweep_algo::*;

// ── SweepShape ────────────────────────────────────────────────────────────────

#[test]
fn test_sweep_shape_lateral_faces_basic() {
    let s = SweepShape::new(4, 3, 10.0);
    assert_eq!(s.nb_lateral_faces(), 12);
}

#[test]
fn test_sweep_shape_lateral_faces_single_edge() {
    let s = SweepShape::new(1, 1, 1.0);
    assert_eq!(s.nb_lateral_faces(), 1);
}

#[test]
fn test_sweep_shape_lateral_faces_zero_profile() {
    let s = SweepShape::new(0, 5, 5.0);
    assert_eq!(s.nb_lateral_faces(), 0);
}

#[test]
fn test_sweep_shape_cap_faces_default_is_shell() {
    let s = SweepShape::new(6, 4, 3.0);
    assert!(!s.is_solid());
    assert_eq!(s.nb_top_bottom_faces(), 0);
}

#[test]
fn test_sweep_shape_cap_faces_after_set_solid() {
    let mut s = SweepShape::new(6, 4, 3.0);
    s.set_solid(true);
    assert!(s.is_solid());
    assert_eq!(s.nb_top_bottom_faces(), 2);
}

#[test]
fn test_sweep_shape_set_solid_toggle() {
    let mut s = SweepShape::new(4, 2, 5.0);
    s.set_solid(true);
    assert!(s.is_solid());
    s.set_solid(false);
    assert!(!s.is_solid());
    assert_eq!(s.nb_top_bottom_faces(), 0);
}

#[test]
fn test_sweep_shape_total_faces_shell() {
    let s = SweepShape::new(4, 3, 10.0);
    // 4*3 lateral + 0 caps
    assert_eq!(s.nb_total_faces(), 12);
}

#[test]
fn test_sweep_shape_total_faces_solid() {
    let mut s = SweepShape::new(4, 3, 10.0);
    s.set_solid(true);
    // 4*3 lateral + 2 caps
    assert_eq!(s.nb_total_faces(), 14);
}

#[test]
fn test_sweep_shape_spine_length_stored() {
    let s = SweepShape::new(3, 5, 7.5);
    assert!((s.spine_length() - 7.5).abs() < 1e-12);
}

#[test]
fn test_sweep_shape_spine_length_zero() {
    let s = SweepShape::new(3, 5, 0.0);
    assert!((s.spine_length()).abs() < 1e-12);
}

#[test]
fn test_sweep_shape_volume_approx_stub() {
    // volume_approx = spine_length * nb_profile_edges
    let s = SweepShape::new(4, 3, 2.5);
    assert!((s.volume_approx() - 10.0).abs() < 1e-12);
}

#[test]
fn test_sweep_shape_volume_approx_zero_profile() {
    let s = SweepShape::new(0, 3, 5.0);
    assert!((s.volume_approx()).abs() < 1e-12);
}

#[test]
fn test_sweep_shape_fields_match_constructor() {
    let s = SweepShape::new(5, 7, 3.14);
    assert_eq!(s.nb_profile_edges, 5);
    assert_eq!(s.nb_spine_edges, 7);
    assert!((s.spine_length - 3.14).abs() < 1e-12);
    assert!(!s.is_solid);
}

// ── SweepAlgoResult ───────────────────────────────────────────────────────────

#[test]
fn test_sweep_algo_result_new_not_done() {
    let r = SweepAlgoResult::new();
    assert!(!r.is_done());
    assert_eq!(r.nb_shapes(), 0);
}

#[test]
fn test_sweep_algo_result_default_not_done() {
    let r = SweepAlgoResult::default();
    assert!(!r.is_done());
    assert_eq!(r.nb_shapes(), 0);
}

#[test]
fn test_sweep_algo_result_build_sets_done() {
    let mut r = SweepAlgoResult::new();
    r.build(5);
    assert!(r.is_done());
    assert_eq!(r.nb_shapes(), 5);
}

#[test]
fn test_sweep_algo_result_build_zero_shapes() {
    let mut r = SweepAlgoResult::new();
    r.build(0);
    assert!(r.is_done());
    assert_eq!(r.nb_shapes(), 0);
}

#[test]
fn test_sweep_algo_result_rebuild_updates_count() {
    let mut r = SweepAlgoResult::new();
    r.build(3);
    r.build(9);
    assert!(r.is_done());
    assert_eq!(r.nb_shapes(), 9);
}

// ── SweepLinear ───────────────────────────────────────────────────────────────

#[test]
fn test_sweep_linear_direction_stored() {
    let s = SweepShape::new(4, 2, 5.0);
    let l = SweepLinear::new(s, [0.0, 0.0, 1.0]);
    let d = l.direction();
    assert!(d[0].abs() < 1e-12);
    assert!(d[1].abs() < 1e-12);
    assert!((d[2] - 1.0).abs() < 1e-12);
}

#[test]
fn test_sweep_linear_arbitrary_direction_stored() {
    let s = SweepShape::new(3, 1, 2.0);
    let l = SweepLinear::new(s, [1.0, 2.0, 3.0]);
    let d = l.direction();
    assert!((d[0] - 1.0).abs() < 1e-12);
    assert!((d[1] - 2.0).abs() < 1e-12);
    assert!((d[2] - 3.0).abs() < 1e-12);
}

#[test]
fn test_sweep_linear_not_done_before_build() {
    let s = SweepShape::new(4, 2, 5.0);
    let l = SweepLinear::new(s, [1.0, 0.0, 0.0]);
    assert!(!l.is_done());
    assert!(!l.result().is_done());
}

#[test]
fn test_sweep_linear_done_after_build() {
    let s = SweepShape::new(4, 2, 5.0);
    let mut l = SweepLinear::new(s, [0.0, 1.0, 0.0]);
    l.build();
    assert!(l.is_done());
    assert!(l.result().is_done());
}

#[test]
fn test_sweep_linear_nb_shapes_shell() {
    // shell: nb_total_faces = 4*2=8, plus nb_profile_edges=4 => 12
    let s = SweepShape::new(4, 2, 5.0);
    let mut l = SweepLinear::new(s, [0.0, 0.0, 1.0]);
    l.build();
    assert_eq!(l.result().nb_shapes(), 12);
}

#[test]
fn test_sweep_linear_nb_shapes_solid() {
    // solid: nb_total_faces = 4*2+2=10, plus nb_profile_edges=4 => 14
    let mut s = SweepShape::new(4, 2, 5.0);
    s.set_solid(true);
    let mut l = SweepLinear::new(s, [0.0, 0.0, 1.0]);
    l.build();
    assert_eq!(l.result().nb_shapes(), 14);
}

#[test]
fn test_sweep_linear_nb_shapes_triangle_profile() {
    // triangle shell: 3 lateral * 1 spine + 0 caps + 3 profile edges = 6
    let s = SweepShape::new(3, 1, 8.0);
    let mut l = SweepLinear::new(s, [1.0, 0.0, 0.0]);
    l.build();
    assert_eq!(l.result().nb_shapes(), 6);
}

#[test]
fn test_sweep_linear_nb_shapes_triangle_solid() {
    // triangle solid: 3*1+2 caps + 3 profile edges = 8
    let mut s = SweepShape::new(3, 1, 8.0);
    s.set_solid(true);
    let mut l = SweepLinear::new(s, [1.0, 0.0, 0.0]);
    l.build();
    assert_eq!(l.result().nb_shapes(), 8);
}

#[test]
fn test_sweep_linear_result_ref_consistent() {
    let s = SweepShape::new(6, 4, 1.0);
    let mut l = SweepLinear::new(s, [0.0, 1.0, 0.0]);
    l.build();
    // nb_total_faces = 6*4=24, plus 6 profile edges = 30
    let r = l.result();
    assert!(r.is_done());
    assert_eq!(r.nb_shapes(), 30);
}
