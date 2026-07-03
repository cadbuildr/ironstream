// FILE: adv_app2_var_evaluator_func2_var_o.rs
// occt: AdvApp2Var_EvaluatorFunc2Var

//! Evaluator for 2D functions in advanced approximation.

pub type EvaluatorFn = fn(u: f64, v: f64, result: &mut [f64]) -> bool;

pub struct EvaluatorFunc2Var {
    evaluator: Option<EvaluatorFn>,
}

impl EvaluatorFunc2Var {
    pub fn new(evaluator: EvaluatorFn) -> Self {
        EvaluatorFunc2Var {
            evaluator: Some(evaluator),
        }
    }

    pub fn evaluate(&self, u: f64, v: f64, result: &mut [f64]) -> bool {
        match self.evaluator {
            Some(f) => f(u, v, result),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluator_create() {
        fn dummy_eval(_u: f64, _v: f64, result: &mut [f64]) -> bool {
            if !result.is_empty() {
                result[0] = 1.0;
            }
            true
        }

        let eval = EvaluatorFunc2Var::new(dummy_eval);
        let mut result = vec![0.0];
        assert!(eval.evaluate(0.5, 0.5, &mut result));
        assert_eq!(result[0], 1.0);
    }
}
