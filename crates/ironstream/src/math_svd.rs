//! `math_SVD` — Singular Value Decomposition (Golub-Reinsch algorithm).
//!
//! Faithful reproduction of OpenCascade's `math_SVD`
//! (`src/FoundationClasses/TKMath/math/math_SVD.{hxx,cxx}`).
//!
//! From the OCCT documentation:
//!
//! > This class implements the Singular Value Decomposition of a matrix and
//! > the different possible derived calculations:
//! > - Solution of a set of linear equations.
//! > - Pseudo inverse of a matrix.
//!
//! Given an `m × n` matrix `A`, the constructor computes the SVD
//! `A = U · W · Vᵀ` where:
//! - `U` is an `m × n` column-orthogonal matrix,
//! - `W` is an `n × n` diagonal matrix of singular values,
//! - `V` is an `n × n` orthogonal matrix.
//!
//! The algorithm is the Golub-Reinsch bidiagonalisation followed by QR
//! iteration (`math_Recipes.cxx::SVDecompose`).
//!
//! To stay faithful to OCCT's 1-indexed, arbitrarily-bounded `math_Matrix` /
//! `math_Vector`, this module re-uses the self-contained types from
//! [`crate::math_crout`].

use std::fmt;

pub use crate::math_crout::{MathMatrix, MathVector};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// `SIGN(a, b)` — returns `|a|` with the sign of `b`.
#[inline]
fn sign(a: f64, b: f64) -> f64 {
    if b >= 0.0 { a.abs() } else { -a.abs() }
}

/// Numerically stable `sqrt(a² + b²)`.
#[inline]
fn pythag(a: f64, b: f64) -> f64 {
    let absa = a.abs();
    let absb = b.abs();
    if absa > absb {
        absa * (1.0 + (absb / absa) * (absb / absa)).sqrt()
    } else if absb == 0.0 {
        0.0
    } else {
        absb * (1.0 + (absa / absb) * (absa / absb)).sqrt()
    }
}

// ---------------------------------------------------------------------------
// Golub-Reinsch SVD
//
// Exact port of Numerical Recipes in C §2.6 `svdcmp` (1-indexed).
// Variable layout matches the C source exactly to avoid subtle off-by-one
// errors in the V-accumulation `g`/`l` carry-over pattern.
// ---------------------------------------------------------------------------

