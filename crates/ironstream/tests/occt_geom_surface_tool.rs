// FILE: tests/occt_geom_surface_tool.rs

extern crate ironstream;
use ironstream::geom_surface_tool::*;
use std::f64::consts::PI;

const EPS: f64 = 1e-12;

fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() < EPS,
        "expected {a} ≈ {b} (tol {EPS}), diff={}",
        (a - b).abs()
    );
}

fn vec_near(a: [f64; 3], b: [f64; 3]) {
    near(a[0], b[0]);
    near(a[1], b[1]);
    near(a[2], b[2]);
}

// ── GeomSurfaceType ───────────────────────────────────────────────────────────

#[test]
fn surface_type_all_variants_distinct() {
    let all = [
        GeomSurfaceType::Plane,
        GeomSurfaceType::Cylinder,
        GeomSurfaceType::Cone,
        GeomSurfaceType::Sphere,
        GeomSurfaceType::Torus,
        GeomSurfaceType::BezierSurface,
        GeomSurfaceType::BSplineSurface,
        GeomSurfaceType::RevolutionSurface,
        GeomSurfaceType::ExtrudedSurface,
        GeomSurfaceType::OffsetSurface,
        GeomSurfaceType::OtherSurface,
    ];
    for i in 0..all.len() {
        for j in 0..all.len() {
            if i == j {
                assert_eq!(all[i], all[j], "variant {i} should equal itself");
            } else {
                assert_ne!(all[i], all[j], "variants {i} and {j} should differ");
            }
        }
    }
}

#[test]
fn surface_type_is_copy_and_clone() {
    let t = GeomSurfaceType::Torus;
    let copied = t;
    let cloned = t.clone();
    assert_eq!(t, copied);
    assert_eq!(t, cloned);
}

#[test]
fn surface_type_debug_strings() {
    assert_eq!(format!("{:?}", GeomSurfaceType::Plane), "Plane");
    assert_eq!(format!("{:?}", GeomSurfaceType::Cylinder), "Cylinder");
    assert_eq!(format!("{:?}", GeomSurfaceType::Cone), "Cone");
    assert_eq!(format!("{:?}", GeomSurfaceType::Sphere), "Sphere");
    assert_eq!(format!("{:?}", GeomSurfaceType::Torus), "Torus");
    assert_eq!(format!("{:?}", GeomSurfaceType::BezierSurface), "BezierSurface");
    assert_eq!(format!("{:?}", GeomSurfaceType::BSplineSurface), "BSplineSurface");
    assert_eq!(format!("{:?}", GeomSurfaceType::RevolutionSurface), "RevolutionSurface");
    assert_eq!(format!("{:?}", GeomSurfaceType::ExtrudedSurface), "ExtrudedSurface");
    assert_eq!(format!("{:?}", GeomSurfaceType::OffsetSurface), "OffsetSurface");
    assert_eq!(format!("{:?}", GeomSurfaceType::OtherSurface), "OtherSurface");
}

// ── GeomSurfaceInfo ───────────────────────────────────────────────────────────

#[test]
fn info_stores_all_fields_correctly() {
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Sphere, 0.0, 2.0 * PI, -PI / 2.0, PI / 2.0);
    assert_eq!(info.surface_type(), GeomSurfaceType::Sphere);
    near(info.u_first(), 0.0);
    near(info.u_last(), 2.0 * PI);
    near(info.v_first(), -PI / 2.0);
    near(info.v_last(), PI / 2.0);
    assert_eq!(info.continuity(), 0);
}

#[test]
fn info_set_continuity_roundtrip() {
    let mut info = GeomSurfaceInfo::new(GeomSurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
    for c in [0u8, 1, 2, 3, 255] {
        info.set_continuity(c);
        assert_eq!(info.continuity(), c, "continuity should be {c}");
    }
}

#[test]
fn info_continuity_defaults_to_zero() {
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Cylinder, 0.0, 2.0 * PI, -10.0, 10.0);
    assert_eq!(info.continuity(), 0);
}

