// FILE: rust/ironstream/crates/ironstream/tests/occt_bnd_tools_ext.rs
extern crate ironstream;
use ironstream::bnd_tools_ext::*;

// ---- BndAddCurveResult ------------------------------------------------------

#[test]
fn test_curve_result_new_is_void() {
    let r = BndAddCurveResult::new();
    assert!(r.is_void());
    assert_eq!(r.diagonal(), 0.0);
    assert_eq!(r.size(), [0.0, 0.0, 0.0]);
    assert_eq!(r.center(), [0.0, 0.0, 0.0]);
}

#[test]
fn test_curve_result_add_single_point() {
    let mut r = BndAddCurveResult::new();
    r.add_point([1.0, 2.0, 3.0]);
    assert!(!r.is_void());
    assert_eq!(r.min(), [1.0, 2.0, 3.0]);
    assert_eq!(r.max(), [1.0, 2.0, 3.0]);
    assert_eq!(r.size(), [0.0, 0.0, 0.0]);
    assert_eq!(r.center(), [1.0, 2.0, 3.0]);
}

#[test]
fn test_curve_result_add_two_points() {
    let mut r = BndAddCurveResult::new();
    r.add_point([0.0, 0.0, 0.0]);
    r.add_point([2.0, 4.0, 6.0]);
    assert_eq!(r.min(), [0.0, 0.0, 0.0]);
    assert_eq!(r.max(), [2.0, 4.0, 6.0]);
    assert_eq!(r.size(), [2.0, 4.0, 6.0]);
    let c = r.center();
    assert!((c[0] - 1.0).abs() < 1e-12);
    assert!((c[1] - 2.0).abs() < 1e-12);
    assert!((c[2] - 3.0).abs() < 1e-12);
}

#[test]
fn test_curve_result_add_point_expands_min_and_max() {
    let mut r = BndAddCurveResult::new();
    r.add_point([1.0, 1.0, 1.0]);
    r.add_point([-1.0, 3.0, 0.5]);
    assert_eq!(r.min(), [-1.0, 1.0, 0.5]);
    assert_eq!(r.max(), [1.0, 3.0, 1.0]);
}

#[test]
fn test_curve_result_diagonal_unit_cube() {
    let mut r = BndAddCurveResult::new();
    r.add_point([0.0, 0.0, 0.0]);
    r.add_point([1.0, 1.0, 1.0]);
    let d = r.diagonal();
    assert!((d - 3.0_f64.sqrt()).abs() < 1e-12);
}

#[test]
fn test_curve_result_enlarge_expands_all_sides() {
    let mut r = BndAddCurveResult::new();
    r.add_point([0.0, 0.0, 0.0]);
    r.add_point([2.0, 2.0, 2.0]);
    r.enlarge(1.0);
    assert_eq!(r.min(), [-1.0, -1.0, -1.0]);
    assert_eq!(r.max(), [3.0, 3.0, 3.0]);
}

#[test]
fn test_curve_result_enlarge_void_is_noop() {
    let mut r = BndAddCurveResult::new();
    r.enlarge(5.0);
    assert!(r.is_void());
}

#[test]
fn test_curve_result_enlarge_zero_is_noop() {
    let mut r = BndAddCurveResult::new();
    r.add_point([1.0, 2.0, 3.0]);
    r.enlarge(0.0);
    assert_eq!(r.min(), [1.0, 2.0, 3.0]);
    assert_eq!(r.max(), [1.0, 2.0, 3.0]);
}

// ---- BndAddSurfaceResult ----------------------------------------------------

#[test]
fn test_surface_result_new_is_void() {
    let r = BndAddSurfaceResult::new(10, 10);
    assert!(r.is_void());
    assert_eq!(r.u_nb_samples, 10);
    assert_eq!(r.v_nb_samples, 10);
    assert_eq!(r.diagonal(), 0.0);
    assert_eq!(r.center(), [0.0, 0.0, 0.0]);
}

#[test]
fn test_surface_result_add_single_point() {
    let mut r = BndAddSurfaceResult::new(5, 5);
    r.add_point([3.0, 4.0, 5.0]);
    assert!(!r.is_void());
    assert_eq!(r.min(), [3.0, 4.0, 5.0]);
    assert_eq!(r.max(), [3.0, 4.0, 5.0]);
    assert_eq!(r.center(), [3.0, 4.0, 5.0]);
}

#[test]
fn test_surface_result_add_two_points() {
    let mut r = BndAddSurfaceResult::new(4, 4);
    r.add_point([0.0, 0.0, 0.0]);
    r.add_point([6.0, 8.0, 10.0]);
    assert_eq!(r.min(), [0.0, 0.0, 0.0]);
    assert_eq!(r.max(), [6.0, 8.0, 10.0]);
    let c = r.center();
    assert!((c[0] - 3.0).abs() < 1e-12);
    assert!((c[1] - 4.0).abs() < 1e-12);
    assert!((c[2] - 5.0).abs() < 1e-12);
}

