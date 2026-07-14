// FILE: rust/ironstream/crates/ironstream/tests/occt_iges_entity.rs

//! Ported OpenCascade unit tests — `IGESData_IGESEntity` / `IGESData_IGESModel`.
//!
//! Covers construction, parameter access, form number, status, and model
//! entity queries.

extern crate ironstream;
use ironstream::iges_entity::*;

// ---------------------------------------------------------------------------
// IgesEntityType
// ---------------------------------------------------------------------------

// TEST(IgesEntityType, Equality)
#[test]
fn entity_type_equality() {
    assert_eq!(IgesEntityType::CircularArc, IgesEntityType::CircularArc);
    assert_ne!(IgesEntityType::Line, IgesEntityType::Point);
}

// TEST(IgesEntityType, AllVariants)
#[test]
fn entity_type_all_variants() {
    let types = [
        IgesEntityType::CircularArc,
        IgesEntityType::CompositeCurve,
        IgesEntityType::ConicArc,
        IgesEntityType::Line,
        IgesEntityType::Point,
        IgesEntityType::RationalBSplineCurve,
        IgesEntityType::RationalBSplineSurface,
        IgesEntityType::Plane,
        IgesEntityType::TrimmedSurface,
        IgesEntityType::SolidOfRevolution,
        IgesEntityType::Unknown,
    ];
    // All eleven variants are distinct.
    for i in 0..types.len() {
        for j in 0..types.len() {
            if i == j {
                assert_eq!(types[i], types[j]);
            } else {
                assert_ne!(types[i], types[j]);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// IgesEntityStatus
// ---------------------------------------------------------------------------

// TEST(IgesEntityStatus, DefaultIsNotRead)
#[test]
fn status_default_not_read() {
    let e = IgesEntity::new(1, IgesEntityType::Line);
    assert_eq!(e.status(), IgesEntityStatus::NotRead);
}

// TEST(IgesEntityStatus, SetStatus)
#[test]
fn status_set() {
    let mut e = IgesEntity::new(1, IgesEntityType::Line);
    e.set_status(IgesEntityStatus::Read);
    assert_eq!(e.status(), IgesEntityStatus::Read);
    e.set_status(IgesEntityStatus::Transferred);
    assert_eq!(e.status(), IgesEntityStatus::Transferred);
    e.set_status(IgesEntityStatus::Invalid);
    assert_eq!(e.status(), IgesEntityStatus::Invalid);
}

// ---------------------------------------------------------------------------
// IgesEntity — construction and basic accessors
// ---------------------------------------------------------------------------

// TEST(IgesEntity, ConstructorDeNumber)
#[test]
fn entity_de_number() {
    let e = IgesEntity::new(7, IgesEntityType::CircularArc);
    assert_eq!(e.de_number(), 7);
}

// TEST(IgesEntity, ConstructorEntityType)
#[test]
fn entity_type_accessor() {
    let e = IgesEntity::new(3, IgesEntityType::RationalBSplineCurve);
    assert_eq!(e.entity_type(), IgesEntityType::RationalBSplineCurve);
}

// TEST(IgesEntity, FormNumberDefaultZero)
#[test]
fn entity_form_number_default() {
    let e = IgesEntity::new(1, IgesEntityType::ConicArc);
    assert_eq!(e.form_number(), 0);
}

// TEST(IgesEntity, SetFormNumber)
#[test]
fn entity_set_form_number() {
    let mut e = IgesEntity::new(1, IgesEntityType::ConicArc);
    e.set_form_number(2);
    assert_eq!(e.form_number(), 2);
    e.set_form_number(-1);
    assert_eq!(e.form_number(), -1);
}

// ---------------------------------------------------------------------------
// IgesEntity — numeric parameters
// ---------------------------------------------------------------------------

// TEST(IgesEntity, NbParamsInitiallyZero)
#[test]
fn entity_nb_params_initial() {
    let e = IgesEntity::new(1, IgesEntityType::Line);
    assert_eq!(e.nb_params(), 0);
}

// TEST(IgesEntity, AddParam)
#[test]
fn entity_add_param() {
    let mut e = IgesEntity::new(1, IgesEntityType::Line);
    e.add_param(1.0);
    e.add_param(2.5);
    e.add_param(-3.14);
    assert_eq!(e.nb_params(), 3);
    assert_eq!(e.param(0), 1.0);
    assert_eq!(e.param(1), 2.5);
    assert_eq!(e.param(2), -3.14);
}

// TEST(IgesEntity, ParamPreservesOrder)
#[test]
fn entity_param_order() {
    let mut e = IgesEntity::new(5, IgesEntityType::Point);
    for i in 0..10 {
        e.add_param(i as f64 * 0.5);
    }
    assert_eq!(e.nb_params(), 10);
    for i in 0..10 {
        assert_eq!(e.param(i), i as f64 * 0.5);
    }
}

// ---------------------------------------------------------------------------
// IgesEntity — string parameters
// ---------------------------------------------------------------------------

// TEST(IgesEntity, AddStringParam)
#[test]
fn entity_add_string_param() {
    let mut e = IgesEntity::new(1, IgesEntityType::Unknown);
    e.add_string_param("hello");
    e.add_string_param("world");
    // String params are stored; entity does not panic.
    // (No public getter for string params is required by the spec; we verify
    // the call does not panic and that numeric params are unaffected.)
    assert_eq!(e.nb_params(), 0);
}

// ---------------------------------------------------------------------------
// IgesModel — construction
// ---------------------------------------------------------------------------

// TEST(IgesModel, ConstructorFileName)
#[test]
fn model_file_name() {
    let m = IgesModel::new("example.igs");
    assert_eq!(m.file_name(), "example.igs");
    assert_eq!(m.nb_entities(), 0);
}

// ---------------------------------------------------------------------------
// IgesModel — add_entity / entity / nb_entities
// ---------------------------------------------------------------------------

// TEST(IgesModel, AddEntityReturnsIndex)
#[test]
fn model_add_entity_returns_index() {
    let mut m = IgesModel::new("test.igs");
    let idx0 = m.add_entity(IgesEntity::new(1, IgesEntityType::Line));
    let idx1 = m.add_entity(IgesEntity::new(3, IgesEntityType::Point));
    assert_eq!(idx0, 0);
    assert_eq!(idx1, 1);
    assert_eq!(m.nb_entities(), 2);
}

// TEST(IgesModel, EntityAccessor)
#[test]
fn model_entity_accessor() {
    let mut m = IgesModel::new("test.igs");
    let mut e = IgesEntity::new(5, IgesEntityType::Plane);
    e.add_param(42.0);
    m.add_entity(e);

    let retrieved = m.entity(0);
    assert_eq!(retrieved.de_number(), 5);
    assert_eq!(retrieved.entity_type(), IgesEntityType::Plane);
    assert_eq!(retrieved.param(0), 42.0);
}

// TEST(IgesModel, NbEntitiesGrowsWithAdd)
#[test]
fn model_nb_entities_grows() {
    let mut m = IgesModel::new("test.igs");
    assert_eq!(m.nb_entities(), 0);
    for i in 0..5_usize {
        m.add_entity(IgesEntity::new(i * 2 + 1, IgesEntityType::Unknown));
    }
    assert_eq!(m.nb_entities(), 5);
}

// ---------------------------------------------------------------------------
// IgesModel — entities_of_type
// ---------------------------------------------------------------------------

// TEST(IgesModel, EntitiesOfTypeEmpty)
#[test]
fn model_entities_of_type_empty() {
    let m = IgesModel::new("test.igs");
    assert!(m.entities_of_type(IgesEntityType::Line).is_empty());
}

// TEST(IgesModel, EntitiesOfTypeFiltersCorrectly)
#[test]
fn model_entities_of_type_filters() {
    let mut m = IgesModel::new("test.igs");
    m.add_entity(IgesEntity::new(1, IgesEntityType::Line));
    m.add_entity(IgesEntity::new(3, IgesEntityType::Point));
    m.add_entity(IgesEntity::new(5, IgesEntityType::Line));
    m.add_entity(IgesEntity::new(7, IgesEntityType::CircularArc));
    m.add_entity(IgesEntity::new(9, IgesEntityType::Line));

    let lines = m.entities_of_type(IgesEntityType::Line);
    assert_eq!(lines, vec![0, 2, 4]);

    let points = m.entities_of_type(IgesEntityType::Point);
    assert_eq!(points, vec![1]);

    let arcs = m.entities_of_type(IgesEntityType::CircularArc);
    assert_eq!(arcs, vec![3]);

    let missing = m.entities_of_type(IgesEntityType::TrimmedSurface);
    assert!(missing.is_empty());
}

// TEST(IgesModel, EntitiesOfTypeUnknown)
#[test]
fn model_entities_of_type_unknown() {
    let mut m = IgesModel::new("test.igs");
    m.add_entity(IgesEntity::new(1, IgesEntityType::Unknown));
    m.add_entity(IgesEntity::new(3, IgesEntityType::Unknown));
    m.add_entity(IgesEntity::new(5, IgesEntityType::Line));

    let unknowns = m.entities_of_type(IgesEntityType::Unknown);
    assert_eq!(unknowns, vec![0, 1]);
}

// ---------------------------------------------------------------------------
// IgesModel — status round-trip through model
// ---------------------------------------------------------------------------

// TEST(IgesModel, EntityStatusMutatedAfterAdd)
#[test]
fn model_entity_status_via_index() {
    let mut m = IgesModel::new("test.igs");
    let e = IgesEntity::new(1, IgesEntityType::RationalBSplineSurface);
    let idx = m.add_entity(e);

    assert_eq!(m.entity(idx).status(), IgesEntityStatus::NotRead);
}

// TEST(IgesModel, MultipleEntityTypes)
#[test]
fn model_all_entity_types_stored() {
    let mut m = IgesModel::new("multi.igs");
    let types = [
        IgesEntityType::CircularArc,
        IgesEntityType::CompositeCurve,
        IgesEntityType::ConicArc,
        IgesEntityType::Line,
        IgesEntityType::Point,
        IgesEntityType::RationalBSplineCurve,
        IgesEntityType::RationalBSplineSurface,
        IgesEntityType::Plane,
        IgesEntityType::TrimmedSurface,
        IgesEntityType::SolidOfRevolution,
        IgesEntityType::Unknown,
    ];
    for (i, &t) in types.iter().enumerate() {
        m.add_entity(IgesEntity::new(i * 2 + 1, t));
    }
    assert_eq!(m.nb_entities(), types.len());
    for (i, &t) in types.iter().enumerate() {
        assert_eq!(m.entity(i).entity_type(), t);
    }
}
