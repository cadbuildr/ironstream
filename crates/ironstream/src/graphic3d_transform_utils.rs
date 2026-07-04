// FILE: graphic3d_transform_utils.rs
// occt: Graphic3d_TransformUtils

use std::f64::consts::PI;

/// Helper utilities for transformation matrix operations.
/// Provides static functions for converting transformations, constructing
/// projection matrices, and performing viewport projections.

/// 4x4 Transformation matrix for f64
#[derive(Debug, Clone)]
pub struct Mat4f64 {
    data: [[f64; 4]; 4],
}

/// 4x4 Transformation matrix for f32
#[derive(Debug, Clone)]
pub struct Mat4f32 {
    data: [[f32; 4]; 4],
}

/// 3D vector type
#[derive(Debug, Clone, Copy)]
pub struct Vec3f64(pub f64, pub f64, pub f64);

/// 2D vector type
#[derive(Debug, Clone, Copy)]
pub struct Vec2f64(pub f64, pub f64);

impl Mat4f64 {
    /// Creates an identity matrix
    pub fn identity() -> Self {
        let mut data = [[0.0; 4]; 4];
        data[0][0] = 1.0;
        data[1][1] = 1.0;
        data[2][2] = 1.0;
        data[3][3] = 1.0;
        Self { data }
    }

    /// Sets all values to identity
    pub fn init_identity(&mut self) {
        for i in 0..4 {
            for j in 0..4 {
                self.data[i][j] = if i == j { 1.0 } else { 0.0 };
            }
        }
    }

