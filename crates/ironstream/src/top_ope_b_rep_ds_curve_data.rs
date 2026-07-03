// FILE: top_ope_b_rep_ds_curve_data.rs
// occt: TopOpeBRepDS_CurveData

use crate::top_ope_b_rep_ds_curve::Curve;
use crate::top_ope_b_rep_ds_geometry_data::GeometryData;

/// Curve data: extends GeometryData with a Curve
#[derive(Debug, Clone)]
pub struct CurveData {
    /// Base geometry data containing interferences
    geometry_data: GeometryData,
    /// The curve
    curve: Curve,
}

impl CurveData {
    /// Create new CurveData
    pub fn new() -> Self {
        CurveData {
            geometry_data: GeometryData::new(),
            curve: Curve::new(),
        }
    }

    /// Create from a Curve
    pub fn from_curve(curve: Curve) -> Self {
        CurveData {
            geometry_data: GeometryData::new(),
            curve,
        }
    }

    /// Get geometry data
    pub fn geometry_data(&self) -> &GeometryData {
        &self.geometry_data
    }

    /// Get mutable geometry data
    pub fn geometry_data_mut(&mut self) -> &mut GeometryData {
        &mut self.geometry_data
    }

    /// Get curve
    pub fn curve(&self) -> &Curve {
        &self.curve
    }

    /// Get mutable curve
    pub fn curve_mut(&mut self) -> &mut Curve {
        &mut self.curve
    }
}

impl Default for CurveData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_data_new() {
        let cd = CurveData::new();
        assert_eq!(cd.geometry_data().interferences().len(), 0);
        assert_eq!(cd.curve().tolerance(), 0.0);
    }

    #[test]
    fn test_curve_data_from_curve() {
        let c = Curve::with_curve(42, 0.5, true);
        let cd = CurveData::from_curve(c);
        assert_eq!(cd.curve().curve(), Some(42));
        assert_eq!(cd.curve().tolerance(), 0.5);
    }

    #[test]
    fn test_curve_data_mut() {
        let mut cd = CurveData::new();
        cd.geometry_data_mut().add_interference(1);
        cd.curve_mut().set_tolerance(0.75);
        assert_eq!(cd.geometry_data().interferences().len(), 1);
        assert_eq!(cd.curve().tolerance(), 0.75);
    }
}
