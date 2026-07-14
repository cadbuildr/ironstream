// FILE: rust/ironstream/crates/ironstream/tests/occt_brep_make_face.rs
//! Integration tests for `brep_make_face` — BRepBuilderAPI_MakeFace module.
//!
//! All tests exercise the public API only (no access to private fields).
//! Mirrors OCCT BRepBuilderAPI_MakeFace usage patterns.

extern crate ironstream;
use ironstream::brep_make_face::*;

const TOL: f64 = 1e-12;

fn near(a: f64, b: f64, label: &str) {
    assert!(
        (a - b).abs() <= TOL,
        "{label}: expected {a} ≈ {b}, diff = {}",
        (a - b).abs()
    );
}

// ─── FaceType ────────────────────────────────────────────────────────────────

#[test]
fn face_type_variants_are_distinct() {
    let types = [
        FaceType::Planar,
        FaceType::Cylindrical,
        FaceType::Spherical,
        FaceType::Conical,
        FaceType::Toroidal,
        FaceType::BSpline,
        FaceType::Offset,
    ];
    for i in 0..types.len() {
        for j in 0..types.len() {
            if i == j {
                assert_eq!(types[i], types[j], "same variant must be equal");
            } else {
                assert_ne!(types[i], types[j], "different variants must differ");
            }
        }
    }
}

#[test]
fn face_type_clone_and_copy() {
    let ft = FaceType::Spherical;
    let ft2 = ft; // Copy
    let ft3 = ft.clone(); // Clone
    assert_eq!(ft, ft2);
    assert_eq!(ft, ft3);
}

// ─── FaceParams ──────────────────────────────────────────────────────────────

#[test]
fn face_params_default_range_is_zero_to_one() {
    let p = FaceParams::new(FaceType::Planar);
    let (u_min, u_max) = p.u_range();
    let (v_min, v_max) = p.v_range();
    near(u_min, 0.0, "u_min");
    near(u_max, 1.0, "u_max");
    near(v_min, 0.0, "v_min");
    near(v_max, 1.0, "v_max");
}

#[test]
fn face_params_default_tolerance_is_1e7() {
    let p = FaceParams::new(FaceType::Cylindrical);
    assert!(
        (p.tolerance() - 1e-7).abs() < 1e-15,
        "default tolerance should be 1e-7, got {}",
        p.tolerance()
    );
}

#[test]
fn face_params_face_type_round_trips() {
    for ft in [
        FaceType::Planar,
        FaceType::Cylindrical,
        FaceType::Spherical,
        FaceType::Conical,
        FaceType::Toroidal,
        FaceType::BSpline,
        FaceType::Offset,
    ] {
        let p = FaceParams::new(ft);
        assert_eq!(p.face_type(), ft, "face_type() must round-trip");
    }
}

#[test]
fn face_params_with_range_stores_values() {
    let p = FaceParams::new(FaceType::Toroidal).with_range(-1.0, 3.0, 0.5, 2.5);
    let (u_min, u_max) = p.u_range();
    let (v_min, v_max) = p.v_range();
    near(u_min, -1.0, "u_min");
    near(u_max,  3.0, "u_max");
    near(v_min,  0.5, "v_min");
    near(v_max,  2.5, "v_max");
}

#[test]
fn face_params_set_tolerance_updates_value() {
    let mut p = FaceParams::new(FaceType::BSpline);
    p.set_tolerance(1e-4);
    assert!(
        (p.tolerance() - 1e-4).abs() < 1e-15,
        "tolerance must be updated to 1e-4, got {}",
        p.tolerance()
    );
}

#[test]
fn face_params_with_range_does_not_alter_face_type() {
    let p = FaceParams::new(FaceType::Conical).with_range(0.0, 6.28, -1.0, 1.0);
    assert_eq!(p.face_type(), FaceType::Conical);
}

#[test]
fn face_params_with_range_does_not_alter_tolerance() {
    let mut p = FaceParams::new(FaceType::Offset);
    p.set_tolerance(5e-6);
    let p = p.with_range(0.0, 10.0, 0.0, 10.0); // consuming builder
    assert!(
        (p.tolerance() - 5e-6).abs() < 1e-18,
        "tolerance must survive with_range, got {}",
        p.tolerance()
    );
}

// ─── FaceWire ────────────────────────────────────────────────────────────────

#[test]
fn face_wire_outer_fields_round_trip() {
    let w = FaceWire::new("outer_boundary", 4, true);
    assert_eq!(w.label(), "outer_boundary");
    assert_eq!(w.nb_edges(), 4);
    assert!(w.is_outer(), "should be outer wire");
}

#[test]
fn face_wire_inner_fields_round_trip() {
    let w = FaceWire::new("hole_1", 6, false);
    assert_eq!(w.label(), "hole_1");
    assert_eq!(w.nb_edges(), 6);
    assert!(!w.is_outer(), "should be inner wire");
}

#[test]
fn face_wire_zero_edge_wire() {
    let w = FaceWire::new("empty", 0, false);
    assert_eq!(w.nb_edges(), 0);
}

