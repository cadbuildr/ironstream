//! Port of OCCT's `Geom_BSplineSurface_Test.cxx` GTest suite to Rust.
//!
//! Source:
//! `src/ModelingData/TKG3d/GTests/Geom_BSplineSurface_Test.cxx`
//!
//! Same numeric inputs, same expected values, same tolerances. OCCT helper
//! types are mapped to their IronStream equivalents:
//! - `gp_Pnt`                     -> `ironstream::gp::Pnt`
//! - `NCollection_Array2<gp_Pnt>` -> `Vec<Vec<Pnt>>` (0-based here, OCCT 1-based)
//! - `NCollection_Array1<double>` -> `Vec<f64>`
//! - `NCollection_Array1<int>`    -> `Vec<usize>` (multiplicities)
//! - `Handle(Geom_BSplineSurface)`-> the `GeomBSplineSurface` value
//! - `Precision::Confusion()`     -> `ironstream::precision::CONFUSION`

use ironstream::geom_bspline_surface::GeomBSplineSurface;
use ironstream::gp::{Pnt, Trsf};

/// Builds the surface used by the OCCT test fixture `SetUp()`:
/// a 3x3 grid of poles, degree 2 in both U and V, clamped knots {0,1} with
/// multiplicities {3,3}.
fn original_surface() -> GeomBSplineSurface {
    // NCollection_Array2<gp_Pnt> aPoles(1, 3, 1, 3);
    // aPoles(i, j) = gp_Pnt(i, j, (i + j) * 0.1);
    let mut poles = vec![vec![Pnt::origin(); 3]; 3];
    for i in 1..=3 {
        for j in 1..=3 {
            poles[i - 1][j - 1] = Pnt::new(i as f64, j as f64, (i + j) as f64 * 0.1);
        }
    }
    GeomBSplineSurface::new(
        poles,
        vec![0.0, 1.0], // anUKnots
        vec![0.0, 1.0], // aVKnots
        vec![3, 3],     // anUMults
        vec![3, 3],     // aVMults
        2,              // UDegree
        2,              // VDegree
        false,
        false,
    )
}

/// Builds the rational surface used by `SetWeight`/`SetWeightRow`/`SetWeightCol`:
/// elevated centre pole, all weights 1 except weight(2,2)=2.
fn rational_center_surface() -> GeomBSplineSurface {
    let mut poles = vec![vec![Pnt::origin(); 3]; 3];
    let mut weights = vec![vec![1.0f64; 3]; 3];
    for i in 1..=3 {
        for j in 1..=3 {
            let z = if i == 2 && j == 2 { 1.0 } else { 0.0 };
            poles[i - 1][j - 1] = Pnt::new(i as f64, j as f64, z);
            weights[i - 1][j - 1] = 1.0;
        }
    }
    weights[1][1] = 2.0;
    GeomBSplineSurface::new_rational(
        poles,
        weights,
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![3, 3],
        vec![3, 3],
        2,
        2,
        false,
        false,
    )
}

// CopyConstructorBasicProperties
#[test]
fn copy_constructor_basic_properties() {
    let orig = original_surface();
    let copied = orig.copy();

    assert_eq!(orig.u_degree(), copied.u_degree());
    assert_eq!(orig.v_degree(), copied.v_degree());
    assert_eq!(orig.nb_u_poles(), copied.nb_u_poles());
    assert_eq!(orig.nb_v_poles(), copied.nb_v_poles());
    assert_eq!(orig.nb_u_knots(), copied.nb_u_knots());
    assert_eq!(orig.nb_v_knots(), copied.nb_v_knots());
    assert_eq!(orig.is_u_periodic(), copied.is_u_periodic());
    assert_eq!(orig.is_v_periodic(), copied.is_v_periodic());
}

