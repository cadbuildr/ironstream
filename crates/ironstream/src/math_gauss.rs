//! `math_GaussPoint` / `math_GaussLeastSquare` / `math_GaussMultipleIntegration` stubs.
//!
//! Faithful stub of the OpenCascade classes:
//!   - `math_GaussPoint`             — a single Gauss-Legendre quadrature node with weight.
//!   - `math_GaussSingleIntegration` — 1-D Gauss-Legendre integration (stub via [`GaussIntegrator`]).
//!   - `math_GaussMultipleIntegration` — n-D Gauss-Legendre integration (stub).
//!   - `math_GaussLeastSquare`       — least-squares solver via Gaussian elimination.
//!
//! All types are pure Rust with no external crates; only `std` is used.

// ===========================================================================
// math_GaussPoint
// ===========================================================================

// occt: math_GaussPoint
/// A single Gauss-Legendre quadrature node: an abscissa `value` and its
/// associated integration `weight`.
///
/// Mirrors OCCT `math_GaussPoint`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GaussPoint {
    /// Abscissa (node position on the canonical interval [-1, 1]).
    pub value: f64,
    /// Weight associated with this node.
    pub weight: f64,
}

// ===========================================================================
// gauss_legendre_points
// ===========================================================================

/// Return `n` Gauss-Legendre quadrature points on `[-1, 1]`.
///
/// This is a stub implementation: for `n <= 5` it returns a small set of
/// hard-coded exact (or near-exact) nodes and weights taken from standard
/// tables; for larger `n` the nodes are uniformly spread and the weights are
/// set to `2.0 / n` (the rectangle-rule fallback). Production code should
/// replace the fallback branch with the full OCCT tabulated data or a
/// Newton-iteration solver.
///
/// Mirrors the intent of `math_GaussPoints` from OCCT.
pub fn gauss_legendre_points(n: usize) -> Vec<GaussPoint> {
    match n {
        0 => vec![],
        1 => vec![GaussPoint {
            value: 0.0,
            weight: 2.0,
        }],
        2 => vec![
            GaussPoint {
                value: -0.577_350_269_189_625_8,
                weight: 1.0,
            },
            GaussPoint {
                value: 0.577_350_269_189_625_8,
                weight: 1.0,
            },
        ],
        3 => vec![
            GaussPoint {
                value: -0.774_596_669_241_483_4,
                weight: 0.555_555_555_555_555_6,
            },
            GaussPoint {
                value: 0.0,
                weight: 0.888_888_888_888_888_9,
            },
            GaussPoint {
                value: 0.774_596_669_241_483_4,
                weight: 0.555_555_555_555_555_6,
            },
        ],
        4 => vec![
            GaussPoint {
                value: -0.861_136_311_594_952_7,
                weight: 0.347_854_845_137_453_9,
            },
            GaussPoint {
                value: -0.339_981_043_584_856_3,
                weight: 0.652_145_154_862_546_1,
            },
            GaussPoint {
                value: 0.339_981_043_584_856_3,
                weight: 0.652_145_154_862_546_1,
            },
            GaussPoint {
                value: 0.861_136_311_594_952_7,
                weight: 0.347_854_845_137_453_9,
            },
        ],
        5 => vec![
            GaussPoint {
                value: -0.906_179_845_938_663_9,
                weight: 0.236_926_885_056_189_1,
            },
            GaussPoint {
                value: -0.538_469_310_105_683_1,
                weight: 0.478_628_670_499_366_5,
            },
            GaussPoint {
                value: 0.0,
                weight: 0.568_888_888_888_888_9,
            },
            GaussPoint {
                value: 0.538_469_310_105_683_1,
                weight: 0.478_628_670_499_366_5,
            },
            GaussPoint {
                value: 0.906_179_845_938_663_9,
                weight: 0.236_926_885_056_189_1,
            },
        ],
        // Stub fallback: evenly-spaced nodes, uniform weights (rectangle rule).
        _ => {
            let w = 2.0 / n as f64;
            (0..n)
                .map(|i| {
                    let t = -1.0 + (2 * i + 1) as f64 / n as f64;
                    GaussPoint { value: t, weight: w }
                })
                .collect()
        }
    }
}

// ===========================================================================
// GaussIntegrator  (math_GaussSingleIntegration / math_GaussMultipleIntegration)
// ===========================================================================

// occt: math_GaussSingleIntegration / math_GaussMultipleIntegration
/// 1-D Gauss-Legendre numerical integrator.
///
/// Wraps a fixed number of quadrature points and applies them to integrate
/// an arbitrary scalar closure over a finite interval `[a, b]`.
///
/// Mirrors the intent of OCCT `math_GaussSingleIntegration` (and the 1-D
/// case of `math_GaussMultipleIntegration`).
#[derive(Clone, Debug)]
pub struct GaussIntegrator {
    /// Number of Gauss-Legendre quadrature points.
    pub n_points: usize,
}

