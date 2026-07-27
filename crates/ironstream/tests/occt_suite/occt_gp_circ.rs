// FILE: tests/occt_gp_circ.rs
extern crate ironstream;
use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::gp_circ::Circ;
use ironstream::precision::CONFUSION;
use std::f64::consts::{FRAC_PI_2, PI};

fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ≈ {b} (tol {tol})");
}

fn pnt_near(a: Pnt, b: Pnt, tol: f64) {
    assert!(a.is_equal(b, tol), "expected {a:?} ≈ {b:?} (tol {tol})");
}

fn unit_circle() -> Circ {
    Circ::new(Ax3::identity(), 1.0)
}

#[test]
fn constructor_stores_radius() {
    let c = Circ::new(Ax3::identity(), 5.0);
    near(c.radius(), 5.0, CONFUSION);
    pnt_near(c.location(), Pnt::origin(), CONFUSION);
}

#[test]
fn area_and_length_unit_circle() {
    let c = unit_circle();
    near(c.area(), PI, CONFUSION);
    near(c.length(), 2.0 * PI, CONFUSION);
}

#[test]
fn area_and_length_r5() {
    let c = Circ::new(Ax3::identity(), 5.0);
    near(c.area(), 25.0 * PI, CONFUSION);
    near(c.length(), 10.0 * PI, CONFUSION);
}

#[test]
fn distance_point_on_circle_is_zero() {
    let c = Circ::new(Ax3::identity(), 3.0);
    near(c.distance(Pnt::new(3.0, 0.0, 0.0)), 0.0, CONFUSION);
    near(c.distance(Pnt::new(0.0, 3.0, 0.0)), 0.0, CONFUSION);
}

#[test]
fn distance_from_center_equals_radius() {
    let c = Circ::new(Ax3::identity(), 4.0);
    near(c.distance(Pnt::origin()), 4.0, CONFUSION);
}

#[test]
fn contains_point_on_circle() {
    let c = unit_circle();
    assert!(c.contains(Pnt::new(1.0, 0.0, 0.0), 1e-6));
    assert!(!c.contains(Pnt::new(2.0, 0.0, 0.0), 1e-6));
}

#[test]
fn translate_moves_center_preserves_radius() {
    let c = unit_circle();
    let v = Pnt::new(10.0, 20.0, 30.0);
    let t = c.translated(v);
    pnt_near(t.location(), v, CONFUSION);
    near(t.radius(), 1.0, CONFUSION);
}

#[test]
fn rotate_moves_center_preserves_radius() {
    let c = Circ::new(
        Ax3::from_origin_normal(
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        ),
        2.0,
    );
    let r = c.rotated(Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0)), FRAC_PI_2);
    pnt_near(r.location(), Pnt::new(0.0, 1.0, 0.0), CONFUSION);
    near(r.radius(), 2.0, CONFUSION);
}

#[test]
fn scale_adjusts_radius_and_center() {
    let c = Circ::new(
        Ax3::from_origin_normal(
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        ),
        3.0,
    );
    let s = c.scaled(Pnt::origin(), 2.0);
    near(s.radius(), 6.0, CONFUSION);
    pnt_near(s.location(), Pnt::new(2.0, 0.0, 0.0), CONFUSION);
}

#[test]
fn reversed_flips_normal() {
    let c = unit_circle();
    let r = c.reversed();
    near(r.axis().direction.z, -1.0, CONFUSION);
    near(c.radius(), r.radius(), CONFUSION);
}

#[test]
fn transform_with_scale() {
    let c = Circ::new(Ax3::identity(), 4.0);
    let mut t = Trsf::default();
    t.set_scale(Pnt::origin(), 3.0);
    let tc = c.transformed(&t);
    near(tc.radius(), 12.0, CONFUSION);
}

#[test]
fn from_center_normal_radius_positions_correctly() {
    let center = Pnt::new(5.0, 6.0, 7.0);
    let c = Circ::from_center_normal_radius(center, Pnt::new(0.0, 0.0, 1.0), 2.5);
    near(c.radius(), 2.5, CONFUSION);
    pnt_near(c.location(), center, CONFUSION);
}

#[test]
fn in_place_translate() {
    let mut c = unit_circle();
    c.translate(Pnt::new(1.0, 2.0, 3.0));
    pnt_near(c.location(), Pnt::new(1.0, 2.0, 3.0), CONFUSION);
}

#[test]
fn mirror_point_reflects_center() {
    let c = Circ::new(
        Ax3::from_origin_normal(
            Pnt::new(2.0, 3.0, 4.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 0.0),
        ),
        1.0,
    );
    let m = c.mirrored_point(Pnt::origin());
    pnt_near(m.location(), Pnt::new(-2.0, -3.0, -4.0), CONFUSION);
    near(m.radius(), 1.0, CONFUSION);
}