// CopyConstructorPoles
#[test]
fn copy_constructor_poles() {
    let orig = original_surface();
    let copied = orig.copy();
    for i in 1..=orig.nb_u_poles() {
        for j in 1..=orig.nb_v_poles() {
            assert!(orig.pole(i, j).is_equal(copied.pole(i, j), 1e-10));
        }
    }
}

// CopyConstructorKnots
#[test]
fn copy_constructor_knots() {
    let orig = original_surface();
    let copied = orig.copy();
    for i in 1..=orig.nb_u_knots() {
        assert_eq!(orig.u_knot(i), copied.u_knot(i));
        assert_eq!(orig.u_multiplicity(i), copied.u_multiplicity(i));
    }
    for i in 1..=orig.nb_v_knots() {
        assert_eq!(orig.v_knot(i), copied.v_knot(i));
        assert_eq!(orig.v_multiplicity(i), copied.v_multiplicity(i));
    }
}

// CopyMethodUsesOptimizedConstructor
#[test]
fn copy_method_uses_optimized_constructor() {
    let orig = original_surface();
    let copied = orig.copy();
    assert_eq!(orig.u_degree(), copied.u_degree());
    assert_eq!(orig.v_degree(), copied.v_degree());

    let mut u = 0.0;
    while u <= 1.0 {
        let mut v = 0.0;
        while v <= 1.0 {
            assert!(orig.value(u, v).is_equal(copied.value(u, v), 1e-10));
            v += 0.5;
        }
        u += 0.5;
    }
}

// CopyIndependence
#[test]
fn copy_independence() {
    let mut orig = original_surface();
    let copied = orig.copy();
    let new_pole = Pnt::new(10.0, 10.0, 10.0);
    orig.set_pole(2, 2, new_pole);
    assert!(!copied.pole(2, 2).is_equal(new_pole, 1e-10));
}

// Evaluation_D0
#[test]
fn evaluation_d0() {
    let s = original_surface();
    assert!(s.d0(0.0, 0.0).is_equal(Pnt::new(1.0, 1.0, 0.2), 1e-10));
    assert!(s.d0(1.0, 1.0).is_equal(Pnt::new(3.0, 3.0, 0.6), 1e-10));
}

// Evaluation_D1
#[test]
fn evaluation_d1() {
    let s = original_surface();
    let (_p, d1u, d1v) = s.d1(0.5, 0.5);
    assert!(d1u.norm() > 0.0);
    assert!(d1v.norm() > 0.0);
}

// Evaluation_D2
#[test]
fn evaluation_d2() {
    let s = original_surface();
    let (_p, d1u, _d1v, _d2u, _d2v, _d2uv) = s.d2(0.5, 0.5);
    assert!(d1u.norm() > 0.0);
}

// Evaluation_DN
#[test]
fn evaluation_dn() {
    let s = original_surface();
    let dn10 = s.dn(0.5, 0.5, 1, 0);
    assert!(dn10.norm() > 0.0);
    let dn01 = s.dn(0.5, 0.5, 0, 1);
    assert!(dn01.norm() > 0.0);
}

// Properties
#[test]
fn properties() {
    let s = original_surface();
    assert_eq!(s.u_degree(), 2);
    assert_eq!(s.v_degree(), 2);
    assert!(!s.is_u_periodic());
    assert!(!s.is_v_periodic());
    assert!(!s.is_u_rational());
    assert!(!s.is_v_rational());
    assert_eq!(s.nb_u_poles(), 3);
    assert_eq!(s.nb_v_poles(), 3);
    assert_eq!(s.nb_u_knots(), 2);
    assert_eq!(s.nb_v_knots(), 2);
}

// Bounds
#[test]
fn bounds() {
    let s = original_surface();
    let (u1, u2, v1, v2) = s.bounds();
    assert_eq!(u1, 0.0);
    assert_eq!(u2, 1.0);
    assert_eq!(v1, 0.0);
    assert_eq!(v2, 1.0);
}