impl GaussIntegrator {
    // occt: math_GaussSingleIntegration::math_GaussSingleIntegration
    /// Construct a new integrator that will use `n` quadrature points.
    pub fn new(n: usize) -> Self {
        Self { n_points: n }
    }

    // occt: math_GaussSingleIntegration::Value
    /// Integrate `f` over `[a, b]` using the Gauss-Legendre rule with
    /// [`n_points`](Self::n_points) nodes.
    ///
    /// The canonical interval `[-1, 1]` is mapped to `[a, b]` via the
    /// standard affine substitution:
    ///
    /// ```text
    /// x = (a + b) / 2 + t * (b - a) / 2,   dx = (b - a) / 2  dt
    /// ```
    pub fn integrate<F: Fn(f64) -> f64>(&self, f: F, a: f64, b: f64) -> f64 {
        let pts = gauss_legendre_points(self.n_points);
        let mid = 0.5 * (a + b);
        let half = 0.5 * (b - a);
        pts.iter()
            .map(|p| p.weight * f(mid + p.value * half))
            .sum::<f64>()
            * half
    }
}

// ===========================================================================
// LeastSquares  (math_GaussLeastSquare)
// ===========================================================================

// occt: math_GaussLeastSquare
/// Least-squares solver for an over-determined (or square) linear system
/// `A · x = b` using Gaussian elimination with partial pivoting.
///
/// The matrix `A` is stored row-major in `matrix` (rows × cols entries);
/// `rhs` stores the right-hand side vector of length `rows`.
///
/// Mirrors OCCT `math_GaussLeastSquare`.
#[derive(Clone, Debug)]
pub struct LeastSquares {
    /// Row-major storage of the coefficient matrix (`rows` × `cols` entries).
    pub matrix: Vec<Vec<f64>>,
    /// Right-hand side vector (length `rows`).
    pub rhs: Vec<f64>,
}

impl LeastSquares {
    // occt: math_GaussLeastSquare::math_GaussLeastSquare
    /// Allocate a zero-initialised `rows × cols` system.
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            matrix: vec![vec![0.0; cols]; rows],
            rhs: vec![0.0; rows],
        }
    }

    // occt: math_GaussLeastSquare — coefficient entry
    /// Set entry `(i, j)` of the coefficient matrix (0-based).
    ///
    /// # Panics
    /// Panics if `i` or `j` are out of bounds.
    pub fn set(&mut self, i: usize, j: usize, v: f64) {
        self.matrix[i][j] = v;
    }

    // occt: math_GaussLeastSquare — rhs entry
    /// Set the `i`-th entry of the right-hand side vector (0-based).
    ///
    /// # Panics
    /// Panics if `i` is out of bounds.
    pub fn set_rhs(&mut self, i: usize, v: f64) {
        self.rhs[i] = v;
    }

    // occt: math_GaussLeastSquare::Solve
    /// Solve the system using Gaussian elimination with partial pivoting.
    ///
    /// When the system is over-determined (`rows > cols`) the normal equations
    /// `(Aᵀ A) x = Aᵀ b` are formed first, reducing the problem to a square
    /// `cols × cols` system before elimination. For square systems the
    /// coefficient matrix is used directly.
    ///
    /// Returns the solution vector of length `cols`, or a zero vector if the
    /// matrix is found to be (near-)singular during elimination.
    pub fn solve(&self) -> Vec<f64> {
        let rows = self.matrix.len();
        let cols = if rows > 0 { self.matrix[0].len() } else { 0 };

        if cols == 0 || rows == 0 {
            return vec![];
        }

        if rows >= cols {
            // Form normal equations: AtA (cols×cols) and Atb (cols).
            let mut ata = vec![vec![0.0_f64; cols]; cols];
            let mut atb = vec![0.0_f64; cols];

            for i in 0..cols {
                for j in 0..cols {
                    for k in 0..rows {
                        ata[i][j] += self.matrix[k][i] * self.matrix[k][j];
                    }
                }
                for k in 0..rows {
                    atb[i] += self.matrix[k][i] * self.rhs[k];
                }
            }

            gauss_eliminate(ata, atb)
        } else {
            // Under-determined: solve the square top sub-system (stub behaviour).
            let sub_matrix: Vec<Vec<f64>> = self.matrix[..cols].to_vec();
            let sub_rhs: Vec<f64> = self.rhs[..cols].to_vec();
            gauss_eliminate(sub_matrix, sub_rhs)
        }
    }
}

