// FILE: adv_app2_var_math_base_o.rs
// occt: AdvApp2Var_MathBase

//! Math base utilities for advanced approximation.

pub struct MathBase;

impl MathBase {
    pub fn matrix_multiply(a: &[Vec<f64>], b: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
        if a.is_empty() || b.is_empty() || a[0].len() != b.len() {
            return None;
        }

        let m = a.len();
        let n = b[0].len();
        let k = b.len();

        let mut result = vec![vec![0.0; n]; m];
        for i in 0..m {
            for j in 0..n {
                for l in 0..k {
                    result[i][j] += a[i][l] * b[l][j];
                }
            }
        }
        Some(result)
    }

    pub fn matrix_transpose(a: &[Vec<f64>]) -> Vec<Vec<f64>> {
        if a.is_empty() {
            return vec![];
        }
        let m = a.len();
        let n = a[0].len();
        let mut result = vec![vec![0.0; m]; n];
        for i in 0..m {
            for j in 0..a[i].len() {
                result[j][i] = a[i][j];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let result = MathBase::matrix_multiply(&a, &b);
        assert!(result.is_some());
    }

    #[test]
    fn test_matrix_transpose() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let result = MathBase::matrix_transpose(&a);
        assert_eq!(result[0][0], 1.0);
        assert_eq!(result[0][1], 3.0);
    }
}
