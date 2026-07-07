// FILE: vrml_data_coordinate.rs
// occt: VrmlData_Coordinate

#[derive(Clone, Debug)]
pub struct VrmlDataCoordinate {
    x: f64,
    y: f64,
    z: f64,
}

impl VrmlDataCoordinate {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        VrmlDataCoordinate { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let c = VrmlDataCoordinate::new(1.0, 2.0, 3.0);
        assert_eq!(c.x(), 1.0);
        assert_eq!(c.y(), 2.0);
        assert_eq!(c.z(), 3.0);
    }
}
