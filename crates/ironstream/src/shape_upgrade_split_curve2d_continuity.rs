// FILE: shape_upgrade_split_curve2d_continuity.rs
// occt: ShapeUpgrade_SplitCurve2dContinuity

pub struct ShapeUpgradeSplitCurve2dContinuity;

impl ShapeUpgradeSplitCurve2dContinuity {
    pub fn new() -> Self {
        ShapeUpgradeSplitCurve2dContinuity
    }
}

impl Default for ShapeUpgradeSplitCurve2dContinuity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeSplitCurve2dContinuity;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeSplitCurve2dContinuity::new();
    }
}
