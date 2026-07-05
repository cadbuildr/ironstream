// FILE: vrml_data_geometry.rs
// occt: VrmlData_Geometry

#[derive(Clone, Debug)]
pub struct VrmlDataGeometry {
    geom_type: String,
}

impl VrmlDataGeometry {
    pub fn new(geom_type: &str) -> Self {
        VrmlDataGeometry {
            geom_type: geom_type.to_string(),
        }
    }

    pub fn geom_type(&self) -> &str {
        &self.geom_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let g = VrmlDataGeometry::new("box");
        assert_eq!(g.geom_type(), "box");
    }
}
