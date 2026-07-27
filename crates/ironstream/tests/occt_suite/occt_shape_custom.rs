extern crate ironstream;

use ironstream::shape_custom::{
    ShapeCustomDirectFaces, ShapeCustomModification, ShapeCustomModType, ShapeCustomRestricted,
    ShapeCustomSweptToElementary,
};

#[test]
fn direct_faces_not_done_before_perform() {
    let df = ShapeCustomDirectFaces::new();
    assert!(!df.is_done());
}

#[test]
fn direct_faces_is_done_after_perform() {
    let mut df = ShapeCustomDirectFaces::new();
    df.perform();
    assert!(df.is_done());
}

#[test]
fn direct_faces_nb_modified_is_zero_after_perform() {
    let mut df = ShapeCustomDirectFaces::new();
    df.perform();
    assert_eq!(df.nb_modified_faces(), 0);
}

#[test]
fn direct_faces_mod_type() {
    let df = ShapeCustomDirectFaces::new();
    assert_eq!(df.mod_type(), ShapeCustomModType::DirectFaces);
}

#[test]
fn direct_faces_tolerance() {
    let df = ShapeCustomDirectFaces::new();
    assert!((df.tolerance() - 1e-7).abs() < 1e-15);
}

#[test]
fn swept_to_elementary_not_done_before_perform() {
    let se = ShapeCustomSweptToElementary::new();
    assert!(!se.is_done());
}

#[test]
fn swept_to_elementary_is_done_after_perform() {
    let mut se = ShapeCustomSweptToElementary::new();
    se.perform();
    assert!(se.is_done());
}

#[test]
fn swept_to_elementary_nb_modified_is_zero() {
    let mut se = ShapeCustomSweptToElementary::new();
    se.perform();
    assert_eq!(se.nb_modified_faces(), 0);
}

#[test]
fn swept_to_elementary_mod_type() {
    let se = ShapeCustomSweptToElementary::new();
    assert_eq!(se.mod_type(), ShapeCustomModType::SweptToElementary);
}

#[test]
fn shape_custom_modification_none_type() {
    let m = ShapeCustomModification::new(ShapeCustomModType::None, 0.01);
    assert_eq!(m.mod_type(), ShapeCustomModType::None);
    assert!(!m.is_done());
}

#[test]
fn shape_custom_modification_bspline_type() {
    let m = ShapeCustomModification::new(ShapeCustomModType::BSplineToElementary, 0.001);
    assert_eq!(m.mod_type(), ShapeCustomModType::BSplineToElementary);
    assert!((m.tolerance() - 0.001).abs() < 1e-15);
}

#[test]
fn shape_custom_modification_perform_sets_done() {
    let mut m = ShapeCustomModification::new(ShapeCustomModType::DirectFaces, 1e-6);
    assert!(!m.is_done());
    m.perform();
    assert!(m.is_done());
    assert_eq!(m.nb_modified_faces(), 0);
}

#[test]
fn restriction_parameters_defaults() {
    let r = ShapeCustomRestricted::default();
    assert_eq!(r.gmax_degree, 15);
    assert_eq!(r.gmax_seg, 10000);
    assert!(r.convert_bspline_to_bezier);
    assert!(r.convert_swept_to_elementary);
    assert!(r.convert_plane_to_canonical);
    assert!(r.convert_elementary_to_canonical);
}

#[test]
fn restriction_parameters_field_mutation() {
    let mut r = ShapeCustomRestricted::default();
    r.gmax_degree = 8;
    r.convert_bspline_to_bezier = false;
    assert_eq!(r.gmax_degree, 8);
    assert!(!r.convert_bspline_to_bezier);
    assert!(r.convert_swept_to_elementary);
}
