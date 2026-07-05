// FILE: geom_lib_array1_of_mat.rs
// occt: GeomLib_Array1OfMat

//! Deprecated: Use Vec<Mat3> directly.
//! Alias for backward compatibility with OCCT.

#[derive(Clone, Debug, PartialEq)]
pub struct Mat3 {
    pub data: [[f64; 3]; 3],
}

impl Mat3 {
    pub fn identity() -> Self {
        Mat3 {
            data: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn zero() -> Self {
        Mat3 {
            data: [[0.0; 3]; 3],
        }
    }
}

pub type GeomLibArray1OfMat = Vec<Mat3>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let mut array: GeomLibArray1OfMat = Vec::new();
        assert_eq!(array.len(), 0);

        array.push(Mat3::identity());
        assert_eq!(array.len(), 1);
        assert_eq!(array[0], Mat3::identity());
    }

    #[test]
    fn test_matrix_identity() {
        let mat = Mat3::identity();
        assert_eq!(mat.data[0][0], 1.0);
        assert_eq!(mat.data[0][1], 0.0);
        assert_eq!(mat.data[1][1], 1.0);
        assert_eq!(mat.data[2][2], 1.0);
    }

    #[test]
    fn test_array_with_matrices() {
        let mut array: GeomLibArray1OfMat = vec![Mat3::identity(), Mat3::zero()];
        assert_eq!(array.len(), 2);
        assert_eq!(array[0], Mat3::identity());
        assert_eq!(array[1], Mat3::zero());
    }
}
