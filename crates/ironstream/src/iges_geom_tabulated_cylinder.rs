// FILE: iges_geom_tabulated_cylinder.rs
// occt: IGESGeom_TabulatedCylinder

/// Defines IGESTabulatedCylinder, Type <122> in package IGESGeom.
#[derive(Clone, Debug)]
pub struct TabulatedCylinder {
    entity_type: i32,
}

impl TabulatedCylinder {
    pub fn new() -> Self {
        TabulatedCylinder { entity_type: 122 }
    }

    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }
}

impl Default for TabulatedCylinder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cyl = TabulatedCylinder::new();
        assert_eq!(cyl.entity_type(), 122);
    }
}
