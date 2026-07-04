// FILE: std_object_gp_trsfs.rs
// occt: StdObject_gp_Trsfs

/// Persistent representation of geometric transformations
#[derive(Clone, Debug)]
pub struct GpTrsf {
    scale: f64,
    translation: [f64; 3],
    rotation: [[f64; 3]; 3],
}

impl GpTrsf {
    /// Create a new transformation
    pub fn new() -> Self {
        GpTrsf {
            scale: 1.0,
            translation: [0.0, 0.0, 0.0],
            rotation: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    /// Create an identity transformation
    pub fn identity() -> Self {
        Self::new()
    }

    /// Get the scale
    pub fn scale(&self) -> f64 {
        self.scale
    }

    /// Set the scale
    pub fn set_scale(&mut self, s: f64) {
        self.scale = s;
    }

    /// Get translation
    pub fn translation(&self) -> &[f64; 3] {
        &self.translation
    }

    /// Set translation
    pub fn set_translation(&mut self, trans: [f64; 3]) {
        self.translation = trans;
    }

    /// Get rotation matrix
    pub fn rotation(&self) -> &[[f64; 3]; 3] {
        &self.rotation
    }

    /// Set rotation matrix
    pub fn set_rotation(&mut self, rot: [[f64; 3]; 3]) {
        self.rotation = rot;
    }
}

impl Default for GpTrsf {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let trsf = GpTrsf::new();
        assert_eq!(trsf.scale(), 1.0);
        assert_eq!(*trsf.translation(), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_identity() {
        let trsf = GpTrsf::identity();
        assert_eq!(trsf.scale(), 1.0);
    }

    #[test]
    fn test_set_scale() {
        let mut trsf = GpTrsf::new();
        trsf.set_scale(2.0);
        assert_eq!(trsf.scale(), 2.0);
    }

    #[test]
    fn test_set_translation() {
        let mut trsf = GpTrsf::new();
        trsf.set_translation([1.0, 2.0, 3.0]);
        assert_eq!(*trsf.translation(), [1.0, 2.0, 3.0]);
    }
}
