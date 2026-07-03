// FILE: shape_upgrade_split_surface_area.rs
// occt: ShapeUpgrade_SplitSurfaceArea

pub struct ShapeUpgradeSplitSurfaceArea;

impl ShapeUpgradeSplitSurfaceArea {
    pub fn new() -> Self {
        ShapeUpgradeSplitSurfaceArea
    }
}

impl Default for ShapeUpgradeSplitSurfaceArea {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeSplitSurfaceArea;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeSplitSurfaceArea::new();
    }
}
