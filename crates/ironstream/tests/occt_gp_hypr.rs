//! Ported OpenCascade unit tests -- `gp_Hypr`.
//!
//! Faithful Rust port of OpenCascade's `gp_Hypr` test suite (OCCT package
//! TKMath / `gp`). Same numeric inputs, expected values, and tolerances as
//! upstream (`Precision::Confusion()` -> [`ironstream::precision::CONFUSION`],
//! `Precision::Angular()` -> [`ironstream::precision::ANGULAR`]).
//!
//! OCCT helper-type mapping:
//!
//! | OCCT type  | IronStream type               |
//! |------------|-------------------------------|
//! | `gp_Hypr`  | `ironstream::gp_hypr::Hypr`   |
//! | `gp_Pnt`   | `ironstream::gp::Pnt`         |
//! | `gp_Ax1`   | `ironstream::gp::Ax1`         |
//! | `gp_Ax2`   | `ironstream::gp::Ax3`         |
//! | `gp_Trsf`  | `ironstream::gp::Trsf`        |

use ironstream::gp::{Ax1, Ax3, Pnt, Trsf};
use ironstream::gp_hypr::Hypr;
use ironstream::precision::{ANGULAR, CONFUSION};

// ─────────────────────────────────── helpers ──────────────────────────────────

/// `EXPECT_NEAR(a, b, tol)` — identical convention to other occt_gp_*.rs tests.
fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "expected {a} ~= {b} (tol {tol})");
}

/// Reproduce OCCT's `gp_Ax2(loc, dir)` constructor: builds an Ax3 whose Z is
/// `normal` and whose X is chosen by the same tie-breaking as OCCT (identical
/// helper to `occt_geom_circle.rs` and `occt_geom_parabola.rs`).
fn ax2(loc: Pnt, normal: Pnt) -> Ax3 {
    let v = normal.normalized();
    let (a, b, c) = (v.x, v.y, v.z);
    let (aa, ba, ca) = (a.abs(), b.abs(), c.abs());
    let xdir = if ba <= aa && ba <= ca {
        if aa > ca {
            Pnt::new(-c, 0.0, a)
        } else {
            Pnt::new(c, 0.0, -a)
        }
    } else if aa <= ba && aa <= ca {
        if ba > ca {
            Pnt::new(0.0, -c, b)
        } else {
            Pnt::new(0.0, c, -b)
        }
    } else if aa > ba {
        Pnt::new(-b, a, 0.0)
    } else {
        Pnt::new(b, -a, 0.0)
    };
    Ax3::from_origin_normal(loc, v, xdir)
}

/// Build the canonical fixture hyperbola: centre at origin, Z along +Z axis,
/// major radius 5, minor radius 12.  Focal half-distance = 13.
fn make_hypr() -> Hypr {
    let a2 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    Hypr::new(a2, 5.0, 12.0)
}

// ─────────────────────────────────── tests ────────────────────────────────────

