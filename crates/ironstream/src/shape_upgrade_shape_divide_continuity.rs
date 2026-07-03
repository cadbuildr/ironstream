// FILE: shape_upgrade_shape_divide_continuity.rs
// occt: ShapeUpgrade_ShapeDivideContinuity

pub struct ShapeUpgradeShapeDivideContinuity;

impl ShapeUpgradeShapeDivideContinuity {
    pub fn new() -> Self {
        ShapeUpgradeShapeDivideContinuity
    }
}

impl Default for ShapeUpgradeShapeDivideContinuity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeShapeDivideContinuity;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeShapeDivideContinuity::new();
    }
}
