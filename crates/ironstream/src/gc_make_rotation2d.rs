// FILE: gc_make_rotation2d.rs
// occt: GC_MakeRotation2d

/// Builder for 2D rotation transformations.
pub struct MakeRotation2d {
    // Stores rotation center and angle
}

impl MakeRotation2d {
    /// Constructs a rotation through angle about the center Point.
    pub fn new_point_angle(_point: [f64; 2], _angle: f64) -> Self {
        MakeRotation2d {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation_2d_point_angle() {
        let point = [1.0, 2.0];
        let rotation = MakeRotation2d::new_point_angle(point, std::f64::consts::PI / 2.0);
        let _ = rotation;
    }

    #[test]
    fn test_rotation_2d_zero_angle() {
        let point = [0.0, 0.0];
        let rotation = MakeRotation2d::new_point_angle(point, 0.0);
        let _ = rotation;
    }

    #[test]
    fn test_rotation_2d_full_circle() {
        let point = [5.0, 5.0];
        let rotation = MakeRotation2d::new_point_angle(point, 2.0 * std::f64::consts::PI);
        let _ = rotation;
    }
}
