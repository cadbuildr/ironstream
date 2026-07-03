//! `math_Jacobi` — Jacobi eigenvalue/eigenvector solver for real symmetric
//! square matrices.
//!
//! Faithful reproduction of OpenCascade's `math_Jacobi`
//! (`src/FoundationClasses/TKMath/math/math_Jacobi.{hxx,cxx,lxx}`), whose
//! numerical core lives in `math_Recipes.cxx::Jacobi` /
//! `math_Recipes.cxx::EigenSort`.
//!
//! From the OCCT header:
//!
//! > This class implements the Jacobi method to find the eigenvalues and the
//! > eigenvectors of a real symmetric square matrix. A sort of eigenvalues is
//! > done.
//!
//! The constructor takes a real `n × n` matrix `A`, computes all of its
//! eigenvalues and eigenvectors with the cyclic Jacobi rotations, then sorts
//! the eigenvalues in *descending* order (carrying the eigenvectors along).
//! The exception `math_NotSquare` is raised (here: a panic) if the matrix is
//! not square. No verification that `A` is really symmetric is done.
//!
//! To stay faithful to OCCT's 1-indexed, arbitrarily-bounded `math_Matrix` /
//! `math_Vector`, this module ships small self-contained [`MathMatrix`] and
//! [`MathVector`] types that carry their lower/upper bounds, exactly like the
//! originals (the bounds matter: `math_Jacobi`'s constructor maps the input
//! matrix `A` — which may have arbitrary bounds such as `(2,3,2,3)` — into the
//! always-1-indexed working matrix `AA`). They are prefixed `Math*` to avoid
//! clashing with anything in std or the rest of the kernel.

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

    /// `math_Matrix::Col(Col)` — extracts column `Col` as a vector whose index
    /// range matches this matrix's row range `[lowerRow, upperRow]`.
    pub fn col(&self, col: i32) -> MathVector {
        let mut v = MathVector::new(self.lower_row, self.upper_row());
        for i in self.lower_row..=self.upper_row() {
            v.set(i, self.get(i, col));
        }
        v
    }
}

// ===========================================================================
// EigenSort — descending sort of eigenvalues, carrying eigenvectors along.
// math_Recipes.cxx::EigenSort
// ===========================================================================

/// `EigenSort(math_Vector& d, math_Matrix& v)` — sort the eigenvalues `d` in
/// descending order, permuting the columns of the eigenvector matrix `v` to
/// match. Both are assumed to be 1-indexed `(1..=n)`.
fn eigen_sort(d: &mut MathVector, v: &mut MathMatrix) {
    let n = d.length();

    for i in 1..n {
        let mut k = i;
        let mut p = d.get(i);
        for j in (i + 1)..=n {
            if d.get(j) >= p {
                k = j;
                p = d.get(j);
            }
        }
        if k != i {
            d.set(k, d.get(i));
            d.set(i, p);
            for j in 1..=n {
                let tmp = v.get(j, i);
                v.set(j, i, v.get(j, k));
                v.set(j, k, tmp);
            }
        }
    }
}

/// Jacobi rotation helper, mirroring OCCT's `ROTATE` macro.
///
/// `g = a(i,j); h = a(k,l);`
/// `a(i,j) = g - s*(h + g*tau); a(k,l) = h + s*(g - h*tau);`
#[inline]
fn rotate(a: &mut MathMatrix, i: i32, j: i32, k: i32, l: i32, s: f64, tau: f64) {
    let g = a.get(i, j);
    let h = a.get(k, l);
    a.set(i, j, g - s * (h + g * tau));
    a.set(k, l, h + s * (g - h * tau));
}

