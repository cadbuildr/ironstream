//! `math_Gauss` — Gauss LU decomposition (Crout algorithm) with partial
//! pivoting (row interchange) of a square matrix.
//!
//! Faithful reproduction of OpenCascade's `math_Gauss`
//! (`src/FoundationClasses/TKMath/math/math_Gauss.{hxx,cxx}`).
//!
//! From the OCCT header:
//!
//! > This class implements the Gauss LU decomposition (Crout algorithm) with
//! > partial pivoting (rows interchange) of a square matrix and the different
//! > possible derived calculation:
//! > - solution of a set of linear equations.
//! > - inverse of a matrix.
//! > - determinant of a matrix.
//!
//! Given an input `n × n` matrix `A`, the constructor performs its LU
//! decomposition with partial pivoting (interchange of rows). The
//! decomposition is stored internally and may be used to solve `A·X = B`,
//! to invert `A`, or to compute its determinant. If the largest pivot found
//! is less than `MinPivot`, the matrix `A` is considered singular and the
//! decomposition is not done.
//!
//! To stay faithful to OCCT's 1-indexed, arbitrarily-bounded `math_Matrix` /
//! `math_Vector`, this module reuses the small self-contained [`MathMatrix`] /
//! [`MathVector`] types from [`crate::math_crout`] — they carry their
//! lower/upper bounds exactly like the originals, which matters because
//! `Gauss`'s constructor and `Solve` map through `LowerRow`/`LowerCol`/`Lower`.

use std::fmt;

pub use crate::math_crout::{MathMatrix, MathVector};

// ===========================================================================
// math_Gauss
// ===========================================================================

// occt: math_Gauss
/// Gauss LU decomposition (Crout algorithm) with partial pivoting of a square
/// matrix, used to solve `A·X = B`, to invert `A`, and to compute its
/// determinant. Construct from a matrix; query [`is_done`](MathGauss::is_done),
/// then [`solve`](MathGauss::solve) / [`invert`](MathGauss::invert) /
/// [`determinant`](MathGauss::determinant).
///
/// Mirrors OCCT `math_Gauss`.
#[derive(Clone, Debug)]
pub struct MathGauss {
    /// `LU` — the LU decomposition of `A`, 1-indexed `(1..=n, 1..=n)`.
    lu: MathMatrix,
    /// `Index` — the row-permutation record (pivot rows), 1-indexed `(1..=n)`.
    index: Vec<i32>,
    /// `D` — `+1`/`-1`, parity of the number of row interchanges (sign of det).
    d: f64,
    /// `Done` — true if the decomposition succeeded.
    done: bool,
}

impl MathGauss {
    /// `math_Gauss(const math_Matrix& A, const Standard_Real MinPivot = 1.0e-20)`.
    ///
    /// Given an input `n × n` matrix `A`, performs its LU decomposition with
    /// partial pivoting (interchange of rows). If the largest pivot found is
    /// less than `MinPivot`, `A` is considered singular.
    ///
    /// # Panics
    /// Panics (OCCT raises `math_NotSquare`) if `A` is not a square matrix.
    pub fn new(a: &MathMatrix) -> Self {
        Self::with_min_pivot(a, 1.0e-20)
    }

