// FILE: vrml_data_cylinder.rs
// occt: VrmlData_Cylinder

#[derive(Clone, Debug)]
pub struct VrmlDataCylinder {
    radius: f64,
    height: f64,
}

impl VrmlDataCylinder {
    pub fn new(radius: f64, height: f64) -> Self {
        VrmlDataCylinder { radius, height }
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
        let c = VrmlDataCylinder::new(1.5, 4.0);
        assert_eq!(c.radius(), 1.5);
        assert_eq!(c.height(), 4.0);
    }
}