/// Decomposes the `m × n` matrix `a_in` (1-indexed, `m ≥ n`) into
/// `U · diag(w) · Vᵀ`.
///
/// Returns `Some((U, w, V))` on convergence, `None` otherwise.
/// `U` is `m × n`, `w` length `n`, `V` is `n × n` — all 1-indexed.
fn svd(a_in: &MathMatrix) -> Option<(MathMatrix, MathVector, MathMatrix)> {
    let m = a_in.row_number();
    let n = a_in.col_number();

    // Copy A into a 1-indexed m×n working buffer.
    let lr = a_in.lower_row();
    let lc = a_in.lower_col();
    let mut a = MathMatrix::new(1, m, 1, n);
    for i in 1..=m {
        for j in 1..=n {
            a.set(i, j, a_in.get(i + lr - 1, j + lc - 1));
        }
    }

    let mut w = MathVector::new(1, n);
    let mut v = MathMatrix::new(1, n, 1, n);
    let mut rv1 = MathVector::new(1, n);

    // Scalar workspace variables — deliberately named to match NR C source.
    let mut g = 0.0_f64;
    let mut scale = 0.0_f64;
    let mut anorm = 0.0_f64;
    let mut s;
    let mut f;
    let mut h;

    // ---- Householder reduction to bidiagonal form --------------------------
    for i in 1..=n {
        let l = i + 1;
        rv1.set(i, scale * g);
        g = 0.0;
        s = 0.0;
        scale = 0.0;
        if i <= m {
            for k in i..=m {
                scale += a.get(k, i).abs();
            }
            if scale != 0.0 {
                for k in i..=m {
                    let t = a.get(k, i) / scale;
                    a.set(k, i, t);
                    s += t * t;
                }
                f = a.get(i, i);
                g = -sign(s.sqrt(), f);
                h = f * g - s;
                a.set(i, i, f - g);
                for j in l..=n {
                    s = 0.0;
                    for k in i..=m {
                        s += a.get(k, i) * a.get(k, j);
                    }
                    f = s / h;
                    for k in i..=m {
                        let t = a.get(k, j) + f * a.get(k, i);
                        a.set(k, j, t);
                    }
                }
                for k in i..=m {
                    let t = a.get(k, i) * scale;
                    a.set(k, i, t);
                }
            }
        }
        w.set(i, scale * g);
        // Right-hand Householder
        g = 0.0;
        s = 0.0;
        scale = 0.0;
        if i <= m && i != n {
            for k in l..=n {
                scale += a.get(i, k).abs();
            }
            if scale != 0.0 {
                for k in l..=n {
                    let t = a.get(i, k) / scale;
                    a.set(i, k, t);
                    s += t * t;
                }
                f = a.get(i, l);
                g = -sign(s.sqrt(), f);
                h = f * g - s;
                a.set(i, l, f - g);
                for k in l..=n {
                    rv1.set(k, a.get(i, k) / h);
                }
                for j in l..=m {
                    s = 0.0;
                    for k in l..=n {
                        s += a.get(j, k) * a.get(i, k);
                    }
                    for k in l..=n {
                        let t = a.get(j, k) + s * rv1.get(k);
                        a.set(j, k, t);
                    }
                }
                for k in l..=n {
                    let t = a.get(i, k) * scale;
                    a.set(i, k, t);
                }
            }
        }
        let tmp = w.get(i).abs() + rv1.get(i).abs();
        if tmp > anorm {
            anorm = tmp;
        }
    }

    // ---- Accumulate right-hand transformations into V ----------------------
    // CRITICAL: The NR C code carries `g` and `l` forward from the *previous*
    // iteration.  At the start of iteration i, `g` holds `rv1[i+1]` (the
    // value written at the end of the previous iteration for i+1) and `l` is
    // `i+1`.  We replicate this with an explicit carry variable.
    {
        let mut l_carry = n; // l at the end of the first (i=n) iteration
        // After the i=n pass we set: g = rv1[n]; l = n.
        // But first iteration (i=n) uses the g/l from the bidiagonalisation.
        // Per NR: the loop body reads g/l from the END of the previous
        // iteration; for i=n (first in the reverse loop) the `if (i < n)`
        // branch is skipped, so g doesn't matter there.
        // We therefore just initialise g/l to match what NR would have at
        // the start of the i=n iteration (where the branch is not taken).
        let mut g_carry = 0.0_f64; // unused for i=n since branch not taken
        for i in (1..=n).rev() {
            let l = l_carry;
            let g = g_carry;
            if i < n {
                if g != 0.0 {
                    for j in l..=n {
                        // double division avoids underflow
                        let t = (a.get(i, j) / a.get(i, l)) / g;
                        v.set(j, i, t);
                    }
                    for j in l..=n {
                        s = 0.0;
                        for k in l..=n {
                            s += a.get(i, k) * v.get(k, j);
                        }
                        for k in l..=n {
                            let t = v.get(k, j) + s * v.get(k, i);
                            v.set(k, j, t);
                        }
                    }
                }
                for j in l..=n {
                    v.set(i, j, 0.0);
                    v.set(j, i, 0.0);
                }
            }
            v.set(i, i, 1.0);
            // Update carry for next (lower) i
            g_carry = rv1.get(i);
            l_carry = i;
        }
    }

    // ---- Accumulate left-hand transformations (U in a) ---------------------
    {
        let limit = if m < n { m } else { n };
        for i in (1..=limit).rev() {
            let l = i + 1;
            g = w.get(i);
            for j in l..=n {
                a.set(i, j, 0.0);
            }
            if g != 0.0 {
                g = 1.0 / g;
                for j in l..=n {
                    s = 0.0;
                    for k in l..=m {
                        s += a.get(k, i) * a.get(k, j);
                    }
                    f = (s / a.get(i, i)) * g;
                    for k in i..=m {
                        let t = a.get(k, j) + f * a.get(k, i);
                        a.set(k, j, t);
                    }
                }
                for j in i..=m {
                    let t = a.get(j, i) * g;
                    a.set(j, i, t);
                }
            } else {
                for j in i..=m {
                    a.set(j, i, 0.0);
                }
            }
            a.set(i, i, a.get(i, i) + 1.0);
        }
    }

    // ---- Diagonalise the bidiagonal form via QR iteration ------------------
    for k in (1..=n).rev() {
        let mut converged = false;
        for _its in 1..=30 {
            // Find L such that the superdiagonal rv1[l] or subdiagonal w[nm]
            // is negligible.
            let mut flag = 1_i32;
            let mut l = k;
            // Find the first l such that rv1[l] ≈ 0 or w[l-1] ≈ 0.
            // rv1[1] is always 0 from the bidiagonalisation initialisation.
            loop {
                if rv1.get(l).abs() + anorm == anorm {
                    flag = 0;
                    break;
                }
                if l <= 1 {
                    // l-1 = 0 would be out of range; rv1[1]=0 guarantees split
                    flag = 0;
                    break;
                }
                let nm_test = l - 1;
                if w.get(nm_test).abs() + anorm == anorm {
                    break;
                }
                l -= 1;
            }
            let mut nm = l - 1;
            if flag != 0 {
                // Cancellation of rv1[l] if l > 1
                let mut c = 0.0_f64;
                s = 1.0;
                for i in l..=k {
                    f = s * rv1.get(i);
                    rv1.set(i, c * rv1.get(i));
                    if f.abs() + anorm == anorm {
                        break;
                    }
                    g = w.get(i);
                    h = pythag(f, g);
                    w.set(i, h);
                    h = 1.0 / h;
                    c = g * h;
                    s = -f * h;
                    for j in 1..=m {
                        let y = a.get(j, nm);
                        let z = a.get(j, i);
                        a.set(j, nm, y * c + z * s);
                        a.set(j, i, -y * s + z * c);
                    }
                }
            }
            let z = w.get(k);
            if l == k {
                // Singular value converged — make non-negative.
                if z < 0.0 {
                    w.set(k, -z);
                    for j in 1..=n {
                        let t = -v.get(j, k);
                        v.set(j, k, t);
                    }
                }
                converged = true;
                break;
            }
            // QR shift from the bottom 2×2 minor.
            let x = w.get(l);
            nm = k - 1;
            let y = w.get(nm);
            g = rv1.get(nm);
            h = rv1.get(k);
            f = ((y - z) * (y + z) + (g - h) * (g + h)) / (2.0 * h * y);
            g = pythag(f, 1.0);
            f = ((x - z) * (x + z) + h * ((y / (f + sign(g, f))) - h)) / x;
            // Chase the bulge from j=l to j=nm.
            let mut c = 1.0_f64;
            let mut ss = 1.0_f64; // NR names this `s` but we have `s` already
            let mut x_cur = x;
            for j in l..=nm {
                let i = j + 1;
                g = rv1.get(i);
                let y2 = w.get(i);
                h = ss * g;
                g *= c;
                let z2 = pythag(f, h);
                rv1.set(j, z2);
                c = f / z2;
                ss = h / z2;
                f = x_cur * c + g * ss;
                g = -x_cur * ss + g * c;
                h = y2 * ss;
                let x_next = y2 * c;
                for jj in 1..=n {
                    let p = v.get(jj, j);
                    let q = v.get(jj, i);
                    v.set(jj, j, p * c + q * ss);
                    v.set(jj, i, -p * ss + q * c);
                }
                let z2 = pythag(f, h);
                w.set(j, z2);
                if z2 != 0.0 {
                    let z2_inv = 1.0 / z2;
                    c = f * z2_inv;
                    ss = h * z2_inv;
                }
                f = c * g + ss * x_next;
                x_cur = -ss * g + c * x_next;
                for jj in 1..=m {
                    let y2 = a.get(jj, j);
                    let z2 = a.get(jj, i);
                    a.set(jj, j, y2 * c + z2 * ss);
                    a.set(jj, i, -y2 * ss + z2 * c);
                }
            }
            rv1.set(l, 0.0);
            rv1.set(k, f);
            w.set(k, x_cur);
        }
        if !converged {
            return None;
        }
    }

    // `a` now holds U.
    Some((a, w, v))
}

