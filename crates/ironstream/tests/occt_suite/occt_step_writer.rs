extern crate ironstream;
use ironstream::step_writer::*;

#[test]
fn test_step_writer_basic() {
    // StepReturnStatus
    assert!(StepReturnStatus::Done.is_ok());
    assert!(!StepReturnStatus::Fail.is_ok());
    assert!(!StepReturnStatus::Void.is_ok());
    assert!(!StepReturnStatus::Error("e".to_string()).is_ok());

    // StepWriter
    let mut w = StepWriter::new();
    assert_eq!(w.schema, "CONFIG_CONTROL_DESIGN");
    assert_eq!(w.shape_count(), 0);

    let status = w.transfer("box_shape");
    assert_eq!(status, StepReturnStatus::Done);
    assert_eq!(w.shape_count(), 1);

    let void_status = w.transfer("");
    assert_eq!(void_status, StepReturnStatus::Void);

    w.add_shape("cylinder_shape");
    assert_eq!(w.shape_count(), 2);

    // with_schema
    let w2 = StepWriter::with_schema("AUTOMOTIVE_DESIGN");
    assert_eq!(w2.schema, "AUTOMOTIVE_DESIGN");

    // write_step_file convenience function
    let shapes = vec!["shape_a".to_string(), "shape_b".to_string()];
    let result = write_step_file(&shapes, "/dev/null");
    // /dev/null is writable on Linux
    assert!(result.is_ok());
}
