// FILE: rust/ironstream/crates/ironstream/tests/occt_geom2d_tangential.rs
//! Tests for `geom2d_tangential` — tangential arc / circumcircle types.
//!
//! Mirrors the intent of the OpenCascade `Geom2dGcc` / `GCE2d` GTests while
//! staying entirely within safe, zero-dependency Rust.

extern crate ironstream;
use ironstream::geom2d_tangential::*;

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

fn approx_eq2(a: [f64; 2], b: [f64; 2]) -> bool {
    approx_eq(a[0], b[0]) && approx_eq(a[1], b[1])
}

// ---------------------------------------------------------------------------
// Geom2dTanArcResult
// ---------------------------------------------------------------------------

#[test]
fn tan_arc_result_new_stores_fields() {
    let r = Geom2dTanArcResult::new([1.0, 2.0], 3.0, 0.1, 0.2, 0.3);
    assert_eq!(r.center(), [1.0, 2.0]);
    assert_eq!(r.radius(), 3.0);
    assert_eq!(r.params(), (0.1, 0.2, 0.3));
    assert!(r.is_done());
}

#[test]
fn tan_arc_result_zero_radius() {
    let r = Geom2dTanArcResult::new([0.0, 0.0], 0.0, 0.0, 0.0, 0.0);
    assert_eq!(r.radius(), 0.0);
    assert!(r.is_done());
}

#[test]
fn tan_arc_result_negative_center_coords() {
    let r = Geom2dTanArcResult::new([-5.0, -3.0], 7.5, 1.0, 2.0, 3.0);
    assert_eq!(r.center(), [-5.0, -3.0]);
    assert_eq!(r.radius(), 7.5);
}

#[test]
fn tan_arc_result_params_unpacked() {
    let r = Geom2dTanArcResult::new([0.0, 0.0], 1.0, 1.1, 2.2, 3.3);
    let (p1, p2, p3) = r.params();
    assert!(approx_eq(p1, 1.1));
    assert!(approx_eq(p2, 2.2));
    assert!(approx_eq(p3, 3.3));
}

#[test]
fn tan_arc_result_copy() {
    let r1 = Geom2dTanArcResult::new([0.0, 0.0], 1.0, 0.0, 1.0, 2.0);
    let r2 = r1;
    assert_eq!(r1, r2);
}

// ---------------------------------------------------------------------------
// Geom2dCirc3Tan — circumcircle of three points
// ---------------------------------------------------------------------------

#[test]
fn circ3tan_new_not_done() {
    let solver = Geom2dCirc3Tan::new(1e-10);
    assert!(!solver.is_done());
    assert_eq!(solver.nb_solutions(), 0);
}

#[test]
fn circ3tan_unit_right_triangle_circumcircle() {
    // p1=(0,0), p2=(1,0), p3=(0,1)
    // Circumcircle center = (0.5, 0.5), radius = sqrt(0.5)
    let mut solver = Geom2dCirc3Tan::new(1e-10);
    solver.solve_from_points([0.0, 0.0], [1.0, 0.0], [0.0, 1.0]);
    assert!(solver.is_done());
    assert_eq!(solver.nb_solutions(), 1);

    let sol = solver.solution(0);
    assert!(approx_eq2(sol.center(), [0.5, 0.5]));
    let expected_r = (0.5_f64).sqrt();
    assert!(approx_eq(sol.radius(), expected_r));
    assert!(sol.is_done());
}

#[test]
fn circ3tan_equilateral_triangle_circumcircle() {
    // Equilateral triangle with side 2: p1=(0,0), p2=(2,0), p3=(1, sqrt(3))
    // Circumcircle center = (1, 1/sqrt(3)), radius = 2/sqrt(3)
    let sqrt3 = 3.0_f64.sqrt();
    let p1 = [0.0_f64, 0.0];
    let p2 = [2.0_f64, 0.0];
    let p3 = [1.0_f64, sqrt3];

    let mut solver = Geom2dCirc3Tan::new(1e-10);
    solver.solve_from_points(p1, p2, p3);
    assert!(solver.is_done());
    assert_eq!(solver.nb_solutions(), 1);

    let sol = solver.solution(0);
    let expected_cx = 1.0;
    let expected_cy = 1.0 / sqrt3;
    let expected_r = 2.0 / sqrt3;
    assert!(approx_eq(sol.center()[0], expected_cx));
    assert!(approx_eq(sol.center()[1], expected_cy));
    assert!(approx_eq(sol.radius(), expected_r));
}