// TEST(gp_Hypr, Constructor)
#[test]
fn constructor() {
    let a2 = ax2(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    let h = Hypr::new(a2, 3.0, 4.0);
    near(h.major_radius(), 3.0, CONFUSION);
    near(h.minor_radius(), 4.0, CONFUSION);
    // centre at origin
    assert!(h.location().is_equal(Pnt::origin(), CONFUSION));
}

// TEST(gp_Hypr, Radii)
#[test]
fn radii() {
    let h = make_hypr();
    near(h.major_radius(), 5.0, CONFUSION);
    near(h.minor_radius(), 12.0, CONFUSION);
}

// TEST(gp_Hypr, FocalDistance)
#[test]
fn focal_distance() {
    // a=5, b=12 => c=13, focal=26
    let h = make_hypr();
    near(h.focal(), 26.0, CONFUSION);
}

// TEST(gp_Hypr, Eccentricity)
#[test]
fn eccentricity() {
    // e = c/a = 13/5 = 2.6
    let h = make_hypr();
    near(h.eccentricity(), 13.0 / 5.0, CONFUSION);
}

// TEST(gp_Hypr, EccentricityGreaterThanOne)
#[test]
fn eccentricity_greater_than_one() {
    let h = make_hypr();
    assert!(h.eccentricity() > 1.0);
}

// TEST(gp_Hypr, Focus1)
#[test]
fn focus1() {
    // F1 = centre + c * XDir = (13, 0, 0)
    let h = make_hypr();
    let f1 = h.focus1();
    near(f1.x, 13.0, CONFUSION);
    near(f1.y, 0.0, CONFUSION);
    near(f1.z, 0.0, CONFUSION);
}

// TEST(gp_Hypr, Focus2)
#[test]
fn focus2() {
    // F2 = centre - c * XDir = (-13, 0, 0)
    let h = make_hypr();
    let f2 = h.focus2();
    near(f2.x, -13.0, CONFUSION);
    near(f2.y, 0.0, CONFUSION);
    near(f2.z, 0.0, CONFUSION);
}

// TEST(gp_Hypr, FocusDistanceSumProperty)
#[test]
fn focus_distance_difference_property() {
    // For any point on the right branch: |d(P,F1) - d(P,F2)| = 2*a = 10
    // Using the parametric equation P(t) = (a*cosh(t), b*sinh(t)):
    let h = make_hypr();
    let f1 = h.focus1();
    let f2 = h.focus2();
    let a = h.major_radius(); // 5
    let b = h.minor_radius(); // 12
    let x_dir = h.position().x_dir;
    let y_dir = h.position().y_dir;
    let centre = h.location();

    for t_int in [-3_i32, -2, -1, 0, 1, 2, 3] {
        let t = t_int as f64 * 0.5;
        let p = centre + x_dir * (a * t.cosh()) + y_dir * (b * t.sinh());
        let d1 = p.distance(f1);
        let d2 = p.distance(f2);
        // On the right branch, d2 - d1 = 2*a
        near((d2 - d1).abs(), 2.0 * a, CONFUSION);
    }
}

// TEST(gp_Hypr, Axis)
#[test]
fn axis_is_z_normal() {
    let h = make_hypr();
    let ax = h.axis();
    // Should be the Z axis
    near(ax.direction.x.abs(), 0.0, ANGULAR);
    near(ax.direction.y.abs(), 0.0, ANGULAR);
    near(ax.direction.z.abs(), 1.0, ANGULAR);
    assert!(ax.location.is_equal(Pnt::origin(), CONFUSION));
}

// TEST(gp_Hypr, XAxis)
#[test]
fn x_axis_is_major_direction() {
    let h = make_hypr();
    let xa = h.x_axis();
    // X axis direction should be unit (check norm)
    let n = xa.direction.norm();
    near(n, 1.0, CONFUSION);
    // Location should be the centre
    assert!(xa.location.is_equal(Pnt::origin(), CONFUSION));
}

// TEST(gp_Hypr, YAxis)
#[test]
fn y_axis_is_minor_direction() {
    let h = make_hypr();
    let ya = h.y_axis();
    let n = ya.direction.norm();
    near(n, 1.0, CONFUSION);
    // Should be orthogonal to XAxis
    let xa = h.x_axis();
    near(xa.direction.dot(ya.direction).abs(), 0.0, ANGULAR);
}

// TEST(gp_Hypr, Asymptote1Direction)
#[test]
fn asymptote1_direction() {
    // For a=5, b=12: direction = (5,12,0)/13
    let h = make_hypr();
    let a1 = h.asymptote1();
    near(a1.direction.x, 5.0 / 13.0, CONFUSION);
    near(a1.direction.y, 12.0 / 13.0, CONFUSION);
    near(a1.direction.z, 0.0, CONFUSION);
    // passes through centre
    assert!(a1.location.is_equal(Pnt::origin(), CONFUSION));
}

// TEST(gp_Hypr, Asymptote2Direction)
#[test]
fn asymptote2_direction() {
    // For a=5, b=12: direction = (5,-12,0)/13
    let h = make_hypr();
    let a2 = h.asymptote2();
    near(a2.direction.x, 5.0 / 13.0, CONFUSION);
    near(a2.direction.y, -12.0 / 13.0, CONFUSION);
    near(a2.direction.z, 0.0, CONFUSION);
}

// TEST(gp_Hypr, AsymptotesAreSymmetric)
#[test]
fn asymptotes_are_symmetric_about_x_axis() {
    let h = make_hypr();
    let a1 = h.asymptote1();
    let a2 = h.asymptote2();
    // The y-components should be equal in magnitude and opposite in sign
    near(a1.direction.y, -a2.direction.y, CONFUSION);
    near(a1.direction.x, a2.direction.x, CONFUSION);
}

// TEST(gp_Hypr, Directrix1)
#[test]
fn directrix1_position() {
    // d = a²/c = 25/13
    let h = make_hypr();
    let dir_line = h.directrix1();
    near(dir_line.location().x, 25.0 / 13.0, CONFUSION);
    near(dir_line.location().y, 0.0, CONFUSION);
    near(dir_line.location().z, 0.0, CONFUSION);
}

// TEST(gp_Hypr, Directrix2)
#[test]
fn directrix2_position() {
    // symmetric to directrix1
    let h = make_hypr();
    let dir2 = h.directrix2();
    near(dir2.location().x, -25.0 / 13.0, CONFUSION);
    near(dir2.location().y, 0.0, CONFUSION);
}

// TEST(gp_Hypr, Translate)
#[test]
fn translate() {
    let mut h = make_hypr();
    h.translate(Pnt::new(3.0, 4.0, 5.0));
    let loc = h.location();
    near(loc.x, 3.0, CONFUSION);
    near(loc.y, 4.0, CONFUSION);
    near(loc.z, 5.0, CONFUSION);
    // Radii unchanged
    near(h.major_radius(), 5.0, CONFUSION);
    near(h.minor_radius(), 12.0, CONFUSION);
}

// TEST(gp_Hypr, TranslatedCopy)
#[test]
fn translated_copy() {
    let h = make_hypr();
    let h2 = h.translated(Pnt::new(1.0, 0.0, 0.0));
    near(h2.location().x, 1.0, CONFUSION);
    // Original untouched
    near(h.location().x, 0.0, CONFUSION);
}

// TEST(gp_Hypr, Translate2Pts)
#[test]
fn translate_2pts() {
    let h = make_hypr();
    let h2 = h.translated_2pts(Pnt::new(1.0, 0.0, 0.0), Pnt::new(4.0, 0.0, 0.0));
    near(h2.location().x, 3.0, CONFUSION);
}

// TEST(gp_Hypr, Rotate180AboutZ)
#[test]
fn rotate_180_about_z() {
    let h = make_hypr();
    let axis = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let hr = h.rotated(axis, std::f64::consts::PI);
    // Centre stays at origin
    assert!(hr.location().is_equal(Pnt::origin(), CONFUSION));
    // Focus1 should now be at (-13, 0, 0) since X axis flipped
    let f1 = hr.focus1();
    near(f1.x, -13.0, CONFUSION);
    near(f1.y, 0.0, CONFUSION);
    // Radii unchanged
    near(hr.major_radius(), 5.0, CONFUSION);
    near(hr.minor_radius(), 12.0, CONFUSION);
}

// TEST(gp_Hypr, RotateInPlace)
#[test]
fn rotate_in_place() {
    let mut h = make_hypr();
    let axis = Ax1::new(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 1.0));
    h.rotate(axis, std::f64::consts::FRAC_PI_2);
    // After 90° rotation about Z: x_dir was (1,0,0) -> (0,1,0)
    let xd = h.position().x_dir;
    near(xd.x, 0.0, CONFUSION);
    near(xd.y, 1.0, CONFUSION);
}