#[test]
fn face_wire_clone_preserves_all_fields() {
    let w = FaceWire::new("ring", 8, true);
    let w2 = w.clone();
    assert_eq!(w.label(), w2.label());
    assert_eq!(w.nb_edges(), w2.nb_edges());
    assert_eq!(w.is_outer(), w2.is_outer());
}

// ─── BrepMakeFace ────────────────────────────────────────────────────────────

#[test]
fn brep_make_face_is_not_done_before_build() {
    let p = FaceParams::new(FaceType::Planar);
    let builder = BrepMakeFace::new(p);
    assert!(!builder.is_done(), "must not be done before build()");
}

#[test]
fn brep_make_face_is_done_after_build() {
    let p = FaceParams::new(FaceType::Planar);
    let mut builder = BrepMakeFace::new(p);
    builder.build();
    assert!(builder.is_done(), "must be done after build()");
}

#[test]
fn brep_make_face_no_inner_wires_initially() {
    let p = FaceParams::new(FaceType::Spherical);
    let builder = BrepMakeFace::new(p);
    assert_eq!(builder.nb_inner_wires(), 0);
}

#[test]
fn brep_make_face_set_outer_wire_does_not_count_as_inner() {
    let p = FaceParams::new(FaceType::Cylindrical);
    let mut builder = BrepMakeFace::new(p);
    builder.set_outer_wire(FaceWire::new("outer", 4, true));
    assert_eq!(builder.nb_inner_wires(), 0);
}

#[test]
fn brep_make_face_add_inner_wires_increments_count() {
    let p = FaceParams::new(FaceType::Planar);
    let mut builder = BrepMakeFace::new(p);
    builder.set_outer_wire(FaceWire::new("outer", 4, true));
    builder.add_inner_wire(FaceWire::new("hole_a", 3, false));
    assert_eq!(builder.nb_inner_wires(), 1);
    builder.add_inner_wire(FaceWire::new("hole_b", 5, false));
    assert_eq!(builder.nb_inner_wires(), 2);
}

#[test]
fn brep_make_face_face_type_matches_params() {
    for ft in [
        FaceType::Planar,
        FaceType::Cylindrical,
        FaceType::Spherical,
        FaceType::Conical,
        FaceType::Toroidal,
        FaceType::BSpline,
        FaceType::Offset,
    ] {
        let builder = BrepMakeFace::new(FaceParams::new(ft));
        assert_eq!(builder.face_type(), ft, "face_type() must match params");
    }
}

#[test]
fn brep_make_face_area_default_range() {
    let p = FaceParams::new(FaceType::Planar); // [0,1]x[0,1]
    let builder = BrepMakeFace::new(p);
    near(builder.area(), 1.0, "default area");
}

#[test]
fn brep_make_face_area_custom_range() {
    let p = FaceParams::new(FaceType::Cylindrical).with_range(0.0, 3.0, 0.0, 4.0);
    let builder = BrepMakeFace::new(p);
    near(builder.area(), 12.0, "area 3*4");
}

#[test]
fn brep_make_face_area_negative_range() {
    let p = FaceParams::new(FaceType::Toroidal).with_range(-2.0, 2.0, -1.0, 1.0);
    let builder = BrepMakeFace::new(p);
    near(builder.area(), 8.0, "area 4*2 with negative bounds");
}

#[test]
fn brep_make_face_area_zero_for_degenerate_range() {
    let p = FaceParams::new(FaceType::Planar).with_range(0.0, 0.0, 0.0, 1.0);
    let builder = BrepMakeFace::new(p);
    near(builder.area(), 0.0, "area with zero u span");
}

#[test]
fn brep_make_face_build_is_idempotent() {
    let p = FaceParams::new(FaceType::Planar);
    let mut builder = BrepMakeFace::new(p);
    builder.build();
    builder.build();
    assert!(builder.is_done());
}

#[test]
fn brep_make_face_inner_wires_after_build() {
    let p = FaceParams::new(FaceType::BSpline).with_range(0.0, 5.0, 0.0, 5.0);
    let mut builder = BrepMakeFace::new(p);
    builder.set_outer_wire(FaceWire::new("boundary", 4, true));
    builder.add_inner_wire(FaceWire::new("cutout_1", 4, false));
    builder.add_inner_wire(FaceWire::new("cutout_2", 6, false));
    builder.add_inner_wire(FaceWire::new("cutout_3", 8, false));
    builder.build();
    assert!(builder.is_done());
    assert_eq!(builder.nb_inner_wires(), 3);
    near(builder.area(), 25.0, "5x5 area");
}

#[test]
fn brep_make_face_clone_is_independent() {
    let p = FaceParams::new(FaceType::Spherical).with_range(0.0, 2.0, 0.0, 3.0);
    let mut original = BrepMakeFace::new(p);
    original.set_outer_wire(FaceWire::new("outer", 4, true));
    let mut clone = original.clone();
    clone.build();
    clone.add_inner_wire(FaceWire::new("hole", 4, false));
    // original must be unaffected
    assert!(!original.is_done(), "original must not be done");
    assert_eq!(original.nb_inner_wires(), 0, "original must have no inner wires");
    // clone should have done flag and inner wire
    assert!(clone.is_done());
    assert_eq!(clone.nb_inner_wires(), 1);
}
