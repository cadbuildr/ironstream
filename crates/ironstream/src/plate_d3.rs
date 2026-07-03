// FILE: plate_d3.rs
// occt: Plate_D3

use crate::plate_d1::XYZ;

/// Plate_D3: Define order 3 derivatives of a 3D-valued function of a 2D variable
#[derive(Clone, Copy, Debug)]
pub struct PlateD3 {
    pub duuu: XYZ,
    pub duuv: XYZ,
    pub duvv: XYZ,
    pub dvvv: XYZ,
}

impl PlateD3 {
    pub fn new(duuu: XYZ, duuv: XYZ, duvv: XYZ, dvvv: XYZ) -> Self {
        PlateD3 {
            duuu,
            duuv,
            duvv,
            dvvv,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plate_d3_creation() {
        let duuu = XYZ::new(1.0, 0.0, 0.0);
        let duuv = XYZ::new(0.0, 1.0, 0.0);
        let duvv = XYZ::new(0.0, 0.0, 1.0);
        let dvvv = XYZ::new(1.0, 1.0, 1.0);
        let d3 = PlateD3::new(duuu, duuv, duvv, dvvv);
        assert_eq!(d3.duuu.x, 1.0);
        assert_eq!(d3.duuv.y, 1.0);
    }

    #[test]
    fn test_plate_d3_clone() {
        let duuu = XYZ::new(1.0, 2.0, 3.0);
        let duuv = XYZ::new(4.0, 5.0, 6.0);
        let duvv = XYZ::new(7.0, 8.0, 9.0);
        let dvvv = XYZ::new(10.0, 11.0, 12.0);
        let d3 = PlateD3::new(duuu, duuv, duvv, dvvv);
        let d3_copy = d3;
        assert_eq!(d3_copy.dvvv.z, 12.0);
    }
}