// SetPole
#[test]
fn set_pole() {
    let mut s = original_surface();
    let new_pole = Pnt::new(5.0, 5.0, 5.0);
    s.set_pole(2, 2, new_pole);
    assert!(s.pole(2, 2).is_equal(new_pole, 1e-10));
}

// SetWeight
#[test]
fn set_weight() {
    let mut s = rational_center_surface();
    assert!(s.is_u_rational() || s.is_v_rational());

    let val_before = s.value(0.5, 0.5);
    s.set_weight(2, 2, 10.0);
    assert_eq!(s.weight(2, 2), 10.0);
    let val_after = s.value(0.5, 0.5);
    // Higher weight at the elevated centre pole pulls the surface up.
    assert!(val_after.z > val_before.z);
}

// IncreaseDegree
#[test]
fn increase_degree() {
    let mut s = original_surface();
    let val_before = s.value(0.5, 0.5);
    s.increase_degree(4, 3);
    assert_eq!(s.u_degree(), 4);
    assert_eq!(s.v_degree(), 3);
    let val_after = s.value(0.5, 0.5);
    assert!(val_before.is_equal(val_after, 1e-10));
}

// InsertUKnot
#[test]
fn insert_u_knot() {
    let mut s = original_surface();
    let val_before = s.value(0.5, 0.5);
    s.insert_u_knot(0.5, 1, 0.0);
    assert_eq!(s.nb_u_knots(), 3);
    let val_after = s.value(0.5, 0.5);
    assert!(val_before.is_equal(val_after, 1e-10));
}

// InsertVKnot
#[test]
fn insert_v_knot() {
    let mut s = original_surface();
    let val_before = s.value(0.5, 0.5);
    s.insert_v_knot(0.5, 1, 0.0);
    assert_eq!(s.nb_v_knots(), 3);
    let val_after = s.value(0.5, 0.5);
    assert!(val_before.is_equal(val_after, 1e-10));
}

// RemoveUKnot
#[test]
fn remove_u_knot() {
    let mut s = original_surface();
    s.insert_u_knot(0.5, 1, 0.0);
    assert_eq!(s.nb_u_knots(), 3);

    let val_before = s.value(0.25, 0.5);
    let removed = s.remove_u_knot(2, 0, 1e-6);
    assert!(removed);
    assert_eq!(s.nb_u_knots(), 2);
    let val_after = s.value(0.25, 0.5);
    assert!(val_before.is_equal(val_after, 1e-6));
}

// Segment
#[test]
fn segment() {
    let mut s = original_surface();
    let pnt_before = s.value(0.5, 0.5);
    s.segment(0.25, 0.75, 0.25, 0.75);

    let (u1, u2, v1, v2) = s.bounds();
    assert!((u1 - 0.25).abs() < 1e-10);
    assert!((u2 - 0.75).abs() < 1e-10);
    assert!((v1 - 0.25).abs() < 1e-10);
    assert!((v2 - 0.75).abs() < 1e-10);

    let pnt_after = s.value(0.5, 0.5);
    assert!(pnt_before.is_equal(pnt_after, 1e-6));
}

// ExchangeUV
#[test]
fn exchange_uv() {
    let mut s = original_surface();
    let val_before = s.value(0.3, 0.7);
    let u_deg = s.u_degree();
    let v_deg = s.v_degree();

    s.exchange_uv();
    assert_eq!(s.u_degree(), v_deg);
    assert_eq!(s.v_degree(), u_deg);

    let val_after = s.value(0.7, 0.3);
    assert!(val_before.is_equal(val_after, 1e-10));
}

// UReverse
#[test]
fn u_reverse() {
    let mut s = original_surface();
    let val_before = s.value(0.3, 0.5);
    s.u_reverse();
    let val_after = s.value(0.7, 0.5);
    assert!(val_before.is_equal(val_after, 1e-10));
}

// VReverse
#[test]
fn v_reverse() {
    let mut s = original_surface();
    let val_before = s.value(0.5, 0.3);
    s.v_reverse();
    let val_after = s.value(0.5, 0.7);
    assert!(val_before.is_equal(val_after, 1e-10));
}

