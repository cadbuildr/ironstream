// FILE: tests/occt_int_tools.rs
extern crate ironstream;

use ironstream::int_tools::{
    IntToolsCommonPrt, IntToolsContext, IntToolsEdgeEdge, IntToolsEdgeEdgeStatus,
    IntToolsFClass2d,
};

#[test]
fn edge_edge_starts_not_done() {
    let ee = IntToolsEdgeEdge::new(1e-7);
    assert!(!ee.is_done());
}

#[test]
fn edge_edge_perform_marks_done() {
    let mut ee = IntToolsEdgeEdge::new(1e-7);
    ee.perform();
    assert!(ee.is_done());
}

#[test]
fn edge_edge_no_parts_before_add() {
    let mut ee = IntToolsEdgeEdge::new(1e-7);
    ee.perform();
    assert_eq!(ee.nb_common_parts(), 0);
    assert!(ee.common_parts().is_empty());
}

#[test]
fn edge_edge_add_intersect_part() {
    let mut ee = IntToolsEdgeEdge::new(1e-7);
    ee.perform();
    let part = IntToolsCommonPrt::new(0.25, 0.75, 0.1, 0.9, IntToolsEdgeEdgeStatus::Intersect);
    ee.add_part(part);
    assert_eq!(ee.nb_common_parts(), 1);
    let parts = ee.common_parts();
    assert_eq!(parts[0].status, IntToolsEdgeEdgeStatus::Intersect);
    assert!((parts[0].param1_start - 0.25).abs() < 1e-15);
    assert!((parts[0].param1_end - 0.75).abs() < 1e-15);
}

#[test]
fn edge_edge_add_multiple_parts() {
    let mut ee = IntToolsEdgeEdge::new(1e-7);
    ee.perform();
    ee.add_part(IntToolsCommonPrt::new(0.0, 1.0, 0.0, 1.0, IntToolsEdgeEdgeStatus::Coincide));
    ee.add_part(IntToolsCommonPrt::new(0.5, 0.5, 0.5, 0.5, IntToolsEdgeEdgeStatus::Intersect));
    ee.add_part(IntToolsCommonPrt::new(0.0, 0.0, 0.0, 0.0, IntToolsEdgeEdgeStatus::Parallel));
    assert_eq!(ee.nb_common_parts(), 3);
}

#[test]
fn edge_edge_common_parts_status_sequence() {
    let mut ee = IntToolsEdgeEdge::new(1e-4);
    ee.add_part(IntToolsCommonPrt::new(0.0, 0.5, 0.0, 0.5, IntToolsEdgeEdgeStatus::Coincide));
    ee.add_part(IntToolsCommonPrt::new(0.6, 0.6, 0.6, 0.6, IntToolsEdgeEdgeStatus::Intersect));
    let parts = ee.common_parts();
    assert_eq!(parts[0].status, IntToolsEdgeEdgeStatus::Coincide);
    assert_eq!(parts[1].status, IntToolsEdgeEdgeStatus::Intersect);
}

#[test]
fn fclass2d_not_initialized_by_default() {
    let fc = IntToolsFClass2d::new(1e-6);
    assert!(!fc.is_initialized());
}

#[test]
fn fclass2d_initialize_sets_flag() {
    let mut fc = IntToolsFClass2d::new(1e-6);
    fc.initialize();
    assert!(fc.is_initialized());
}

#[test]
fn fclass2d_perform_inside_positive_quadrant() {
    let fc = IntToolsFClass2d::new(1e-6);
    assert!(fc.perform(0.0, 0.0));
    assert!(fc.perform(10.0, 10.0));
    assert!(fc.perform(1e-10, 1e-10));
}

#[test]
fn fclass2d_perform_outside() {
    let fc = IntToolsFClass2d::new(1e-6);
    assert!(!fc.perform(-1.0, 0.0));
    assert!(!fc.perform(0.0, -1.0));
    assert!(!fc.perform(-0.001, -0.001));
}

#[test]
fn context_default_tolerance_is_1e7() {
    let ctx = IntToolsContext::new();
    assert!((ctx.tolerance() - 1e-7).abs() < 1e-15);
}

#[test]
fn context_set_and_get_tolerance() {
    let mut ctx = IntToolsContext::new();
    ctx.set_tolerance(1e-3);
    assert!((ctx.tolerance() - 1e-3).abs() < 1e-15);
}

#[test]
fn context_parallel_default_false_and_toggle() {
    let mut ctx = IntToolsContext::new();
    assert!(!ctx.is_parallel());
    ctx.set_parallel(true);
    assert!(ctx.is_parallel());
    ctx.set_parallel(false);
    assert!(!ctx.is_parallel());
}

#[test]
fn status_enum_copy_and_debug() {
    let s = IntToolsEdgeEdgeStatus::NotDone;
    let s2 = s;
    assert_eq!(s, s2);
    let variants = [
        IntToolsEdgeEdgeStatus::NotDone,
        IntToolsEdgeEdgeStatus::Parallel,
        IntToolsEdgeEdgeStatus::Coincide,
        IntToolsEdgeEdgeStatus::Intersect,
    ];
    for v in variants {
        let _ = format!("{v:?}");
    }
}

#[test]
fn common_prt_param2_fields() {
    let p = IntToolsCommonPrt::new(0.0, 1.0, 2.0, 3.0, IntToolsEdgeEdgeStatus::NotDone);
    assert!((p.param2_start - 2.0).abs() < 1e-15);
    assert!((p.param2_end - 3.0).abs() < 1e-15);
    assert_eq!(p.status, IntToolsEdgeEdgeStatus::NotDone);
}