// ===========================================================================
// math_SVD
// ===========================================================================

// occt: math_SVD
/// Singular Value Decomposition of an `m × n` matrix `A` (`m ≥ n`).
///
/// Computes `A = U · diag(W) · Vᵀ` using the Golub-Reinsch algorithm.
/// Query [`is_done`](MathSVD::is_done), then use [`solve`](MathSVD::solve) or
/// [`pseudo_inverse`](MathSVD::pseudo_inverse).
///
/// Mirrors OCCT `math_SVD`.
#[derive(Clone, Debug)]
pub struct MathSVD {
    /// `U` — `m × n` column-orthogonal matrix (1-indexed).
    u: MathMatrix,
    /// `W` — singular values vector (1-indexed, length `n`).
    w: MathVector,
    /// `V` — `n × n` orthogonal matrix (1-indexed).
    v: MathMatrix,
    /// `Done` — true if the SVD converged.
    done: bool,
}

impl MathSVD {
    /// `math_SVD(const math_Matrix& A)`.
    ///
    /// Constructs the SVD of the `m × n` matrix `A` (requires `m ≥ n`).
    /// If the algorithm does not converge within 30 QR iterations per singular
    /// value, [`is_done`](MathSVD::is_done) returns `false`.
    ///
    /// # Panics
    /// Panics (`Standard_DimensionError`) if `m < n`.
    pub fn new(a: &MathMatrix) -> Self {
        let m = a.row_number();
        let n = a.col_number();
        assert!(
            m >= n,
            "math_SVD: Standard_DimensionError — m ({m}) < n ({n})"
        );
        match svd(a) {
            Some((u, w, v)) => Self { u, w, v, done: true },
            None => Self {
                u: MathMatrix::new(1, m, 1, n),
                w: MathVector::new(1, n),
                v: MathMatrix::new(1, n, 1, n),
                done: false,
            },
        }
    }

