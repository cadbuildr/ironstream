// FILE: app_cont_function_o.rs
// occt: AppCont_Function

//! Base function for approximation continuum.

pub trait ContinuumFunction {
    fn eval(&self, t: f64) -> Option<f64>;
    fn eval_derivative(&self, t: f64) -> Option<f64> {
        None
    }
}

pub struct SimpleFunction {
    f: Box<dyn Fn(f64) -> Option<f64>>,
}

impl SimpleFunction {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(f64) -> Option<f64> + 'static,
    {
        SimpleFunction { f: Box::new(f) }
    }
}

impl ContinuumFunction for SimpleFunction {
    fn eval(&self, t: f64) -> Option<f64> {
        (self.f)(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_function() {
        let f = SimpleFunction::new(|t| Some(t * t));
        assert_eq!(f.eval(2.0), Some(4.0));
    }
}