// UIso
#[test]
fn u_iso() {
    let s = original_surface();
    let iso = s.u_iso(0.5);
    // UIso at u=0.5 should match surface evaluation (curve parameterized by V).
    let mut v = 0.0;
    while v <= 1.0 {
        let surf_pnt = s.value(0.5, v);
        let iso_pnt = iso.value(v);
        assert!(surf_pnt.is_equal(iso_pnt, 1e-10), "v={v}");
        v += 0.25;
    }
}

// VIso
#[test]
fn v_iso() {
    let s = original_surface();
    let iso = s.v_iso(0.5);
    let mut u = 0.0;
    while u <= 1.0 {
        let surf_pnt = s.value(u, 0.5);
        let iso_pnt = iso.value(u);
        assert!(surf_pnt.is_equal(iso_pnt, 1e-10), "u={u}");
        u += 0.25;
    }
}

// Resolution
#[test]
fn resolution() {
    let s = original_surface();
    let (u_tol, v_tol) = s.resolution(1.0);
    assert!(u_tol > 0.0);
    assert!(v_tol > 0.0);
}

// Transform
#[test]
fn transform() {
    let mut s = original_surface();
    let mut trsf = Trsf::identity();
    trsf.set_translation(Pnt::new(10.0, 20.0, 30.0));
    let pt_before = s.value(0.5, 0.5);
    s.transform(&trsf);
    let pt_after = s.value(0.5, 0.5);
    assert!((pt_after.x - (pt_before.x + 10.0)).abs() < 1e-10);
    assert!((pt_after.y - (pt_before.y + 20.0)).abs() < 1e-10);
    assert!((pt_after.z - (pt_before.z + 30.0)).abs() < 1e-10);
}

// KnotsAccess
#[test]
fn knots_access() {
    let s = original_surface();
    let u_knots = s.u_knots();
    assert_eq!(u_knots.len(), 2);
    assert_eq!(u_knots[0], 0.0);
    assert_eq!(u_knots[1], 1.0);

    let v_knots = s.v_knots();
    assert_eq!(v_knots.len(), 2);

    let u_mults = s.u_multiplicities();
    assert_eq!(u_mults.len(), 2);
    assert_eq!(u_mults[0], 3);

    let v_mults = s.v_multiplicities();
    assert_eq!(v_mults.len(), 2);
}

// WeightsAccess_NonRational
#[test]
fn weights_access_non_rational() {
    let s = original_surface();
    assert!(s.weights().is_none());
}

// PeriodicSurface_SetUNotPeriodic
#[test]
fn periodic_surface_set_u_not_periodic() {
    // U-periodic surface: degree 2, 4 U-knots of mult 1 -> NbUPoles = 3.
    let mut poles = vec![vec![Pnt::origin(); 3]; 3];
    for i in 1..=3 {
        for j in 1..=3 {
            let ang = 2.0 * std::f64::consts::PI * (i - 1) as f64 / 3.0;
            poles[i - 1][j - 1] = Pnt::new(ang.cos(), ang.sin(), j as f64 * 0.5);
        }
    }
    let mut s = GeomBSplineSurface::new(
        poles,
        vec![0.0, 1.0, 2.0, 3.0], // anUKnots
        vec![0.0, 1.0],           // aVKnots
        vec![1, 1, 1, 1],         // anUMults (Init 1)
        vec![3, 3],               // aVMults
        2,
        2,
        true,
        false,
    );
    assert!(s.is_u_periodic());

    let val = s.value(0.5, 0.5);
    s.set_u_not_periodic();
    assert!(!s.is_u_periodic());
    let val2 = s.value(0.5, 0.5);
    assert!(val.is_equal(val2, 1e-10));
}

