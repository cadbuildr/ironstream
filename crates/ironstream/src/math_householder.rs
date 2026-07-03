//! `math_Householder` — least-squares solver via the Householder method.
//!
//! Faithful reproduction of OpenCascade's `math_Householder`
//! (`src/FoundationClasses/TKMath/math/math_Householder.{hxx,cxx,lxx}`).
//!
//! From the OCCT header:
//!
//! > This class implements the least square solution of a set of linear
//! > equations of m unknowns (n >= m) using the Householder method. It solves
//! > `A.X = B`. This algorithm has more numerical stability than
//! > GaussLeastSquare but is longer. It must be used if the matrix is singular
//! > or nearly singular.
//!
//! The constructor reduces `A` to an upper-triangular form by a sequence of
//! Householder reflections (a QR factorisation), applying the same reflections
//! to each column of `B`, then back-substitutes to obtain the least-square
//! solution of `A.X = B` for every column of `B`. If, while processing a
//! column, the relevant sub-column norm drops to (or below) `EPS`, the
//! resolution cannot be done and `is_done()` returns `false`.
//!
//! To stay faithful to OCCT's 1-indexed, arbitrarily-bounded `math_Matrix` /
//! `math_Vector`, this module ships small self-contained [`MathMatrix`] and
//! [`MathVector`] types that carry their lower/upper bounds, exactly like the
//! originals (the bounds matter: the range constructor and `Perform` map through
//! `LowerRow`/`LowerCol`). They are prefixed `Math*` to avoid clashing with
//! anything in std or the rest of the kernel.

use std::fmt;

// ===========================================================================
// math_Vector — 1-indexed (by default) vector with arbitrary bounds.
// ===========================================================================

/// A dense vector mirroring OCCT's `math_Vector`: it carries an arbitrary
/// inclusive index range `[lower, upper]` and is accessed with those indices.
#[derive(Clone, Debug, PartialEq)]
pub struct MathVector {
    lower: i32,
    data: Vec<f64>,
}

impl MathVector {
    /// `math_Vector(Lower, Upper)` — all elements initialised to `0.0`.
    pub fn new(lower: i32, upper: i32) -> Self {
        assert!(upper >= lower, "math_Vector: upper < lower");
        let len = (upper - lower + 1) as usize;
        Self {
            lower,
            data: vec![0.0; len],
        }
    }

    /// `math_Vector(Lower, Upper, InitialValue)`.
    pub fn with_value(lower: i32, upper: i32, value: f64) -> Self {
        assert!(upper >= lower, "math_Vector: upper < lower");
        let len = (upper - lower + 1) as usize;
        Self {
            lower,
            data: vec![value; len],
        }
    }

    /// `math_Vector::Lower()`.
    pub fn lower(&self) -> i32 {
        self.lower
    }

    /// `math_Vector::Upper()`.
    pub fn upper(&self) -> i32 {
        self.lower + self.data.len() as i32 - 1
    }

    /// `math_Vector::Length()`.
    pub fn length(&self) -> i32 {
        self.data.len() as i32
    }

    /// Read element `i` (using the vector's own index range).
    pub fn get(&self, i: i32) -> f64 {
        self.data[(i - self.lower) as usize]
    }

    /// Write element `i` (using the vector's own index range).
    pub fn set(&mut self, i: i32, value: f64) {
        self.data[(i - self.lower) as usize] = value;
    }
}

// ===========================================================================
// math_Matrix — 1-indexed (by default) dense matrix with arbitrary bounds.
// ===========================================================================

/// A dense matrix mirroring OCCT's `math_Matrix`: it carries arbitrary
/// inclusive row range `[lowerRow, upperRow]` and column range
/// `[lowerCol, upperCol]`, and is accessed with those indices.
#[derive(Clone, Debug, PartialEq)]
pub struct MathMatrix {
    lower_row: i32,
    lower_col: i32,
    rows: i32,
    cols: i32,
    data: Vec<f64>,
}

impl MathMatrix {
    /// `math_Matrix(LowerRow, UpperRow, LowerCol, UpperCol)` — zero-filled.
    pub fn new(lower_row: i32, upper_row: i32, lower_col: i32, upper_col: i32) -> Self {
        assert!(upper_row >= lower_row, "math_Matrix: upperRow < lowerRow");
        assert!(upper_col >= lower_col, "math_Matrix: upperCol < lowerCol");
        let rows = upper_row - lower_row + 1;
        let cols = upper_col - lower_col + 1;
        Self {
            lower_row,
            lower_col,
            rows,
            cols,
            data: vec![0.0; (rows * cols) as usize],
        }
    }