    /// `math_SVD::IsDone()` — true if the SVD succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// `math_SVD::Solve(const math_Vector& B, math_Vector& X, const Standard_Real Eps)`.
    ///
    /// Computes the minimum-norm least-squares solution `X` of `A · X ≈ B`
    /// using back-substitution through the SVD.  Singular values below
    /// `eps × max(W)` are treated as zero (numerical rank detection).
    ///
    /// # Panics
    /// Panics (`StdFail_NotDone`) if the SVD failed.
    /// Panics (`Standard_DimensionError`) if `B` length ≠ `m` or `X` length ≠ `n`.
    pub fn solve(&self, b: &MathVector, x: &mut MathVector, eps: f64) {
        assert!(self.done, "math_SVD::Solve: StdFail_NotDone");
        let m = self.u.row_number();
        let n = self.u.col_number();
        assert!(
            b.length() == m,
            "math_SVD::Solve: Standard_DimensionError (B length {} != m {})",
            b.length(), m
        );
        assert!(
            x.length() == n,
            "math_SVD::Solve: Standard_DimensionError (X length {} != n {})",
            x.length(), n
        );

        // Threshold for treating a singular value as zero.
        let mut w_max = 0.0_f64;
        for j in 1..=n {
            let wj = self.w.get(j);
            if wj > w_max {
                w_max = wj;
            }
        }
        let thresh = eps * w_max;

        let lb = b.lower();
        // tmp[j] = (Uᵀ · B)[j] / w[j]  (zero if |w[j]| ≤ thresh)
        let mut tmp = MathVector::new(1, n);
        for j in 1..=n {
            let wj = self.w.get(j);
            let val = if wj.abs() > thresh {
                let mut acc = 0.0_f64;
                for i in 1..=m {
                    acc += self.u.get(i, j) * b.get(i + lb - 1);
                }
                acc / wj
            } else {
                0.0
            };
            tmp.set(j, val);
        }

        // X = V · tmp
        let lx = x.lower();
        for j in 1..=n {
            let mut s = 0.0_f64;
            for jj in 1..=n {
                s += self.v.get(j, jj) * tmp.get(jj);
            }
            x.set(j + lx - 1, s);
        }
    }