// PeriodicSurface_SetVNotPeriodic
#[test]
fn periodic_surface_set_v_not_periodic() {
    let mut poles = vec![vec![Pnt::origin(); 3]; 3];
    for i in 1..=3 {
        for j in 1..=3 {
            let ang = 2.0 * std::f64::consts::PI * (j - 1) as f64 / 3.0;
            poles[i - 1][j - 1] = Pnt::new(i as f64 * 0.5, ang.cos(), ang.sin());
        }
    }
    let mut s = GeomBSplineSurface::new(
        poles,
        vec![0.0, 1.0],
        vec![0.0, 1.0, 2.0, 3.0],
        vec![3, 3],
        vec![1, 1, 1, 1],
        2,
        2,
        false,
        true,
    );
    assert!(s.is_v_periodic());

    let val = s.value(0.5, 0.5);
    s.set_v_not_periodic();
    assert!(!s.is_v_periodic());
    let val2 = s.value(0.5, 0.5);
    assert!(val.is_equal(val2, 1e-10));
}

// RationalSurface
#[test]
fn rational_surface() {
    let mut poles = vec![vec![Pnt::origin(); 3]; 3];
    let mut weights = vec![vec![1.0f64; 3]; 3];
    for i in 1..=3 {
        for j in 1..=3 {
            poles[i - 1][j - 1] = Pnt::new(i as f64, j as f64, 0.0);
            weights[i - 1][j - 1] = 1.0;
        }
    }
    weights[1][1] = 3.0;

    let s = GeomBSplineSurface::new_rational(
        poles,
        weights,
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![3, 3],
        vec![3, 3],
        2,
        2,
        false,
        false,
    );
    assert!(s.is_u_rational());
    assert!(s.is_v_rational());

    let w = s.weights().expect("rational weights present");
    assert_eq!(w[1][1], 3.0);

    // High weight at centre should pull surface toward (2,2,0).
    let mid = s.value(0.5, 0.5);
    assert!((mid.x - 2.0).abs() < 0.1);
    assert!((mid.y - 2.0).abs() < 0.1);
}

// LocalEvaluation
#[test]
fn local_evaluation() {
    let mut s = original_surface();
    s.insert_u_knot(0.5, 1, 0.0);
    assert_eq!(s.nb_u_knots(), 3);

    let global = s.value(0.25, 0.5);
    let local = s.local_d0(0.25, 0.5, 1, 2, 1, 2);
    assert!(global.is_equal(local, 1e-10));
}

// SetPoleRow
#[test]
fn set_pole_row() {
    let mut s = original_surface();
    let new_row = vec![
        Pnt::new(10.0, 1.0, 0.0),
        Pnt::new(10.0, 2.0, 0.0),
        Pnt::new(10.0, 3.0, 0.0),
    ];
    s.set_pole_row(2, &new_row);
    for j in 1..=3 {
        assert!((s.pole(2, j).x - 10.0).abs() < 1e-10);
    }
}

// SetPoleCol
#[test]
fn set_pole_col() {
    let mut s = original_surface();
    let new_col = vec![
        Pnt::new(1.0, 10.0, 0.0),
        Pnt::new(2.0, 10.0, 0.0),
        Pnt::new(3.0, 10.0, 0.0),
    ];
    s.set_pole_col(2, &new_col);
    for i in 1..=3 {
        assert!((s.pole(i, 2).y - 10.0).abs() < 1e-10);
    }
}

// SetWeightRow
#[test]
fn set_weight_row() {
    let mut s = rational_center_surface();
    let new_weights = vec![3.0, 4.0, 5.0];
    s.set_weight_row(2, &new_weights);
    assert_eq!(s.weight(2, 1), 3.0);
    assert_eq!(s.weight(2, 2), 4.0);
    assert_eq!(s.weight(2, 3), 5.0);
}