    /// `math_Matrix::LowerRow()`.
    pub fn lower_row(&self) -> i32 {
        self.lower_row
    }

    /// `math_Matrix::LowerCol()`.
    pub fn lower_col(&self) -> i32 {
        self.lower_col
    }

    /// `math_Matrix::UpperRow()`.
    pub fn upper_row(&self) -> i32 {
        self.lower_row + self.rows - 1
    }

    /// `math_Matrix::UpperCol()`.
    pub fn upper_col(&self) -> i32 {
        self.lower_col + self.cols - 1
    }

    /// `math_Matrix::RowNumber()`.
    pub fn row_number(&self) -> i32 {
        self.rows
    }

    /// `math_Matrix::ColNumber()`.
    pub fn col_number(&self) -> i32 {
        self.cols
    }

    #[inline]
    fn index(&self, i: i32, j: i32) -> usize {
        ((i - self.lower_row) * self.cols + (j - self.lower_col)) as usize
    }

    /// Read element `(i, j)` (using the matrix's own index ranges).
    pub fn get(&self, i: i32, j: i32) -> f64 {
        self.data[self.index(i, j)]
    }

    /// Write element `(i, j)` (using the matrix's own index ranges).
    pub fn set(&mut self, i: i32, j: i32, value: f64) {
        let idx = self.index(i, j);
        self.data[idx] = value;
    }

    /// `math_Matrix::SetCol(Col, V)` — set column `col` from a vector whose
    /// length must equal this matrix's row count.
    pub fn set_col(&mut self, col: i32, v: &MathVector) {
        assert!(
            v.length() == self.rows,
            "math_Matrix::SetCol: Standard_DimensionError"
        );
        for i in 0..self.rows {
            let value = v.get(v.lower() + i);
            self.set(self.lower_row + i, col, value);
        }
    }

    /// `math_Matrix::Col(Col)` — return column `col` as a 1-indexed vector.
    pub fn col(&self, col: i32) -> MathVector {
        let mut v = MathVector::new(1, self.rows);
        for i in 0..self.rows {
            let value = self.get(self.lower_row + i, col);
            v.set(1 + i, value);
        }
        v
    }
}

// ===========================================================================
// math_Householder
// ===========================================================================

// occt: math_Householder
/// Least-square solution of `A.X = B` (with `n >= m`, where `A` is `l x n`)
/// using the Householder QR method. Mirrors OCCT `math_Householder`.
///
/// Construct from `A` and `B` (a vector or a matrix of right-hand sides); query
/// [`is_done`](MathHouseholder::is_done), then read solutions with
/// [`value`](MathHouseholder::value) (per column) or
/// [`all_values`](MathHouseholder::all_values).
#[derive(Clone, Debug)]
pub struct MathHouseholder {
    /// `Sol(1..=n, 1..=m)` — column `j` holds the solution for column `j` of B.
    sol: MathMatrix,
    /// `Q(1..=l, 1..=n)` — the reduced (factorised) copy of `A`.
    q: MathMatrix,
    done: bool,
    mylower_arow: i32,
    // `myupper_arow`/`myupper_acol` mirror OCCT's stored members; retained for
    // field parity even though `Perform` only reads the lower bounds.
    #[allow(dead_code)]
    myupper_arow: i32,
    mylower_acol: i32,
    #[allow(dead_code)]
    myupper_acol: i32,
}

impl MathHouseholder {
    /// Default epsilon used by the OCCT constructors (`EPS = 1.0e-20`).
    pub const DEFAULT_EPS: f64 = 1.0e-20;