#[test]
fn info_is_u_closed_always_false() {
    for st in [
        GeomSurfaceType::Plane,
        GeomSurfaceType::Cylinder,
        GeomSurfaceType::Sphere,
        GeomSurfaceType::Torus,
        GeomSurfaceType::BSplineSurface,
    ] {
        let info = GeomSurfaceInfo::new(st, 0.0, 1.0, 0.0, 1.0);
        assert!(!info.is_u_closed(), "is_u_closed should be false for {st:?}");
    }
}

#[test]
fn info_is_v_closed_always_false() {
    for st in [
        GeomSurfaceType::Plane,
        GeomSurfaceType::Cylinder,
        GeomSurfaceType::Sphere,
        GeomSurfaceType::Torus,
        GeomSurfaceType::BSplineSurface,
    ] {
        let info = GeomSurfaceInfo::new(st, 0.0, 1.0, 0.0, 1.0);
        assert!(!info.is_v_closed(), "is_v_closed should be false for {st:?}");
    }
}

#[test]
fn info_is_u_periodic_always_false() {
    for st in [
        GeomSurfaceType::Plane,
        GeomSurfaceType::Cylinder,
        GeomSurfaceType::Sphere,
        GeomSurfaceType::Torus,
    ] {
        let info = GeomSurfaceInfo::new(st, 0.0, 1.0, 0.0, 1.0);
        assert!(!info.is_u_periodic(), "is_u_periodic should be false for {st:?}");
    }
}

#[test]
fn info_is_v_periodic_always_false() {
    for st in [
        GeomSurfaceType::Plane,
        GeomSurfaceType::Cylinder,
        GeomSurfaceType::Sphere,
        GeomSurfaceType::Torus,
    ] {
        let info = GeomSurfaceInfo::new(st, 0.0, 1.0, 0.0, 1.0);
        assert!(!info.is_v_periodic(), "is_v_periodic should be false for {st:?}");
    }
}

#[test]
fn info_is_copy() {
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Cone, 0.0, PI, -5.0, 5.0);
    let info2 = info;
    assert_eq!(info.surface_type(), info2.surface_type());
    near(info.u_first(), info2.u_first());
    near(info.v_last(), info2.v_last());
}

#[test]
fn info_surface_type_accessor_correct() {
    for st in [
        GeomSurfaceType::RevolutionSurface,
        GeomSurfaceType::ExtrudedSurface,
        GeomSurfaceType::OffsetSurface,
        GeomSurfaceType::OtherSurface,
    ] {
        let info = GeomSurfaceInfo::new(st, 0.0, 1.0, 0.0, 1.0);
        assert_eq!(info.surface_type(), st);
    }
}

// ── GeomSurfaceEvaluator — Plane ─────────────────────────────────────────────

#[test]
fn plane_value_at_origin() {
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
    let ev = GeomSurfaceEvaluator::new(info);
    vec_near(ev.value(0.0, 0.0), [0.0, 0.0, 0.0]);
}

#[test]
fn plane_value_matches_uv() {
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Plane, -5.0, 5.0, -5.0, 5.0);
    let ev = GeomSurfaceEvaluator::new(info);
    for (u, v) in [(1.0, 2.0), (-3.5, 0.0), (0.0, -7.0), (100.0, -100.0)] {
        let pt = ev.value(u, v);
        near(pt[0], u);
        near(pt[1], v);
        near(pt[2], 0.0);
    }
}

#[test]
fn plane_d1_point_matches_value() {
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
    let ev = GeomSurfaceEvaluator::new(info);
    let (u, v) = (3.0, -1.5);
    let (pt, _, _) = ev.d1(u, v);
    vec_near(pt, ev.value(u, v));
}

#[test]
fn plane_d1_du_is_unit_x() {
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
    let ev = GeomSurfaceEvaluator::new(info);
    let (_, du, _) = ev.d1(0.5, 0.5);
    vec_near(du, [1.0, 0.0, 0.0]);
}

#[test]
fn plane_d1_dv_is_unit_y() {
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
    let ev = GeomSurfaceEvaluator::new(info);
    let (_, _, dv) = ev.d1(0.5, 0.5);
    vec_near(dv, [0.0, 1.0, 0.0]);
}

#[test]
fn plane_normal_is_z_axis() {
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Plane, -10.0, 10.0, -10.0, 10.0);
    let ev = GeomSurfaceEvaluator::new(info);
    for (u, v) in [(0.0, 0.0), (3.0, -2.0), (-5.0, 5.0)] {
        let n = ev.normal(u, v);
        vec_near(n, [0.0, 0.0, 1.0]);
    }
}

