// Integration tests for transfer_brep module
use ironstream::transfer_brep::*;

#[test]
fn transfer_status_is_done_only_when_done() {
    assert!(!TransferStatus::Initial.is_done());
    assert!(!TransferStatus::Run.is_done());
    assert!(TransferStatus::Done.is_done());
    assert!(!TransferStatus::Error.is_done());
    assert!(!TransferStatus::Void.is_done());
}

#[test]
fn transfer_binder_new_then_set_result() {
    let mut b = TransferBinder::new();
    assert!(!b.is_done());
    assert!(!b.has_result());
    b.set_result(42);
    assert!(b.is_done());
    assert!(b.has_result());
    assert_eq!(b.result(), 42);
}

#[test]
fn transfer_binder_add_messages() {
    let mut b = TransferBinder::new();
    assert_eq!(b.nb_messages(), 0);
    b.add_message("first warning");
    b.add_message("second warning");
    assert_eq!(b.nb_messages(), 2);
}

#[test]
fn shape_binder_set_shape_and_query() {
    let mut sb = ShapeBinder::new();
    assert!(!sb.is_done());
    sb.set_shape(100, 4);
    assert!(sb.is_done());
    assert_eq!(sb.result_shape(), 100);
    assert_eq!(sb.shape_type(), 4);
}

#[test]
fn shape_mapper_map_and_query() {
    let mut m = ShapeMapper::new(10);
    assert!(!m.is_mapped());
    assert_eq!(m.result(), None);
    m.map_to(99);
    assert!(m.is_mapped());
    assert_eq!(m.result(), Some(99));
}

#[test]
fn finder_process_bind_find_and_transfer_brep_results() {
    let mut fp = FinderProcess::new();
    let mut b1 = ShapeBinder::new();
    b1.set_shape(10, 4);
    let mut b2 = ShapeBinder::new();
    b2.set_shape(20, 5);
    fp.bind(1, b1);
    fp.bind(2, b2);
    assert!(fp.is_bound(1));
    assert!(fp.is_bound(2));
    assert!(!fp.is_bound(3));
    assert_eq!(fp.nb_mapped(), 2);
    assert_eq!(fp.shape_result(1), Some(10));
    fp.add_warning("gap detected");
    assert_eq!(fp.nb_messages(), 1);
    assert_eq!(TransferBrep::nb_results(&fp), 2);
    let shapes = TransferBrep::shape_results(&fp);
    assert!(shapes.contains(&10));
    assert!(shapes.contains(&20));
    assert_eq!(TransferBrep::shape_result(&fp, 1), Some(10));
}
