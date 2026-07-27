//! `math_GaussLeastSquare` -- least-squares solver via the normal-equations / Gauss LU method.
//!
//! Faithful reproduction of OpenCascade's `math_GaussLeastSquare`
//! (`src/FoundationClasses/TKMath/math/math_GaussLeastSquare.{hxx,cxx,lxx}`).
//!
//! From the OCCT header:
//!
//! > This class implements the least square solution of a set of linear
//! > equations of m unknowns (n >= m) using the Gauss LU decomposition. It
//! > solves `A.X = B`. This algorithm is faster than Householder but less
//! > numerically stable. It should only be used when the matrix is not
//! > singular or nearly singular.
//!
//! The algorithm forms the normal equations `(Aᵀ·A)·X = Aᵀ·B` and then
//! solves the resulting `n × n` system with Gauss LU (partial pivoting).
//! Internally the transpose `Aᵀ` is stored (as `MyTAB` in OCCT) so that
//! `Solve` can form `Aᵀ·B` without re-visiting `A`.
//!
//! `MathMatrix` and `MathVector` are re-exported from [`crate::math_gauss`]
//! (which itself re-exports from [`crate::math_crout`]).

use std::fmt;

pub use crate::math_gauss_lu::{MathGauss, MathMatrix, MathVector};

// ===========================================================================
// math_GaussLeastSquare
// ===========================================================================

// occt-ref: math_GaussLeastSquare
/// Least-squares solution of `A·X = B` (with `l >= n`, where `A` is `l × n`)
/// via the normal equations `(Aᵀ·A)·X = Aᵀ·B` solved with Gauss LU.
///
/// This is faster than the Householder approach but less numerically stable;
/// use it only when `A` is not singular or nearly singular.
///
/// Construct with [`new`](MathGaussLeastSquare::new) or
/// [`with_min_pivot`](MathGaussLeastSquare::with_min_pivot); query
/// [`is_done`](MathGaussLeastSquare::is_done), then call
/// [`solve`](MathGaussLeastSquare::solve).
///
/// Mirrors OCCT `math_GaussLeastSquare`.
#[derive(Clone, Debug)]
pub struct MathGaussLeastSquare {
    /// The Gauss LU decomposition of the normal-equations matrix `Aᵀ·A`.
    gauss: MathGauss,
    /// `MyTAB` — the transpose of `A`, 1-indexed `(1..=n, 1..=l)`.
    /// Stored so that `Solve` can form `Aᵀ·B` without the original `A`.
    tab: MathMatrix,
    /// `Done` — true when the constructor succeeded.
    done: bool,
}

impl MathGaussLeastSquare {
    /// `math_GaussLeastSquare(const math_Matrix& A, const Standard_Real MinPivot = 1.0e-20)`.
    ///
    /// Constructs the least-squares solver from the matrix `A` (which must have
    /// at least as many rows as columns). If the normal-equations matrix is
    /// singular (a pivot ≤ `MinPivot`), `is_done()` returns `false`.
    ///
    /// # Panics
    /// Panics (OCCT raises `Standard_DimensionError`) if `A` has more columns
    /// than rows.
    pub fn new(a: &MathMatrix) -> Self {
        Self::with_min_pivot(a, 1.0e-20)
    }

    /// `math_GaussLeastSquare(const math_Matrix& A, const Standard_Real MinPivot)`.
    ///
    /// # Panics
    /// Panics (OCCT raises `Standard_DimensionError`) if `A` has more columns
    /// than rows.
    pub fn with_min_pivot(a: &MathMatrix, min_pivot: f64) -> Self {
        // Standard_DimensionError_Raise_if(A.RowNumber() < A.ColNumber(), " ");
        assert!(
            a.row_number() >= a.col_number(),
            "math_GaussLeastSquare: Standard_DimensionError — A must have RowNumber >= ColNumber"
        );

        let l = a.row_number(); // number of equations
        let n = a.col_number(); // number of unknowns
        let lowr = a.lower_row();
        let lowc = a.lower_col();

        // Store Aᵀ in a 1-indexed (1..=n, 1..=l) matrix (OCCT's `MyTAB`).
        let mut tab = MathMatrix::new(1, n, 1, l);
        for i in 1..=n {
            for k in 1..=l {
                tab.set(i, k, a.get(k + lowr - 1, i + lowc - 1));
            }
        }

        // Build AtA (n × n) = Aᵀ · A.
        // AtA(i,j) = sum_{k=1}^{l} tab(i,k) * tab(j,k)
        let mut ata = MathMatrix::new(1, n, 1, n);
        for i in 1..=n {
            for j in 1..=n {
                let mut sum = 0.0_f64;
                for k in 1..=l {
                    sum += tab.get(i, k) * tab.get(j, k);
                }
                ata.set(i, j, sum);
            }
        }

        // Gauss LU decomposition of AtA.
        let gauss = MathGauss::with_min_pivot(&ata, min_pivot);
        let done = gauss.is_done();

        Self { gauss, tab, done }
    }

    /// `math_GaussLeastSquare::IsDone()` — true if the computations were successful.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// `math_GaussLeastSquare::Solve(const math_Vector& B, math_Vector& X)`.
    ///
    /// Given the right-hand side vector `B` (length = number of rows of `A`),
    /// computes the least-squares solution `X` (length = number of columns of `A`).
    ///
    /// Internally this forms `Aᵀ·B` (using the stored transpose) and solves
    /// `(Aᵀ·A)·X = Aᵀ·B`.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if the decomposition failed.
    /// Panics (OCCT raises `Standard_DimensionError`) if `B` or `X` have
    /// incorrect lengths.
    pub fn solve(&self, b: &MathVector, x: &mut MathVector) {
        assert!(self.done, "math_GaussLeastSquare::Solve: StdFail_NotDone");
        let n = self.tab.row_number(); // number of unknowns
        let l = self.tab.col_number(); // number of equations
        assert!(
            b.length() == l,
            "math_GaussLeastSquare::Solve: Standard_DimensionError (B length {} != l {})",
            b.length(),
            l
        );
        assert!(
            x.length() == n,
            "math_GaussLeastSquare::Solve: Standard_DimensionError (X length {} != n {})",
            x.length(),
            n
        );

        let lowb = b.lower();

        // Form atb = Aᵀ · B  (length n, 1-indexed).
        let mut atb = MathVector::new(1, n);
        for i in 1..=n {
            let mut sum = 0.0_f64;
            for k in 1..=l {
                sum += self.tab.get(i, k) * b.get(k + lowb - 1);
            }
            atb.set(i, sum);
        }

        // Solve (Aᵀ·A)·X = Aᵀ·B using the stored LU decomposition.
        let mut sol = MathVector::new(1, n);
        self.gauss.solve(&atb, &mut sol);

        // Copy result into X using X's own index range.
        let lowx = x.lower();
        for i in 1..=n {
            x.set(i + lowx - 1, sol.get(i));
        }
    }

    /// Returns the number of unknowns (columns of `A`).
    pub fn unknowns(&self) -> i32 {
        self.tab.row_number()
    }

    /// Returns the number of equations (rows of `A`).
    pub fn equations(&self) -> i32 {
        self.tab.col_number()
    }

    /// `math_GaussLeastSquare::Dump(Standard_OStream& o)` — state of the object.
    pub fn dump(&self) -> String {
        if self.done {
            "math_GaussLeastSquare  Status = Done \n".to_string()
        } else {
            "math_GaussLeastSquare  Status = not Done \n".to_string()
        }
    }
}

impl fmt::Display for MathGaussLeastSquare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.dump())
    }
}
