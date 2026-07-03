// FILE: b_rep_builder_api_shape_modification.rs
// occt: BRepBuilderAPI_ShapeModification

pub struct BrepbuilderapiShapemodification;

impl BrepbuilderapiShapemodification {
    pub fn new() -> Self {
        BrepbuilderapiShapemodification
    }
}

impl Default for BrepbuilderapiShapemodification {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepbuilderapiShapemodification::new();
    }
}
