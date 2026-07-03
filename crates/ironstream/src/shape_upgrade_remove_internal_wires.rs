// FILE: shape_upgrade_remove_internal_wires.rs
// occt: ShapeUpgrade_RemoveInternalWires

pub struct ShapeUpgradeRemoveInternalWires;

impl ShapeUpgradeRemoveInternalWires {
    pub fn new() -> Self {
        ShapeUpgradeRemoveInternalWires
    }
}

impl Default for ShapeUpgradeRemoveInternalWires {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeRemoveInternalWires;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeRemoveInternalWires::new();
    }
}
