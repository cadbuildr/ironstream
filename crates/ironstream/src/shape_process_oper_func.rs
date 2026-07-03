// FILE: shape_process_oper_func.rs
// occt: ShapeProcess_OperFunc

/// Type alias for ShapeProcess operator function
/// Takes context and progress range, returns success status
pub type OperFunc = fn(&str) -> bool;

/// Wraps operator function signature
pub struct OperFuncWrapper {
    func: OperFunc,
}

impl OperFuncWrapper {
    /// Creates wrapper around operator function
    pub fn new(f: OperFunc) -> Self {
        OperFuncWrapper { func: f }
    }

    /// Calls the wrapped function
    pub fn call(&self, context: &str) -> bool {
        (self.func)(context)
    }
}

#[cfg(test)]
mod tests {
    use super::{OperFuncWrapper, OperFunc};

    fn test_op(context: &str) -> bool {
        !context.is_empty()
    }

    #[test]
    fn test_oper_func() {
        let wrapper = OperFuncWrapper::new(test_op as OperFunc);
        assert!(wrapper.call("test_context"));
        assert!(!wrapper.call(""));
    }
}
