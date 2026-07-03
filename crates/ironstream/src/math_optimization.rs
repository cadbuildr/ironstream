// FILE: rust/ironstream/crates/ironstream/src/math_optimization.rs

// occt: math_GlobOptMin result — fields: minimum Vec<f64>, value f64, nb_iter u32, is_done bool
pub struct MathGSSectionResult {
    minimum: Vec<f64>,
    value: f64,
    nb_iter: u32,
    is_done: bool,
}

impl MathGSSectionResult {
    pub fn new() -> Self {
        Self {
            minimum: Vec::new(),
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

// occt: math_GoldenSection — 1D minimization on [a, b] using golden-section search
pub struct MathGoldenSection {
    result: f64,
    value: f64,
    is_done: bool,
    nb_iter: u32,
}

impl MathGoldenSection {
    pub fn new() -> Self {
        Self {
            result: 0.0,
            value: f64::INFINITY,
            is_done: false,
            nb_iter: 0,
        }
    }

    /// Minimize `f` on `[a, b]` to within tolerance `tol`, up to `max_iter` evaluations.
    ///
    /// Implements the standard golden-section search: maintains a bracket [lo, hi]
    /// and two interior probes x1, x2 separated by the golden ratio, eliminating
    /// one sub-interval per iteration.  Stops when |hi - lo| < tol or `max_iter`
    /// is reached.
    pub fn perform(
        &mut self,
        f: &dyn Fn(f64) -> f64,
        a: f64,
        b: f64,
        tol: f64,
        max_iter: u32,
    ) {
        // Golden ratio conjugate: (sqrt(5) - 1) / 2
        const PHI_CONJ: f64 = 0.618_033_988_749_895;

        let mut lo = a;
        let mut hi = b;

        let mut x1 = hi - PHI_CONJ * (hi - lo);
        let mut x2 = lo + PHI_CONJ * (hi - lo);
        let mut f1 = f(x1);
        let mut f2 = f(x2);
        let mut iter = 0u32;

        while (hi - lo) > tol && iter < max_iter {
            if f1 < f2 {
                hi = x2;
                x2 = x1;
                f2 = f1;
                x1 = hi - PHI_CONJ * (hi - lo);
                f1 = f(x1);
            } else {
                lo = x1;
                x1 = x2;
                f1 = f2;
                x2 = lo + PHI_CONJ * (hi - lo);
                f2 = f(x2);
            }
            iter += 1;
        }

        let best_x = (lo + hi) / 2.0;
        self.result = best_x;
        self.value = f(best_x);
        self.nb_iter = iter;
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn result(&self) -> f64 {
        self.result
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn nb_iter(&self) -> u32 {
        self.nb_iter
    }
}

// occt: math_Powell — multidimensional minimization (coordinate-descent stub)
pub struct MathPowell {
    n: usize,
    result: Vec<f64>,
    value: f64,
    is_done: bool,
    nb_iter: u32,
}

impl MathPowell {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            result: vec![0.0; n],
            value: f64::INFINITY,
            is_done: false,
            nb_iter: 0,
        }
    }

    /// Minimize `f` starting from `start` using coordinate descent.
    ///
    /// Cycles through each coordinate axis, applying a golden-section line
    /// search along that axis while keeping all other coordinates fixed.
    /// Stops when the maximum single-coordinate movement in a full sweep
    /// falls below `tol`, or after `max_iter` full sweeps.
    pub fn perform(
        &mut self,
        f: &dyn Fn(&[f64]) -> f64,
        start: Vec<f64>,
        tol: f64,
        max_iter: u32,
    ) {
        assert!(
            start.len() >= self.n,
            "start must have at least n={} elements",
            self.n
        );

        const PHI_CONJ: f64 = 0.618_033_988_749_895;
        // Line-search bracket half-width; large enough to capture distant minima.
        const BRACKET: f64 = 10.0;
        const LINE_TOL: f64 = 1e-10;
        const LINE_ITER: u32 = 200;

        let n = self.n;
        let mut x: Vec<f64> = start[..n].to_vec();
        let mut fx = f(&x);
        let mut iter = 0u32;

        'outer: for _ in 0..max_iter {
            let mut max_move = 0.0f64;

            for coord in 0..n {
                let x_old = x[coord];

                // Build a 1-D closure along `coord` with all others fixed.
                // We capture a snapshot of the current point to avoid aliasing.
                let mut probe = x.clone();
                let mut line_f = |t: f64| {
                    probe[coord] = t;
                    f(&probe)
                };

                let lo = x_old - BRACKET;
                let hi = x_old + BRACKET;

                // Golden-section search on [lo, hi].
                let mut a = lo;
                let mut b = hi;
                let mut lx1 = b - PHI_CONJ * (b - a);
                let mut lx2 = a + PHI_CONJ * (b - a);
                let mut lf1 = line_f(lx1);
                let mut lf2 = line_f(lx2);
                let mut line_iter = 0u32;
                while (b - a) > LINE_TOL && line_iter < LINE_ITER {
                    if lf1 < lf2 {
                        b = lx2;
                        lx2 = lx1;
                        lf2 = lf1;
                        lx1 = b - PHI_CONJ * (b - a);
                        lf1 = line_f(lx1);
                    } else {
                        a = lx1;
                        lx1 = lx2;
                        lf1 = lf2;
                        lx2 = a + PHI_CONJ * (b - a);
                        lf2 = line_f(lx2);
                    }
                    line_iter += 1;
                }
                let best_t = (a + b) / 2.0;
                x[coord] = best_t;
                fx = f(&x);
                let move_dist = (x[coord] - x_old).abs();
                if move_dist > max_move {
                    max_move = move_dist;
                }
            }

            iter += 1;

            if max_move < tol {
                break 'outer;
            }
        }

        self.result = x;
        self.value = fx;
        self.nb_iter = iter;
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn result(&self) -> &[f64] {
        &self.result
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn nb_iter(&self) -> u32 {
        self.nb_iter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gs_result_new_defaults() {
        let r = MathGSSectionResult::new();
        assert!(!r.is_done());
        assert_eq!(r.nb_iter(), 0);
        assert!(r.value().is_infinite());
        assert!(r.minimum().is_empty());
    }

    #[test]
    fn golden_section_minimizes_x_squared() {
        let mut gs = MathGoldenSection::new();
        gs.perform(&|x| x * x, -5.0, 5.0, 1e-10, 200);
        assert!(gs.is_done());
        assert!(gs.result().abs() < 1e-6, "result={}", gs.result());
        assert!(gs.value() < 1e-12, "value={}", gs.value());
        assert!(gs.nb_iter() > 0);
    }

    #[test]
    fn golden_section_shifted_parabola() {
        let mut gs = MathGoldenSection::new();
        gs.perform(&|x| (x - 3.0).powi(2), 0.0, 6.0, 1e-10, 200);
        assert!(gs.is_done());
        assert!((gs.result() - 3.0).abs() < 1e-6, "result={}", gs.result());
    }

    #[test]
    fn powell_minimizes_2d_bowl() {
        let mut pw = MathPowell::new(2);
        let f = |x: &[f64]| x[0] * x[0] + x[1] * x[1];
        pw.perform(&f, vec![3.0, -4.0], 1e-8, 200);
        assert!(pw.is_done());
        assert!(pw.result()[0].abs() < 1e-5, "x={}", pw.result()[0]);
        assert!(pw.result()[1].abs() < 1e-5, "y={}", pw.result()[1]);
        assert!(pw.value() < 1e-10, "value={}", pw.value());
    }

    #[test]
    fn powell_result_slice_length_matches_n() {
        let mut pw = MathPowell::new(3);
        let f = |x: &[f64]| x[0] * x[0] + x[1] * x[1] + x[2] * x[2];
        pw.perform(&f, vec![1.0, 2.0, 3.0], 1e-8, 100);
        assert_eq!(pw.result().len(), 3);
    }
}
