// FILE: graphic3d_mat4.rs
// occt: Graphic3d_Mat4

//! 4x4 transformation matrix for 3D graphics.
//! Deprecated: Use standard matrix type directly.

#[derive(Clone, Debug, PartialEq)]
pub struct Mat4 {
    pub data: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn identity() -> Self {
        Mat4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn zero() -> Self {
        Mat4 {
            data: [[0.0; 4]; 4],
        }
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        let mut mat = Self::identity();
        mat.data[0][3] = x;
        mat.data[1][3] = y;
        mat.data[2][3] = z;
        mat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let mat = Mat4::identity();
        assert_eq!(mat.data[0][0], 1.0);
        assert_eq!(mat.data[1][1], 1.0);
        assert_eq!(mat.data[2][2], 1.0);
        assert_eq!(mat.data[3][3], 1.0);
        assert_eq!(mat.data[0][1], 0.0);
    }

    #[test]
    fn test_zero() {
        let mat = Mat4::zero();
        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(mat.data[i][j], 0.0);
            }
        }
    }

    #[test]
    fn test_translate() {
        let mat = Mat4::translate(1.0, 2.0, 3.0);
        assert_eq!(mat.data[0][3], 1.0);
        assert_eq!(mat.data[1][3], 2.0);
        assert_eq!(mat.data[2][3], 3.0);
        assert_eq!(mat.data[3][3], 1.0);
    }

    #[test]
    fn test_equality() {
        let m1 = Mat4::identity();
        let m2 = Mat4::identity();
        assert_eq!(m1, m2);
    }
}
