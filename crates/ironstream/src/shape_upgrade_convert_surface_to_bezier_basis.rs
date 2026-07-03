// FILE: shape_upgrade_convert_surface_to_bezier_basis.rs
// occt: ShapeUpgrade_ConvertSurfaceToBezierBasis

pub struct ShapeUpgradeConvertSurfaceToBezierBasis;

impl ShapeUpgradeConvertSurfaceToBezierBasis {
    pub fn new() -> Self {
        ShapeUpgradeConvertSurfaceToBezierBasis
    }
}

impl Default for ShapeUpgradeConvertSurfaceToBezierBasis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeConvertSurfaceToBezierBasis;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeConvertSurfaceToBezierBasis::new();
    }
}