// TEST(gp_Hypr, Scale)
#[test]
fn scale() {
    let h = make_hypr().scaled(Pnt::origin(), 2.0);
    near(h.major_radius(), 10.0, CONFUSION);
    near(h.minor_radius(), 24.0, CONFUSION);
    // focal distance also doubles
    near(h.focal(), 52.0, CONFUSION);
}

// TEST(gp_Hypr, ScaleNegative)
#[test]
fn scale_negative_factor() {
    // Scaling by -1 should still produce positive radii
    let h = make_hypr().scaled(Pnt::origin(), -1.0);
    near(h.major_radius(), 5.0, CONFUSION);
    near(h.minor_radius(), 12.0, CONFUSION);
}

// TEST(gp_Hypr, Transform)
#[test]
fn transform_translation() {
    let mut h = make_hypr();
    let trsf = Trsf::translation(Pnt::new(7.0, 0.0, 0.0));
    h.transform(&trsf);
    near(h.location().x, 7.0, CONFUSION);
    near(h.major_radius(), 5.0, CONFUSION);
}

// TEST(gp_Hypr, TransformScale)
#[test]
fn transform_scale() {
    let mut h = make_hypr();
    let mut trsf = Trsf::identity();
    trsf.set_scale(Pnt::origin(), 3.0);
    h.transform(&trsf);
    near(h.major_radius(), 15.0, CONFUSION);
    near(h.minor_radius(), 36.0, CONFUSION);
}