    /// `math_SVD::PseudoInverse(math_Matrix& Inv, const Standard_Real Eps)`.
    ///
    /// Computes the Moore-Penrose pseudo-inverse `A⁺ = V · diag(1/wᵢ) · Uᵀ`.
    /// Singular values below `eps × max(W)` are treated as zero.
    ///
    /// `Inv` must be a pre-allocated `n × m` matrix.
    ///
    /// # Panics
    /// Panics (`StdFail_NotDone`) if the SVD failed.
    /// Panics (`Standard_DimensionError`) if `Inv` is not `n × m`.
    pub fn pseudo_inverse(&self, inv: &mut MathMatrix, eps: f64) {
        assert!(self.done, "math_SVD::PseudoInverse: StdFail_NotDone");
        let m = self.u.row_number();
        let n = self.u.col_number();
        assert!(
            inv.row_number() == n && inv.col_number() == m,
            "math_SVD::PseudoInverse: Standard_DimensionError \
             (Inv must be {}×{}, got {}×{})",
            n, m, inv.row_number(), inv.col_number()
        );

        let mut w_max = 0.0_f64;
        for j in 1..=n {
            let wj = self.w.get(j);
            if wj > w_max {
                w_max = wj;
            }
        }
        let thresh = eps * w_max;

        let lr = inv.lower_row();
        let lc = inv.lower_col();

        // A⁺(j, i) = Σ_k  V(j,k) · U(i,k) / w(k)
        for j in 1..=n {
            for i in 1..=m {
                let mut acc = 0.0_f64;
                for k in 1..=n {
                    let wk = self.w.get(k);
                    if wk.abs() > thresh {
                        acc += self.v.get(j, k) * self.u.get(i, k) / wk;
                    }
                }
                inv.set(j + lr - 1, i + lc - 1, acc);
            }
        }
    }

    /// Singular values vector `W` (1-indexed, length `n`).
    ///
    /// # Panics
    /// Panics (`StdFail_NotDone`) if the SVD failed.
    pub fn singular_values(&self) -> &MathVector {
        assert!(self.done, "math_SVD::SingularValues: StdFail_NotDone");
        &self.w
    }

    /// `U` matrix (column-orthogonal, `m × n`, 1-indexed).
    ///
    /// # Panics
    /// Panics (`StdFail_NotDone`) if the SVD failed.
    pub fn u_matrix(&self) -> &MathMatrix {
        assert!(self.done, "math_SVD::U: StdFail_NotDone");
        &self.u
    }

    /// `V` matrix (orthogonal, `n × n`, 1-indexed).
    ///
    /// # Panics
    /// Panics (`StdFail_NotDone`) if the SVD failed.
    pub fn v_matrix(&self) -> &MathMatrix {
        assert!(self.done, "math_SVD::V: StdFail_NotDone");
        &self.v
    }

    /// `math_SVD::Dump` — state string.
    pub fn dump(&self) -> String {
        if self.done {
            "math_SVD  Status = Done \n".to_string()
        } else {
            "math_SVD  Status = not Done \n".to_string()
        }
    }
}

impl fmt::Display for MathSVD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.dump())
    }
}
