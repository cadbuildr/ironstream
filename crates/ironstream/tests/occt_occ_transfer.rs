// FILE: tests/occt_occ_transfer.rs
extern crate ironstream;

use ironstream::occ_transfer::{
    TransferStatus, TransferResult, TransientProcess, TransferFinder, XSControlTransferReader,
};

#[test]
fn transfer_status_is_done() {
    assert!(TransferStatus::Done.is_done());
    assert!(TransferStatus::DoneWithWarning.is_done());
    assert!(!TransferStatus::Error.is_done());
    assert!(!TransferStatus::Fail.is_done());
    assert!(!TransferStatus::NotStarted.is_done());
}

#[test]
fn transfer_result_ok_and_failed() {
    let ok = TransferResult::ok(1, 1001);
    assert!(ok.is_done());
    assert_eq!(ok.result_shape_id, 1001);

    let fail = TransferResult::failed(2, "bad entity");
    assert!(!fail.is_done());
    assert_eq!(fail.result_shape_id, 0);
    assert_eq!(fail.message, "bad entity");
}

#[test]
fn transient_process_bind_and_find() {
    let mut p = TransientProcess::new();
    p.bind(10, 20010);
    p.bind(11, 20011);
    assert_eq!(p.find_result(10), Some(20010));
    assert_eq!(p.find_result(11), Some(20011));
    assert_eq!(p.find_result(99), None);
    assert_eq!(p.nb_mapped(), 2);
}

#[test]
fn transient_process_errors_and_warnings() {
    let mut p = TransientProcess::new();
    p.add_error("e1".into());
    p.add_error("e2".into());
    p.add_warning("w1".into());
    assert_eq!(p.nb_errors(), 2);
    assert_eq!(p.nb_warnings(), 1);
    p.clear();
    assert_eq!(p.nb_errors(), 0);
    assert_eq!(p.nb_mapped(), 0);
}

#[test]
fn transfer_finder_found_and_not_found() {
    let mut f = TransferFinder::new(7);
    assert!(!f.is_found());
    assert_eq!(f.result(), None);
    f.set_entity(42);
    f.set_result(9000);
    assert!(f.is_found());
    assert_eq!(f.result(), Some(9000));
}

#[test]
fn xs_control_transfer_reader_transfer_roots() {
    let mut r = XSControlTransferReader::new(5);
    assert!(!r.is_done());
    let n = r.transfer_roots(vec![1, 2, 3]);
    assert_eq!(n, 3);
    assert!(r.is_done());
    assert_eq!(r.nb_shapes(), 3);
    // shape_id = entity_id + 60000
    assert_eq!(r.one_shape(), Some(1 + 60000));
    r.clear();
    assert_eq!(r.nb_shapes(), 0);
    assert_eq!(r.status, TransferStatus::NotStarted);
}
