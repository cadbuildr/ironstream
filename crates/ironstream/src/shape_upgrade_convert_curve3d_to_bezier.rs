// FILE: shape_upgrade_convert_curve3d_to_bezier.rs
// occt: ShapeUpgrade_ConvertCurve3dToBezier

pub struct ShapeUpgradeConvertCurve3dToBezier;

impl ShapeUpgradeConvertCurve3dToBezier {
    pub fn new() -> Self {
        ShapeUpgradeConvertCurve3dToBezier
    }
}

impl Default for ShapeUpgradeConvertCurve3dToBezier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeConvertCurve3dToBezier;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeConvertCurve3dToBezier::new();
    }
}
