extern crate ironstream;
use ironstream::rw_step_reader::*;

#[test]
fn test_rw_step_reader_basic() {
    // StepReader
    let mut r = StepReader::new();
    assert_eq!(r.root_count(), 0);
    assert!(r.shapes.is_empty());

    // read_file with valid path
    let result = r.read_file("/some/model.step");
    assert!(result.is_ok());
    assert_eq!(r.file_path, "/some/model.step");
    assert_eq!(r.root_count(), 1);

    // transfer_roots
    let count = r.transfer_roots();
    assert_eq!(count, 1);
    assert!(!r.shapes.is_empty());
    assert!(r.shapes[0].starts_with("shape_"));

    // one_shape
    let joined = r.one_shape();
    assert!(!joined.is_empty());

    // empty path returns error
    assert!(StepReader::new().read_file("").is_err());

    // WorkSession
    let mut ws = WorkSession::new();
    assert_eq!(ws.entity_count(), 0);
    assert!(ws.load_file("/model.step").is_ok());
    assert!(ws.entity_count() > 0);
    assert!(ws.entity_at(0).is_some());
    assert!(ws.entity_at(999).is_none());
    assert!(ws.load_file("").is_err());

    // read_step free function
    let shapes = read_step("/test.step").expect("should succeed");
    assert!(!shapes.is_empty());
}
