// FILE: shape_upgrade_wire_divide.rs
// occt: ShapeUpgrade_WireDivide

pub struct ShapeUpgradeWireDivide;

impl ShapeUpgradeWireDivide {
    pub fn new() -> Self {
        ShapeUpgradeWireDivide
    }
}

impl Default for ShapeUpgradeWireDivide {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeWireDivide;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeWireDivide::new();
    }
}