#[test]
fn test_surface_result_diagonal_unit_cube() {
    let mut r = BndAddSurfaceResult::new(2, 2);
    r.add_point([0.0, 0.0, 0.0]);
    r.add_point([1.0, 1.0, 1.0]);
    assert!((r.diagonal() - 3.0_f64.sqrt()).abs() < 1e-12);
}

#[test]
fn test_surface_result_add_point_expands_correctly() {
    let mut r = BndAddSurfaceResult::new(8, 8);
    r.add_point([-1.0, 2.0, -3.0]);
    r.add_point([4.0, -1.0, 5.0]);
    assert_eq!(r.min(), [-1.0, -1.0, -3.0]);
    assert_eq!(r.max(), [4.0, 2.0, 5.0]);
}

// ---- bnd_add_line -----------------------------------------------------------

#[test]
fn test_bnd_add_line_axis_aligned_x() {
    // Line along X axis from x=1 to x=5
    let r = bnd_add_line([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], 0.0, 4.0, 0.0);
    assert!(!r.is_void());
    assert_eq!(r.min(), [1.0, 0.0, 0.0]);
    assert_eq!(r.max(), [5.0, 0.0, 0.0]);
}

#[test]
fn test_bnd_add_line_with_gap() {
    let r = bnd_add_line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 0.0, 2.0, 0.5);
    assert!((r.min()[0] - (-0.5)).abs() < 1e-12);
    assert!((r.max()[0] - 2.5).abs() < 1e-12);
    assert!((r.min()[1] - (-0.5)).abs() < 1e-12);
    assert!((r.max()[1] - 0.5).abs() < 1e-12);
}

#[test]
fn test_bnd_add_line_diagonal() {
    // Line from (0,0,0) to (1,1,1)
    let r = bnd_add_line([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], 0.0, 1.0, 0.0);
    let s = r.size();
    assert!((s[0] - 1.0).abs() < 1e-12);
    assert!((s[1] - 1.0).abs() < 1e-12);
    assert!((s[2] - 1.0).abs() < 1e-12);
    assert!((r.diagonal() - 3.0_f64.sqrt()).abs() < 1e-12);
}

#[test]
fn test_bnd_add_line_reversed_range() {
    // u_start > u_end: both endpoints are still sampled, box is correct
    let r = bnd_add_line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 3.0, 1.0, 0.0);
    assert_eq!(r.min()[0], 1.0);
    assert_eq!(r.max()[0], 3.0);
}

#[test]
fn test_bnd_add_line_center() {
    let r = bnd_add_line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 0.0, 4.0, 0.0);
    let c = r.center();
    assert!((c[0] - 2.0).abs() < 1e-12);
    assert!(c[1].abs() < 1e-12);
    assert!(c[2].abs() < 1e-12);
}

// ---- bnd_add_circle ---------------------------------------------------------

#[test]
fn test_bnd_add_circle_unit_at_origin() {
    let r = bnd_add_circle([0.0, 0.0, 0.0], 1.0, 0.0);
    assert!(!r.is_void());
    assert_eq!(r.min(), [-1.0, -1.0, -1.0]);
    assert_eq!(r.max(), [1.0, 1.0, 1.0]);
}

#[test]
fn test_bnd_add_circle_center_offset() {
    let r = bnd_add_circle([3.0, 4.0, 5.0], 2.0, 0.0);
    assert_eq!(r.min(), [1.0, 2.0, 3.0]);
    assert_eq!(r.max(), [5.0, 6.0, 7.0]);
}

#[test]
fn test_bnd_add_circle_with_gap() {
    let r = bnd_add_circle([0.0, 0.0, 0.0], 3.0, 0.5);
    assert_eq!(r.min(), [-3.5, -3.5, -3.5]);
    assert_eq!(r.max(), [3.5, 3.5, 3.5]);
}

#[test]
fn test_bnd_add_circle_center_is_correct() {
    let r = bnd_add_circle([1.0, 2.0, 3.0], 5.0, 0.0);
    let c = r.center();
    assert!((c[0] - 1.0).abs() < 1e-12);
    assert!((c[1] - 2.0).abs() < 1e-12);
    assert!((c[2] - 3.0).abs() < 1e-12);
}

#[test]
fn test_bnd_add_circle_diagonal() {
    // Cube side = 2*r, diagonal = 2*r*sqrt(3)
    let r = bnd_add_circle([0.0, 0.0, 0.0], 1.0, 0.0);
    assert!((r.diagonal() - 2.0_f64 * 3.0_f64.sqrt()).abs() < 1e-12);
}

#[test]
fn test_bnd_add_circle_zero_radius() {
    let r = bnd_add_circle([5.0, 6.0, 7.0], 0.0, 0.0);
    assert!(!r.is_void());
    assert_eq!(r.min(), [5.0, 6.0, 7.0]);
    assert_eq!(r.max(), [5.0, 6.0, 7.0]);
    assert_eq!(r.diagonal(), 0.0);
}
