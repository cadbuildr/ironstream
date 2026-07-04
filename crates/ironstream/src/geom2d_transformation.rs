// FILE: geom2d_transformation.rs
// occt: Geom2d_Transformation

/// 2D geometric transformation.
#[derive(Clone, Debug)]
pub struct Transformation {
    matrix: [[f64; 3]; 3],
}

impl Transformation {
    pub fn identity() -> Self {
        Self {
            matrix: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn translate(tx: f64, ty: f64) -> Self {
        Self {
            matrix: [[1.0, 0.0, tx], [0.0, 1.0, ty], [0.0, 0.0, 1.0]],
        }
    }

    pub fn scale(sx: f64, sy: f64) -> Self {
        Self {
            matrix: [[sx, 0.0, 0.0], [0.0, sy, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn rotate(angle: f64) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            matrix: [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn apply_to_point(&self, x: f64, y: f64) -> (f64, f64) {
        let x_new = self.matrix[0][0] * x + self.matrix[0][1] * y + self.matrix[0][2];
        let y_new = self.matrix[1][0] * x + self.matrix[1][1] * y + self.matrix[1][2];
        (x_new, y_new)
    }

    pub fn is_identity(&self) -> bool {
        let id = Self::identity();
        (0..3).all(|i| (0..3).all(|j| (self.matrix[i][j] - id.matrix[i][j]).abs() < 1.0e-15))
    }
}

impl Default for Transformation {
    fn default() -> Self {
        Self::identity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let t = Transformation::identity();
        assert!(t.is_identity());
    }

    #[test]
    fn test_translate() {
        let t = Transformation::translate(1.0, 2.0);
        let (x, y) = t.apply_to_point(0.0, 0.0);
        assert!((x - 1.0).abs() < 1.0e-10);
        assert!((y - 2.0).abs() < 1.0e-10);
    }

    #[test]
    fn test_scale() {
        let t = Transformation::scale(2.0, 3.0);
        let (x, y) = t.apply_to_point(1.0, 1.0);
        assert!((x - 2.0).abs() < 1.0e-10);
        assert!((y - 3.0).abs() < 1.0e-10);
    }

    #[test]
    fn test_rotate() {
        let t = Transformation::rotate(std::f64::consts::PI / 2.0);
        let (x, y) = t.apply_to_point(1.0, 0.0);
        assert!(x.abs() < 1.0e-10);
        assert!((y - 1.0).abs() < 1.0e-10);
    }
}
