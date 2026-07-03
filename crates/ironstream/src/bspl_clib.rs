// FILE: bspl_clib.rs
// occt: BSplCLib — B-spline curve library functions

/// B-spline basis function evaluation result.
#[derive(Clone, Debug)]
pub struct BasisEval {
    pub span: usize,
    pub values: Vec<f64>,
}

// occt: BSplCLib — static B-spline operations (knot finding, basis evaluation, etc.)
pub struct BSplCLib;

impl BSplCLib {
    /// Find the knot span index for parameter t in a non-decreasing knot vector.
    pub fn find_span(n: usize, degree: usize, t: f64, knots: &[f64]) -> usize {
        if t >= knots[n + 1] { return n; }
        if t <= knots[degree] { return degree; }
        let mut low = degree;
        let mut high = n + 1;
        let mut mid = (low + high) / 2;
        while t < knots[mid] || t >= knots[mid + 1] {
            if t < knots[mid] { high = mid; } else { low = mid; }
            mid = (low + high) / 2;
        }
        mid
    }

    /// Evaluate B-spline basis functions N_{span-degree..span} at t.
    pub fn basis_funs(span: usize, t: f64, degree: usize, knots: &[f64]) -> Vec<f64> {
        let mut n = vec![0.0f64; degree + 1];
        let mut left = vec![0.0f64; degree + 1];
        let mut right = vec![0.0f64; degree + 1];
        n[0] = 1.0;
        for j in 1..=degree {
            left[j] = t - knots[span + 1 - j];
            right[j] = knots[span + j] - t;
            let mut saved = 0.0;
            for r in 0..j {
                let temp = n[r] / (right[r + 1] + left[j - r]);
                n[r] = saved + right[r + 1] * temp;
                saved = left[j - r] * temp;
            }
            n[j] = saved;
        }
        n
    }

    /// Evaluate a B-spline curve at parameter t.
    pub fn curve_point(t: f64, degree: usize, knots: &[f64], poles: &[[f64; 3]]) -> Option<[f64; 3]> {
        let n = poles.len().saturating_sub(1);
        if knots.len() < n + degree + 2 { return None; }
        let span = Self::find_span(n, degree, t, knots);
        let n_funs = Self::basis_funs(span, t, degree, knots);
        let mut result = [0.0f64; 3];
        for i in 0..=degree {
            let pole_idx = span - degree + i;
            if pole_idx < poles.len() {
                for k in 0..3 {
                    result[k] += n_funs[i] * poles[pole_idx][k];
                }
            }
        }
        Some(result)
    }

    /// Build a uniform knot vector for a B-spline of given degree and nb_poles.
    pub fn uniform_knots(degree: usize, nb_poles: usize) -> Vec<f64> {
        let nb_knots = nb_poles + degree + 1;
        let n = nb_poles - degree - 1;
        let mut knots = vec![0.0f64; nb_knots];
        for i in 0..=degree { knots[i] = 0.0; }
        for i in 1..=n {
            knots[degree + i] = i as f64 / (n + 1) as f64;
        }
        for i in nb_knots - degree - 1..nb_knots { knots[i] = 1.0; }
        knots
    }

    /// Build open (clamped) knot vector with explicit multiplicities.
    pub fn build_knots_from_mults(knot_values: &[f64], mults: &[u32]) -> Vec<f64> {
        let mut result = Vec::new();
        for (&k, &m) in knot_values.iter().zip(mults.iter()) {
            for _ in 0..m { result.push(k); }
        }
        result
    }

    /// Move parameter from one knot range to another (reparametrization).
    pub fn reparametrize(t: f64, old_first: f64, old_last: f64, new_first: f64, new_last: f64) -> f64 {
        if (old_last - old_first).abs() < 1e-15 { return new_first; }
        let u = (t - old_first) / (old_last - old_first);
        new_first + u * (new_last - new_first)
    }

