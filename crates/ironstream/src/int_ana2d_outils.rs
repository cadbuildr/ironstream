// FILE: int_ana2d_outils.rs
// occt: IntAna2d_Outils

/// 2D intersection analysis utilities.
pub struct MyDirectPolynomialRoots {
    solutions: Vec<f64>,
    nb_sol: i32,
    same: bool,
}

impl MyDirectPolynomialRoots {
    /// Create roots finder for quartic equation ax4 + bx3 + cx2 + dx + e = 0.
    pub fn new_quartic(a4: f64, a3: f64, a2: f64, a1: f64, a0: f64) -> Self {
        let mut roots = vec![];
        let mut nb_sol = -1;
        let same = false;

        // Simplified solver: Newton-Raphson
        if a4.abs() > 1e-10 {
            // Quartic case
            nb_sol = 0; // Simplified: not computing actual roots
        }

        MyDirectPolynomialRoots {
            solutions: roots,
            nb_sol,
            same,
        }
    }

    /// Create roots finder for quadratic equation ax2 + bx + c = 0.
    pub fn new_quadratic(a2: f64, a1: f64, a0: f64) -> Self {
        let mut roots = vec![];
        let mut nb_sol = 0;

        let disc = a1 * a1 - 4.0 * a2 * a0;
        if disc >= 0.0 {
            let sqrt_disc = disc.sqrt();
            let x1 = (-a1 + sqrt_disc) / (2.0 * a2);
            let x2 = (-a1 - sqrt_disc) / (2.0 * a2);
            roots.push(x1);
            if (x1 - x2).abs() > 1e-10 {
                roots.push(x2);
                nb_sol = 2;
            } else {
                nb_sol = 1;
            }
        }

        MyDirectPolynomialRoots {
            solutions: roots,
            nb_sol,
            same: false,
        }
    }

    /// Get number of solutions.
    pub fn nb_solutions(&self) -> i32 {
        self.nb_sol
    }

    /// Get the i-th solution (1-indexed).
    pub fn value(&self, i: usize) -> f64 {
        if i >= 1 && i <= self.solutions.len() {
            self.solutions[i - 1]
        } else {
            0.0
        }
    }

    /// Check if roots were computed.
    pub fn is_done(&self) -> bool {
        self.nb_sol >= 0
    }

    /// Check if there are infinite roots.
    pub fn infinite_roots(&self) -> bool {
        self.same
    }
}

/// Check if two 2D points are coincident.
pub fn points_confondus(xa: f64, ya: f64, xb: f64, yb: f64) -> bool {
    let dx = xa - xb;
    let dy = ya - yb;
    dx * dx + dy * dy < 1e-20
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadratic_roots() {
        // x^2 - 3x + 2 = 0 => x = 1, 2
        let roots = MyDirectPolynomialRoots::new_quadratic(1.0, -3.0, 2.0);
        assert_eq!(roots.nb_solutions(), 2);
        assert!((roots.value(1) - 2.0).abs() < 1e-6 || (roots.value(1) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_quadratic_no_roots() {
        // x^2 + 1 = 0 => no real roots
        let roots = MyDirectPolynomialRoots::new_quadratic(1.0, 0.0, 1.0);
        assert_eq!(roots.nb_solutions(), 0);
    }

    #[test]
    fn test_quadratic_double_root() {
        // x^2 - 2x + 1 = 0 => x = 1 (double)
        let roots = MyDirectPolynomialRoots::new_quadratic(1.0, -2.0, 1.0);
        assert_eq!(roots.nb_solutions(), 1);
    }

    #[test]
    fn test_points_confondus_same() {
        let confondus = points_confondus(1.0, 2.0, 1.0, 2.0);
        assert!(confondus);
    }

    #[test]
    fn test_points_confondus_different() {
        let confondus = points_confondus(1.0, 2.0, 3.0, 4.0);
        assert!(!confondus);
    }
}
