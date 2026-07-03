// FILE: shape_upgrade_split_surface_continuity.rs
// occt: ShapeUpgrade_SplitSurfaceContinuity

pub struct ShapeUpgradeSplitSurfaceContinuity;

impl ShapeUpgradeSplitSurfaceContinuity {
    pub fn new() -> Self {
        ShapeUpgradeSplitSurfaceContinuity
    }
}

impl Default for ShapeUpgradeSplitSurfaceContinuity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeSplitSurfaceContinuity;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeSplitSurfaceContinuity::new();
    }
}