#[test]
fn plane_normal_unit_length() {
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
    let ev = GeomSurfaceEvaluator::new(info);
    let n = ev.normal(0.0, 0.0);
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    near(len, 1.0);
}

#[test]
fn plane_normal_perpendicular_to_d1() {
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
    let ev = GeomSurfaceEvaluator::new(info);
    let (_, du, dv) = ev.d1(0.3, 0.7);
    let n = ev.normal(0.3, 0.7);
    let dot_du = n[0] * du[0] + n[1] * du[1] + n[2] * du[2];
    let dot_dv = n[0] * dv[0] + n[1] * dv[1] + n[2] * dv[2];
    assert!(dot_du.abs() < EPS, "normal not perp to du: {dot_du}");
    assert!(dot_dv.abs() < EPS, "normal not perp to dv: {dot_dv}");
}

// ── GeomSurfaceEvaluator — Cylinder ──────────────────────────────────────────

#[test]
fn cylinder_value_at_u0() {
    // value(0, v) = [v, 0, 0]
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Cylinder, 0.0, 2.0 * PI, 0.0, 5.0);
    let ev = GeomSurfaceEvaluator::new(info);
    for v in [0.5, 1.0, 2.5, 5.0] {
        let pt = ev.value(0.0, v);
        near(pt[0], v);
        near(pt[1], 0.0);
        near(pt[2], 0.0);
    }
}

#[test]
fn cylinder_value_at_u_pi_over_2() {
    // value(PI/2, v) = [0, v, 0]
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Cylinder, 0.0, 2.0 * PI, 0.0, 5.0);
    let ev = GeomSurfaceEvaluator::new(info);
    let pt = ev.value(PI / 2.0, 3.0);
    assert!((pt[0]).abs() < 1e-12, "x should be ~0, got {}", pt[0]);
    near(pt[1], 3.0);
    near(pt[2], 0.0);
}

#[test]
fn cylinder_value_at_u_pi() {
    // value(PI, v) = [-v, 0, 0]
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Cylinder, 0.0, 2.0 * PI, 0.0, 5.0);
    let ev = GeomSurfaceEvaluator::new(info);
    let pt = ev.value(PI, 2.0);
    near(pt[0], -2.0);
    assert!((pt[1]).abs() < 1e-12, "y should be ~0, got {}", pt[1]);
    near(pt[2], 0.0);
}

#[test]
fn cylinder_value_formula_matches_expected() {
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Cylinder, 0.0, 2.0 * PI, 0.0, 10.0);
    let ev = GeomSurfaceEvaluator::new(info);
    let u = 1.2;
    let v = 4.0;
    let pt = ev.value(u, v);
    near(pt[0], u.cos() * v);
    near(pt[1], u.sin() * v);
    near(pt[2], 0.0);
}

#[test]
fn cylinder_d1_point_matches_value() {
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Cylinder, 0.0, 2.0 * PI, 0.0, 10.0);
    let ev = GeomSurfaceEvaluator::new(info);
    let (u, v) = (0.9, 3.5);
    let (pt, _, _) = ev.d1(u, v);
    vec_near(pt, ev.value(u, v));
}

#[test]
fn cylinder_d1_du_formula() {
    // d/du [cos(u)*v, sin(u)*v, 0] = [-sin(u)*v, cos(u)*v, 0]
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Cylinder, 0.0, 2.0 * PI, 0.0, 10.0);
    let ev = GeomSurfaceEvaluator::new(info);
    let u = 0.6;
    let v = 2.0;
    let (_, du, _) = ev.d1(u, v);
    near(du[0], -u.sin() * v);
    near(du[1], u.cos() * v);
    near(du[2], 0.0);
}

#[test]
fn cylinder_d1_dv_formula() {
    // d/dv [cos(u)*v, sin(u)*v, 0] = [cos(u), sin(u), 0]
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Cylinder, 0.0, 2.0 * PI, 0.0, 10.0);
    let ev = GeomSurfaceEvaluator::new(info);
    let u = 1.1;
    let v = 5.0;
    let (_, _, dv) = ev.d1(u, v);
    near(dv[0], u.cos());
    near(dv[1], u.sin());
    near(dv[2], 0.0);
}