// TEST(gp_Hypr, TransformedCopy)
#[test]
fn transformed_copy() {
    let h = make_hypr();
    let trsf = Trsf::translation(Pnt::new(2.0, 3.0, 4.0));
    let h2 = h.transformed(&trsf);
    near(h2.location().x, 2.0, CONFUSION);
    near(h2.location().y, 3.0, CONFUSION);
    near(h2.location().z, 4.0, CONFUSION);
    // Original untouched
    assert!(h.location().is_equal(Pnt::origin(), CONFUSION));
}

// TEST(gp_Hypr, MirrorPoint)
#[test]
fn mirror_point() {
    // Reflect about (10, 0, 0): centre moves to (20, 0, 0)
    let mut h = make_hypr();
    h.mirror_point(Pnt::new(10.0, 0.0, 0.0));
    near(h.location().x, 20.0, CONFUSION);
    near(h.location().y, 0.0, CONFUSION);
    near(h.major_radius(), 5.0, CONFUSION);
}

// TEST(gp_Hypr, MirrorAxis)
#[test]
fn mirror_axis() {
    // Reflect across the Z axis: focus1 (13,0,0) maps to (-13,0,0)
    let h = make_hypr();
    let mirror = Ax1::new(Pnt::origin(), Pnt::new(0.0, 0.0, 1.0));
    let hm = h.mirrored_axis(mirror);
    // The major-axis direction should now point in -X
    let x_dir = hm.position().x_dir;
    near(x_dir.x, -1.0, CONFUSION);
    near(x_dir.y, 0.0, CONFUSION);
}

// TEST(gp_Hypr, MirrorPlane)
#[test]
fn mirror_plane_xy() {
    // Reflect hyperbola shifted to z=5 across XY plane: z goes to -5
    let a2 = ax2(Pnt::new(0.0, 0.0, 5.0), Pnt::new(0.0, 0.0, 1.0));
    let h = Hypr::new(a2, 5.0, 12.0);
    let xy_plane = Ax3::identity(); // XY plane: origin=(0,0,0), normal=Z
    let hm = h.mirrored_plane(xy_plane);
    near(hm.location().z, -5.0, CONFUSION);
    near(hm.major_radius(), 5.0, CONFUSION);
}

