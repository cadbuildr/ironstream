// FILE: shape_upgrade_fix_small_curves.rs
// occt: ShapeUpgrade_FixSmallCurves

pub struct ShapeUpgradeFixSmallCurves;

impl ShapeUpgradeFixSmallCurves {
    pub fn new() -> Self {
        ShapeUpgradeFixSmallCurves
    }
}

impl Default for ShapeUpgradeFixSmallCurves {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeFixSmallCurves;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeFixSmallCurves::new();
    }
}
