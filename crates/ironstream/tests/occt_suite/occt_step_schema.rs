// FILE: rust/ironstream/crates/ironstream/tests/occt_step_schema.rs

extern crate ironstream;
use ironstream::step_schema::*;

// --- StepEntityType ---

#[test]
fn entity_type_eq() {
    assert_eq!(StepEntityType::CartesianPoint, StepEntityType::CartesianPoint);
    assert_ne!(StepEntityType::Line, StepEntityType::Circle);
}

#[test]
fn entity_type_copy() {
    let t = StepEntityType::ManifoldSolidBrep;
    let t2 = t; // Copy
    assert_eq!(t, t2);
}

#[test]
fn entity_type_unknown() {
    let t = StepEntityType::Unknown;
    assert_eq!(t, StepEntityType::Unknown);
}

// --- StepEntityRef ---

#[test]
fn entity_ref_new_and_accessors() {
    let r = StepEntityRef::new(42, StepEntityType::AdvancedFace);
    assert_eq!(r.id(), 42);
    assert_eq!(r.entity_type(), StepEntityType::AdvancedFace);
}

#[test]
fn entity_ref_clone_eq() {
    let r = StepEntityRef::new(7, StepEntityType::Direction);
    let r2 = r.clone();
    assert_eq!(r, r2);
}

#[test]
fn entity_ref_different_ids_not_eq() {
    let r1 = StepEntityRef::new(1, StepEntityType::Line);
    let r2 = StepEntityRef::new(2, StepEntityType::Line);
    assert_ne!(r1, r2);
}

// --- StepEntity ---

#[test]
fn step_entity_new_empty() {
    let e = StepEntity::new(10, StepEntityType::CartesianPoint);
    assert_eq!(e.id(), 10);
    assert_eq!(e.entity_type(), StepEntityType::CartesianPoint);
    assert_eq!(e.nb_params(), 0);
    assert_eq!(e.nb_refs(), 0);
}

#[test]
fn step_entity_add_params() {
    let mut e = StepEntity::new(1, StepEntityType::CartesianPoint);
    e.add_param("0.0");
    e.add_param("1.0");
    e.add_param("2.0");
    assert_eq!(e.nb_params(), 3);
    assert_eq!(e.param(0), "0.0");
    assert_eq!(e.param(1), "1.0");
    assert_eq!(e.param(2), "2.0");
}

#[test]
fn step_entity_add_refs() {
    let mut e = StepEntity::new(5, StepEntityType::ClosedShell);
    e.add_ref(StepEntityRef::new(10, StepEntityType::AdvancedFace));
    e.add_ref(StepEntityRef::new(11, StepEntityType::AdvancedFace));
    assert_eq!(e.nb_refs(), 2);
    assert_eq!(e.entity_ref(0).id(), 10);
    assert_eq!(e.entity_ref(1).id(), 11);
}

#[test]
fn step_entity_mixed_params_and_refs() {
    let mut e = StepEntity::new(99, StepEntityType::ShapeRepresentation);
    e.add_param("'repr name'");
    e.add_ref(StepEntityRef::new(3, StepEntityType::Axis2Placement3D));
    assert_eq!(e.nb_params(), 1);
    assert_eq!(e.nb_refs(), 1);
    assert_eq!(e.param(0), "'repr name'");
    assert_eq!(e.entity_ref(0).entity_type(), StepEntityType::Axis2Placement3D);
}

// --- StepModel ---

#[test]
fn step_model_new() {
    let m = StepModel::new("ISO-10303-21");
    assert_eq!(m.header, "ISO-10303-21");
    assert_eq!(m.nb_entities(), 0);
    assert_eq!(m.next_id, 1);
}

#[test]
fn step_model_add_and_count() {
    let mut m = StepModel::new("ISO-10303-21");
    let e1 = StepEntity::new(1, StepEntityType::CartesianPoint);
    let e2 = StepEntity::new(2, StepEntityType::Direction);
    m.add_entity(e1);
    m.add_entity(e2);
    assert_eq!(m.nb_entities(), 2);
}

#[test]
fn step_model_add_returns_index() {
    let mut m = StepModel::new("ISO-10303-21");
    let idx0 = m.add_entity(StepEntity::new(1, StepEntityType::CartesianPoint));
    let idx1 = m.add_entity(StepEntity::new(2, StepEntityType::Line));
    assert_eq!(idx0, 0);
    assert_eq!(idx1, 1);
}