/// `Jacobi(math_Matrix& a, math_Vector& d, math_Matrix& v, int& nrot)`.
///
/// Computes the eigenvalues `d` and eigenvectors `v` (as columns) of the real
/// symmetric matrix `a` (destroyed in the process), returning the number of
/// rotations performed in `nrot`. Returns `true` on convergence (OCCT
/// `math_Status_OK`, value 0) and `false` on non-convergence
/// (`math_Status_NoConvergence`). All inputs are 1-indexed `(1..=n)`.
fn jacobi(a: &mut MathMatrix, d: &mut MathVector, v: &mut MathMatrix, nrot: &mut i32) -> bool {
    let n = a.row_number();

    let mut b = MathVector::new(1, n);
    let mut z = MathVector::new(1, n);

    // v = identity
    for ip in 1..=n {
        for iq in 1..=n {
            v.set(ip, iq, 0.0);
        }
        v.set(ip, ip, 1.0);
    }
    // b = d = diag(a); z = 0
    for ip in 1..=n {
        let aii = a.get(ip, ip);
        b.set(ip, aii);
        d.set(ip, aii);
        z.set(ip, 0.0);
    }

    *nrot = 0;
    for i in 1..=50 {
        // sum of off-diagonal magnitudes
        let mut sm = 0.0;
        for ip in 1..n {
            for iq in (ip + 1)..=n {
                sm += a.get(ip, iq).abs();
            }
        }
        if sm == 0.0 {
            eigen_sort(d, v);
            return true; // math_Status_OK
        }

        let tresh = if i < 4 {
            0.2 * sm / ((n * n) as f64)
        } else {
            0.0
        };

        for ip in 1..n {
            for iq in (ip + 1)..=n {
                let g = 100.0 * a.get(ip, iq).abs();
                if i > 4
                    && d.get(ip).abs() + g == d.get(ip).abs()
                    && d.get(iq).abs() + g == d.get(iq).abs()
                {
                    a.set(ip, iq, 0.0);
                } else if a.get(ip, iq).abs() > tresh {
                    let mut h = d.get(iq) - d.get(ip);
                    let t: f64;
                    if h.abs() + g == h.abs() {
                        t = a.get(ip, iq) / h;
                    } else {
                        let theta = 0.5 * h / a.get(ip, iq);
                        let mut tt = 1.0 / (theta.abs() + (1.0 + theta * theta).sqrt());
                        if theta < 0.0 {
                            tt = -tt;
                        }
                        t = tt;
                    }
                    let c = 1.0 / (1.0 + t * t).sqrt();
                    let s = t * c;
                    let tau = s / (1.0 + c);
                    h = t * a.get(ip, iq);
                    z.set(ip, z.get(ip) - h);
                    z.set(iq, z.get(iq) + h);
                    d.set(ip, d.get(ip) - h);
                    d.set(iq, d.get(iq) + h);
                    a.set(ip, iq, 0.0);
                    for j in 1..ip {
                        rotate(a, j, ip, j, iq, s, tau);
                    }
                    for j in (ip + 1)..iq {
                        rotate(a, ip, j, j, iq, s, tau);
                    }
                    for j in (iq + 1)..=n {
                        rotate(a, ip, j, iq, j, s, tau);
                    }
                    for j in 1..=n {
                        rotate(v, j, ip, j, iq, s, tau);
                    }
                    *nrot += 1;
                }
            }
        }

        for ip in 1..=n {
            b.set(ip, b.get(ip) + z.get(ip));
            d.set(ip, b.get(ip));
            z.set(ip, 0.0);
        }
    }

    eigen_sort(d, v);
    false // math_Status_NoConvergence
}

// ===========================================================================
// math_Jacobi
// ===========================================================================

// occt: math_Jacobi
/// Jacobi eigenvalue/eigenvector decomposition of a real symmetric square
/// matrix. Construct from a matrix; query [`is_done`](MathJacobi::is_done),
/// then read [`values`](MathJacobi::values) / [`value`](MathJacobi::value) and
/// [`vectors`](MathJacobi::vectors) / [`vector`](MathJacobi::vector).
///
/// Mirrors OCCT `math_Jacobi`. Eigenvalues are sorted in descending order and
/// the eigenvectors are the columns of [`vectors`](MathJacobi::vectors).
#[derive(Clone, Debug)]
pub struct MathJacobi {
    done: bool,
    nb_rotations: i32,
    eigen_values: MathVector,
    eigen_vectors: MathMatrix,
}

