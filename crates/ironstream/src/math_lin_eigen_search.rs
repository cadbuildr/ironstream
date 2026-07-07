// FILE: math_lin_eigen_search.rs
// occt: MathLin_EigenSearch

use std::f64;

/// Status codes for eigenvalue decomposition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Ok,
    NotConverged,
    InvalidInput,
    MaxIterations,
}

/// Result of eigenvalue decomposition.
pub struct EigenResult {
    pub status: Status,
    pub eigen_values: Option<Vec<f64>>,
    pub eigen_vectors: Option<Vec<Vec<f64>>>,
    pub dimension: usize,
}

impl EigenResult {
    pub fn is_done(&self) -> bool {
        self.status == Status::Ok
    }
}

/// Eigenvalue decomposition of symmetric tridiagonal matrix using QL algorithm.
pub fn eigen_tridiagonal(
    diagonal: &[f64],
    subdiagonal: &[f64],
    max_iterations: usize,
) -> EigenResult {
    let n = diagonal.len();
    if subdiagonal.len() != n {
        return EigenResult {
            status: Status::InvalidInput,
            eigen_values: None,
            eigen_vectors: None,
            dimension: n,
        };
    }

    if n == 0 {
        return EigenResult {
            status: Status::InvalidInput,
            eigen_values: None,
            eigen_vectors: None,
            dimension: 0,
        };
    }

    // Create working copies
    let mut diag = diagonal.to_vec();
    let mut subdiag = vec![0.0; n];

    // Shift subdiagonal for QL algorithm
    for i in 1..n {
        subdiag[i - 1] = subdiagonal[i];
    }
    subdiag[n - 1] = 0.0;

    // Initialize eigenvector matrix as identity
    let mut eigen_vec = vec![vec![0.0; n]; n];
    for i in 0..n {
        eigen_vec[i][i] = 1.0;
    }

    if n == 1 {
        return EigenResult {
            status: Status::Ok,
            eigen_values: Some(diag),
            eigen_vectors: Some(eigen_vec),
            dimension: n,
        };
    }

    // QL Algorithm with implicit shifts
    for start in 0..n {
        let mut iter_count = 0;
        loop {
            let end = find_submatrix_end(&diag, &subdiag, start, n);

            if end != start {
                if iter_count >= max_iterations {
                    return EigenResult {
                        status: Status::MaxIterations,
                        eigen_values: None,
                        eigen_vectors: None,
                        dimension: n,
                    };
                }
                iter_count += 1;

                let shift = compute_wilkinson_shift(&diag, &subdiag, start, end);
                let _ = perform_ql_step(&mut diag, &mut subdiag, &mut eigen_vec, start, end, shift, n);
            } else {
                break;
            }
        }
    }

    EigenResult {
        status: Status::Ok,
        eigen_values: Some(diag),
        eigen_vectors: Some(eigen_vec),
        dimension: n,
    }
}

fn find_submatrix_end(diag: &[f64], subdiag: &[f64], start: usize, n: usize) -> usize {
    // Mirrors OCCT findSubmatrixEnd: scan up to the second-to-last row; if no
    // negligible subdiagonal element is found, the submatrix ends at the last row.
    for end in start..n - 1 {
        let diag_sum = diag[end].abs() + diag[end + 1].abs();
        if subdiag[end].abs() + diag_sum == diag_sum {
            return end;
        }
    }
    n - 1
}

fn compute_wilkinson_shift(diag: &[f64], subdiag: &[f64], start: usize, end: usize) -> f64 {
    // OCCT computeWilkinsonShift uses theDiagWork(theStart + 1), not the end row.
    let mut shift = (diag[start + 1] - diag[start]) / (2.0 * subdiag[start]);
    let radius = (1.0 + shift * shift).sqrt();

    if shift < 0.0 {
        shift = diag[end] - diag[start] + subdiag[start] / (shift - radius);
    } else {
        shift = diag[end] - diag[start] + subdiag[start] / (shift + radius);
    }
    shift
}

fn perform_ql_step(
    diag: &mut [f64],
    subdiag: &mut [f64],
    eigen_vec: &mut [Vec<f64>],
    start: usize,
    end: usize,
    shift: f64,
    n: usize,
) -> bool {
    let mut sine = 1.0;
    let mut cosine = 1.0;
    let mut prev_diag = 0.0;
    let mut s = shift;

    for row_idx in (start..end).rev() {
        let temp_val = sine * subdiag[row_idx];
        let subdiag_temp = cosine * subdiag[row_idx];
        let radius = (temp_val * temp_val + s * s).sqrt();

        subdiag[row_idx + 1] = radius;

        if radius == 0.0 {
            diag[row_idx + 1] -= prev_diag;
            subdiag[end] = 0.0;
            // OCCT performQLStep returns here without updating the start row
            // (the `aRadius == 0.0 && aRowIdx >= 1` early-return path).
            return true;
        }

        sine = temp_val / radius;
        cosine = s / radius;
        s = diag[row_idx + 1] - prev_diag;

        let radius_temp = (diag[row_idx] - s) * sine + 2.0 * cosine * subdiag_temp;
        prev_diag = sine * radius_temp;
        diag[row_idx + 1] = s + prev_diag;
        s = cosine * radius_temp - subdiag_temp;

        // Update eigenvector matrix
        for vec_idx in 0..n {
            let temp_vec = eigen_vec[vec_idx][row_idx + 1];
            eigen_vec[vec_idx][row_idx + 1] = sine * eigen_vec[vec_idx][row_idx] + cosine * temp_vec;
            eigen_vec[vec_idx][row_idx] = cosine * eigen_vec[vec_idx][row_idx] - sine * temp_vec;
        }
    }

    diag[start] -= prev_diag;
    subdiag[start] = s;
    subdiag[end] = 0.0;

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1x1_matrix() {
        let diag = vec![5.0];
        let subdiag = vec![0.0];
        let result = eigen_tridiagonal(&diag, &subdiag, 30);
        assert!(result.is_done());
        assert_eq!(result.eigen_values.as_ref().unwrap()[0], 5.0);
    }

    #[test]
    fn test_2x2_matrix() {
        // Tridiagonal: [1, 0; 0, 2]
        // Per OCCT math_EigenValuesSearcher, the subdiagonal array has the same
        // length as the diagonal; its first element is unused.
        let diag = vec![1.0, 2.0];
        let subdiag = vec![0.0, 0.0];
        let result = eigen_tridiagonal(&diag, &subdiag, 30);
        assert!(result.is_done());
        let eigs = result.eigen_values.unwrap();
        assert!(eigs[0] > 0.5 && eigs[0] < 1.5);
        assert!(eigs[1] > 1.5 && eigs[1] < 2.5);
    }

    #[test]
    fn test_invalid_input() {
        // OCCT requires Subdiagonal.Length() == Diagonal.Length(); a shorter
        // subdiagonal is a dimension mismatch (IsDone() stays false).
        let diag = vec![1.0, 2.0];
        let subdiag = vec![0.5]; // Wrong length
        let result = eigen_tridiagonal(&diag, &subdiag, 30);
        assert_eq!(result.status, Status::InvalidInput);
    }

    #[test]
    fn test_empty_input() {
        let diag = vec![];
        let subdiag = vec![];
        let result = eigen_tridiagonal(&diag, &subdiag, 30);
        assert_eq!(result.status, Status::InvalidInput);
    }
}
