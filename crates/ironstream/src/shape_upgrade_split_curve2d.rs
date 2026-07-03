// FILE: shape_upgrade_split_curve2d.rs
// occt: ShapeUpgrade_SplitCurve2d

pub struct ShapeUpgradeSplitCurve2d;

impl ShapeUpgradeSplitCurve2d {
    pub fn new() -> Self {
        ShapeUpgradeSplitCurve2d
    }
}

impl Default for ShapeUpgradeSplitCurve2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeSplitCurve2d;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeSplitCurve2d::new();
    }
}
