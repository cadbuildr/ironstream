// FILE: app_par_curves.rs
// occt: AppParCurves

//! Parallel approximation in n curves.
//! Ports the static helpers of AppParCurves: Bernstein matrices and
//! B-spline basis function evaluation (SplineFunction).

/// 1-based vector of f64 mirroring math_Vector(1, n).
#[derive(Clone, Debug)]
pub struct Vec1 {
    d: Vec<f64>,
}

impl Vec1 {
    pub fn new(n: usize) -> Self {
        Vec1 { d: vec![0.0; n + 1] }
    }
    pub fn from_slice(s: &[f64]) -> Self {
        let mut v = Vec1::new(s.len());
        for (i, x) in s.iter().enumerate() {
            v.d[i + 1] = *x;
        }
        v
    }
    pub fn len(&self) -> usize {
        self.d.len() - 1
    }
    pub fn get(&self, i: usize) -> f64 {
        self.d[i]
    }
    pub fn set(&mut self, i: usize, v: f64) {
        self.d[i] = v;
    }
}

/// 1-based matrix of f64 mirroring math_Matrix(1, r, 1, c).
#[derive(Clone, Debug)]
pub struct Mat1 {
    rows: usize,
    cols: usize,
    d: Vec<f64>,
}

impl Mat1 {
    pub fn new(rows: usize, cols: usize) -> Self {
        Mat1 { rows, cols, d: vec![0.0; rows * cols] }
    }
    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn cols(&self) -> usize {
        self.cols
    }
    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.d[(i - 1) * self.cols + (j - 1)]
    }
    pub fn set(&mut self, i: usize, j: usize, v: f64) {
        self.d[(i - 1) * self.cols + (j - 1)] = v;
    }
}

/// Parallel curves approximation utilities (static class AppParCurves).
pub struct AppParCurves;

impl AppParCurves {
    /// AppParCurves::BernsteinMatrix — fills A(i,j) = B_{j-1}^{NbPoles-1}(U(i)).
    pub fn bernstein_matrix(nb_poles: usize, u: &Vec1, a: &mut Mat1) {
        let mut b = Vec1::new(nb_poles - 1);
        for i in 1..=u.len() {
            b.set(1, 1.0);
            let u0 = u.get(i);
            let u1 = 1.0 - u0;
            for id in 2..=nb_poles - 1 {
                let mut y0 = b.get(1);
                let mut y1 = u0 * y0;
                b.set(1, y0 - y1);
                for j in 2..=id - 1 {
                    let xs = y1;
                    y0 = b.get(j);
                    y1 = u0 * y0;
                    b.set(j, y0 - y1 + xs);
                }
                b.set(id, y1);
            }
            a.set(i, 1, u1 * b.get(1));
            a.set(i, nb_poles, u0 * b.get(nb_poles - 1));
            for j in 2..=nb_poles - 1 {
                a.set(i, j, u1 * b.get(j) + u0 * b.get(j - 1));
            }
        }
    }

    /// AppParCurves::Bernstein — Bernstein matrix A and first derivative DA.
    pub fn bernstein(nb_poles: usize, u: &Vec1, a: &mut Mat1, da: &mut Mat1) {
        let ndeg = (nb_poles - 1) as f64;
        let mut b = Vec1::new(nb_poles - 1);
        for i in 1..=u.len() {
            b.set(1, 1.0);
            let u0 = u.get(i);
            let u1 = 1.0 - u0;
            for id in 2..=nb_poles - 1 {
                let mut y0 = b.get(1);
                let mut y1 = u0 * y0;
                b.set(1, y0 - y1);
                for j in 2..=id - 1 {
                    let xs = y1;
                    y0 = b.get(j);
                    y1 = u0 * y0;
                    b.set(j, y0 - y1 + xs);
                }
                b.set(id, y1);
            }
            da.set(i, 1, -ndeg * b.get(1));
            da.set(i, nb_poles, ndeg * b.get(nb_poles - 1));
            a.set(i, 1, u1 * b.get(1));
            a.set(i, nb_poles, u0 * b.get(nb_poles - 1));
            for j in 2..=nb_poles - 1 {
                let bj = b.get(j);
                let bj1 = b.get(j - 1);
                da.set(i, j, ndeg * (bj1 - bj));
                a.set(i, j, u1 * bj + u0 * bj1);
            }
        }
    }

