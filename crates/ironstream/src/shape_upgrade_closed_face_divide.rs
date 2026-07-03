// FILE: shape_upgrade_closed_face_divide.rs
// occt: ShapeUpgrade_ClosedFaceDivide

pub struct ShapeUpgradeClosedFaceDivide;

impl ShapeUpgradeClosedFaceDivide {
    pub fn new() -> Self {
        ShapeUpgradeClosedFaceDivide
    }
}

impl Default for ShapeUpgradeClosedFaceDivide {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeClosedFaceDivide;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeClosedFaceDivide::new();
    }
}
