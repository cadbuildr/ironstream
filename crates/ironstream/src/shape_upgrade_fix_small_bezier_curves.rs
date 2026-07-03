// FILE: shape_upgrade_fix_small_bezier_curves.rs
// occt: ShapeUpgrade_FixSmallBezierCurves

pub struct ShapeUpgradeFixSmallBezierCurves;

impl ShapeUpgradeFixSmallBezierCurves {
    pub fn new() -> Self {
        ShapeUpgradeFixSmallBezierCurves
    }
}

impl Default for ShapeUpgradeFixSmallBezierCurves {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeFixSmallBezierCurves;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeFixSmallBezierCurves::new();
    }
}
