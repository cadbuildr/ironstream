// FILE: vrml_data_cone.rs
// occt: VrmlData_Cone

#[derive(Clone, Debug)]
pub struct VrmlDataCone {
    radius: f64,
    height: f64,
}

impl VrmlDataCone {
    pub fn new(radius: f64, height: f64) -> Self {
        VrmlDataCone { radius, height }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn height(&self) -> f64 {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let c = VrmlDataCone::new(2.0, 5.0);
        assert_eq!(c.radius(), 2.0);
        assert_eq!(c.height(), 5.0);
    }
}
