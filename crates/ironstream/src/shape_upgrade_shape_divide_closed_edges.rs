// FILE: shape_upgrade_shape_divide_closed_edges.rs
// occt: ShapeUpgrade_ShapeDivideClosedEdges

pub struct ShapeUpgradeShapeDivideClosedEdges;

impl ShapeUpgradeShapeDivideClosedEdges {
    pub fn new() -> Self {
        ShapeUpgradeShapeDivideClosedEdges
    }
}

impl Default for ShapeUpgradeShapeDivideClosedEdges {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeShapeDivideClosedEdges;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeShapeDivideClosedEdges::new();
    }
}
