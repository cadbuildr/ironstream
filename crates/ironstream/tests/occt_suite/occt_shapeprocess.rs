// FILE: tests/occt_shapeprocess.rs
extern crate ironstream;

use ironstream::shapeprocess::{
    ShapeProcessContext, ShapeProcessMergeEdgesOp, ShapeProcessOpStatus,
    ShapeProcessOperator, ShapeProcessSameDomainOp, ShapeProcessShell,
};

#[test]
fn context_default_trace_level_is_zero() {
    let ctx = ShapeProcessContext::new();
    assert_eq!(ctx.trace_level(), 0);
}

#[test]
fn context_set_get_string_param() {
    let mut ctx = ShapeProcessContext::new();
    ctx.set_param("LinearTolerance", "0.001");
    assert_eq!(ctx.get_param("LinearTolerance"), Some("0.001"));
}

#[test]
fn context_missing_param_returns_none() {
    let ctx = ShapeProcessContext::new();
    assert_eq!(ctx.get_param("NonExistent"), None);
}

#[test]
fn context_has_param_after_set() {
    let mut ctx = ShapeProcessContext::new();
    assert!(!ctx.has_param("Mode"));
    ctx.set_param("Mode", "merge");
    assert!(ctx.has_param("Mode"));
}

#[test]
fn context_overwrite_param() {
    let mut ctx = ShapeProcessContext::new();
    ctx.set_param("Tol", "1e-5");
    ctx.set_param("Tol", "1e-7");
    assert_eq!(ctx.get_param("Tol"), Some("1e-7"));
    assert_eq!(ctx.nb_params(), 1);
}

#[test]
fn context_nb_params_counts_unique_keys() {
    let mut ctx = ShapeProcessContext::new();
    ctx.set_param("A", "1");
    ctx.set_param("B", "2");
    ctx.set_param("C", "3");
    assert_eq!(ctx.nb_params(), 3);
}

#[test]
fn context_set_trace_level() {
    let mut ctx = ShapeProcessContext::new();
    ctx.set_trace_level(5);
    assert_eq!(ctx.trace_level(), 5);
    ctx.set_trace_level(0);
    assert_eq!(ctx.trace_level(), 0);
}

#[test]
fn same_domain_op_name_and_tolerances() {
    let op = ShapeProcessSameDomainOp {
        linear_tol: 1e-7,
        angular_tol: 1e-4,
    };
    assert_eq!(op.operator_name(), "SameDomain");
    assert!((op.linear_tol - 1e-7).abs() < 1e-15);
    assert!((op.angular_tol - 1e-4).abs() < 1e-12);
}

#[test]
fn same_domain_op_returns_done() {
    let op = ShapeProcessSameDomainOp {
        linear_tol: 1e-6,
        angular_tol: 1e-3,
    };
    let mut ctx = ShapeProcessContext::new();
    ctx.set_param("SameDomain.LinearTol", "1e-6");
    assert_eq!(op.perform(&ctx), ShapeProcessOpStatus::Done);
}

#[test]
fn merge_edges_op_name_and_tolerance() {
    let op = ShapeProcessMergeEdgesOp { tolerance: 0.01 };
    assert_eq!(op.operator_name(), "MergeEdges");
    assert!((op.tolerance - 0.01).abs() < 1e-12);
}

#[test]
fn merge_edges_op_returns_done() {
    let op = ShapeProcessMergeEdgesOp { tolerance: 1e-5 };
    let ctx = ShapeProcessContext::new();
    assert_eq!(op.perform(&ctx), ShapeProcessOpStatus::Done);
}

#[test]
fn shell_new_has_zero_operators() {
    let shell = ShapeProcessShell::new();
    assert_eq!(shell.nb_operators(), 0);
}

#[test]
fn shell_perform_empty_yields_empty_vec() {
    let shell = ShapeProcessShell::new();
    let ctx = ShapeProcessContext::new();
    let results = shell.perform(&ctx);
    assert!(results.is_empty());
}

#[test]
fn shell_with_both_ops_returns_two_done_statuses() {
    let mut shell = ShapeProcessShell::new();
    shell.add(Box::new(ShapeProcessSameDomainOp {
        linear_tol: 1e-7,
        angular_tol: 1e-5,
    }));
    shell.add(Box::new(ShapeProcessMergeEdgesOp { tolerance: 1e-6 }));

    let mut ctx = ShapeProcessContext::new();
    ctx.set_param("SameDomain.LinearTol", "1e-7");
    ctx.set_param("MergeEdges.Tol", "1e-6");
    ctx.set_trace_level(1);

    assert_eq!(shell.nb_operators(), 2);
    let results = shell.perform(&ctx);
    assert_eq!(results.len(), 2);
    assert_eq!(results[0], ShapeProcessOpStatus::Done);
    assert_eq!(results[1], ShapeProcessOpStatus::Done);
}

#[test]
fn op_status_copy_and_debug() {
    let s = ShapeProcessOpStatus::NotDone;
    let t = s;
    assert_eq!(s, t);
    let dbg = format!("{:?}", ShapeProcessOpStatus::Fail);
    assert!(dbg.contains("Fail"));
}