    /// `math_Householder(const math_Matrix& A, const math_Vector& B, EPS)`.
    ///
    /// Performs the least-square resolution of `A.X = B` for the single
    /// right-hand side `B`.
    ///
    /// # Panics
    /// Panics (OCCT raises `Standard_DimensionError`) if the length of `B`
    /// differs from `A`'s row number, or if `n > l`.
    pub fn from_vector(a: &MathMatrix, b: &MathVector, eps: f64) -> Self {
        // Sol(1, A.ColNumber(), 1, 1); Q(1, A.RowNumber(), 1, A.ColNumber())
        let sol = MathMatrix::new(1, a.col_number(), 1, 1);
        let q = MathMatrix::new(1, a.row_number(), 1, a.col_number());

        let mut h = Self {
            sol,
            q,
            done: false,
            mylower_arow: a.lower_row(),
            myupper_arow: a.upper_row(),
            mylower_acol: a.lower_col(),
            myupper_acol: a.upper_col(),
        };

        // math_Matrix B1(1, B.Length(), 1, 1); B1.SetCol(1, B);
        let mut b1 = MathMatrix::new(1, b.length(), 1, 1);
        b1.set_col(1, b);
        h.perform(a, &b1, eps);
        h
    }

    /// `math_Householder(const math_Matrix& A, const math_Matrix& B, EPS)`.
    ///
    /// Performs the least-square resolution of `A.X = B` for each column of `B`.
    ///
    /// # Panics
    /// Panics (OCCT raises `Standard_DimensionError`) if `B`'s row number
    /// differs from `A`'s row number, or if `n > l`.
    pub fn from_matrix(a: &MathMatrix, b: &MathMatrix, eps: f64) -> Self {
        // Sol(1, A.ColNumber(), 1, B.ColNumber());
        // Q(1, A.RowNumber(), A.LowerCol(), A.UpperCol())
        let sol = MathMatrix::new(1, a.col_number(), 1, b.col_number());
        let q = MathMatrix::new(1, a.row_number(), a.lower_col(), a.upper_col());

        let mut h = Self {
            sol,
            q,
            done: false,
            mylower_arow: a.lower_row(),
            myupper_arow: a.upper_row(),
            mylower_acol: a.lower_col(),
            myupper_acol: a.upper_col(),
        };
        h.perform(a, b, eps);
        h
    }

    /// `math_Householder(A, B, lowerArow, upperArow, lowerAcol, upperAcol, EPS)`.
    ///
    /// As [`from_matrix`](MathHouseholder::from_matrix), but only the
    /// sub-block of `A` delimited by the given inclusive row/column bounds is
    /// used.
    #[allow(clippy::too_many_arguments)]
    pub fn from_matrix_range(
        a: &MathMatrix,
        b: &MathMatrix,
        lower_arow: i32,
        upper_arow: i32,
        lower_acol: i32,
        upper_acol: i32,
        eps: f64,
    ) -> Self {
        // Sol(1, upperAcol - lowerAcol + 1, 1, B.ColNumber());
        // Q(1, upperArow - lowerArow + 1, 1, upperAcol - lowerAcol + 1)
        let sol = MathMatrix::new(1, upper_acol - lower_acol + 1, 1, b.col_number());
        let q = MathMatrix::new(1, upper_arow - lower_arow + 1, 1, upper_acol - lower_acol + 1);

        let mut h = Self {
            sol,
            q,
            done: false,
            mylower_arow: lower_arow,
            myupper_arow: upper_arow,
            mylower_acol: lower_acol,
            myupper_acol: upper_acol,
        };
        h.perform(a, b, eps);
        h
    }

