// FILE: shape_upgrade_convert_curve2d_to_bezier.rs
// occt: ShapeUpgrade_ConvertCurve2dToBezier

pub struct ShapeUpgradeConvertCurve2dToBezier;

impl ShapeUpgradeConvertCurve2dToBezier {
    pub fn new() -> Self {
        ShapeUpgradeConvertCurve2dToBezier
    }
}

impl Default for ShapeUpgradeConvertCurve2dToBezier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeConvertCurve2dToBezier;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeConvertCurve2dToBezier::new();
    }
}