// SetWeightCol
#[test]
fn set_weight_col() {
    let mut s = rational_center_surface();
    let new_weights = vec![6.0, 7.0, 8.0];
    s.set_weight_col(2, &new_weights);
    assert_eq!(s.weight(1, 2), 6.0);
    assert_eq!(s.weight(2, 2), 7.0);
    assert_eq!(s.weight(3, 2), 8.0);
}

// RemoveVKnot
#[test]
fn remove_v_knot() {
    let mut s = original_surface();
    s.insert_v_knot(0.5, 1, 0.0);
    assert_eq!(s.nb_v_knots(), 3);

    let val_before = s.value(0.5, 0.25);
    let removed = s.remove_v_knot(2, 0, 1e-6);
    assert!(removed);
    assert_eq!(s.nb_v_knots(), 2);
    let val_after = s.value(0.5, 0.25);
    assert!(val_before.is_equal(val_after, 1e-6));
}

// MovePoint
#[test]
fn move_point() {
    let mut s = original_surface();
    let target = Pnt::new(2.0, 2.0, 1.0);
    let nb_u = s.nb_u_poles();
    let nb_v = s.nb_v_poles();
    let (_uf, _ul, _vf, _vl) = s.move_point(0.5, 0.5, target, 1, nb_u, 1, nb_v);
    let moved = s.value(0.5, 0.5);
    assert!(moved.is_equal(target, 1e-6));
}

// LocateU
#[test]
fn locate_u() {
    let mut s = original_surface();
    s.insert_u_knot(0.5, 1, 0.0);
    let (i1, i2) = s.locate_u(0.25, 1e-10);
    assert!(i1 >= 1);
    assert!(i2 <= s.nb_u_knots());
}

// LocateV
#[test]
fn locate_v() {
    let mut s = original_surface();
    s.insert_v_knot(0.5, 1, 0.0);
    let (i1, i2) = s.locate_v(0.25, 1e-10);
    assert!(i1 >= 1);
    assert!(i2 <= s.nb_v_knots());
}

// RationalSurface_SetUNotPeriodic
#[test]
fn rational_surface_set_u_not_periodic() {
    let mut poles = vec![vec![Pnt::origin(); 3]; 3];
    let mut weights = vec![vec![1.0f64; 3]; 3];
    for i in 1..=3 {
        for j in 1..=3 {
            let ang = 2.0 * std::f64::consts::PI * (i - 1) as f64 / 3.0;
            poles[i - 1][j - 1] = Pnt::new(ang.cos(), ang.sin(), j as f64 * 0.5);
            weights[i - 1][j - 1] = 1.0 + 0.5 * (i - 1) as f64;
        }
    }
    let mut s = GeomBSplineSurface::new_rational(
        poles,
        weights,
        vec![0.0, 1.0, 2.0, 3.0],
        vec![0.0, 1.0],
        vec![1, 1, 1, 1],
        vec![3, 3],
        2,
        2,
        true,
        false,
    );
    assert!(s.is_u_periodic());
    assert!(s.is_u_rational() || s.is_v_rational());

    let val = s.value(0.5, 0.5);
    s.set_u_not_periodic();
    assert!(!s.is_u_periodic());
    let val2 = s.value(0.5, 0.5);
    assert!(val.is_equal(val2, 1e-10));
}

// RationalSurface_SetVNotPeriodic
#[test]
fn rational_surface_set_v_not_periodic() {
    let mut poles = vec![vec![Pnt::origin(); 3]; 3];
    let mut weights = vec![vec![1.0f64; 3]; 3];
    for i in 1..=3 {
        for j in 1..=3 {
            let ang = 2.0 * std::f64::consts::PI * (j - 1) as f64 / 3.0;
            poles[i - 1][j - 1] = Pnt::new(i as f64 * 0.5, ang.cos(), ang.sin());
            weights[i - 1][j - 1] = 1.0 + 0.5 * (j - 1) as f64;
        }
    }
    let mut s = GeomBSplineSurface::new_rational(
        poles,
        weights,
        vec![0.0, 1.0],
        vec![0.0, 1.0, 2.0, 3.0],
        vec![3, 3],
        vec![1, 1, 1, 1],
        2,
        2,
        false,
        true,
    );
    assert!(s.is_v_periodic());
    assert!(s.is_u_rational() || s.is_v_rational());

    let val = s.value(0.5, 0.5);
    s.set_v_not_periodic();
    assert!(!s.is_v_periodic());
    let val2 = s.value(0.5, 0.5);
    assert!(val.is_equal(val2, 1e-10));
}

