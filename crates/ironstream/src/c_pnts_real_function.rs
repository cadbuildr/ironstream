// FILE: c_pnts_real_function.rs
// occt: CPnts_RealFunction

/// Base function interface for curve parameter computations.
pub type CPntsRealFunction = fn(f64) -> f64;

/// Function evaluation wrapper.
pub struct CPntsRealFunctionEvaluator {
    func: Option<CPntsRealFunction>,
}

impl CPntsRealFunctionEvaluator {
    /// Creates a new function evaluator.
    pub fn new() -> Self {
        CPntsRealFunctionEvaluator { func: None }
    }

    /// Sets the function to evaluate.
    pub fn set_function(&mut self, f: CPntsRealFunction) {
        self.func = Some(f);
    }

    /// Evaluates the function at x.
    pub fn eval(&self, x: f64) -> Option<f64> {
        self.func.map(|f| f(x))
    }

    /// Checks if function is set.
    pub fn has_function(&self) -> bool {
        self.func.is_some()
    }
}

impl Default for CPntsRealFunctionEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let eval = CPntsRealFunctionEvaluator::new();
        assert!(!eval.has_function());
    }

    #[test]
    fn test_set_and_eval() {
        let mut eval = CPntsRealFunctionEvaluator::new();
        eval.set_function(|x| x * 2.0);
        assert!(eval.has_function());
        assert_eq!(eval.eval(3.0), Some(6.0));
    }

    #[test]
    fn test_eval_without_function() {
        let eval = CPntsRealFunctionEvaluator::new();
        assert_eq!(eval.eval(1.0), None);
    }

    #[test]
    fn test_multiple_functions() {
        let mut eval = CPntsRealFunctionEvaluator::new();
        eval.set_function(|x| x + 1.0);
        assert_eq!(eval.eval(5.0), Some(6.0));

        eval.set_function(|x| x * x);
        assert_eq!(eval.eval(4.0), Some(16.0));
    }
}
