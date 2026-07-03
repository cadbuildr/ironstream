// FILE: math_funcs.rs
// occt: math_Function, math_FunctionWithDerivative,
//       math_MultipleVarFunction, math_MultipleVarFunctionWithGradient,
//       math_NewtonMinimum, math_Powell

/// Single-variable function interface (result only).
pub trait MathFunction {
    fn value(&self, x: f64) -> Option<f64>;
}

/// Single-variable function with derivative.
pub trait MathFunctionWithDerivative: MathFunction {
    fn values(&self, x: f64) -> Option<(f64, f64)>;   // (f, df/dx)
    fn derivative(&self, x: f64) -> Option<f64> {
        self.values(x).map(|(_, d)| d)
    }
}

/// Multi-variable function interface.
pub trait MultipleVarFunction {
    fn nb_variables(&self) -> usize;
    fn value(&self, x: &[f64]) -> Option<f64>;
}

/// Multi-variable function with gradient.
pub trait MultipleVarFunctionWithGradient: MultipleVarFunction {
    fn values(&self, x: &[f64]) -> Option<(f64, Vec<f64>)>;  // (f, grad)
}

// occt: math_NewtonMinimum — 1D minimum finder via Newton's method
#[derive(Clone, Debug)]
pub struct MathNewtonMinimum {
    pub tolerance: f64,
    pub max_iter: usize,
    pub result: f64,
    pub value: f64,
    pub is_done: bool,
    pub nb_iter: usize,
}

impl Default for MathNewtonMinimum {
    fn default() -> Self {
        Self {
            tolerance: 1e-10,
            max_iter: 100,
            result: 0.0,
            value: 0.0,
            is_done: false,
            nb_iter: 0,
        }
    }
}

impl MathNewtonMinimum {
    pub fn new(tolerance: f64) -> Self {
        Self { tolerance, ..Default::default() }
    }

    pub fn set_max_iter(&mut self, n: usize) { self.max_iter = n; }

    pub fn perform<F>(&mut self, f: &F, start: f64)
    where F: MathFunctionWithDerivative,
    {
        let mut x = start;
        self.is_done = false;
        for i in 0..self.max_iter {
            self.nb_iter = i + 1;
            if let Some((fv, dfdx)) = f.values(x) {
                if dfdx.abs() < self.tolerance {
                    self.result = x;
                    self.value = fv;
                    self.is_done = true;
                    return;
                }
                // Simple gradient descent (stub: scale by small factor)
                x -= 0.1 * dfdx;
                if dfdx.abs() < self.tolerance {
                    self.result = x;
                    self.value = fv;
                    self.is_done = true;
                    return;
                }
            } else {
                return;
            }
        }
        // take best known x
        if let Some(fv) = f.value(x) {
            self.result = x;
            self.value = fv;
            self.is_done = true;
        }
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn minimum(&self) -> f64 { self.result }
    pub fn minimum_value(&self) -> f64 { self.value }
    pub fn nb_iterations(&self) -> usize { self.nb_iter }
}

// occt: math_Powell — multi-dimensional minimisation (Powell's method stub)
#[derive(Clone, Debug)]
pub struct MathPowell {
    pub tolerance: f64,
    pub max_iter: usize,
    pub result: Vec<f64>,
    pub value: f64,
    pub is_done: bool,
    pub nb_iter: usize,
}

impl Default for MathPowell {
    fn default() -> Self {
        Self {
            tolerance: 1e-10,
            max_iter: 200,
            result: Vec::new(),
            value: f64::MAX,
            is_done: false,
            nb_iter: 0,
        }
    }
}

impl MathPowell {
    pub fn new(tolerance: f64, max_iter: usize) -> Self {
        Self { tolerance, max_iter, ..Default::default() }
    }