// Evaluation_D3
#[test]
fn evaluation_d3() {
    let s = original_surface();
    let (_p, _d1u, _d1v, _d2u, _d2v, _d2uv, d3u, d3v, _d3uuv, _d3uvv) = s.d3(0.5, 0.5);
    // Degree 2 surface: D3 should be zero.
    assert!(d3u.norm() < 1e-10);
    assert!(d3v.norm() < 1e-10);
}

// IncreaseUMultiplicity
#[test]
fn increase_u_multiplicity() {
    let mut s = original_surface();
    s.insert_u_knot(0.5, 1, 0.0);
    assert_eq!(s.u_multiplicity(2), 1);

    let val_before = s.value(0.5, 0.5);
    s.increase_u_multiplicity(2, 2);
    assert_eq!(s.u_multiplicity(2), 2);
    let val_after = s.value(0.5, 0.5);
    assert!(val_before.is_equal(val_after, 1e-10));
}

// SetUKnot
#[test]
fn set_u_knot() {
    let mut s = original_surface();
    s.insert_u_knot(0.5, 1, 0.0);
    s.set_u_knot(2, 0.6);
    assert_eq!(s.u_knot(2), 0.6);
}

// SetVKnot
#[test]
fn set_v_knot() {
    let mut s = original_surface();
    s.insert_v_knot(0.5, 1, 0.0);
    s.set_v_knot(2, 0.6);
    assert_eq!(s.v_knot(2), 0.6);
}

// RationalSurface_UIso
#[test]
fn rational_surface_u_iso() {
    let mut poles = vec![vec![Pnt::origin(); 3]; 3];
    let mut weights = vec![vec![1.0f64; 3]; 3];
    for i in 1..=3 {
        for j in 1..=3 {
            poles[i - 1][j - 1] = Pnt::new(i as f64, j as f64, (i + j) as f64 * 0.1);
            weights[i - 1][j - 1] = 1.0 + 0.5 * ((i - 1) + (j - 1)) as f64;
        }
    }
    let s = GeomBSplineSurface::new_rational(
        poles,
        weights,
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![3, 3],
        vec![3, 3],
        2,
        2,
        false,
        false,
    );

    let iso = s.u_iso(0.5);
    let mut v = 0.0;
    while v <= 1.0 {
        let surf_pnt = s.value(0.5, v);
        let iso_pnt = iso.value(v);
        assert!(surf_pnt.is_equal(iso_pnt, 1e-10), "v={v}");
        v += 0.25;
    }
}

// CopyIndependence_Knots
#[test]
fn copy_independence_knots() {
    let mut orig = original_surface();
    let copy = orig.copy();
    orig.insert_u_knot(0.5, 1, 0.0);
    assert_eq!(copy.nb_u_knots(), 2);
    assert_eq!(orig.nb_u_knots(), 3);
}

// WeightsArray_NonRational_ReturnsUnitWeights
#[test]
fn weights_array_non_rational_returns_unit_weights() {
    let s = original_surface();
    assert!(!s.is_u_rational());
    assert!(!s.is_v_rational());

    let weights = s.weights_array();
    assert_eq!(weights.len(), s.nb_u_poles()); // ColLength == NbUPoles
    assert_eq!(weights[0].len(), s.nb_v_poles()); // RowLength == NbVPoles
    for row in &weights {
        for &w in row {
            assert_eq!(w, 1.0);
        }
    }
}

