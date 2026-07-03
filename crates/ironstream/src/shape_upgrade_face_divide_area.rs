// FILE: shape_upgrade_face_divide_area.rs
// occt: ShapeUpgrade_FaceDivideArea

pub struct ShapeUpgradeFaceDivideArea;

impl ShapeUpgradeFaceDivideArea {
    pub fn new() -> Self {
        ShapeUpgradeFaceDivideArea
    }
}

impl Default for ShapeUpgradeFaceDivideArea {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ShapeUpgradeFaceDivideArea;

    #[test]
    fn test_new() {
        let _ = ShapeUpgradeFaceDivideArea::new();
    }
}
