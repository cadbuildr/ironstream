use ironstream::step_shape::*;

#[test]
fn step_return_status_predicates() {
    assert!(StepReturnStatus::Done.is_done());
    assert!(!StepReturnStatus::Void.is_done());
    assert!(!StepReturnStatus::Error.is_done());
    assert!(StepReturnStatus::Error.is_error());
    assert!(StepReturnStatus::Fail.is_error());
    assert!(!StepReturnStatus::Done.is_error());
    assert!(!StepReturnStatus::Void.is_error());
    assert!(!StepReturnStatus::Stop.is_error());
}

#[test]
fn step_control_reader_read_file_done() {
    let mut r = StepControlReader::new();
    assert_eq!(r.status(), StepReturnStatus::Void);
    let st = r.read_file("model.step");
    assert!(st.is_done());
    assert_eq!(r.status(), StepReturnStatus::Done);
}

#[test]
fn step_control_reader_read_file_error() {
    let mut r = StepControlReader::new();
    let st = r.read_file("");
    assert!(st.is_error());
    assert_eq!(r.status(), StepReturnStatus::Error);
}

#[test]
fn step_control_reader_nb_roots_nb_shapes() {
    let mut r = StepControlReader::new();
    r.read_file("part.stp");
    assert_eq!(r.nb_roots(), 1);
    assert_eq!(r.nb_shapes(), 1);

    // shape(1) is positive; shape(0) or out-of-range is 0
    assert!(r.shape(1) > 0);
    assert_eq!(r.shape(0), 0);
    assert_eq!(r.shape(2), 0);
}

#[test]
fn step_control_writer_transfer_and_write() {
    let mut w = StepControlWriter::new();

    // transfer a valid shape
    let st = w.transfer(42, StepModelType::ManifoldSolidBrep);
    assert!(st.is_done());
    assert_eq!(w.nb_shapes(), 1);
    assert_eq!(w.model_type(), StepModelType::ManifoldSolidBrep);

    // transfer another
    w.transfer(7, StepModelType::ManifoldSolidBrep);
    assert_eq!(w.nb_shapes(), 2);

    // write should succeed when shapes are present
    let st2 = w.write("output.step");
    assert!(st2.is_done());

    // invalid shape_id == 0 returns error
    let mut w2 = StepControlWriter::new();
    let bad = w2.transfer(0, StepModelType::AsIs);
    assert!(bad.is_error());
}

#[test]
fn step_construct_styles_add_find_style_by_index() {
    let mut s = StepConstructStyles::new();
    assert_eq!(s.nb_styles(), 0);

    s.add_style(1, [1.0, 0.0, 0.0]);
    s.add_style(2, [0.0, 1.0, 0.0]);
    s.add_style(3, [0.0, 0.0, 1.0]);
    assert_eq!(s.nb_styles(), 3);

    // style() is 1-based
    assert_eq!(s.style(1), Some((1, [1.0, 0.0, 0.0])));
    assert_eq!(s.style(2), Some((2, [0.0, 1.0, 0.0])));
    assert_eq!(s.style(3), Some((3, [0.0, 0.0, 1.0])));
    assert_eq!(s.style(0), None);   // 0 is out of range
    assert_eq!(s.style(99), None);  // beyond end

    // find color by entity id
    assert_eq!(s.color_for_entity(2), Some([0.0, 1.0, 0.0]));
    assert_eq!(s.color_for_entity(99), None);
}