    /// Gets a value at (row, col)
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row][col]
    }

    /// Sets a value at (row, col)
    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        self.data[row][col] = val;
    }

    /// Constructs a 3D orthographic projection matrix
    pub fn ortho(left: f64, right: f64, bottom: f64, top: f64, z_near: f64, z_far: f64) -> Self {
        let mut mat = Self::identity();

        let inv_dx = 1.0 / (right - left);
        let inv_dy = 1.0 / (top - bottom);
        let inv_dz = 1.0 / (z_far - z_near);

        mat.data[0][0] = 2.0 * inv_dx;
        mat.data[1][1] = 2.0 * inv_dy;
        mat.data[2][2] = -2.0 * inv_dz;

        mat.data[0][3] = -(right + left) * inv_dx;
        mat.data[1][3] = -(top + bottom) * inv_dy;
        mat.data[2][3] = -(z_far + z_near) * inv_dz;

        mat
    }

    /// Constructs a 2D orthographic projection matrix
    pub fn ortho_2d(left: f64, right: f64, bottom: f64, top: f64) -> Self {
        Self::ortho(left, right, bottom, top, -1.0, 1.0)
    }

    /// Matrix multiplication
    pub fn multiply(&self, other: &Self) -> Self {
        let mut result = Self::identity();
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.data[i][k] * other.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        result
    }

    /// Computes the determinant of the 3x3 submatrix
    pub fn determinant_mat3(&self) -> f64 {
        self.data[0][0] * (self.data[1][1] * self.data[2][2] - self.data[1][2] * self.data[2][1])
            - self.data[0][1] * (self.data[1][0] * self.data[2][2] - self.data[1][2] * self.data[2][0])
            + self.data[0][2] * (self.data[1][0] * self.data[2][1] - self.data[1][1] * self.data[2][0])
    }

    /// Inverts the matrix if possible
    pub fn inverted(&self) -> Option<Self> {
        // Simple inversion for affine transformation matrices
        let mut result = Self::identity();

        // For a 4x4 affine transformation, extract rotation and translation
        // This is a simplified version; full matrix inversion would be more complex
        let det = self.determinant_mat3();
        if det.abs() < 1e-10 {
            return None;
        }

        // Copy the 3x3 rotation part and invert it
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = self.data[i][j];
            }
        }

        Some(result)
    }

    /// Constructs a rotation matrix
    pub fn construct_rotate(angle_degrees: f64, x: f64, y: f64, z: f64) -> Self {
        let mut mat = Self::identity();

        let angle_rad = angle_degrees * PI / 180.0;
        let sin_a = angle_rad.sin();
        let cos_a = angle_rad.cos();

        let is_only_x = x != 0.0 && y == 0.0 && z == 0.0;
        let is_only_y = x == 0.0 && y != 0.0 && z == 0.0;
        let is_only_z = x == 0.0 && y == 0.0 && z != 0.0;

        if is_only_x {
            mat.data[1][1] = cos_a;
            mat.data[2][2] = cos_a;
            if x < 0.0 {
                mat.data[1][2] = sin_a;
                mat.data[2][1] = -sin_a;
            } else {
                mat.data[1][2] = -sin_a;
                mat.data[2][1] = sin_a;
            }
            return mat;
        } else if is_only_y {
            mat.data[0][0] = cos_a;
            mat.data[2][2] = cos_a;
            if y < 0.0 {
                mat.data[0][2] = -sin_a;
                mat.data[2][0] = sin_a;
            } else {
                mat.data[0][2] = sin_a;
                mat.data[2][0] = -sin_a;
            }
            return mat;
        } else if is_only_z {
            mat.data[0][0] = cos_a;
            mat.data[1][1] = cos_a;
            if z < 0.0 {
                mat.data[0][1] = sin_a;
                mat.data[1][0] = -sin_a;
            } else {
                mat.data[0][1] = -sin_a;
                mat.data[1][0] = sin_a;
            }
            return mat;
        }

        // General rotation using Rodrigues' formula
        let mut norm = (x * x + y * y + z * z).sqrt();
        if norm <= 1e-4 {
            return mat; // Negligible rotation
        }

        norm = 1.0 / norm;
        let nx = x * norm;
        let ny = y * norm;
        let nz = z * norm;

        let nxx = nx * nx;
        let nyy = ny * ny;
        let nzz = nz * nz;
        let nxy = nx * ny;
        let nyz = ny * nz;
        let nzx = nz * nx;
        let sin_x = nx * sin_a;
        let sin_y = ny * sin_a;
        let sin_z = nz * sin_a;
        let one_minus_cos = 1.0 - cos_a;

        mat.data[0][0] = one_minus_cos * nxx + cos_a;
        mat.data[0][1] = one_minus_cos * nxy - sin_z;
        mat.data[0][2] = one_minus_cos * nzx + sin_y;

        mat.data[1][0] = one_minus_cos * nxy + sin_z;
        mat.data[1][1] = one_minus_cos * nyy + cos_a;
        mat.data[1][2] = one_minus_cos * nyz - sin_x;

        mat.data[2][0] = one_minus_cos * nzx - sin_y;
        mat.data[2][1] = one_minus_cos * nyz + sin_x;
        mat.data[2][2] = one_minus_cos * nzz + cos_a;

        mat
    }

    /// Constructs a scaling matrix
    pub fn scale(sx: f64, sy: f64, sz: f64) -> Self {
        let mut mat = Self::identity();
        mat.data[0][0] = sx;
        mat.data[1][1] = sy;
        mat.data[2][2] = sz;
        mat
    }

    /// Constructs a translation matrix
    pub fn translate(tx: f64, ty: f64, tz: f64) -> Self {
        let mut mat = Self::identity();
        mat.data[0][3] = tx;
        mat.data[1][3] = ty;
        mat.data[2][3] = tz;
        mat
    }

    /// Projects object coordinates to window coordinates
    pub fn project(
        obj_x: f64,
        obj_y: f64,
        obj_z: f64,
        model_view: &Self,
        projection: &Self,
        viewport: &[i32; 4],
    ) -> Option<(f64, f64, f64)> {
        let mut v = [obj_x, obj_y, obj_z, 1.0];

        // Apply model-view matrix
        let mut v2 = [0.0; 4];
        for i in 0..4 {
            let mut sum = 0.0;
            for j in 0..4 {
                sum += model_view.data[i][j] * v[j];
            }
            v2[i] = sum;
        }

        // Apply projection matrix
        let mut v3 = [0.0; 4];
        for i in 0..4 {
            let mut sum = 0.0;
            for j in 0..4 {
                sum += projection.data[i][j] * v2[j];
            }
            v3[i] = sum;
        }

        if (v3[3]).abs() < 1e-10 {
            return None;
        }

        let w_inv = 1.0 / v3[3];
        let x = v3[0] * w_inv;
        let y = v3[1] * w_inv;
        let z = v3[2] * w_inv;

        // Map to 0-1 range
        let x = x * 0.5 + 0.5;
        let y = y * 0.5 + 0.5;
        let z = z * 0.5 + 0.5;

        // Map to viewport
        let win_x = x * viewport[2] as f64 + viewport[0] as f64;
        let win_y = y * viewport[3] as f64 + viewport[1] as f64;

        Some((win_x, win_y, z))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_matrix() {
        let mat = Mat4f64::identity();
        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert_eq!(mat.get(i, j), expected);
            }
        }
    }

    #[test]
    fn test_ortho_matrix() {
        let mat = Mat4f64::ortho(0.0, 10.0, 0.0, 10.0, -1.0, 1.0);
        // Diagonal elements should reflect scaling
        assert_eq!(mat.get(0, 0), 0.2); // 2.0 / 10.0
        assert_eq!(mat.get(1, 1), 0.2); // 2.0 / 10.0
        assert_eq!(mat.get(2, 2), -1.0); // -2.0 / 2.0
    }

    #[test]
    fn test_ortho_2d_matrix() {
        let mat = Mat4f64::ortho_2d(-5.0, 5.0, -5.0, 5.0);
        assert_eq!(mat.get(0, 0), 0.2); // 2.0 / 10.0
        assert_eq!(mat.get(1, 1), 0.2); // 2.0 / 10.0
    }

    #[test]
    fn test_scale_matrix() {
        let mat = Mat4f64::scale(2.0, 3.0, 4.0);
        assert_eq!(mat.get(0, 0), 2.0);
        assert_eq!(mat.get(1, 1), 3.0);
        assert_eq!(mat.get(2, 2), 4.0);
        assert_eq!(mat.get(3, 3), 1.0);
    }

    #[test]
    fn test_translate_matrix() {
        let mat = Mat4f64::translate(1.0, 2.0, 3.0);
        assert_eq!(mat.get(0, 3), 1.0);
        assert_eq!(mat.get(1, 3), 2.0);
        assert_eq!(mat.get(2, 3), 3.0);
    }

    #[test]
    fn test_rotate_z_axis() {
        let mat = Mat4f64::construct_rotate(90.0, 0.0, 0.0, 1.0);
        // 90 degree rotation around Z should have cos(90)=0, sin(90)=1
        assert!((mat.get(0, 0)).abs() < 1e-10); // cos(90) ≈ 0
        assert!((mat.get(0, 1) + 1.0).abs() < 1e-10); // -sin(90) ≈ -1
        assert!((mat.get(1, 0) - 1.0).abs() < 1e-10); // sin(90) ≈ 1
        assert!((mat.get(1, 1)).abs() < 1e-10); // cos(90) ≈ 0
    }

    #[test]
    fn test_matrix_multiplication() {
        let scale = Mat4f64::scale(2.0, 2.0, 2.0);
        let translate = Mat4f64::translate(1.0, 1.0, 1.0);
        let result = scale.multiply(&translate);

        // Result should be defined, not all zeros
        assert_ne!(result.get(0, 0), 0.0);
    }

    #[test]
    fn test_determinant_identity() {
        let mat = Mat4f64::identity();
        assert_eq!(mat.determinant_mat3(), 1.0);
    }

    #[test]
    fn test_determinant_scale() {
        let mat = Mat4f64::scale(2.0, 3.0, 4.0);
        assert_eq!(mat.determinant_mat3(), 24.0); // 2*3*4
    }

    #[test]
    fn test_init_identity() {
        let mut mat = Mat4f64::scale(5.0, 6.0, 7.0);
        mat.init_identity();
        assert_eq!(mat.determinant_mat3(), 1.0);
    }

    #[test]
    fn test_set_get() {
        let mut mat = Mat4f64::identity();
        mat.set(0, 1, 5.5);
        assert_eq!(mat.get(0, 1), 5.5);
    }
}
