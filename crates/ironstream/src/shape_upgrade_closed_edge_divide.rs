// FILE: shape_upgrade_closed_edge_divide.rs
// occt: ShapeUpgrade_ClosedEdgeDivide

/// Divides closed edges with maximal segment length
pub struct ShapeUpgradeClosedEdgeDivide;

impl ShapeUpgradeClosedEdgeDivide {
    pub fn new() -> Self {
        ShapeUpgradeClosedEdgeDivide
    }
}

impl Default for ShapeUpgradeClosedEdgeDivide {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeClosedEdgeDivide;

    #[test]
    fn test_new() {
        let _divider = ShapeUpgradeClosedEdgeDivide::new();
    }
}