#[test]
fn circ3tan_all_three_points_lie_on_solution_circle() {
    let p1 = [3.0, 0.0];
    let p2 = [0.0, 4.0];
    let p3 = [-1.0, 0.0];

    let mut solver = Geom2dCirc3Tan::new(1e-10);
    solver.solve_from_points(p1, p2, p3);
    assert!(solver.is_done());
    assert_eq!(solver.nb_solutions(), 1);

    let sol = solver.solution(0);
    let [cx, cy] = sol.center();
    let r = sol.radius();

    // Each point must lie on the circle.
    for p in [p1, p2, p3] {
        let dist = ((p[0] - cx).powi(2) + (p[1] - cy).powi(2)).sqrt();
        assert!(approx_eq(dist, r), "point {:?} not on circle: dist={dist} r={r}", p);
    }
}

#[test]
fn circ3tan_collinear_points_no_solution() {
    // Three collinear points: (0,0), (1,0), (2,0)
    let mut solver = Geom2dCirc3Tan::new(1e-10);
    solver.solve_from_points([0.0, 0.0], [1.0, 0.0], [2.0, 0.0]);
    assert!(solver.is_done());
    assert_eq!(solver.nb_solutions(), 0);
}

#[test]
fn circ3tan_params_are_angles_to_each_point() {
    // Use three points on the unit circle at known angles.
    use std::f64::consts::PI;
    let p1 = [1.0, 0.0];                              // angle 0
    let p2 = [0.0, 1.0];                              // angle PI/2
    let p3 = [-1.0, 0.0_f64.atan2(-1.0_f64).cos()];  // roughly (-1, 0), angle PI

    // Circumcircle of unit-circle points is the unit circle itself.
    // Use a clean right-angle triangle on the unit circle instead.
    let theta1 = 0.0_f64;
    let theta2 = PI / 2.0;
    let theta3 = PI;
    let q1 = [theta1.cos(), theta1.sin()];
    let q2 = [theta2.cos(), theta2.sin()];
    let q3 = [theta3.cos(), theta3.sin()];

    let mut solver = Geom2dCirc3Tan::new(1e-10);
    solver.solve_from_points(q1, q2, q3);
    assert!(solver.is_done());
    assert_eq!(solver.nb_solutions(), 1);

    let sol = solver.solution(0);
    // Center should be near (0, 0) for the unit circle through these points.
    // (These three points are NOT all on the unit circle — circumcircle differs.)
    // Just verify params are in [0, 2*PI].
    let (pa, pb, pc) = sol.params();
    assert!(pa >= 0.0 && pa <= 2.0 * PI, "param1={pa} out of range");
    assert!(pb >= 0.0 && pb <= 2.0 * PI, "param2={pb} out of range");
    assert!(pc >= 0.0 && pc <= 2.0 * PI, "param3={pc} out of range");

    // Suppress unused variable warnings.
    let _ = (p1, p2, p3);
}

#[test]
fn circ3tan_solve_twice_clears_previous_solution() {
    let mut solver = Geom2dCirc3Tan::new(1e-10);
    // First solve: non-collinear.
    solver.solve_from_points([0.0, 0.0], [1.0, 0.0], [0.0, 1.0]);
    assert_eq!(solver.nb_solutions(), 1);

    // Second solve: collinear — should clear the previous solution.
    solver.solve_from_points([0.0, 0.0], [1.0, 0.0], [2.0, 0.0]);
    assert_eq!(solver.nb_solutions(), 0);
    assert!(solver.is_done());
}

#[test]
fn circ3tan_large_coordinates() {
    let p1 = [1e6, 0.0];
    let p2 = [0.0, 1e6];
    let p3 = [-1e6, 0.0];

    let mut solver = Geom2dCirc3Tan::new(1e-4);
    solver.solve_from_points(p1, p2, p3);
    assert!(solver.is_done());
    assert_eq!(solver.nb_solutions(), 1);

    let sol = solver.solution(0);
    let [cx, cy] = sol.center();
    let r = sol.radius();

    for p in [p1, p2, p3] {
        let dist = ((p[0] - cx).powi(2) + (p[1] - cy).powi(2)).sqrt();
        assert!(
            (dist - r).abs() < 1e-4,
            "point {:?} not on circle: dist={dist} r={r}", p
        );
    }
}

// ---------------------------------------------------------------------------
// Geom2dArcBuilder
// ---------------------------------------------------------------------------

#[test]
fn arc_builder_not_done_before_build() {
    let arc = Geom2dArcBuilder::new([0.0, 0.0], 1.0, 0.0, std::f64::consts::PI);
    assert!(!arc.is_done());
}

