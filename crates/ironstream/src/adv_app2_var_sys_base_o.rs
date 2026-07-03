// FILE: adv_app2_var_sys_base_o.rs
// occt: AdvApp2Var_SysBase

//! System-level base utilities for advanced approximation.

pub struct SysBase;

impl SysBase {
    pub fn solve_linear_system(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
        if a.is_empty() || a[0].is_empty() || b.len() != a.len() {
            return None;
        }

        let n = a.len();
        let mut a_copy = a.to_vec();
        let mut b_copy = b.to_vec();

        // Simple Gaussian elimination
        for i in 0..n {
            // Find pivot
            let mut max_row = i;
            for k in (i + 1)..n {
                if a_copy[k][i].abs() > a_copy[max_row][i].abs() {
                    max_row = k;
                }
            }

            if a_copy[max_row][i].abs() < 1e-15 {
                return None;
            }

            a_copy.swap(i, max_row);
            b_copy.swap(i, max_row);

            // Eliminate below
            for k in (i + 1)..n {
                let factor = a_copy[k][i] / a_copy[i][i];
                for j in i..n {
                    a_copy[k][j] -= factor * a_copy[i][j];
                }
                b_copy[k] -= factor * b_copy[i];
            }
        }

        // Back substitution
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut sum = b_copy[i];
            for j in (i + 1)..n {
                sum -= a_copy[i][j] * x[j];
            }
            x[i] = sum / a_copy[i][i];
        }

        Some(x)
    }

    pub fn norm_l2(v: &[f64]) -> f64 {
        v.iter().fold(0.0, |acc, &x| acc + x * x).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_linear_system() {
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let b = vec![8.0, 13.0];
        let result = SysBase::solve_linear_system(&a, &b);
        assert!(result.is_some());
    }

    #[test]
    fn test_norm_l2() {
        let v = vec![3.0, 4.0];
        assert!((SysBase::norm_l2(&v) - 5.0).abs() < 1e-9);
    }
}