    /// `void math_Householder::Perform(const math_Matrix& A, const math_Matrix& B, EPS)`.
    ///
    /// Internal driver used by every constructor; faithful line-by-line port of
    /// the OCCT `Perform`.
    fn perform(&mut self, a: &MathMatrix, b: &MathMatrix, eps: f64) {
        let n = self.q.col_number();
        let l = self.q.row_number();
        let m = b.col_number();

        // math_Matrix B2(1, l, 1, m);
        let mut b2 = MathMatrix::new(1, l, 1, m);

        let lbrow = b.lower_row();
        for i in 1..=l {
            for j in 1..=n {
                let v = a.get(i + self.mylower_arow - 1, j + self.mylower_acol - 1);
                self.q.set(i, j, v);
            }
            for j in 1..=m {
                let v = b.get(i + lbrow - 1, j);
                b2.set(i, j, v);
            }
        }

        // Standard_DimensionError_Raise_if(l != B.RowNumber() || n > l, " ");
        assert!(
            l == b.row_number() && n <= l,
            "math_Householder: Standard_DimensionError"
        );

        // Traitement de chaque colonne de A:
        for i in 1..=n {
            let mut h = 0.0_f64;
            for k in i..=l {
                let qki = self.q.get(k, i);
                h += qki * qki; // = ||a||*||a||
            }
            let f = self.q.get(i, i); // = a1
            let g = if f < 1.0e-15 { h.sqrt() } else { -h.sqrt() };
            if g.abs() <= eps {
                self.done = false;
                return;
            }
            h -= f * g; // = (v*v)/2
            let alfaii = g - f; // = v
            for j in (i + 1)..=n {
                let mut scale = 0.0_f64;
                for k in i..=l {
                    scale += self.q.get(k, i) * self.q.get(k, j);
                }
                let cj = (g * self.q.get(i, j) - scale) / h;
                let v = self.q.get(i, j) - alfaii * cj;
                self.q.set(i, j, v);
                for k in (i + 1)..=l {
                    let v = self.q.get(k, j) + cj * self.q.get(k, i);
                    self.q.set(k, j, v);
                }
            }

            // Modification de B:
            for j in 1..=m {
                let mut scale = self.q.get(i, i) * b2.get(i, j);
                for k in (i + 1)..=l {
                    scale += self.q.get(k, i) * b2.get(k, j);
                }
                let cj = (g * b2.get(i, j) - scale) / h;
                let v = b2.get(i, j) - cj * alfaii;
                b2.set(i, j, v);
                for k in (i + 1)..=l {
                    let v = b2.get(k, j) + cj * self.q.get(k, i);
                    b2.set(k, j, v);
                }
            }
            self.q.set(i, i, g);
        }

        // Remontee (back-substitution):
        for j in 1..=m {
            let v = b2.get(n, j) / self.q.get(n, n);
            self.sol.set(n, j, v);
            let mut i = n - 1;
            while i >= 1 {
                let mut scale = 0.0_f64;
                for k in (i + 1)..=n {
                    scale += self.q.get(i, k) * self.sol.get(k, j);
                }
                let v = (b2.get(i, j) - scale) / self.q.get(i, i);
                self.sol.set(i, j, v);
                i -= 1;
            }
        }
        self.done = true;
    }

    /// `math_Householder::IsDone()` — true if the computations succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// `math_Householder::Value(math_Vector& sol, const int Index = 1)`.
    ///
    /// Returns the least-square solution corresponding to column `index` of `B`.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if the resolution was not done.
    /// Panics (OCCT raises `Standard_OutOfRange`) if `index <= 0` or `index` is
    /// greater than the number of columns of `B`.
    pub fn value(&self, index: i32) -> MathVector {
        assert!(self.done, "math_Householder::Value: StdFail_NotDone");
        assert!(
            index >= 1 && index <= self.sol.col_number(),
            "math_Householder::Value: Standard_OutOfRange"
        );
        // sol = Sol.Col(Index);
        self.sol.col(index)
    }

    /// `math_Householder::Value(math_Vector& sol, Index)` writing into `sol`.
    ///
    /// Convenience variant matching OCCT's signature where the caller supplies a
    /// `math_Vector` to be overwritten.
    ///
    /// # Panics
    /// Same panics as [`value`](MathHouseholder::value), plus a
    /// `Standard_DimensionError` if `sol`'s length differs from the solution
    /// length.
    pub fn value_into(&self, sol: &mut MathVector, index: i32) {
        let v = self.value(index);
        assert!(
            sol.length() == v.length(),
            "math_Householder::Value: Standard_DimensionError"
        );
        for i in 0..v.length() {
            sol.set(sol.lower() + i, v.get(1 + i));
        }
    }

    /// `math_Householder::AllValues()` — matrix of all solutions of `A.X = B`.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if the resolution was not done.
    pub fn all_values(&self) -> &MathMatrix {
        assert!(self.done, "math_Householder::AllValues: StdFail_NotDone");
        &self.sol
    }

    /// `math_Householder::Dump(Standard_OStream& o)` — state of the object.
    pub fn dump(&self) -> String {
        if self.done {
            "math_Householder  Status = Done \n".to_string()
        } else {
            "math_Householder Status = not Done \n".to_string()
        }
    }
}

impl fmt::Display for MathHouseholder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.dump())
    }
}
