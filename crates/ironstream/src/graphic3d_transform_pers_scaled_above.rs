// FILE: graphic3d_transform_pers_scaled_above.rs
// occt: Graphic3d_TransformPersScaledAbove

/// Zoom persistence transformation with an upper scale boundary.
///
/// This persistence applies zoom-based transformation only when the camera
/// scale value is below a specified threshold. When the camera scale exceeds
/// the persistence scale, no persistence is applied.
#[derive(Debug, Clone)]
pub struct Graphic3dTransformPersScaledAbove {
    /// The scale threshold value
    scale: f64,
    /// The 3D anchor point for the persistence
    pnt_x: f64,
    pnt_y: f64,
    pnt_z: f64,
}

impl Graphic3dTransformPersScaledAbove {
    /// Creates a zoom transformation persistence with an anchor 3D point
    /// and a scale threshold value.
    pub fn new(scale: f64, pnt_x: f64, pnt_y: f64, pnt_z: f64) -> Self {
        Self {
            scale,
            pnt_x,
            pnt_y,
            pnt_z,
        }
    }

    /// Returns the scale threshold value.
    pub fn scale(&self) -> f64 {
        self.scale
    }

    /// Returns the anchor point coordinates.
    pub fn point(&self) -> (f64, f64, f64) {
        (self.pnt_x, self.pnt_y, self.pnt_z)
    }

    /// Computes the persistent scale based on camera position and viewport dimensions.
    ///
    /// If the camera scale value is less than the persistence scale threshold,
    /// zoom persistence is not applied and this returns 1.0 (identity scale).
    /// Otherwise, it returns the appropriate scale factor for the transformation.
    pub fn persistent_scale(&self, camera_scale: f64, _viewport_width: i32, _viewport_height: i32) -> f64 {
        // If camera scale is less than the persistence scale threshold,
        // no persistence is applied
        if camera_scale < self.scale {
            1.0
        } else {
            // Apply persistence transformation based on camera scale
            // The scale factor increases as the camera scale exceeds the threshold
            camera_scale / self.scale
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_persistence() {
        let pers = Graphic3dTransformPersScaledAbove::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(pers.scale(), 1.0);
        assert_eq!(pers.point(), (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_point_coordinates() {
        let pers = Graphic3dTransformPersScaledAbove::new(2.5, 1.5, 2.0, 3.0);
        let (x, y, z) = pers.point();
        assert_eq!(x, 1.5);
        assert_eq!(y, 2.0);
        assert_eq!(z, 3.0);
    }

    #[test]
    fn test_persistent_scale_below_threshold() {
        let pers = Graphic3dTransformPersScaledAbove::new(5.0, 0.0, 0.0, 0.0);

        // Camera scale below threshold: no persistence applied
        let scale = pers.persistent_scale(2.0, 800, 600);
        assert_eq!(scale, 1.0);
    }

    #[test]
    fn test_persistent_scale_at_threshold() {
        let pers = Graphic3dTransformPersScaledAbove::new(5.0, 0.0, 0.0, 0.0);

        // Camera scale exactly at threshold: minimal persistence
        let scale = pers.persistent_scale(5.0, 800, 600);
        assert_eq!(scale, 1.0);
    }

    #[test]
    fn test_persistent_scale_above_threshold() {
        let pers = Graphic3dTransformPersScaledAbove::new(5.0, 0.0, 0.0, 0.0);

        // Camera scale above threshold: persistence applied
        let scale = pers.persistent_scale(10.0, 800, 600);
        assert_eq!(scale, 2.0); // 10.0 / 5.0
    }

    #[test]
    fn test_persistent_scale_different_values() {
        let pers = Graphic3dTransformPersScaledAbove::new(2.0, 1.0, 1.0, 1.0);

        assert_eq!(pers.persistent_scale(1.0, 640, 480), 1.0); // Below
        assert_eq!(pers.persistent_scale(4.0, 640, 480), 2.0); // Above: 4.0/2.0
        assert_eq!(pers.persistent_scale(8.0, 640, 480), 4.0); // Above: 8.0/2.0
    }

    #[test]
    fn test_clone_persistence() {
        let pers = Graphic3dTransformPersScaledAbove::new(3.0, 2.0, 3.0, 4.0);
        let cloned = pers.clone();

        assert_eq!(cloned.scale(), pers.scale());
        assert_eq!(cloned.point(), pers.point());
    }

    #[test]
    fn test_negative_scale_threshold() {
        let pers = Graphic3dTransformPersScaledAbove::new(-1.0, 0.0, 0.0, 0.0);
        // Negative scale threshold: camera scale will always be above it
        let scale = pers.persistent_scale(5.0, 800, 600);
        assert_eq!(scale, -5.0); // 5.0 / -1.0
    }
}
