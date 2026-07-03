// FILE: n_collection_mat4.rs
// occt: NCollection_Mat4

/// 4x4 matrix for 3D transformations.
/// Empty constructor returns identity matrix.
#[derive(Copy, Clone, Debug)]
pub struct Mat4<T> {
    data: [T; 16],
}

impl<T: Copy + Default> Mat4<T> {
    /// Create identity matrix.
    pub fn identity() -> Self
    where
        T: From<i32> + PartialEq,
    {
        let mut m = Mat4 {
            data: [T::default(); 16],
        };
        m.data[0] = T::from(1);   // (0,0)
        m.data[5] = T::from(1);   // (1,1)
        m.data[10] = T::from(1);  // (2,2)
        m.data[15] = T::from(1);  // (3,3)
        m
    }

    /// Create zero matrix.
    pub fn zero() -> Self {
        Mat4 {
            data: [T::default(); 16],
        }
    }

    /// Number of rows.
    pub const fn rows() -> usize {
        4
    }

    /// Number of columns.
    pub const fn cols() -> usize {
        4
    }

    /// Get element at (row, col). Column-major order.
    pub fn get(&self, row: usize, col: usize) -> T {
        self.data[col * 4 + row]
    }

    /// Set element at (row, col). Column-major order.
    pub fn set(&mut self, row: usize, col: usize, val: T) {
        self.data[col * 4 + row] = val;
    }

    /// Get mutable reference to underlying data.
    pub fn data_mut(&mut self) -> &mut [T; 16] {
        &mut self.data
    }

    /// Get reference to underlying data.
    pub fn data(&self) -> &[T; 16] {
        &self.data
    }
}

impl<T: PartialEq + Copy> PartialEq for Mat4<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: Eq + Copy> Eq for Mat4<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_matrix() {
        let m: Mat4<f64> = Mat4::identity();
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(1, 1), 1.0);
        assert_eq!(m.get(2, 2), 1.0);
        assert_eq!(m.get(3, 3), 1.0);
        assert_eq!(m.get(0, 1), 0.0);
    }

    #[test]
    fn test_zero_matrix() {
        let m: Mat4<i32> = Mat4::zero();
        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(m.get(i, j), 0);
            }
        }
    }

    #[test]
    fn test_dimensions() {
        assert_eq!(Mat4::<f64>::rows(), 4);
        assert_eq!(Mat4::<f64>::cols(), 4);
    }

    #[test]
    fn test_set_get() {
        let mut m: Mat4<f64> = Mat4::zero();
        m.set(2, 3, 7.5);
        assert_eq!(m.get(2, 3), 7.5);
    }
}
