// FILE: plate_d2.rs
// occt: Plate_D2

use crate::plate_d1::XYZ;

/// Plate_D2: Define order 2 derivatives of a 3D-valued function of a 2D variable
#[derive(Clone, Copy, Debug)]
pub struct PlateD2 {
    pub duu: XYZ,
    pub duv: XYZ,
    pub dvv: XYZ,
}

impl PlateD2 {
    pub fn new(duu: XYZ, duv: XYZ, dvv: XYZ) -> Self {
        PlateD2 { duu, duv, dvv }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plate_d2_creation() {
        let duu = XYZ::new(1.0, 0.0, 0.0);
        let duv = XYZ::new(0.0, 1.0, 0.0);
        let dvv = XYZ::new(0.0, 0.0, 1.0);
        let d2 = PlateD2::new(duu, duv, dvv);
        assert_eq!(d2.duu.x, 1.0);
        assert_eq!(d2.duv.y, 1.0);
        assert_eq!(d2.dvv.z, 1.0);
    }

    #[test]
    fn test_plate_d2_clone() {
        let duu = XYZ::new(1.0, 2.0, 3.0);
        let duv = XYZ::new(4.0, 5.0, 6.0);
        let dvv = XYZ::new(7.0, 8.0, 9.0);
        let d2 = PlateD2::new(duu, duv, dvv);
        let d2_copy = d2;
        assert_eq!(d2_copy.duu.x, 1.0);
    }
}