/// Solve the square system `a · x = b` (sizes `n × n` and `n`) via Gaussian
/// elimination with partial pivoting. Returns the solution vector, or all
/// zeros if a (near-)singular pivot is encountered.
fn gauss_eliminate(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Vec<f64> {
    let n = b.len();
    const MIN_PIVOT: f64 = 1.0e-20;

    // Forward elimination with partial pivoting.
    for col in 0..n {
        // Find the row with the largest absolute value in this column.
        let pivot_row = (col..n)
            .max_by(|&r1, &r2| a[r1][col].abs().partial_cmp(&a[r2][col].abs()).unwrap());

        let pivot_row = match pivot_row {
            Some(r) => r,
            None => return vec![0.0; n],
        };

        // Swap rows.
        if pivot_row != col {
            a.swap(col, pivot_row);
            b.swap(col, pivot_row);
        }

        let pivot = a[col][col];
        if pivot.abs() <= MIN_PIVOT {
            // Singular or near-singular; return zero vector (stub behaviour).
            return vec![0.0; n];
        }

        // Eliminate entries below the pivot.
        for row in (col + 1)..n {
            let factor = a[row][col] / pivot;
            for c in col..n {
                let delta = factor * a[col][c];
                a[row][c] -= delta;
            }
            let delta = factor * b[col];
            b[row] -= delta;
        }
    }

    // Back substitution.
    let mut x = vec![0.0_f64; n];
    for i in (0..n).rev() {
        let mut s = b[i];
        for j in (i + 1)..n {
            s -= a[i][j] * x[j];
        }
        x[i] = s / a[i][i];
    }

    x
}

// ===========================================================================
// dot_product
// ===========================================================================

/// Compute the dot product of two equal-length slices.
///
/// Mirrors the intent of scalar-product helpers used throughout OCCT's
/// `math_*` classes (e.g. `math_Vector::Dot`).
///
/// # Panics
/// Panics if `a.len() != b.len()`.
pub fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(
        a.len(),
        b.len(),
        "dot_product: slices must have the same length"
    );
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gauss_points_count() {
        for n in 0..=5 {
            assert_eq!(gauss_legendre_points(n).len(), n);
        }
        assert_eq!(gauss_legendre_points(10).len(), 10);
    }

    #[test]
    fn gauss_points_weights_sum_to_two() {
        for n in 1..=5 {
            let pts = gauss_legendre_points(n);
            let total: f64 = pts.iter().map(|p| p.weight).sum();
            assert!((total - 2.0).abs() < 1e-14, "n={n}: weight sum = {total}");
        }
    }

    #[test]
    fn integrator_constant_function() {
        // integral of 3 over [0, 2] = 6
        let gi = GaussIntegrator::new(3);
        let result = gi.integrate(|_| 3.0, 0.0, 2.0);
        assert!((result - 6.0).abs() < 1e-12, "result = {result}");
    }

    #[test]
    fn integrator_linear_function() {
        // integral of x over [0, 1] = 0.5
        let gi = GaussIntegrator::new(2);
        let result = gi.integrate(|x| x, 0.0, 1.0);
        assert!((result - 0.5).abs() < 1e-12, "result = {result}");
    }

    #[test]
    fn integrator_quadratic() {
        // integral of x^2 over [0, 1] = 1/3
        let gi = GaussIntegrator::new(3);
        let result = gi.integrate(|x| x * x, 0.0, 1.0);
        assert!((result - 1.0 / 3.0).abs() < 1e-12, "result = {result}");
    }

    #[test]
    fn least_squares_square_system() {
        // 2x = 4  =>  x = 2
        let mut ls = LeastSquares::new(1, 1);
        ls.set(0, 0, 2.0);
        ls.set_rhs(0, 4.0);
        let sol = ls.solve();
        assert!((sol[0] - 2.0).abs() < 1e-12, "sol = {:?}", sol);
    }

    #[test]
    fn least_squares_2x2() {
        // x + y = 3, x - y = 1  =>  x=2, y=1
        let mut ls = LeastSquares::new(2, 2);
        ls.set(0, 0, 1.0);
        ls.set(0, 1, 1.0);
        ls.set(1, 0, 1.0);
        ls.set(1, 1, -1.0);
        ls.set_rhs(0, 3.0);
        ls.set_rhs(1, 1.0);
        let sol = ls.solve();
        assert!((sol[0] - 2.0).abs() < 1e-12, "x = {}", sol[0]);
        assert!((sol[1] - 1.0).abs() < 1e-12, "y = {}", sol[1]);
    }

    #[test]
    fn least_squares_overdetermined() {
        // Over-determined: y = x through (0,0),(1,1),(2,2) => slope = 1
        let mut ls = LeastSquares::new(3, 1);
        ls.set(0, 0, 0.0);
        ls.set(1, 0, 1.0);
        ls.set(2, 0, 2.0);
        ls.set_rhs(0, 0.0);
        ls.set_rhs(1, 1.0);
        ls.set_rhs(2, 2.0);
        let sol = ls.solve();
        assert!((sol[0] - 1.0).abs() < 1e-12, "slope = {}", sol[0]);
    }

    #[test]
    fn dot_product_basic() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        assert!((dot_product(&a, &b) - 32.0).abs() < 1e-12);
    }

    #[test]
    fn dot_product_empty() {
        assert_eq!(dot_product(&[], &[]), 0.0);
    }
}