impl MathJacobi {
    /// `math_Jacobi(const math_Matrix& A)`.
    ///
    /// Given a real `n × n` matrix `A`, computes all of its eigenvalues and
    /// eigenvectors using the Jacobi method. No verification that `A` is really
    /// symmetric is done.
    ///
    /// # Panics
    /// Panics (OCCT raises `math_NotSquare`) if `A` is not square.
    pub fn new(a: &MathMatrix) -> Self {
        let n = a.row_number();

        // math_NotSquare_Raise_if(A.RowNumber() != A.ColNumber(), " ");
        assert!(
            n == a.col_number(),
            "math_Jacobi: math_NotSquare — A is not a square matrix"
        );

        // AA(1, n, 1, n); EigenValues(1, n); EigenVectors(1, n, 1, n)
        let mut aa = MathMatrix::new(1, n, 1, n);
        let mut eigen_values = MathVector::new(1, n);
        let mut eigen_vectors = MathMatrix::new(1, n, 1, n);

        // AA = A — copy through A's own bounds into the 1-indexed working copy.
        let lr = a.lower_row();
        let lc = a.lower_col();
        for i in 1..=n {
            for j in 1..=n {
                aa.set(i, j, a.get(i + lr - 1, j + lc - 1));
            }
        }

        let mut nb_rotations = 0;
        let ok = jacobi(&mut aa, &mut eigen_values, &mut eigen_vectors, &mut nb_rotations);

        Self {
            done: ok,
            nb_rotations,
            eigen_values,
            eigen_vectors,
        }
    }

    /// `math_Jacobi::IsDone()` — true if the computation was successful.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Number of Jacobi rotations that were performed.
    pub fn nb_rotations(&self) -> i32 {
        self.nb_rotations
    }

    /// `math_Jacobi::Values()` — the eigenvalues vector (1-indexed, descending).
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if the calculation was not done.
    pub fn values(&self) -> &MathVector {
        assert!(self.done, "math_Jacobi::Values: StdFail_NotDone");
        &self.eigen_values
    }

    /// `math_Jacobi::Value(Num)` — eigenvalue number `Num` (range `1..=n`).
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if the calculation was not done.
    pub fn value(&self, num: i32) -> f64 {
        assert!(self.done, "math_Jacobi::Value: StdFail_NotDone");
        self.eigen_values.get(num)
    }

    /// `math_Jacobi::Vectors()` — the eigenvectors matrix (columns).
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if the calculation was not done.
    pub fn vectors(&self) -> &MathMatrix {
        assert!(self.done, "math_Jacobi::Vectors: StdFail_NotDone");
        &self.eigen_vectors
    }

    /// `math_Jacobi::Vector(Num, math_Vector& V)` — eigenvector number `Num`
    /// (range `1..=n`), written into `v` (`V = EigenVectors.Col(Num)`).
    ///
    /// # Panics
    /// Panics (OCCT raises `StdFail_NotDone`) if the calculation was not done.
    pub fn vector(&self, num: i32, v: &mut MathVector) {
        assert!(self.done, "math_Jacobi::Vector: StdFail_NotDone");
        let col = self.eigen_vectors.col(num);
        let lo = v.lower();
        for i in 1..=col.length() {
            v.set(i + lo - 1, col.get(i + col.lower() - 1));
        }
    }

    /// `math_Jacobi::Dump(Standard_OStream& o)` — state of the object.
    pub fn dump(&self) -> String {
        if self.done {
            let mut s = String::from("math_Jacobi  Status = Done \n");
            s.push_str(" The eigenvalues vector is: ");
            for i in 1..=self.eigen_values.length() {
                if i > 1 {
                    s.push(' ');
                }
                s.push_str(&format!("{}", self.eigen_values.get(i)));
            }
            s.push('\n');
            s
        } else {
            String::from("math_Jacobi Status = not Done \n")
        }
    }
}

impl fmt::Display for MathJacobi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.dump())
    }
}
