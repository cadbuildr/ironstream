//! `math_Crout` — Crout LDLᵀ solver/inverter for symmetric matrices.
//!
//! Faithful reproduction of OpenCascade's `math_Crout`
//! (`src/FoundationClasses/TKMath/math/math_Crout.{hxx,cxx,lxx}`).
//!
//! From the OCCT header:
//!
//! > This class implements the Crout algorithm used to solve a system
//! > `A*X = B` where `A` is a symmetric matrix. It can be used to invert a
//! > symmetric matrix. This algorithm is similar to Gauss but is faster than
//! > Gauss. Only the inferior triangle of `A` and the diagonal can be given.
//!
//! `A` is decomposed as `A = L * D * Lᵀ` where `L` is lower-triangular and `D`
//! is diagonal. If one diagonal element of `D` (a pivot) is `<= MinPivot`, `A`
//! is considered singular and the decomposition is not done.
//!
//! To stay faithful to OCCT's 1-indexed, arbitrarily-bounded `math_Matrix` /
//! `math_Vector`, this module ships small self-contained [`MathMatrix`] and
//! [`MathVector`] types that carry their lower/upper bounds, exactly like the
//! originals (the bounds matter: `Crout`'s constructor and `Solve` map through
//! `LowerRow`/`LowerCol`/`Lower`). They are prefixed `Math*` to avoid clashing
//! with anything in std or the rest of the kernel.

use std::fmt;

// ===========================================================================
// math_Vector — 1-indexed (by default) vector with arbitrary bounds.
// ===========================================================================

// occt: math_Vector
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

// occt: math_Matrix
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
}

// ===========================================================================
// math_Crout
// ===========================================================================

// occt: math_Crout
/// Crout LDLᵀ decomposition of a symmetric matrix, used to solve `A*X = B` and
/// to invert `A`. Construct from a matrix; query [`is_done`](Crout::is_done),
/// then [`solve`](Crout::solve) / [`inverse`](Crout::inverse) /
/// [`determinant`](Crout::determinant).
///
/// Mirrors OCCT `math_Crout`. Only the inferior triangle of the result
/// (`InvA`) is meaningful — it is symmetric.
#[derive(Clone, Debug)]
pub struct Crout {
    /// Inverse of `A`. Only the inferior triangle is filled; 1-indexed
    /// `(1..=n, 1..=n)`, matching OCCT's `InvA`.
    inv_a: MathMatrix,
    done: bool,
    det: f64,
}

impl Crout {
    /// `math_Crout(const math_Matrix& A, const Standard_Real MinPivot = 1.0e-20)`.
    ///
    /// Inverts `A` by the Crout algorithm. The user can give only the inferior
    /// triangle. `A = L * D * Lᵀ`. If `|D(i)| <= MinPivot` the matrix is
    /// considered singular and the decomposition is not done.
    ///
    /// # Panics
    /// Panics (OCCT raises `math_NotSquare`) if `A` is not square.
    pub fn new(a: &MathMatrix) -> Self {
        Self::with_min_pivot(a, 1.0e-20)
    }

