// Integration tests for step_data_ext
use ironstream::step_data_ext::*;

#[test]
fn step_entity_attributes() {
    let mut e = StepEntity::new(10, "ADVANCED_BREP_SHAPE_REPRESENTATION");
    e.set_attr("name", "MyShape");
    e.set_attr("id", "10");
    assert_eq!(e.get_attr("name"), Some("MyShape"));
    assert_eq!(e.get_attr("missing"), None);
    assert_eq!(e.category, StepEntityCategory::Unknown);
}

#[test]
fn step_model_entity_by_index() {
    let mut m = StepDataStepModel::new();
    m.add_entity(StepEntity::new(5, "PLANE"));
    m.add_entity(StepEntity::new(6, "CIRCLE"));
    assert_eq!(m.entity_by_index(0).unwrap().id, 5);
    assert_eq!(m.entity_by_index(1).unwrap().id, 6);
    assert!(m.entity_by_index(2).is_none());
}

#[test]
fn step_model_clear_and_schema() {
    let mut m = StepDataStepModel::new();
    m.set_schema("AP242");
    assert_eq!(m.schema, "AP242");
    m.add_entity(StepEntity::new(1, "X"));
    m.clear();
    assert_eq!(m.nb_entities(), 0);
    assert!(!m.has_entity(1));
}

#[test]
fn step_protocol_schema_name() {
    let mut p = StepDataProtocol::new("AP203");
    p.add_type("PRODUCT_DEFINITION");
    p.add_type("NEXT_ASSEMBLY_USAGE_OCCURRENCE");
    assert_eq!(p.schema_name(), "AP203");
    assert_eq!(p.nb_types(), 2);
    assert!(!p.recognizes("ADVANCED_FACE"));
}

#[test]
fn select_member_real_and_string() {
    let r = StepDataSelectMember::real("LENGTH_MEASURE", 3.14);
    assert!((r.as_real().unwrap() - 3.14).abs() < 1e-10);
    assert!(r.as_integer().is_none());

    let s = StepDataSelectMember::string("LABEL", "hello");
    assert_eq!(s.as_string(), Some("hello"));
    assert_eq!(s.kind(), "LABEL");
}

#[test]
fn edescr_super_types_and_complex() {
    let mut d = StepDataEDescr::new("FACE_SURFACE");
    d.add_field("face_geometry");
    d.add_field("same_sense");
    d.add_super_type("FACE");
    d.add_super_type("GEOMETRIC_REPRESENTATION_ITEM");
    assert_eq!(d.nb_fields(), 2);
    assert!(d.has_field("same_sense"));
    assert!(!d.is_complex);
    assert_eq!(d.type_name(), "FACE_SURFACE");
    assert_eq!(d.super_types.len(), 2);
}