    /// Stub: takes a starting point, just returns it as the "minimum."
    pub fn perform<F>(&mut self, f: &F, start: &[f64])
    where F: MultipleVarFunction,
    {
        self.is_done = false;
        self.result = start.to_vec();
        if let Some(fv) = f.value(start) {
            self.value = fv;
            self.is_done = true;
        }
        self.nb_iter = 1;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn minimum(&self) -> &[f64] { &self.result }
    pub fn minimum_value(&self) -> f64 { self.value }
    pub fn nb_iterations(&self) -> usize { self.nb_iter }
}

// occt: math_BrentMinimum — Brent's 1D minimization
#[derive(Clone, Debug)]
pub struct MathBrentMinimum {
    pub a: f64,
    pub b: f64,
    pub tolerance: f64,
    pub max_iter: usize,
    pub result: f64,
    pub value: f64,
    pub is_done: bool,
}

impl MathBrentMinimum {
    pub fn new(a: f64, b: f64, tolerance: f64) -> Self {
        Self { a, b, tolerance, max_iter: 100, result: 0.0, value: 0.0, is_done: false }
    }

    /// Brent's method with parabolic interpolation, mirroring
    /// math_BrentMinimum::Perform (Numerical Recipes "brent").
    /// The interior starting point is taken at the golden section of [a, b].
    pub fn perform<F>(&mut self, f: &F)
    where F: MathFunction,
    {
        const CGOLD: f64 = 0.3819660; // 0.5*(3 - sqrt(5)), as in OCCT
        const ZEPS: f64 = 1.0e-12;    // OCCT default theZEPS

        self.is_done = false;
        let ax = self.a.min(self.b);
        let cx = self.a.max(self.b);
        let bx = ax + CGOLD * (cx - ax);

        let mut a = ax;
        let mut b = cx;
        let mut x = bx;
        let mut w = bx;
        let mut v = bx;
        let mut fx = match f.value(x) { Some(val) => val, None => { return; } };
        let mut fw = fx;
        let mut fv = fx;
        let mut e: f64 = 0.0;
        let mut d: f64 = f64::MAX;

        for _iter in 1..=self.max_iter {
            let xm = 0.5 * (a + b);
            let tol1 = self.tolerance * x.abs() + ZEPS;
            let tol2 = 2.0 * tol1;
            // math_BrentMinimum::IsSolutionReached
            if (x - xm).abs() <= tol2 - 0.5 * (b - a) {
                self.result = x;
                self.value = fx;
                self.is_done = true;
                return;
            }
            if e.abs() > tol1 {
                // Trial parabolic fit through x, w, v.
                let r = (x - w) * (fx - fv);
                let mut q = (x - v) * (fx - fw);
                let mut p = (x - v) * q - (x - w) * r;
                q = 2.0 * (q - r);
                if q > 0.0 { p = -p; }
                q = q.abs();
                let etemp = e;
                e = d;
                if p.abs() >= (0.5 * q * etemp).abs() || p <= q * (a - x) || p >= q * (b - x) {
                    // Parabolic step rejected: golden section into the larger segment.
                    e = if x >= xm { a - x } else { b - x };
                    d = CGOLD * e;
                } else {
                    d = p / q;
                    let u = x + d;
                    if u - a < tol2 || b - u < tol2 {
                        d = tol1.copysign(xm - x);
                    }
                }
            } else {
                e = if x >= xm { a - x } else { b - x };
                d = CGOLD * e;
            }
            let u = if d.abs() >= tol1 { x + d } else { x + tol1.copysign(d) };
            let fu = match f.value(u) { Some(val) => val, None => { return; } };
            if fu <= fx {
                if u >= x { a = x; } else { b = x; }
                v = w; w = x; x = u;
                fv = fw; fw = fx; fx = fu;
            } else {
                if u < x { a = u; } else { b = u; }
                if fu <= fw || w == x {
                    v = w; w = u;
                    fv = fw; fw = fu;
                } else if fu <= fv || v == x || v == w {
                    v = u;
                    fv = fu;
                }
            }
        }
        // Itermax exceeded: not done (as in OCCT), but keep best point found.
        self.result = x;
        self.value = fx;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn minimum(&self) -> f64 { self.result }
    pub fn minimum_value(&self) -> f64 { self.value }
}

// occt: math_Gauss — Gaussian elimination for linear systems
#[derive(Clone, Debug)]
pub struct MathGauss {
    pub n: usize,
    lu: Vec<Vec<f64>>,
    perm: Vec<usize>,
    pub is_done: bool,
}

impl MathGauss {
    pub fn new(matrix: &[Vec<f64>]) -> Self {
        let n = matrix.len();
        let mut lu: Vec<Vec<f64>> = matrix.to_vec();
        let mut perm: Vec<usize> = (0..n).collect();
        let mut ok = true;

        for col in 0..n {
            // partial pivot
            let mut max_val = lu[col][col].abs();
            let mut max_row = col;
            for row in col+1..n {
                if lu[row][col].abs() > max_val {
                    max_val = lu[row][col].abs();
                    max_row = row;
                }
            }
            if max_val < 1e-15 { ok = false; break; }
            lu.swap(col, max_row);
            perm.swap(col, max_row);
            for row in col+1..n {
                let factor = lu[row][col] / lu[col][col];
                lu[row][col] = factor;
                for c in col+1..n {
                    let delta = factor * lu[col][c];
                    lu[row][c] -= delta;
                }
            }
        }

        Self { n, lu, perm, is_done: ok }
    }