// WeightsArray_Rational_ReturnsOwning
#[test]
fn weights_array_rational_returns_owning() {
    let mut poles = vec![vec![Pnt::origin(); 3]; 3];
    let mut weights_in = vec![vec![1.0f64; 3]; 3];
    for i in 1..=3 {
        for j in 1..=3 {
            poles[i - 1][j - 1] = Pnt::new(i as f64, j as f64, 0.0);
            weights_in[i - 1][j - 1] = if i == 2 && j == 2 { 2.0 } else { 1.0 };
        }
    }
    let rational = GeomBSplineSurface::new_rational(
        poles,
        weights_in,
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        vec![3, 3],
        vec![3, 3],
        2,
        2,
        false,
        false,
    );
    let weights = rational.weights_array();
    assert_eq!(weights.len(), 3);
    assert_eq!(weights[0].len(), 3);
    assert_eq!(weights[1][1], 2.0);
    assert_eq!(weights[0][0], 1.0);
}

// OCC30990_CacheConsistencyAtKnots
#[test]
fn occ30990_cache_consistency_at_knots() {
    // Degree-3 B-Spline surface with 3 interior knots in U (4 spans) and 2 in V.
    // Poles: 7 x 5.
    let nb_u = 7;
    let nb_v = 5;
    let mut poles = vec![vec![Pnt::origin(); nb_v]; nb_u];
    for i in 1..=nb_u {
        for j in 1..=nb_v {
            poles[i - 1][j - 1] = Pnt::new(
                (i - 1) as f64,
                (j - 1) as f64,
                (i as f64 * 0.5).sin() * (j as f64 * 0.7).cos(),
            );
        }
    }

    // U knots [0, 0.25, 0.5, 0.75, 1] mults [4, 1, 1, 1, 4].
    let u_knots = vec![0.0, 0.25, 0.5, 0.75, 1.0];
    let u_mults = vec![4, 1, 1, 1, 4];
    // V knots [0, 0.5, 1] mults [4, 1, 4].
    let v_knots = vec![0.0, 0.5, 1.0];
    let v_mults = vec![4, 1, 4];

    let surf = GeomBSplineSurface::new(
        poles, u_knots, v_knots, u_mults, v_mults, 3, 3, false, false,
    );

    let mut nb_err = 0;

    // Interior U knots: evaluation at the knot must be consistent regardless of
    // which adjacent span was evaluated beforehand.
    for i in 2..surf.nb_u_knots() {
        let u_knot = surf.u_knot(i);
        let u_prev = 0.5 * (u_knot + surf.u_knot(i - 1));
        let u_next = 0.5 * (u_knot + surf.u_knot(i + 1));
        for j in 1..surf.nb_v_knots() {
            let v = 0.5 * (surf.v_knot(j) + surf.v_knot(j + 1));
            let _ = surf.value(u_prev, v);
            let p1 = surf.value(u_knot, v);
            let _ = surf.value(u_next, v);
            let p2 = surf.value(u_knot, v);
            if p1.x != p2.x || p1.y != p2.y || p1.z != p2.z {
                nb_err += 1;
            }
        }
    }

    // Interior V knots.
    for j in 2..surf.nb_v_knots() {
        let v_knot = surf.v_knot(j);
        let v_prev = 0.5 * (v_knot + surf.v_knot(j - 1));
        let v_next = 0.5 * (v_knot + surf.v_knot(j + 1));
        for i in 1..surf.nb_u_knots() {
            let u = 0.5 * (surf.u_knot(i) + surf.u_knot(i + 1));
            let _ = surf.value(u, v_prev);
            let p1 = surf.value(u, v_knot);
            let _ = surf.value(u, v_next);
            let p2 = surf.value(u, v_knot);
            if p1.x != p2.x || p1.y != p2.y || p1.z != p2.z {
                nb_err += 1;
            }
        }
    }

    assert_eq!(nb_err, 0, "BSpline surface cache is inconsistent at span knots");
}
