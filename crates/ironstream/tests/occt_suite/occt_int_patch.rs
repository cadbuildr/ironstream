// FILE: tests/occt_int_patch.rs
extern crate ironstream;
use ironstream::int_patch::*;

// ---------------------------------------------------------------------------
// IntPatchPoint tests
// ---------------------------------------------------------------------------

#[test]
fn point_new_stores_coords_and_params() {
    let pt = IntPatchPoint::new([1.0, 2.0, 3.0], 0.1, 0.2, 0.3, 0.4);
    assert_eq!(pt.point(), [1.0, 2.0, 3.0]);
    let (u1, v1) = pt.params_on_s1();
    assert_eq!(u1, 0.1);
    assert_eq!(v1, 0.2);
    let (u2, v2) = pt.params_on_s2();
    assert_eq!(u2, 0.3);
    assert_eq!(v2, 0.4);
}

#[test]
fn point_default_tolerance_is_zero() {
    let pt = IntPatchPoint::new([0.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0);
    assert_eq!(pt.tolerance(), 0.0);
}

#[test]
fn point_set_tolerance_roundtrip() {
    let mut pt = IntPatchPoint::new([0.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0);
    pt.set_tolerance(1e-7);
    assert!((pt.tolerance() - 1e-7).abs() < 1e-20);
}

#[test]
fn point_default_flags_are_false() {
    let pt = IntPatchPoint::new([0.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0);
    assert!(!pt.is_vertex());
    assert!(!pt.is_on_domain());
}

#[test]
fn point_set_vertex_flag() {
    let mut pt = IntPatchPoint::new([0.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0);
    pt.set_vertex(true);
    assert!(pt.is_vertex());
    pt.set_vertex(false);
    assert!(!pt.is_vertex());
}

#[test]
fn point_set_on_domain_flag() {
    let mut pt = IntPatchPoint::new([0.0, 0.0, 0.0], 0.0, 0.0, 0.0, 0.0);
    pt.set_on_domain(true);
    assert!(pt.is_on_domain());
    pt.set_on_domain(false);
    assert!(!pt.is_on_domain());
}

#[test]
fn point_is_copy() {
    let pt = IntPatchPoint::new([5.0, 6.0, 7.0], 0.5, 0.6, 0.7, 0.8);
    let pt2 = pt; // Copy — original still usable
    assert_eq!(pt.point(), pt2.point());
    assert_eq!(pt.params_on_s1(), pt2.params_on_s1());
    assert_eq!(pt.params_on_s2(), pt2.params_on_s2());
}

#[test]
fn point_equality_and_inequality() {
    let a = IntPatchPoint::new([1.0, 0.0, 0.0], 0.0, 0.0, 1.0, 1.0);
    let b = IntPatchPoint::new([1.0, 0.0, 0.0], 0.0, 0.0, 1.0, 1.0);
    let c = IntPatchPoint::new([2.0, 0.0, 0.0], 0.0, 0.0, 1.0, 1.0);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn point_debug_non_empty() {
    let pt = IntPatchPoint::new([0.0, 1.0, 2.0], 0.1, 0.2, 0.3, 0.4);
    let s = format!("{:?}", pt);
    assert!(!s.is_empty());
    assert!(s.contains("IntPatchPoint"));
}

// ---------------------------------------------------------------------------
// IntPatchLineType tests
// ---------------------------------------------------------------------------

#[test]
fn line_type_all_eight_variants_distinct() {
    let variants = [
        IntPatchLineType::Lin,
        IntPatchLineType::Circle,
        IntPatchLineType::Ellipse,
        IntPatchLineType::Parabola,
        IntPatchLineType::Hyperbola,
        IntPatchLineType::Walking,
        IntPatchLineType::Restriction,
        IntPatchLineType::Unknown,
    ];
    assert_eq!(variants.len(), 8);
    for i in 0..variants.len() {
        for j in 0..variants.len() {
            if i == j {
                assert_eq!(variants[i], variants[j]);
            } else {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }
}

#[test]
fn line_type_is_copy() {
    let t = IntPatchLineType::Walking;
    let t2 = t;
    assert_eq!(t, t2);
}

#[test]
fn line_type_debug_non_empty() {
    let variants = [
        IntPatchLineType::Lin,
        IntPatchLineType::Circle,
        IntPatchLineType::Ellipse,
        IntPatchLineType::Parabola,
        IntPatchLineType::Hyperbola,
        IntPatchLineType::Walking,
        IntPatchLineType::Restriction,
        IntPatchLineType::Unknown,
    ];
    for v in &variants {
        let s = format!("{:?}", v);
        assert!(!s.is_empty());
    }
    assert_eq!(format!("{:?}", IntPatchLineType::Lin), "Lin");
    assert_eq!(format!("{:?}", IntPatchLineType::Walking), "Walking");
    assert_eq!(format!("{:?}", IntPatchLineType::Unknown), "Unknown");
}

#[test]
fn line_type_hash_in_set() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(IntPatchLineType::Lin);
    set.insert(IntPatchLineType::Circle);
    set.insert(IntPatchLineType::Lin); // duplicate
    assert_eq!(set.len(), 2);
}

// ---------------------------------------------------------------------------
// IntPatchLine tests
// ---------------------------------------------------------------------------

#[test]
fn line_new_stores_type() {
    let line = IntPatchLine::new(IntPatchLineType::Ellipse);
    assert_eq!(line.line_type(), IntPatchLineType::Ellipse);
}

#[test]
fn line_default_nb_vertex_is_zero() {
    let line = IntPatchLine::new(IntPatchLineType::Lin);
    assert_eq!(line.nb_vertex(), 0);
}

#[test]
fn line_set_nb_vertex_roundtrip() {
    let mut line = IntPatchLine::new(IntPatchLineType::Walking);
    line.set_nb_vertex(5);
    assert_eq!(line.nb_vertex(), 5);
    line.set_nb_vertex(0);
    assert_eq!(line.nb_vertex(), 0);
}

#[test]
fn line_default_iso_flags_are_false() {
    let line = IntPatchLine::new(IntPatchLineType::Restriction);
    assert!(!line.is_ui_iso());
    assert!(!line.is_vi_iso());
}

#[test]
fn line_equality_same_type_same_vertices() {
    let a = IntPatchLine::new(IntPatchLineType::Circle);
    let b = IntPatchLine::new(IntPatchLineType::Circle);
    assert_eq!(a, b);
}

#[test]
fn line_inequality_different_type() {
    let a = IntPatchLine::new(IntPatchLineType::Lin);
    let b = IntPatchLine::new(IntPatchLineType::Circle);
    assert_ne!(a, b);
}

#[test]
fn line_inequality_different_nb_vertex() {
    let mut a = IntPatchLine::new(IntPatchLineType::Hyperbola);
    let b = IntPatchLine::new(IntPatchLineType::Hyperbola);
    a.set_nb_vertex(3);
    assert_ne!(a, b);
}

#[test]
fn line_clone_is_equal() {
    let mut line = IntPatchLine::new(IntPatchLineType::Parabola);
    line.set_nb_vertex(7);
    let cloned = line.clone();
    assert_eq!(line, cloned);
}

#[test]
fn line_debug_non_empty() {
    let line = IntPatchLine::new(IntPatchLineType::Unknown);
    let s = format!("{:?}", line);
    assert!(!s.is_empty());
    assert!(s.contains("IntPatchLine"));
}

#[test]
fn line_all_types_construct() {
    let types = [
        IntPatchLineType::Lin,
        IntPatchLineType::Circle,
        IntPatchLineType::Ellipse,
        IntPatchLineType::Parabola,
        IntPatchLineType::Hyperbola,
        IntPatchLineType::Walking,
        IntPatchLineType::Restriction,
        IntPatchLineType::Unknown,
    ];
    for t in types {
        let line = IntPatchLine::new(t);
        assert_eq!(line.line_type(), t);
        assert_eq!(line.nb_vertex(), 0);
        assert!(!line.is_ui_iso());
        assert!(!line.is_vi_iso());
    }
}
