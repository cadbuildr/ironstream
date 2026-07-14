// FILE: tests/occt_iges_extra.rs
extern crate ironstream;

use ironstream::iges_extra::{IgesBSplineCurve, IgesColorDef, IgesEntity, IgesEntityType, IgesModel};

#[test]
fn iges_entity_type_is_curve_is_surface() {
    assert!(IgesEntityType::BSplineCurve.is_curve());
    assert!(IgesEntityType::Line.is_curve());
    assert!(IgesEntityType::CircularArc.is_curve());
    assert!(!IgesEntityType::BSplineCurve.is_surface());
    assert!(IgesEntityType::BSplineSurface.is_surface());
    assert!(IgesEntityType::TrimmedSurface.is_surface());
    assert!(!IgesEntityType::BSplineSurface.is_curve());
}

#[test]
fn iges_model_nb_curves_nb_surfaces() {
    let mut model = IgesModel::new();
    model.add_entity(IgesEntity::new(IgesEntityType::Line, 1));
    model.add_entity(IgesEntity::new(IgesEntityType::BSplineCurve, 3));
    model.add_entity(IgesEntity::new(IgesEntityType::BSplineSurface, 5));
    assert_eq!(model.nb_entities(), 3);
    assert_eq!(model.nb_curves(), 2);
    assert_eq!(model.nb_surfaces(), 1);
}

#[test]
fn iges_bspline_curve_is_valid() {
    let pts = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 1.0, 0.0]];
    let c = IgesBSplineCurve::new(2, pts);
    assert!(c.is_valid());
    assert_eq!(c.degree, 2);
    assert_eq!(c.nb_control_pts, 3);

    // empty curve is not valid
    let empty = IgesBSplineCurve::new(0, vec![]);
    assert!(!empty.is_valid());
}

#[test]
fn iges_color_def_to_linear() {
    let c = IgesColorDef::new(100.0, 50.0, 0.0, "orange");
    let lin = c.to_linear();
    assert!((lin[0] - 1.0).abs() < 1e-6);
    assert!((lin[1] - 0.5).abs() < 1e-6);
    assert!((lin[2]).abs() < 1e-6);
}