#[test]
fn step_model_find_entity_present() {
    let mut m = StepModel::new("ISO-10303-21");
    m.add_entity(StepEntity::new(10, StepEntityType::Plane));
    m.add_entity(StepEntity::new(20, StepEntityType::Circle));
    let found = m.find_entity(20);
    assert!(found.is_some());
    assert_eq!(found.unwrap().id(), 20);
    assert_eq!(found.unwrap().entity_type(), StepEntityType::Circle);
}

#[test]
fn step_model_find_entity_absent() {
    let mut m = StepModel::new("ISO-10303-21");
    m.add_entity(StepEntity::new(1, StepEntityType::Vector));
    assert!(m.find_entity(999).is_none());
}

#[test]
fn step_model_entities_of_type_none() {
    let mut m = StepModel::new("ISO-10303-21");
    m.add_entity(StepEntity::new(1, StepEntityType::Line));
    let result = m.entities_of_type(StepEntityType::Circle);
    assert!(result.is_empty());
}

#[test]
fn step_model_entities_of_type_some() {
    let mut m = StepModel::new("ISO-10303-21");
    m.add_entity(StepEntity::new(1, StepEntityType::CartesianPoint));
    m.add_entity(StepEntity::new(2, StepEntityType::CartesianPoint));
    m.add_entity(StepEntity::new(3, StepEntityType::Direction));
    m.add_entity(StepEntity::new(4, StepEntityType::CartesianPoint));
    let indices = m.entities_of_type(StepEntityType::CartesianPoint);
    assert_eq!(indices.len(), 3);
    assert!(indices.contains(&0));
    assert!(indices.contains(&1));
    assert!(indices.contains(&3));
}

#[test]
fn step_model_entities_of_type_returns_storage_indices() {
    let mut m = StepModel::new("ISO-10303-21");
    m.add_entity(StepEntity::new(100, StepEntityType::ManifoldSolidBrep));
    m.add_entity(StepEntity::new(200, StepEntityType::ClosedShell));
    m.add_entity(StepEntity::new(300, StepEntityType::ManifoldSolidBrep));
    let indices = m.entities_of_type(StepEntityType::ManifoldSolidBrep);
    // indices are positions in entities Vec, not STEP ids
    assert_eq!(indices, vec![0, 2]);
}

#[test]
fn step_model_next_id_advances() {
    let mut m = StepModel::new("ISO-10303-21");
    assert_eq!(m.next_id, 1);
    m.add_entity(StepEntity::new(5, StepEntityType::Line));
    assert_eq!(m.next_id, 6);
    m.add_entity(StepEntity::new(3, StepEntityType::Line));
    // next_id should stay at max(prev, id+1)
    assert_eq!(m.next_id, 6);
    m.add_entity(StepEntity::new(10, StepEntityType::Circle));
    assert_eq!(m.next_id, 11);
}

#[test]
fn step_model_empty_header() {
    let m = StepModel::new("");
    assert_eq!(m.header, "");
}

#[test]
fn step_model_product_definition_round_trip() {
    let mut m = StepModel::new("ISO-10303-21");
    let mut e = StepEntity::new(1, StepEntityType::ProductDefinition);
    e.add_param("'part-001'");
    e.add_param("'description'");
    m.add_entity(e);

    let found = m.find_entity(1).unwrap();
    assert_eq!(found.entity_type(), StepEntityType::ProductDefinition);
    assert_eq!(found.param(0), "'part-001'");
    assert_eq!(found.param(1), "'description'");
}

#[test]
fn step_model_geometric_curve_set() {
    let mut m = StepModel::new("ISO-10303-21");
    let mut set_entity = StepEntity::new(50, StepEntityType::GeometricCurveSet);
    set_entity.add_ref(StepEntityRef::new(51, StepEntityType::Line));
    set_entity.add_ref(StepEntityRef::new(52, StepEntityType::Circle));
    m.add_entity(set_entity);

    let e = m.find_entity(50).unwrap();
    assert_eq!(e.nb_refs(), 2);
    assert_eq!(e.entity_ref(0).entity_type(), StepEntityType::Line);
    assert_eq!(e.entity_ref(1).entity_type(), StepEntityType::Circle);
}
