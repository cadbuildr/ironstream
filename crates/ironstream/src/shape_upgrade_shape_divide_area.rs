// FILE: shape_upgrade_shape_divide_area.rs
// occt: ShapeUpgrade_ShapeDivideArea

pub struct ShapeUpgradeShapeDivideArea;

impl ShapeUpgradeShapeDivideArea {
    pub fn new() -> Self {
        ShapeUpgradeShapeDivideArea
    }
}

impl Default for ShapeUpgradeShapeDivideArea {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeShapeDivideArea;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeShapeDivideArea::new();
    }
}
