// FILE: shape_fix_fix_small_solid.rs
// occt: ShapeFix_FixSmallSolid

/// Fixing solids with small size
pub struct ShapeFixFixSmallSolid {
    fix_mode: i32,
    volume_threshold: f64,
    width_factor_threshold: f64,
}

impl ShapeFixFixSmallSolid {
    /// Construct
    pub fn new() -> Self {
        ShapeFixFixSmallSolid {
            fix_mode: 0,
            volume_threshold: -1.0,
            width_factor_threshold: -1.0,
        }
    }

    /// Set working mode:
    /// - 0: use both WidthFactorThreshold and VolumeThreshold
    /// - 1: use only WidthFactorThreshold
    /// - 2: use only VolumeThreshold
    pub fn set_fix_mode(&mut self, mode: i32) {
        self.fix_mode = mode;
    }

    /// Set or clear volume threshold for small solids
    /// Pass -1.0 to disable
    pub fn set_volume_threshold(&mut self, threshold: f64) {
        self.volume_threshold = threshold;
    }

    /// Set or clear width factor threshold for small solids
    /// Pass -1.0 to disable
    pub fn set_width_factor_threshold(&mut self, threshold: f64) {
        self.width_factor_threshold = threshold;
    }

    /// Remove small solids from the given shape
    pub fn remove(&self, _shape: &str) -> String {
        // Placeholder for small solid removal
        String::from("result_shape")
    }

    /// Merge small solids to adjacent non-small ones
    pub fn merge(&self, _shape: &str) -> String {
        // Placeholder for small solid merging
        String::from("merged_shape")
    }

    /// Check if thresholds are set
    fn is_thresholds_set(&self) -> bool {
        self.volume_threshold >= 0.0 || self.width_factor_threshold >= 0.0
    }

    /// Check if a solid is small
    fn is_small(&self, _solid: &str) -> bool {
        // Placeholder for small solid checking
        false
    }

    /// Check if width factor threshold is being used
    fn is_used_width_factor_threshold(&self) -> bool {
        (self.fix_mode == 0 || self.fix_mode == 1) && self.width_factor_threshold >= 0.0
    }

    /// Check if volume threshold is being used
    fn is_used_volume_threshold(&self) -> bool {
        (self.fix_mode == 0 || self.fix_mode == 2) && self.volume_threshold >= 0.0
    }
}

impl Default for ShapeFixFixSmallSolid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeFixFixSmallSolid;

    #[test]
    fn test_new() {
        let fixer = ShapeFixFixSmallSolid::new();
        assert_eq!(fixer.fix_mode, 0);
        assert_eq!(fixer.volume_threshold, -1.0);
    }

    #[test]
    fn test_set_fix_mode() {
        let mut fixer = ShapeFixFixSmallSolid::new();
        fixer.set_fix_mode(1);
        assert_eq!(fixer.fix_mode, 1);
    }

    #[test]
    fn test_set_volume_threshold() {
        let mut fixer = ShapeFixFixSmallSolid::new();
        fixer.set_volume_threshold(0.1);
        assert_eq!(fixer.volume_threshold, 0.1);
    }

    #[test]
    fn test_set_width_factor_threshold() {
        let mut fixer = ShapeFixFixSmallSolid::new();
        fixer.set_width_factor_threshold(0.5);
        assert_eq!(fixer.width_factor_threshold, 0.5);
    }

    #[test]
    fn test_remove() {
        let fixer = ShapeFixFixSmallSolid::new();
        let result = fixer.remove("test_solid");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_merge() {
        let fixer = ShapeFixFixSmallSolid::new();
        let result = fixer.merge("test_solid");
        assert!(!result.is_empty());
    }
}