    /// AppParCurves::SecondDerivativeBernstein — DDA(j) = B''_{j-1}^{deg}(U),
    /// with deg = DDA.Length() - 1.
    pub fn second_derivative_bernstein(u: f64, dda: &mut Vec1) {
        let nb_poles = dda.len();
        let deg = nb_poles - 1;
        let n4 = (deg * deg.saturating_sub(1)) as f64;
        if deg == 1 {
            dda.set(1, 0.0);
            dda.set(2, 0.0);
        } else if deg == 2 {
            dda.set(1, 2.0);
            dda.set(2, -4.0);
            dda.set(3, 2.0);
        } else {
            let mut b = Vec1::new(deg - 1);
            b.set(1, 1.0);
            for id in 2..=deg - 1 {
                let mut y0 = b.get(1);
                let mut y1 = u * y0;
                b.set(1, y0 - y1);
                for j in 2..=id - 1 {
                    let xs = y1;
                    y0 = b.get(j);
                    y1 = u * y0;
                    b.set(j, y0 - y1 + xs);
                }
                b.set(id, y1);
            }
            dda.set(1, n4 * b.get(1));
            dda.set(2, n4 * (-2.0 * b.get(1) + b.get(2)));
            dda.set(deg, n4 * (b.get(deg - 2) - 2.0 * b.get(deg - 1)));
            dda.set(deg + 1, n4 * b.get(deg - 1));
            for j in 2..=deg.saturating_sub(2) {
                dda.set(j + 1, n4 * (b.get(j - 1) - 2.0 * b.get(j) + b.get(j + 1)));
            }
        }
    }

    /// Local port of BSplCLib::LocateParameter for a non-periodic flat knot vector:
    /// finds kindex in [from_k1, to_k2 - 1] with flatknots(kindex) <= u < flatknots(kindex+1),
    /// clamped to that range.
    fn locate_parameter(flatknots: &Vec1, u: f64, from_k1: usize, to_k2: usize) -> usize {
        let mut k = from_k1;
        while k < to_k2 - 1 && flatknots.get(k + 1) <= u {
            k += 1;
        }
        k
    }

