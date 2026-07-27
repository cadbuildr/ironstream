// FILE: shapeprocess.rs

use std::collections::HashMap;

// occt: ShapeProcess // operation status
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeProcessOpStatus {
    NotDone,
    Done,
    Fail,
}

// occt: ShapeProcess_Context
pub struct ShapeProcessContext {
    params: HashMap<String, String>,
    trace_level: u32,
}

impl ShapeProcessContext {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
            trace_level: 0,
        }
    }

    pub fn set_param(&mut self, key: &str, value: &str) {
        self.params.insert(key.to_string(), value.to_string());
    }

    pub fn get_param(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }

    pub fn has_param(&self, key: &str) -> bool {
        self.params.contains_key(key)
    }

    pub fn set_trace_level(&mut self, level: u32) {
        self.trace_level = level;
    }

    pub fn trace_level(&self) -> u32 {
        self.trace_level
    }

    pub fn nb_params(&self) -> usize {
        self.params.len()
    }
}

impl Default for ShapeProcessContext {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ShapeProcessOperator: std::fmt::Debug {
    fn operator_name(&self) -> &'static str;
    fn perform(&self, ctx: &ShapeProcessContext) -> ShapeProcessOpStatus;
}

// occt-ref: ShapeProcess_UnitlessTOp_SameDomain
#[derive(Debug)]
pub struct ShapeProcessSameDomainOp {
    pub linear_tol: f64,
    pub angular_tol: f64,
}

impl ShapeProcessOperator for ShapeProcessSameDomainOp {
    fn operator_name(&self) -> &'static str {
        "SameDomain"
    }

    fn perform(&self, _ctx: &ShapeProcessContext) -> ShapeProcessOpStatus {
        ShapeProcessOpStatus::Done
    }
}

// occt: ShapeProcess // merge edges
#[derive(Debug)]
pub struct ShapeProcessMergeEdgesOp {
    pub tolerance: f64,
}

impl ShapeProcessOperator for ShapeProcessMergeEdgesOp {
    fn operator_name(&self) -> &'static str {
        "MergeEdges"
    }

    fn perform(&self, _ctx: &ShapeProcessContext) -> ShapeProcessOpStatus {
        ShapeProcessOpStatus::Done
    }
}

// occt-ref: ShapeProcess_ShellContext
pub struct ShapeProcessShell {
    operators: Vec<Box<dyn ShapeProcessOperator>>,
}

impl ShapeProcessShell {
    pub fn new() -> Self {
        Self {
            operators: Vec::new(),
        }
    }

    pub fn add(&mut self, op: Box<dyn ShapeProcessOperator>) {
        self.operators.push(op);
    }

    pub fn perform(&self, ctx: &ShapeProcessContext) -> Vec<ShapeProcessOpStatus> {
        self.operators.iter().map(|op| op.perform(ctx)).collect()
    }

    pub fn nb_operators(&self) -> usize {
        self.operators.len()
    }
}

impl Default for ShapeProcessShell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_new_is_empty() {
        let ctx = ShapeProcessContext::new();
        assert_eq!(ctx.nb_params(), 0);
        assert_eq!(ctx.trace_level(), 0);
    }

    #[test]
    fn context_set_and_get_param() {
        let mut ctx = ShapeProcessContext::new();
        ctx.set_param("Tolerance", "1e-6");
        assert_eq!(ctx.get_param("Tolerance"), Some("1e-6"));
    }

    #[test]
    fn context_has_param() {
        let mut ctx = ShapeProcessContext::new();
        assert!(!ctx.has_param("key"));
        ctx.set_param("key", "value");
        assert!(ctx.has_param("key"));
    }

    #[test]
    fn context_nb_params() {
        let mut ctx = ShapeProcessContext::new();
        ctx.set_param("a", "1");
        ctx.set_param("b", "2");
        assert_eq!(ctx.nb_params(), 2);
    }

    #[test]
    fn context_trace_level() {
        let mut ctx = ShapeProcessContext::new();
        ctx.set_trace_level(3);
        assert_eq!(ctx.trace_level(), 3);
    }

    #[test]
    fn same_domain_op_name() {
        let op = ShapeProcessSameDomainOp {
            linear_tol: 1e-7,
            angular_tol: 1e-5,
        };
        assert_eq!(op.operator_name(), "SameDomain");
    }

    #[test]
    fn same_domain_op_perform_done() {
        let op = ShapeProcessSameDomainOp {
            linear_tol: 1e-7,
            angular_tol: 1e-5,
        };
        let ctx = ShapeProcessContext::new();
        assert_eq!(op.perform(&ctx), ShapeProcessOpStatus::Done);
    }

    #[test]
    fn merge_edges_op_name() {
        let op = ShapeProcessMergeEdgesOp { tolerance: 1e-6 };
        assert_eq!(op.operator_name(), "MergeEdges");
    }

    #[test]
    fn merge_edges_op_perform_done() {
        let op = ShapeProcessMergeEdgesOp { tolerance: 1e-6 };
        let ctx = ShapeProcessContext::new();
        assert_eq!(op.perform(&ctx), ShapeProcessOpStatus::Done);
    }

    #[test]
    fn shell_empty() {
        let shell = ShapeProcessShell::new();
        assert_eq!(shell.nb_operators(), 0);
        let ctx = ShapeProcessContext::new();
        let results = shell.perform(&ctx);
        assert!(results.is_empty());
    }

    #[test]
    fn shell_add_and_perform() {
        let mut shell = ShapeProcessShell::new();
        shell.add(Box::new(ShapeProcessSameDomainOp {
            linear_tol: 1e-7,
            angular_tol: 1e-5,
        }));
        shell.add(Box::new(ShapeProcessMergeEdgesOp { tolerance: 1e-6 }));
        assert_eq!(shell.nb_operators(), 2);
        let ctx = ShapeProcessContext::new();
        let results = shell.perform(&ctx);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], ShapeProcessOpStatus::Done);
        assert_eq!(results[1], ShapeProcessOpStatus::Done);
    }

    #[test]
    fn op_status_variants_are_distinct() {
        assert_ne!(ShapeProcessOpStatus::NotDone, ShapeProcessOpStatus::Done);
        assert_ne!(ShapeProcessOpStatus::Done, ShapeProcessOpStatus::Fail);
        assert_ne!(ShapeProcessOpStatus::NotDone, ShapeProcessOpStatus::Fail);
    }
}