// TEST(gp_Hypr, OtherBranch)
#[test]
fn other_branch() {
    let h = make_hypr();
    let ob = h.other_branch();
    // Same radii
    near(ob.major_radius(), h.major_radius(), CONFUSION);
    near(ob.minor_radius(), h.minor_radius(), CONFUSION);
    // Same centre
    assert!(ob.location().is_equal(h.location(), CONFUSION));
    // X direction negated
    let xd = ob.position().x_dir;
    near(xd.x, -h.position().x_dir.x, CONFUSION);
    // Focus of the other branch is where focus2 of original was
    let f1_ob = ob.focus1();
    near(f1_ob.x, -13.0, CONFUSION);
}

// TEST(gp_Hypr, ConjugateBranch1)
#[test]
fn conjugate_branch1_radii_swapped() {
    let h = make_hypr(); // a=5, b=12
    let cb = h.conjugate_branch1(); // should have a=12, b=5
    near(cb.major_radius(), 12.0, CONFUSION);
    near(cb.minor_radius(), 5.0, CONFUSION);
    // Same centre
    assert!(cb.location().is_equal(h.location(), CONFUSION));
}

// TEST(gp_Hypr, ConjugateBranch2)
#[test]
fn conjugate_branch2() {
    let h = make_hypr();
    let cb1 = h.conjugate_branch1();
    let cb2 = h.conjugate_branch2();
    // Branch2 is the opposite of branch1: x_dirs are negated
    let xd1 = cb1.position().x_dir;
    let xd2 = cb2.position().x_dir;
    near(xd1.x + xd2.x, 0.0, CONFUSION);
    near(xd1.y + xd2.y, 0.0, CONFUSION);
}

// TEST(gp_Hypr, SetMajorRadius)
#[test]
fn set_major_radius() {
    let mut h = make_hypr();
    h.set_major_radius(8.0);
    near(h.major_radius(), 8.0, CONFUSION);
    // Eccentricity updates accordingly
    let expected_e = (8.0_f64 * 8.0 + 12.0 * 12.0).sqrt() / 8.0;
    near(h.eccentricity(), expected_e, CONFUSION);
}

// TEST(gp_Hypr, SetMinorRadius)
#[test]
fn set_minor_radius() {
    let mut h = make_hypr();
    h.set_minor_radius(3.0);
    near(h.minor_radius(), 3.0, CONFUSION);
}

// TEST(gp_Hypr, SetLocation)
#[test]
fn set_location() {
    let mut h = make_hypr();
    h.set_location(Pnt::new(1.0, 2.0, 3.0));
    assert!(h.location().is_equal(Pnt::new(1.0, 2.0, 3.0), CONFUSION));
}

// TEST(gp_Hypr, FocalScalesWithTransform)
#[test]
fn focal_scales_with_transform() {
    let h = make_hypr(); // focal = 26
    let mut trsf = Trsf::identity();
    trsf.set_scale(Pnt::origin(), 0.5);
    let h2 = h.transformed(&trsf);
    near(h2.focal(), 13.0, CONFUSION);
}

// TEST(gp_Hypr, NonZeroCentre)
#[test]
fn non_zero_centre() {
    let centre = Pnt::new(1.0, 2.0, 3.0);
    let a2 = ax2(centre, Pnt::new(0.0, 0.0, 1.0));
    let h = Hypr::new(a2, 3.0, 4.0);
    // c = 5; Focus1 = centre + 5 * XDir
    let f1 = h.focus1();
    near(f1.x, centre.x + 5.0, CONFUSION);
    near(f1.y, centre.y, CONFUSION);
    near(f1.z, centre.z, CONFUSION);
    let f2 = h.focus2();
    near(f2.x, centre.x - 5.0, CONFUSION);
}

// TEST(gp_Hypr, PositionRoundTrip)
#[test]
fn position_roundtrip() {
    let h = make_hypr();
    let pos = h.position();
    // Position's z_dir should equal the axis direction
    let ax_dir = h.axis().direction;
    near(ax_dir.dot(pos.z_dir).abs(), 1.0, ANGULAR);
}