    /// AppParCurves::SplineFunction — evaluates B-spline basis functions and first
    /// derivatives at each parameter. A(i,j)/DA(i,j) receive the value/derivative
    /// of basis function j at Parameters(i); index(i) receives the knot-span offset.
    pub fn spline_function(
        nbpoles: usize,
        deg: usize,
        parameters: &Vec1,
        flatknots: &Vec1,
        a: &mut Mat1,
        da: &mut Mat1,
        index: &mut [i64],
    ) {
        let deg1 = deg + 1;
        let mut locpoles = Vec1::new(deg1);
        let mut locdpoles = Vec1::new(deg1);

        for i in 1..=parameters.len() {
            let u = parameters.get(i);
            let kindex = Self::locate_parameter(flatknots, u, deg1, nbpoles + 1);
            index[i - 1] = kindex as i64 - deg as i64 - 1;

            locpoles.set(1, 1.0);
            for qq in 2..=deg {
                locpoles.set(qq, 0.0);
                for pp in 1..=qq - 1 {
                    let inverse =
                        1.0 / (flatknots.get(kindex + pp) - flatknots.get(kindex - qq + pp + 1));
                    let saved =
                        (u - flatknots.get(kindex - qq + pp + 1)) * inverse * locpoles.get(pp);
                    let mut lp = locpoles.get(pp) * ((flatknots.get(kindex + pp) - u) * inverse);
                    lp += locpoles.get(qq);
                    locpoles.set(pp, lp);
                    locpoles.set(qq, saved);
                }
            }

            let qq = deg + 1;
            for pp in 1..=deg {
                locdpoles.set(pp, locpoles.get(pp));
            }

            let mut locqq = 0.0;
            let mut locdqq = 0.0;
            for pp in 1..=deg {
                let inverse =
                    1.0 / (flatknots.get(kindex + pp) - flatknots.get(kindex - qq + pp + 1));
                let saved = (u - flatknots.get(kindex - qq + pp + 1)) * inverse * locpoles.get(pp);
                let mut lp = locpoles.get(pp) * ((flatknots.get(kindex + pp) - u) * inverse);
                lp += locqq;
                locpoles.set(pp, lp);
                locqq = saved;
                let local_inverse = deg as f64 * inverse;
                let saved_d = local_inverse * locdpoles.get(pp);
                let mut ldp = locdpoles.get(pp) * (-local_inverse);
                ldp += locdqq;
                locdpoles.set(pp, ldp);
                locdqq = saved_d;
            }
            locpoles.set(qq, locqq);
            locdpoles.set(qq, locdqq);

            for j in 1..=deg1 {
                let theindex = (j + kindex).saturating_sub(deg1);
                a.set(i, theindex, locpoles.get(j));
                da.set(i, theindex, locdpoles.get(j));
            }
            let mut j = 1;
            while j + deg < kindex {
                a.set(i, j, 0.0);
                da.set(i, j, 0.0);
                j += 1;
            }
            for j in kindex + 1..=nbpoles {
                a.set(i, j, 0.0);
                da.set(i, j, 0.0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-12;

    #[test]
    fn test_bernstein_matrix_cubic() {
        // NbPoles = 4 -> cubic Bernstein basis.
        let u = Vec1::from_slice(&[0.0, 0.5, 1.0]);
        let mut a = Mat1::new(3, 4);
        AppParCurves::bernstein_matrix(4, &u, &mut a);
        // u = 0: only B0 = 1.
        assert!((a.get(1, 1) - 1.0).abs() < EPS);
        assert!(a.get(1, 2).abs() < EPS);
        assert!(a.get(1, 3).abs() < EPS);
        assert!(a.get(1, 4).abs() < EPS);
        // u = 0.5: [1/8, 3/8, 3/8, 1/8].
        assert!((a.get(2, 1) - 0.125).abs() < EPS);
        assert!((a.get(2, 2) - 0.375).abs() < EPS);
        assert!((a.get(2, 3) - 0.375).abs() < EPS);
        assert!((a.get(2, 4) - 0.125).abs() < EPS);
        // u = 1: only B3 = 1.
        assert!(a.get(3, 1).abs() < EPS);
        assert!((a.get(3, 4) - 1.0).abs() < EPS);
        // Partition of unity at every parameter.
        for i in 1..=3 {
            let s: f64 = (1..=4).map(|j| a.get(i, j)).sum();
            assert!((s - 1.0).abs() < EPS);
        }
    }

    #[test]
    fn test_bernstein_with_derivative() {
        let u = Vec1::from_slice(&[0.25, 0.5, 0.75]);
        let mut a = Mat1::new(3, 4);
        let mut da = Mat1::new(3, 4);
        AppParCurves::bernstein(4, &u, &mut a, &mut da);
        // At u = 0.5 (row 2): dB = [-0.75, -0.75, 0.75, 0.75].
        assert!((da.get(2, 1) + 0.75).abs() < EPS);
        assert!((da.get(2, 2) + 0.75).abs() < EPS);
        assert!((da.get(2, 3) - 0.75).abs() < EPS);
        assert!((da.get(2, 4) - 0.75).abs() < EPS);
        // A matches bernstein_matrix and derivatives sum to zero.
        let mut a2 = Mat1::new(3, 4);
        AppParCurves::bernstein_matrix(4, &u, &mut a2);
        for i in 1..=3 {
            let mut dsum = 0.0;
            for j in 1..=4 {
                assert!((a.get(i, j) - a2.get(i, j)).abs() < EPS);
                dsum += da.get(i, j);
            }
            assert!(dsum.abs() < EPS);
        }
    }

    #[test]
    fn test_second_derivative_bernstein_cubic() {
        // Cubic: B0''=6(1-u), B1''=6(3u-2), B2''=6(1-3u), B3''=6u.
        let mut dda = Vec1::new(4);
        AppParCurves::second_derivative_bernstein(0.5, &mut dda);
        assert!((dda.get(1) - 3.0).abs() < EPS);
        assert!((dda.get(2) + 3.0).abs() < EPS);
        assert!((dda.get(3) + 3.0).abs() < EPS);
        assert!((dda.get(4) - 3.0).abs() < EPS);
        let s: f64 = (1..=4).map(|j| dda.get(j)).sum();
        assert!(s.abs() < EPS);
    }

    #[test]
    fn test_second_derivative_bernstein_low_degrees() {
        let mut dda1 = Vec1::new(2);
        AppParCurves::second_derivative_bernstein(0.3, &mut dda1);
        assert_eq!(dda1.get(1), 0.0);
        assert_eq!(dda1.get(2), 0.0);
        let mut dda2 = Vec1::new(3);
        AppParCurves::second_derivative_bernstein(0.7, &mut dda2);
        assert_eq!(dda2.get(1), 2.0);
        assert_eq!(dda2.get(2), -4.0);
        assert_eq!(dda2.get(3), 2.0);
    }

    #[test]
    fn test_spline_function_partition_of_unity() {
        // nbpoles = 4, deg = 2, flat knots [0,0,0,0.5,1,1,1].
        let nbpoles = 4;
        let deg = 2;
        let flatknots = Vec1::from_slice(&[0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0]);
        let params = Vec1::from_slice(&[0.1, 0.25, 0.5, 0.75, 0.9]);
        let mut a = Mat1::new(5, nbpoles);
        let mut da = Mat1::new(5, nbpoles);
        let mut index = vec![0i64; 5];
        AppParCurves::spline_function(nbpoles, deg, &params, &flatknots, &mut a, &mut da, &mut index);
        for i in 1..=5 {
            let s: f64 = (1..=nbpoles).map(|j| a.get(i, j)).sum();
            let ds: f64 = (1..=nbpoles).map(|j| da.get(i, j)).sum();
            assert!((s - 1.0).abs() < 1e-10, "row {} sums to {}", i, s);
            assert!(ds.abs() < 1e-10, "row {} dsum {}", i, ds);
            for j in 1..=nbpoles {
                assert!(a.get(i, j) >= -1e-12, "basis functions are non-negative");
            }
        }
    }

    #[test]
    fn test_spline_function_matches_bezier_when_no_interior_knots() {
        // With flat knots [0,0,0,0,1,1,1,1] the B-spline basis of degree 3 is the
        // cubic Bernstein basis.
        let nbpoles = 4;
        let deg = 3;
        let flatknots = Vec1::from_slice(&[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0]);
        let params = Vec1::from_slice(&[0.25, 0.5]);
        let mut a = Mat1::new(2, nbpoles);
        let mut da = Mat1::new(2, nbpoles);
        let mut index = vec![0i64; 2];
        AppParCurves::spline_function(nbpoles, deg, &params, &flatknots, &mut a, &mut da, &mut index);
        let mut ab = Mat1::new(2, nbpoles);
        let mut dab = Mat1::new(2, nbpoles);
        AppParCurves::bernstein(nbpoles, &params, &mut ab, &mut dab);
        for i in 1..=2 {
            for j in 1..=nbpoles {
                assert!((a.get(i, j) - ab.get(i, j)).abs() < 1e-10);
                assert!((da.get(i, j) - dab.get(i, j)).abs() < 1e-10);
            }
        }
    }
}