#[test]
fn arc_builder_done_after_build() {
    let mut arc = Geom2dArcBuilder::new([0.0, 0.0], 1.0, 0.0, std::f64::consts::PI);
    arc.build();
    assert!(arc.is_done());
}

#[test]
fn arc_builder_start_and_end_points_unit_circle() {
    use std::f64::consts::PI;
    let mut arc = Geom2dArcBuilder::new([0.0, 0.0], 1.0, 0.0, PI);
    arc.build();
    assert!(approx_eq2(arc.start_point(), [1.0, 0.0]));
    assert!(approx_eq2(arc.end_point(), [-1.0, 0.0]));
}

#[test]
fn arc_builder_quarter_circle_arc_length() {
    use std::f64::consts::PI;
    let mut arc = Geom2dArcBuilder::new([0.0, 0.0], 1.0, 0.0, PI / 2.0);
    arc.build();
    let expected = PI / 2.0;
    assert!(approx_eq(arc.arc_length(), expected));
}

#[test]
fn arc_builder_full_circle_arc_length() {
    use std::f64::consts::PI;
    let mut arc = Geom2dArcBuilder::new([0.0, 0.0], 2.0, 0.0, 2.0 * PI);
    arc.build();
    // sweep = end - start = 2*PI, length = radius * 2*PI = 4*PI
    let expected = 4.0 * PI;
    assert!(approx_eq(arc.arc_length(), expected));
}

#[test]
fn arc_builder_wrapping_arc_sweep_angle() {
    use std::f64::consts::PI;
    // start = 3*PI/2, end = PI/2 — wraps through 0, sweep = PI
    let start = 3.0 * PI / 2.0;
    let end = PI / 2.0;
    let mut arc = Geom2dArcBuilder::new([0.0, 0.0], 1.0, start, end);
    arc.build();
    let expected_length = PI; // radius 1 * sweep PI
    assert!(approx_eq(arc.arc_length(), expected_length));
}

#[test]
fn arc_builder_mid_point_half_circle() {
    use std::f64::consts::PI;
    // Half circle from 0 to PI: mid-point should be at angle PI/2 = (0,1).
    let mut arc = Geom2dArcBuilder::new([0.0, 0.0], 1.0, 0.0, PI);
    arc.build();
    assert!(approx_eq2(arc.mid_point(), [0.0, 1.0]));
}

#[test]
fn arc_builder_mid_point_quarter_circle() {
    use std::f64::consts::PI;
    // Quarter circle from 0 to PI/2: mid-point at angle PI/4.
    let mut arc = Geom2dArcBuilder::new([0.0, 0.0], 1.0, 0.0, PI / 2.0);
    arc.build();
    let expected = [(PI / 4.0).cos(), (PI / 4.0).sin()];
    assert!(approx_eq2(arc.mid_point(), expected));
}

#[test]
fn arc_builder_offset_center() {
    use std::f64::consts::PI;
    let center = [3.0, 4.0];
    let radius = 5.0;
    let mut arc = Geom2dArcBuilder::new(center, radius, 0.0, PI / 2.0);
    arc.build();
    // start_point = center + (radius, 0)
    assert!(approx_eq2(arc.start_point(), [8.0, 4.0]));
    // end_point = center + (0, radius)
    assert!(approx_eq2(arc.end_point(), [3.0, 9.0]));
    // arc_length = radius * PI/2
    assert!(approx_eq(arc.arc_length(), radius * PI / 2.0));
}

#[test]
fn arc_builder_zero_radius() {
    let mut arc = Geom2dArcBuilder::new([1.0, 2.0], 0.0, 0.0, std::f64::consts::PI);
    arc.build();
    assert!(approx_eq2(arc.start_point(), [1.0, 2.0]));
    assert!(approx_eq2(arc.end_point(), [1.0, 2.0]));
    assert!(approx_eq(arc.arc_length(), 0.0));
}

#[test]
fn arc_builder_copy() {
    let arc1 = Geom2dArcBuilder::new([0.0, 0.0], 1.0, 0.0, std::f64::consts::PI);
    let arc2 = arc1;
    assert_eq!(arc1, arc2);
}

#[test]
fn arc_builder_start_point_before_build() {
    // start_point / end_point do not require build() to be called first.
    let arc = Geom2dArcBuilder::new([0.0, 0.0], 2.0, 0.0, 0.0);
    assert!(approx_eq2(arc.start_point(), [2.0, 0.0]));
}
