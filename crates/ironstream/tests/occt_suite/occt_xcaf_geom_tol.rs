extern crate ironstream;
use ironstream::xcaf_geom_tol::*;

#[test]
fn geom_tolerance_type_categories() {
    assert!(GeomToleranceType::Straightness.is_form());
    assert!(GeomToleranceType::Flatness.is_form());
    assert!(GeomToleranceType::Circularity.is_form());
    assert!(GeomToleranceType::Cylindricity.is_form());
    assert!(GeomToleranceType::Angularity.is_orientation());
    assert!(GeomToleranceType::Position.is_location());
    assert!(GeomToleranceType::TotalRunout.is_runout());
    assert!(GeomToleranceType::CircularRunout.is_runout());
}

#[test]
fn geom_tolerance_object_value_and_type() {
    let t = GeomToleranceObject::new(GeomToleranceType::Flatness, 0.02);
    assert!((t.value() - 0.02).abs() < 1e-10);
    assert_eq!(t.tol_type(), GeomToleranceType::Flatness);
    assert_eq!(t.tol_type().name(), "Flatness");
    assert!(!t.has_datum_refs());
}

#[test]
fn geom_tolerance_datum_refs() {
    let mut t = GeomToleranceObject::new(GeomToleranceType::Position, 0.1);
    t.add_datum_ref("A");
    t.add_datum_ref("B");
    assert!(t.has_datum_refs());
    assert_eq!(t.datum_refs.len(), 2);
}

#[test]
fn geom_tolerance_with_modifier() {
    let t = GeomToleranceObject::new(GeomToleranceType::Circularity, 0.05)
        .with_modifier(GeomToleranceModifier::LeastMaterialCondition);
    assert_eq!(t.modifier, Some(GeomToleranceModifier::LeastMaterialCondition));
}

#[test]
fn datum_object_modifiers() {
    let mut d = DatumObject::new("B", DatumTargetType::Line);
    assert!(!d.has_modifiers());
    d.add_modifier(DatumModifier::LineDatum);
    assert!(d.has_modifiers());
    assert_eq!(d.name(), "B");
    assert_eq!(d.target_type, DatumTargetType::Line);
}