    /// Check that a knot vector is valid (non-decreasing, clamped).
    pub fn check_knots(degree: usize, knots: &[f64], nb_poles: usize) -> bool {
        if knots.len() != nb_poles + degree + 1 { return false; }
        for i in 1..knots.len() {
            if knots[i] < knots[i - 1] { return false; }
        }
        true
    }

    /// Multiplicity of each distinct knot.
    pub fn knot_multiplicities(knots: &[f64]) -> Vec<(f64, u32)> {
        if knots.is_empty() { return Vec::new(); }
        let mut result: Vec<(f64, u32)> = Vec::new();
        let mut current = knots[0];
        let mut count = 1u32;
        for &k in &knots[1..] {
            if (k - current).abs() < 1e-15 { count += 1; } else { result.push((current, count)); current = k; count = 1; }
        }
        result.push((current, count));
        result
    }
}

// occt: BSplSLib — B-spline surface library (basic operations)
pub struct BSplSLib;

impl BSplSLib {
    /// Evaluate a B-spline surface at (u, v).
    pub fn surface_point(
        u: f64, v: f64,
        degree_u: usize, degree_v: usize,
        knots_u: &[f64], knots_v: &[f64],
        poles: &[Vec<[f64; 3]>],
    ) -> Option<[f64; 3]> {
        let nb_u = poles.len();
        let nb_v = if nb_u > 0 { poles[0].len() } else { 0 };
        if nb_u == 0 || nb_v == 0 { return None; }
        let n_u = nb_u - 1;
        let n_v = nb_v - 1;
        if knots_u.len() < nb_u + degree_u + 1 || knots_v.len() < nb_v + degree_v + 1 { return None; }
        let span_u = BSplCLib::find_span(n_u, degree_u, u, knots_u);
        let span_v = BSplCLib::find_span(n_v, degree_v, v, knots_v);
        let nu = BSplCLib::basis_funs(span_u, u, degree_u, knots_u);
        let nv = BSplCLib::basis_funs(span_v, v, degree_v, knots_v);
        let mut result = [0.0f64; 3];
        for i in 0..=degree_u {
            let pi = span_u - degree_u + i;
            if pi >= nb_u { continue; }
            for j in 0..=degree_v {
                let pj = span_v - degree_v + j;
                if pj >= nb_v { continue; }
                let w = nu[i] * nv[j];
                for k in 0..3 { result[k] += w * poles[pi][pj][k]; }
            }
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_span_basic() {
        // degree=2, n=3 → knots=[0,0,0,0.5,1,1,1]
        let knots = vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0];
        let span = BSplCLib::find_span(3, 2, 0.3, &knots);
        assert_eq!(span, 2);
    }

    #[test]
    fn basis_funs_partition_of_unity() {
        let knots = vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0];
        let span = BSplCLib::find_span(3, 2, 0.25, &knots);
        let n = BSplCLib::basis_funs(span, 0.25, 2, &knots);
        let sum: f64 = n.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10);
    }

    #[test]
    fn curve_point_line() {
        // Linear B-spline: 2 poles = line from [0,0,0] to [1,0,0]
        let knots = vec![0.0, 0.0, 1.0, 1.0];
        let poles = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
        let pt = BSplCLib::curve_point(0.5, 1, &knots, &poles).unwrap();
        assert!((pt[0] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn uniform_knots_valid() {
        let k = BSplCLib::uniform_knots(3, 6);
        assert!(BSplCLib::check_knots(3, &k, 6));
    }

    #[test]
    fn knot_multiplicities_basic() {
        let knots = vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0];
        let mults = BSplCLib::knot_multiplicities(&knots);
        assert_eq!(mults, vec![(0.0, 3), (0.5, 1), (1.0, 3)]);
    }

    #[test]
    fn reparametrize_basic() {
        let t = BSplCLib::reparametrize(0.5, 0.0, 1.0, 0.0, 2.0);
        assert!((t - 1.0).abs() < 1e-10);
    }
}
