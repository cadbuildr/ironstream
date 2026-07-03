// FILE: shape_upgrade_split_curve3d_continuity.rs
// occt: ShapeUpgrade_SplitCurve3dContinuity

pub struct ShapeUpgradeSplitCurve3dContinuity;

impl ShapeUpgradeSplitCurve3dContinuity {
    pub fn new() -> Self {
        ShapeUpgradeSplitCurve3dContinuity
    }
}

impl Default for ShapeUpgradeSplitCurve3dContinuity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeSplitCurve3dContinuity;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeSplitCurve3dContinuity::new();
    }
}
