// FILE: tests/occt_step_extra.rs
extern crate ironstream;

use ironstream::step_extra::{
    StepPoint3d, StepPointIterator, StepReader, StepSchemaType, StepWriter,
};

#[test]
fn step_schema_type_supports_color_pmi() {
    assert!(StepSchemaType::AP214.supports_color());
    assert!(StepSchemaType::AP242.supports_color());
    assert!(!StepSchemaType::AP203.supports_color());
    assert!(StepSchemaType::AP242.supports_pmi());
    assert!(!StepSchemaType::AP214.supports_pmi());
    assert!(!StepSchemaType::AP203.supports_pmi());
}

#[test]
fn step_point_iterator_more_next_reset() {
    let pts = vec![
        StepPoint3d::new(1, [0.0, 0.0, 0.0]),
        StepPoint3d::new(2, [1.0, 2.0, 3.0]),
        StepPoint3d::new(3, [4.0, 5.0, 6.0]),
    ];
    let mut it = StepPointIterator::new(pts);
    assert_eq!(it.nb_points(), 3);
    assert!(it.more());
    let p = it.next().unwrap();
    assert_eq!(p.id, 1);
    it.next();
    it.next();
    assert!(!it.more());
    it.reset();
    assert!(it.more());
    assert_eq!(it.next().unwrap().id, 1);
}

#[test]
fn step_reader_read_file() {
    let mut reader = StepReader::new();
    let res = reader.read_file("assembly.stp");
    assert!(res.is_ok());
    assert!(reader.is_done);

    let res2 = reader.read_file("not_a_step.txt");
    assert!(!res2.is_ok());
}

#[test]
fn step_writer_nb_entities() {
    let mut writer = StepWriter::new(StepSchemaType::AP214);
    writer.write_header("TestModel", "Author", "OrgName");
    writer.write_shape("Part-1");
    writer.write_shape("Part-2");
    assert_eq!(writer.nb_shapes, 2);
    assert!(writer.nb_entities() > 2, "header + body entities should exceed shape count");
}