#[test]
fn cylinder_normal_is_unit_length() {
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Cylinder, 0.0, 2.0 * PI, 0.0, 10.0);
    let ev = GeomSurfaceEvaluator::new(info);
    for u in [0.0, PI / 6.0, PI / 4.0, PI / 3.0, PI / 2.0, PI, 2.0 * PI] {
        let n = ev.normal(u, 1.0);
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        assert!((len - 1.0).abs() < 1e-12, "normal not unit at u={u}: len={len}");
    }
}

#[test]
fn cylinder_normal_perpendicular_to_du_and_dv() {
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Cylinder, 0.0, 2.0 * PI, 0.0, 10.0);
    let ev = GeomSurfaceEvaluator::new(info);
    for (u, v) in [(0.3, 1.0), (1.0, 2.5), (PI / 3.0, 0.5)] {
        let (_, du, dv) = ev.d1(u, v);
        let n = ev.normal(u, v);
        let dot_du = n[0] * du[0] + n[1] * du[1] + n[2] * du[2];
        let dot_dv = n[0] * dv[0] + n[1] * dv[1] + n[2] * dv[2];
        assert!(dot_du.abs() < 1e-10, "normal not perp to du at u={u},v={v}: {dot_du}");
        assert!(dot_dv.abs() < 1e-10, "normal not perp to dv at u={u},v={v}: {dot_dv}");
    }
}

// ── GeomSurfaceEvaluator — fallback types ────────────────────────────────────

#[test]
fn fallback_types_use_plane_formula() {
    for st in [
        GeomSurfaceType::Sphere,
        GeomSurfaceType::Torus,
        GeomSurfaceType::Cone,
        GeomSurfaceType::BezierSurface,
        GeomSurfaceType::BSplineSurface,
        GeomSurfaceType::RevolutionSurface,
        GeomSurfaceType::ExtrudedSurface,
        GeomSurfaceType::OffsetSurface,
        GeomSurfaceType::OtherSurface,
    ] {
        let info = GeomSurfaceInfo::new(st, 0.0, 1.0, 0.0, 1.0);
        let ev = GeomSurfaceEvaluator::new(info);
        let (u, v) = (1.5, -3.0);
        let pt = ev.value(u, v);
        assert!((pt[0] - u).abs() < EPS, "{st:?}: x should be {u}, got {}", pt[0]);
        assert!((pt[1] - v).abs() < EPS, "{st:?}: y should be {v}, got {}", pt[1]);
        assert!((pt[2]).abs() < EPS, "{st:?}: z should be 0, got {}", pt[2]);
    }
}

#[test]
fn fallback_types_d1_gives_plane_partials() {
    for st in [
        GeomSurfaceType::Sphere,
        GeomSurfaceType::Cone,
        GeomSurfaceType::OtherSurface,
    ] {
        let info = GeomSurfaceInfo::new(st, 0.0, 1.0, 0.0, 1.0);
        let ev = GeomSurfaceEvaluator::new(info);
        let (_, du, dv) = ev.d1(0.5, 0.5);
        vec_near(du, [1.0, 0.0, 0.0]);
        vec_near(dv, [0.0, 1.0, 0.0]);
    }
}

// ── GeomSurfaceEvaluator — accessor ──────────────────────────────────────────

#[test]
fn evaluator_surface_info_returns_same_data() {
    let info = GeomSurfaceInfo::new(GeomSurfaceType::RevolutionSurface, -PI, PI, -2.0, 2.0);
    let ev = GeomSurfaceEvaluator::new(info);
    let ret = ev.surface_info();
    assert_eq!(ret.surface_type(), GeomSurfaceType::RevolutionSurface);
    near(ret.u_first(), -PI);
    near(ret.u_last(), PI);
    near(ret.v_first(), -2.0);
    near(ret.v_last(), 2.0);
}

#[test]
fn evaluator_is_copy() {
    let info = GeomSurfaceInfo::new(GeomSurfaceType::Plane, 0.0, 1.0, 0.0, 1.0);
    let ev = GeomSurfaceEvaluator::new(info);
    let ev2 = ev;
    vec_near(ev.value(0.7, 0.3), ev2.value(0.7, 0.3));
}