    /// `math_Gauss(A, MinPivot)`.
    ///
    /// # Panics
    /// Panics (OCCT raises `math_NotSquare`) if `A` is not a square matrix.
    pub fn with_min_pivot(a: &MathMatrix, min_pivot: f64) -> Self {
        // math_NotSquare_Raise_if(A.RowNumber() != A.ColNumber(), " ");
        assert!(
            a.row_number() == a.col_number(),
            "math_Gauss: math_NotSquare — A is not a square matrix"
        );

        let n = a.row_number();

        // LU is a private copy of A, but re-based to 1-indexed (1..=n, 1..=n),
        // mirroring OCCT's `LU(A)` then internal use of A.Lower*().
        // We copy the values from A's own bounds into a 1-indexed matrix so the
        // internal algorithm can work uniformly in 1-based indexing.
        let lowr = a.lower_row();
        let lowc = a.lower_col();
        let mut lu = MathMatrix::new(1, n, 1, n);
        for i in 1..=n {
            for j in 1..=n {
                lu.set(i, j, a.get(i + lowr - 1, j + lowc - 1));
            }
        }

        // Index(1..=n)
        let mut index = vec![0_i32; (n + 1) as usize]; // index[1..=n] used.

        // D starts at +1 and flips sign on every row interchange.
        let mut d = 1.0_f64;
        let mut done = true;

        // Scaling information: implicit pivoting (Numerical-Recipes "ludcmp"
        // style), which is what OCCT's math_Gauss performs.
        // vv(i) = 1 / max_j |LU(i,j)|.
        let mut vv = vec![0.0_f64; (n + 1) as usize];
        for i in 1..=n {
            let mut big = 0.0_f64;
            for j in 1..=n {
                let temp = lu.get(i, j).abs();
                if temp > big {
                    big = temp;
                }
            }
            if big <= min_pivot {
                // A row of (near-)zeros: singular.
                done = false;
                // OCCT still records Index for consistency; we leave it.
                return Self {
                    lu,
                    index: index[..].to_vec(),
                    d,
                    done,
                };
            }
            vv[i as usize] = 1.0 / big;
        }

        // Crout's algorithm with partial pivoting.
        for j in 1..=n {
            // Upper triangle (without diagonal): beta_ij for i < j.
            for i in 1..j {
                let mut sum = lu.get(i, j);
                for k in 1..i {
                    sum -= lu.get(i, k) * lu.get(k, j);
                }
                lu.set(i, j, sum);
            }

            // Lower triangle (with diagonal) and search for the pivot.
            let mut big = 0.0_f64;
            let mut imax = j;
            for i in j..=n {
                let mut sum = lu.get(i, j);
                for k in 1..j {
                    sum -= lu.get(i, k) * lu.get(k, j);
                }
                lu.set(i, j, sum);
                let dum = vv[i as usize] * sum.abs();
                if dum >= big {
                    big = dum;
                    imax = i;
                }
            }

            // Interchange rows if necessary.
            if j != imax {
                for k in 1..=n {
                    let dum = lu.get(imax, k);
                    lu.set(imax, k, lu.get(j, k));
                    lu.set(j, k, dum);
                }
                d = -d;
                vv[imax as usize] = vv[j as usize];
            }
            index[j as usize] = imax;

            // Singularity test on the pivot.
            if lu.get(j, j).abs() <= min_pivot {
                done = false;
                return Self {
                    lu,
                    index: index[..].to_vec(),
                    d,
                    done,
                };
            }

            // Divide the column below the diagonal by the pivot.
            if j != n {
                let dum = 1.0 / lu.get(j, j);
                for i in (j + 1)..=n {
                    let v = lu.get(i, j) * dum;
                    lu.set(i, j, v);
                }
            }
        }

        Self {
            lu,
            index: index[..].to_vec(),
            d,
            done,
        }
    }

    /// `math_Gauss::IsDone()` — true if the computations were successful.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// `math_Gauss::Solve(const math_Vector& B, math_Vector& X)`.
    ///
    /// Given the input vector `B`, returns the solution `X` of `A·X = B`.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if the decomposition failed.
    /// Panics (OCCT raises `Standard_DimensionError`) if the length of `B` is
    /// not equal to the number of rows of `A`, or `X` is not the same length.
    pub fn solve(&self, b: &MathVector, x: &mut MathVector) {
        assert!(self.done, "math_Gauss::Solve: StdFail_NotDone");
        let n = self.lu.row_number();
        assert!(
            b.length() == n && x.length() == n,
            "math_Gauss::Solve: Standard_DimensionError"
        );

        // Copy B into X (re-based to 1-indexed work), then solve in place on X.
        let lowx = x.lower();
        let lowb = b.lower();
        for i in 1..=n {
            x.set(i + lowx - 1, b.get(i + lowb - 1));
        }
        self.solve_in_place_1based(x, lowx);
    }

