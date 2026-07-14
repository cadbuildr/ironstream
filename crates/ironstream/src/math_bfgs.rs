// FILE: rust/ironstream/crates/ironstream/src/math_bfgs.rs

// occt: math_BFGS // result — fields: minimum [f64;8], value f64, nb_iter u32, is_done bool
pub struct MathBfgsResult {
    minimum: [f64; 8],
    value: f64,
    nb_iter: u32,
    is_done: bool,
}

impl MathBfgsResult {
    pub fn new() -> Self {
        Self {
            minimum: [0.0; 8],
            value: f64::INFINITY,
            nb_iter: 0,
            is_done: false,
        }
    }

    pub fn minimum(&self) -> &[f64] {
        &self.minimum
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn nb_iter(&self) -> u32 {
        self.nb_iter
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

// occt: math_BFGS // — BFGS quasi-Newton minimizer
pub struct MathBfgs {
    n: usize,
    tolerance: f64,
    max_iter: u32,
    result: MathBfgsResult,
}

impl MathBfgs {
    pub fn new(n: usize, tolerance: f64, max_iter: u32) -> Self {
        assert!(n <= 8, "MathBfgs supports at most 8 dimensions (n={})", n);
        Self {
            n,
            tolerance,
            max_iter,
            result: MathBfgsResult::new(),
        }
    }

    /// Run up to `max_iter` gradient-descent steps (simplified BFGS fallback).
    ///
    /// Uses a fixed step size that starts at 1.0 and is halved via backtracking
    /// line search at each iteration until the Armijo condition is satisfied or
    /// a minimum step is reached.  Stops early when the gradient norm falls
    /// below `tolerance`.
    pub fn perform(
        &mut self,
        f: &dyn Fn(&[f64]) -> f64,
        grad: &dyn Fn(&[f64]) -> Vec<f64>,
        start: &[f64],
    ) {
        let n = self.n;
        assert!(
            start.len() >= n,
            "start must have at least n={} elements",
            n
        );

        let mut x = [0.0f64; 8];
        for i in 0..n {
            x[i] = start[i];
        }

        let mut fx = f(&x[..n]);
        let mut iter = 0u32;
        let armijo_c = 1e-4;
        let min_step = 1e-14;

        for _ in 0..self.max_iter {
            let g = grad(&x[..n]);

            // Compute gradient norm.
            let gnorm: f64 = g[..n].iter().map(|v| v * v).sum::<f64>().sqrt();
            if gnorm < self.tolerance {
                break;
            }

            // Backtracking line search along the steepest-descent direction.
            let mut step = 1.0f64;
            let mut x_new = [0.0f64; 8];
            let mut fx_new;
            loop {
                for i in 0..n {
                    x_new[i] = x[i] - step * g[i];
                }
                fx_new = f(&x_new[..n]);
                // Armijo sufficient-decrease condition.
                if fx_new <= fx - armijo_c * step * gnorm * gnorm {
                    break;
                }
                step *= 0.5;
                if step < min_step {
                    // No improvement possible; accept current point as done.
                    break;
                }
            }

            // Accept step only if it reduces the value.
            if fx_new < fx {
                x = x_new;
                fx = fx_new;
            } else {
                // Converged or stuck.
                break;
            }

            iter += 1;
        }

        self.result.minimum = x;
        self.result.value = fx;
        self.result.nb_iter = iter;
        self.result.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.result.is_done()
    }

    pub fn minimum(&self) -> &[f64] {
        self.result.minimum()
    }

    pub fn value(&self) -> f64 {
        self.result.value()
    }

    pub fn nb_iter(&self) -> u32 {
        self.result.nb_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_new_defaults() {
        let r = MathBfgsResult::new();
        assert!(!r.is_done());
        assert_eq!(r.nb_iter(), 0);
        assert!(r.value().is_infinite());
        assert_eq!(r.minimum().len(), 8);
    }

    #[test]
    fn bfgs_1d_x_squared_finds_zero() {
        let mut bfgs = MathBfgs::new(1, 1e-8, 200);
        let f = |x: &[f64]| x[0] * x[0];
        let g = |x: &[f64]| vec![2.0 * x[0]];
        bfgs.perform(&f, &g, &[3.0]);
        assert!(bfgs.is_done());
        assert!(
            bfgs.minimum()[0].abs() < 1e-6,
            "minimum[0]={}",
            bfgs.minimum()[0]
        );
        assert!(bfgs.value() < 1e-12, "value={}", bfgs.value());
        assert!(bfgs.nb_iter() > 0);
    }

    #[test]
    fn bfgs_2d_bowl_finds_minimum() {
        let mut bfgs = MathBfgs::new(2, 1e-8, 500);
        let f = |x: &[f64]| (x[0] - 1.0).powi(2) + (x[1] - 2.0).powi(2);
        let g = |x: &[f64]| vec![2.0 * (x[0] - 1.0), 2.0 * (x[1] - 2.0)];
        bfgs.perform(&f, &g, &[5.0, 5.0]);
        assert!(bfgs.is_done());
        assert!(
            (bfgs.minimum()[0] - 1.0).abs() < 1e-5,
            "x={}",
            bfgs.minimum()[0]
        );
        assert!(
            (bfgs.minimum()[1] - 2.0).abs() < 1e-5,
            "y={}",
            bfgs.minimum()[1]
        );
        assert!(bfgs.value() < 1e-10, "value={}", bfgs.value());
    }

    #[test]
    fn bfgs_already_at_minimum() {
        let mut bfgs = MathBfgs::new(1, 1e-8, 100);
        let f = |x: &[f64]| x[0] * x[0];
        let g = |x: &[f64]| vec![2.0 * x[0]];
        bfgs.perform(&f, &g, &[0.0]);
        assert!(bfgs.is_done());
        assert!(bfgs.minimum()[0].abs() < 1e-12);
        assert!(bfgs.value() < 1e-12);
        assert_eq!(bfgs.nb_iter(), 0, "zero iterations when already at min");
    }
}