    /// `math_Crout(A, MinPivot)`.
    pub fn with_min_pivot(a: &MathMatrix, min_pivot: f64) -> Self {
        let nctl = a.row_number();
        let lowr = a.lower_row();
        let lowc = a.lower_col();

        // math_NotSquare_Raise_if(Nctl != A.ColNumber(), " ");
        assert!(
            nctl == a.col_number(),
            "math_Crout: math_NotSquare — A is not a square matrix"
        );

        // InvA(1, A.RowNumber(), 1, A.ColNumber())
        let mut inv_a = MathMatrix::new(1, nctl, 1, nctl);

        // Working storage, all 1-indexed (1..=Nctl), as in OCCT.
        let mut l = MathMatrix::new(1, nctl, 1, nctl);
        let mut diag = MathVector::new(1, nctl);

        let mut det = 1.0_f64;

        // --- LDLᵀ decomposition --------------------------------------------
        for i in 1..=nctl {
            for j in 1..=(i - 1) {
                let mut scale = 0.0;
                for k in 1..=(j - 1) {
                    scale += l.get(i, k) * l.get(j, k) * diag.get(k);
                }
                let v = (a.get(i + lowr - 1, j + lowc - 1) - scale) / diag.get(j);
                l.set(i, j, v);
            }
            let mut scale = 0.0;
            for k in 1..=(i - 1) {
                scale += l.get(i, k) * l.get(i, k) * diag.get(k);
            }
            let d = a.get(i + lowr - 1, i + lowc - 1) - scale;
            diag.set(i, d);
            det *= d;
            if d.abs() <= min_pivot {
                return Self {
                    inv_a,
                    done: false,
                    det,
                };
            }
            l.set(i, i, 1.0);
        }

        // --- Inverse of L ---------------------------------------------------
        {
            let v = 1.0 / l.get(1, 1);
            l.set(1, 1, v);
        }
        for i in 2..=nctl {
            for k in 1..=(i - 1) {
                let mut scale = 0.0;
                for j in k..=(i - 1) {
                    scale += l.get(i, j) * l.get(j, k);
                }
                let v = -scale / l.get(i, i);
                l.set(i, k, v);
            }
            let v = 1.0 / l.get(i, i);
            l.set(i, i, v);
        }

        // --- Inverse of A (inferior triangle only) --------------------------
        for j in 1..=nctl {
            let mut scale = l.get(j, j) * l.get(j, j) / diag.get(j);
            for k in (j + 1)..=nctl {
                scale += l.get(k, j) * l.get(k, j) / diag.get(k);
            }
            inv_a.set(j, j, scale);
            for i in (j + 1)..=nctl {
                let mut scale = l.get(i, j) * l.get(i, i) / diag.get(i);
                for k in (i + 1)..=nctl {
                    scale += l.get(k, j) * l.get(k, i) / diag.get(k);
                }
                inv_a.set(i, j, scale);
            }
        }

        Self {
            inv_a,
            done: true,
            det,
        }
    }

    /// `math_Crout::IsDone()` — returns true if everything was done correctly.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// `math_Crout::Solve(const math_Vector& B, math_Vector& X)`.
    ///
    /// Given input vector `B`, returns the solution of `A . X = B` in `X`.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if the decomposition was not done.
    /// Panics (OCCT raises `Standard_DimensionError`) if the length of `B` is
    /// not equal to the row range of `A`, or `X` is not the same length as `B`.
    pub fn solve(&self, b: &MathVector, x: &mut MathVector) {
        assert!(self.done, "math_Crout::Solve: StdFail_NotDone");
        assert!(
            b.length() == self.inv_a.row_number() && x.length() == b.length(),
            "math_Crout::Solve: Standard_DimensionError"
        );

        let n = self.inv_a.row_number();
        let lowb = b.lower();
        let lowx = x.lower();

        for i in 1..=n {
            // X(i) = InvA(i,1) * B(1)
            let mut acc = self.inv_a.get(i, 1) * b.get(1 + lowb - 1);
            for j in 2..=i {
                acc += self.inv_a.get(i, j) * b.get(j + lowb - 1);
            }
            for j in (i + 1)..=n {
                // Use the symmetric (upper) entries via the stored lower ones.
                acc += self.inv_a.get(j, i) * b.get(j + lowb - 1);
            }
            x.set(i + lowx - 1, acc);
        }
    }

    /// `math_Crout::Inverse()` — the inverse matrix of `A`. Only the inferior
    /// triangle is meaningful (the matrix is symmetric).
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if the decomposition failed.
    pub fn inverse(&self) -> &MathMatrix {
        assert!(self.done, "math_Crout::Inverse: StdFail_NotDone");
        &self.inv_a
    }

    /// `math_Crout::Invert(math_Matrix& Inv)` — copies the inverse into `Inv`.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if the decomposition failed.
    pub fn invert(&self) -> MathMatrix {
        assert!(self.done, "math_Crout::Invert: StdFail_NotDone");
        self.inv_a.clone()
    }

    /// `math_Crout::Determinant()` — determinant of the decomposed matrix.
    /// Zero is returned when `A` is considered singular.
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if the decomposition failed.
    pub fn determinant(&self) -> f64 {
        assert!(self.done, "math_Crout::Determinant: StdFail_NotDone");
        self.det
    }

    /// `math_Crout::Dump(Standard_OStream& o)` — state of the object.
    pub fn dump(&self) -> String {
        if self.done {
            "math_Crout  Status = Done \n".to_string()
        } else {
            "math_Crout  Status = not Done \n".to_string()
        }
    }
}

impl fmt::Display for Crout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.dump())
    }
}
