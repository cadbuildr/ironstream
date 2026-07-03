// FILE: shape_upgrade_split_surface_angle.rs
// occt: ShapeUpgrade_SplitSurfaceAngle

pub struct ShapeUpgradeSplitSurfaceAngle;

impl ShapeUpgradeSplitSurfaceAngle {
    pub fn new() -> Self {
        ShapeUpgradeSplitSurfaceAngle
    }
}

impl Default for ShapeUpgradeSplitSurfaceAngle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeSplitSurfaceAngle;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeSplitSurfaceAngle::new();
    }
}
