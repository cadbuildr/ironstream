extern crate ironstream;
use ironstream::rw_step_ext::*;

const STEP_CONTENT: &str = "ISO-10303-21;\nHEADER;\nFILE_SCHEMA(('AP203'));\nENDSEC;\nDATA;\n#1 = CARTESIAN_POINT('',(0.,0.,0.));\n#2 = CARTESIAN_POINT('',(1.,0.,0.));\n#3 = DIRECTION('',(0.,0.,1.));\nENDSEC;\nEND-ISO-10303-21;\n";

#[test]
fn test_step_reader_parse_and_schema() {
    let mut r = StepReader::new();
    r.read_string(STEP_CONTENT);
    assert!(r.is_done());
    assert_eq!(r.schema, "AP203");
    assert_eq!(r.nb_roots_for_transfer(), 3);
}

#[test]
fn test_step_reader_entities_of_type() {
    let mut r = StepReader::new();
    r.read_string(STEP_CONTENT);
    let pts = r.entities_of_type(StepEntityType::CartesianPoint);
    assert_eq!(pts.len(), 2);
    let dirs = r.entities_of_type(StepEntityType::Direction);
    assert_eq!(dirs.len(), 1);
}

#[test]
fn test_step_writer_basic() {
    let mut w = StepWriter::new();
    w.write_header("AP214");
    let id1 = w.write_cartesian_point(0.0, 0.0, 0.0);
    let id2 = w.write_direction(0.0, 0.0, 1.0);
    w.finish();
    let s = w.content_str();
    assert!(s.contains("CARTESIAN_POINT"));
    assert!(s.contains("DIRECTION"));
    assert!(s.contains("END-ISO-10303-21"));
    assert!(id1 < id2);
    assert!(w.byte_count() > 0);
}

#[test]
fn test_step_entity_params() {
    let e = StepEntity::new(42, StepEntityType::Plane)
        .with_param("param1")
        .with_param("param2");
    assert_eq!(e.nb_params(), 2);
    assert_eq!(e.id, 42);
}

#[test]
fn test_step_explorer_count_by_type() {
    let entities = vec![
        StepEntity::new(1, StepEntityType::CartesianPoint),
        StepEntity::new(2, StepEntityType::CartesianPoint),
        StepEntity::new(3, StepEntityType::Direction),
    ];
    let counts = StepExplorer::count_by_type(&entities);
    let cp_count = counts.iter().find(|(t, _)| *t == StepEntityType::CartesianPoint).map(|(_, c)| *c).unwrap_or(0);
    assert_eq!(cp_count, 2);
}