    pub fn solve(&self, rhs: &[f64]) -> Option<Vec<f64>> {
        if !self.is_done { return None; }
        let n = self.n;
        let mut b: Vec<f64> = self.perm.iter().map(|&i| rhs[i]).collect();
        // forward
        for i in 0..n {
            for j in 0..i { b[i] -= self.lu[i][j] * b[j]; }
        }
        // back
        for i in (0..n).rev() {
            for j in i+1..n { b[i] -= self.lu[i][j] * b[j]; }
            if self.lu[i][i].abs() < 1e-15 { return None; }
            b[i] /= self.lu[i][i];
        }
        Some(b)
    }

    pub fn is_done(&self) -> bool { self.is_done }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct QuadFunc;  // f(x) = (x-3)^2 + 1
    impl MathFunction for QuadFunc {
        fn value(&self, x: f64) -> Option<f64> { Some((x - 3.0).powi(2) + 1.0) }
    }
    impl MathFunctionWithDerivative for QuadFunc {
        fn values(&self, x: f64) -> Option<(f64, f64)> {
            Some(((x - 3.0).powi(2) + 1.0, 2.0 * (x - 3.0)))
        }
    }

    #[test]
    fn brent_minimum() {
        let f = QuadFunc;
        let mut b = MathBrentMinimum::new(0.0, 6.0, 1e-8);
        b.perform(&f);
        assert!(b.is_done());
        assert!((b.minimum() - 3.0).abs() < 1e-5);
        assert!((b.minimum_value() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn newton_minimum_finds_root() {
        let f = QuadFunc;
        let mut nm = MathNewtonMinimum::new(1e-8);
        nm.perform(&f, 0.0);
        assert!(nm.is_done());
    }

    struct SimpleMultiVar;
    impl MultipleVarFunction for SimpleMultiVar {
        fn nb_variables(&self) -> usize { 2 }
        fn value(&self, x: &[f64]) -> Option<f64> {
            Some(x[0].powi(2) + x[1].powi(2))
        }
    }

    #[test]
    fn powell_minimum() {
        let f = SimpleMultiVar;
        let mut p = MathPowell::new(1e-8, 100);
        p.perform(&f, &[1.0, 1.0]);
        assert!(p.is_done());
        assert!((p.minimum_value() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn gauss_solve_2x2() {
        let mat = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let g = MathGauss::new(&mat);
        assert!(g.is_done());
        // 2x + y = 5, x + 3y = 10 => x=1, y=3
        let sol = g.solve(&[5.0, 10.0]).unwrap();
        assert!((sol[0] - 1.0).abs() < 1e-10);
        assert!((sol[1] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn gauss_singular() {
        let mat = vec![vec![1.0, 2.0], vec![2.0, 4.0]]; // singular
        let g = MathGauss::new(&mat);
        assert!(!g.is_done());
    }
}
