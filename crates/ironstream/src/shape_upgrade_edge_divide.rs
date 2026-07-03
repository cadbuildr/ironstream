// FILE: shape_upgrade_edge_divide.rs
// occt: ShapeUpgrade_EdgeDivide

pub struct ShapeUpgradeEdgeDivide;

impl ShapeUpgradeEdgeDivide {
    pub fn new() -> Self {
        ShapeUpgradeEdgeDivide
    }
}

impl Default for ShapeUpgradeEdgeDivide {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeEdgeDivide;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeEdgeDivide::new();
    }
}
