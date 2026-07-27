// FILE: tests/occt_bop_ds.rs
extern crate ironstream;

use ironstream::bop_ds::{
    BopBuilder, BopCheckResult, BopCheckStatus, BopDS, BopFaceInfo, BopOperation,
    BopPaveBlock, BopShapeInfo, BopShapeType,
};

#[test]
fn shape_info_bbox_diagonal() {
    let info = BopShapeInfo::new(0, BopShapeType::Solid)
        .with_bbox([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let diag = info.diagonal();
    assert!((diag - 3.0f64.sqrt()).abs() < 1e-10);
}

#[test]
fn pave_block_contains_param() {
    let pb = BopPaveBlock::new(0, 0.0, 1.0, 0, 1);
    assert!(pb.contains_param(0.0));
    assert!(pb.contains_param(0.5));
    assert!(pb.contains_param(1.0));
    assert!(!pb.contains_param(-0.1));
    assert!(!pb.contains_param(1.1));
    assert!(!pb.is_degenerate());
}

#[test]
fn face_info_is_touched() {
    let mut fi = BopFaceInfo::new(0);
    assert!(!fi.is_touched(), "empty face info should not be touched");
    fi.in_pave_blocks.push(0);
    assert!(fi.is_touched(), "face info with in_pave_blocks should be touched");
}

#[test]
fn bop_builder_is_generated() {
    let mut b = BopBuilder::new(BopOperation::Fuse);
    b.perform(3);
    assert!(b.is_done);
    b.generated.push((0, 3));
    b.generated.push((1, 4));
    assert!(b.is_generated(0));
    assert!(b.is_generated(1));
    assert!(!b.is_generated(2));
    assert_eq!(b.nb_generated(), 2);
}

#[test]
fn bop_ds_add_pave_blocks() {
    let mut ds = BopDS::new(1e-7);
    let pb = BopPaveBlock::new(0, 0.0, 2.0, 0, 1);
    let idx = ds.add_pave_block(pb);
    assert_eq!(idx, 0);
    assert_eq!(ds.nb_pave_blocks(), 1);
    assert!(ds.pave_block(0).is_some());
}
