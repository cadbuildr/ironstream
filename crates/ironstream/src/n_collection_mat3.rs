// FILE: n_collection_mat3.rs
// occt: NCollection_Mat3

/// 3x3 matrix for linear algebra operations.
/// Empty constructor returns identity matrix.
#[derive(Copy, Clone, Debug)]
pub struct Mat3<T> {
    data: [T; 9],
}

impl<T: Copy + Default> Mat3<T> {
    /// Create identity matrix.
    pub fn identity() -> Self
    where
        T: From<i32> + PartialEq,
    {
        let mut m = Mat3 {
            data: [T::default(); 9],
        };
        m.data[0] = T::from(1); // (0,0)
        m.data[4] = T::from(1); // (1,1)
        m.data[8] = T::from(1); // (2,2)
        m
    }

    /// Create zero matrix.
    pub fn zero() -> Self {
        Mat3 {
            data: [T::default(); 9],
        }
    }

    /// Get element at (row, col). Column-major order.
    pub fn get(&self, row: usize, col: usize) -> T {
        self.data[col * 3 + row]
    }

    /// Set element at (row, col). Column-major order.
    pub fn set(&mut self, row: usize, col: usize, val: T) {
        self.data[col * 3 + row] = val;
    }

    /// Get mutable reference to underlying data.
    pub fn data_mut(&mut self) -> &mut [T; 9] {
        &mut self.data
    }

    /// Get reference to underlying data.
    pub fn data(&self) -> &[T; 9] {
        &self.data
    }
}

impl<T: PartialEq + Copy> PartialEq for Mat3<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: Eq + Copy> Eq for Mat3<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_matrix() {
        let m: Mat3<f64> = Mat3::identity();
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(1, 1), 1.0);
        assert_eq!(m.get(2, 2), 1.0);
        assert_eq!(m.get(0, 1), 0.0);
    }

    #[test]
    fn test_zero_matrix() {
        let m: Mat3<i32> = Mat3::zero();
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(m.get(i, j), 0);
            }
        }
    }

    #[test]
    fn test_set_get() {
        let mut m: Mat3<f64> = Mat3::zero();
        m.set(1, 2, 5.0);
        assert_eq!(m.get(1, 2), 5.0);
    }

    #[test]
    fn test_equality() {
        let m1: Mat3<i32> = Mat3::identity();
        let m2: Mat3<i32> = Mat3::identity();
        assert_eq!(m1, m2);
    }
}
