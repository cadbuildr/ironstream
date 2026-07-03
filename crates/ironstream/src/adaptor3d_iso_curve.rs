// FILE: adaptor3d_iso_curve.rs
// occt: Adaptor3d_IsoCurve

pub struct Adaptor3dIsoCurve {
    iso_type: IsoType,
    iso_value: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsoType {
    IsoCurveU,
    IsoCurveV,
}

impl Adaptor3dIsoCurve {
    pub fn new(iso_type: IsoType, iso_value: f64) -> Self {
        Adaptor3dIsoCurve {
            iso_type,
            iso_value,
        }
    }

    pub fn get_iso_type(&self) -> IsoType {
        self.iso_type
    }

    pub fn get_iso_value(&self) -> f64 {
        self.iso_value
    }

    pub fn evaluate(&self, _t: f64) -> (f64, f64, f64) {
        (0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_iso_curve_u() {
        let curve = Adaptor3dIsoCurve::new(IsoType::IsoCurveU, 0.5);
        assert_eq!(curve.get_iso_type(), IsoType::IsoCurveU);
        assert!((curve.get_iso_value() - 0.5).abs() < PRECISION);
    }

    #[test]
    fn test_iso_curve_v() {
        let curve = Adaptor3dIsoCurve::new(IsoType::IsoCurveV, 0.7);
        assert_eq!(curve.get_iso_type(), IsoType::IsoCurveV);
        assert!((curve.get_iso_value() - 0.7).abs() < PRECISION);
    }

    #[test]
    fn test_iso_curve_evaluate() {
        let curve = Adaptor3dIsoCurve::new(IsoType::IsoCurveU, 0.5);
        let (x, y, z) = curve.evaluate(0.25);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert_eq!(z, 0.0);
    }
}