    /// `math_Gauss::Solve(math_Vector& B)` — in-place: `B` is replaced by the
    /// solution `X` of `A·X = B`.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if the decomposition failed.
    /// Panics (OCCT raises `Standard_DimensionError`) if the length of `B` is
    /// not equal to the number of rows of `A`.
    pub fn solve_in_place(&self, b: &mut MathVector) {
        assert!(self.done, "math_Gauss::Solve: StdFail_NotDone");
        let n = self.lu.row_number();
        assert!(
            b.length() == n,
            "math_Gauss::Solve: Standard_DimensionError"
        );
        let lowb = b.lower();
        self.solve_in_place_1based(b, lowb);
    }

    /// Forward/back substitution (Numerical-Recipes "lubksb"), operating on a
    /// vector `v` whose logical index 1..=n maps to physical `low..`.
    fn solve_in_place_1based(&self, v: &mut MathVector, low: i32) {
        let n = self.lu.row_number();

        // Forward substitution (taking the row permutation into account).
        // ii tracks the first non-vanishing element of B.
        let mut ii = 0_i32;
        for i in 1..=n {
            let ip = self.index[i as usize];
            let mut sum = v.get(ip + low - 1);
            v.set(ip + low - 1, v.get(i + low - 1));
            if ii != 0 {
                for k in ii..i {
                    sum -= self.lu.get(i, k) * v.get(k + low - 1);
                }
            } else if sum != 0.0 {
                ii = i;
            }
            v.set(i + low - 1, sum);
        }

        // Back substitution.
        for i in (1..=n).rev() {
            let mut sum = v.get(i + low - 1);
            for k in (i + 1)..=n {
                sum -= self.lu.get(i, k) * v.get(k + low - 1);
            }
            v.set(i + low - 1, sum / self.lu.get(i, i));
        }
    }

    /// `math_Gauss::Determinant()` — determinant of the LU-decomposed matrix.
    /// Zero is returned if the matrix `A` was considered singular.
    pub fn determinant(&self) -> f64 {
        if !self.done {
            return 0.0;
        }
        let n = self.lu.row_number();
        let mut det = self.d;
        for i in 1..=n {
            det *= self.lu.get(i, i);
        }
        det
    }

    /// `math_Gauss::Invert(math_Matrix& Inv)` — returns the inverse of `A`.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if the decomposition failed.
    /// Panics (OCCT raises `Standard_DimensionError`) if `Inv` is not square or
    /// not the same size as `A` — here the returned matrix uses the caller's
    /// bounds.
    pub fn invert_into(&self, inv: &mut MathMatrix) {
        assert!(self.done, "math_Gauss::Invert: StdFail_NotDone");
        let n = self.lu.row_number();
        assert!(
            inv.row_number() == n && inv.col_number() == n,
            "math_Gauss::Invert: Standard_DimensionError"
        );

        let lowr = inv.lower_row();
        let lowc = inv.lower_col();

        // Solve A · X = e_j for each unit column e_j; the resulting X columns
        // assemble the inverse.
        for j in 1..=n {
            let mut col = MathVector::new(1, n);
            col.set(j, 1.0);
            self.solve_in_place_1based(&mut col, 1);
            for i in 1..=n {
                inv.set(i + lowr - 1, j + lowc - 1, col.get(i));
            }
        }
    }

    /// `math_Gauss::Invert` — convenience returning a freshly-allocated,
    /// 1-indexed inverse matrix `(1..=n, 1..=n)`.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if the decomposition failed.
    pub fn invert(&self) -> MathMatrix {
        let n = self.lu.row_number();
        let mut inv = MathMatrix::new(1, n, 1, n);
        self.invert_into(&mut inv);
        inv
    }

    /// `math_Gauss::Dump(Standard_OStream& o)` — state of the object.
    pub fn dump(&self) -> String {
        if self.done {
            "math_Gauss  Status = Done \n".to_string()
        } else {
            "math_Gauss  Status = not Done \n".to_string()
        }
    }
}

impl fmt::Display for MathGauss {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.dump())
    }
}
