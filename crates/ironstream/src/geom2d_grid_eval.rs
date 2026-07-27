// FILE: geom2d_grid_eval.rs
// occt: Geom2dGridEval
// occt: Geom2dGridEval // ::CurveD1
// occt: Geom2dGridEval // ::CurveD2
// occt: Geom2dGridEval // ::CurveD3

/// Result structure for curve D1 evaluation (point and first derivative).
#[derive(Clone, Copy, Debug)]
pub struct CurveD1 {
    pub point_x: f64,
    pub point_y: f64,
    pub d1_x: f64,
    pub d1_y: f64,
}

impl CurveD1 {
    pub fn new(point_x: f64, point_y: f64, d1_x: f64, d1_y: f64) -> Self {
        CurveD1 {
            point_x,
            point_y,
            d1_x,
            d1_y,
        }
    }

    pub fn zero() -> Self {
        CurveD1 {
            point_x: 0.0,
            point_y: 0.0,
            d1_x: 0.0,
            d1_y: 0.0,
        }
    }
}

impl Default for CurveD1 {
    fn default() -> Self {
        CurveD1::zero()
    }
}

/// Result structure for curve D2 evaluation (point and first two derivatives).
#[derive(Clone, Copy, Debug)]
pub struct CurveD2 {
    pub point_x: f64,
    pub point_y: f64,
    pub d1_x: f64,
    pub d1_y: f64,
    pub d2_x: f64,
    pub d2_y: f64,
}

impl CurveD2 {
    pub fn new(
        point_x: f64,
        point_y: f64,
        d1_x: f64,
        d1_y: f64,
        d2_x: f64,
        d2_y: f64,
    ) -> Self {
        CurveD2 {
            point_x,
            point_y,
            d1_x,
            d1_y,
            d2_x,
            d2_y,
        }
    }

    pub fn zero() -> Self {
        CurveD2 {
            point_x: 0.0,
            point_y: 0.0,
            d1_x: 0.0,
            d1_y: 0.0,
            d2_x: 0.0,
            d2_y: 0.0,
        }
    }

    pub fn from_d1(d1: CurveD1) -> Self {
        CurveD2 {
            point_x: d1.point_x,
            point_y: d1.point_y,
            d1_x: d1.d1_x,
            d1_y: d1.d1_y,
            d2_x: 0.0,
            d2_y: 0.0,
        }
    }
}

impl Default for CurveD2 {
    fn default() -> Self {
        CurveD2::zero()
    }
}

/// Result structure for curve D3 evaluation (point and first three derivatives).
#[derive(Clone, Copy, Debug)]
pub struct CurveD3 {
    pub point_x: f64,
    pub point_y: f64,
    pub d1_x: f64,
    pub d1_y: f64,
    pub d2_x: f64,
    pub d2_y: f64,
    pub d3_x: f64,
    pub d3_y: f64,
}

impl CurveD3 {
    pub fn new(
        point_x: f64,
        point_y: f64,
        d1_x: f64,
        d1_y: f64,
        d2_x: f64,
        d2_y: f64,
        d3_x: f64,
        d3_y: f64,
    ) -> Self {
        CurveD3 {
            point_x,
            point_y,
            d1_x,
            d1_y,
            d2_x,
            d2_y,
            d3_x,
            d3_y,
        }
    }

    pub fn zero() -> Self {
        CurveD3 {
            point_x: 0.0,
            point_y: 0.0,
            d1_x: 0.0,
            d1_y: 0.0,
            d2_x: 0.0,
            d2_y: 0.0,
            d3_x: 0.0,
            d3_y: 0.0,
        }
    }

    pub fn from_d1(d1: CurveD1) -> Self {
        CurveD3 {
            point_x: d1.point_x,
            point_y: d1.point_y,
            d1_x: d1.d1_x,
            d1_y: d1.d1_y,
            d2_x: 0.0,
            d2_y: 0.0,
            d3_x: 0.0,
            d3_y: 0.0,
        }
    }

    pub fn from_d2(d2: CurveD2) -> Self {
        CurveD3 {
            point_x: d2.point_x,
            point_y: d2.point_y,
            d1_x: d2.d1_x,
            d1_y: d2.d1_y,
            d2_x: d2.d2_x,
            d2_y: d2.d2_y,
            d3_x: 0.0,
            d3_y: 0.0,
        }
    }
}

impl Default for CurveD3 {
    fn default() -> Self {
        CurveD3::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_curve_d1_new() {
        let d1 = CurveD1::new(1.0, 2.0, 3.0, 4.0);
        assert!((d1.point_x - 1.0).abs() < PRECISION);
        assert!((d1.point_y - 2.0).abs() < PRECISION);
        assert!((d1.d1_x - 3.0).abs() < PRECISION);
        assert!((d1.d1_y - 4.0).abs() < PRECISION);
    }

    #[test]
    fn test_curve_d1_zero() {
        let d1 = CurveD1::zero();
        assert!(d1.point_x.abs() < PRECISION);
        assert!(d1.point_y.abs() < PRECISION);
        assert!(d1.d1_x.abs() < PRECISION);
        assert!(d1.d1_y.abs() < PRECISION);
    }

    #[test]
    fn test_curve_d1_default() {
        let d1 = CurveD1::default();
        assert!(d1.point_x.abs() < PRECISION);
    }

    #[test]
    fn test_curve_d2_new() {
        let d2 = CurveD2::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        assert!((d2.point_x - 1.0).abs() < PRECISION);
        assert!((d2.point_y - 2.0).abs() < PRECISION);
        assert!((d2.d1_x - 3.0).abs() < PRECISION);
        assert!((d2.d1_y - 4.0).abs() < PRECISION);
        assert!((d2.d2_x - 5.0).abs() < PRECISION);
        assert!((d2.d2_y - 6.0).abs() < PRECISION);
    }

    #[test]
    fn test_curve_d2_from_d1() {
        let d1 = CurveD1::new(1.0, 2.0, 3.0, 4.0);
        let d2 = CurveD2::from_d1(d1);
        assert!((d2.point_x - 1.0).abs() < PRECISION);
        assert!((d2.d1_x - 3.0).abs() < PRECISION);
        assert!(d2.d2_x.abs() < PRECISION);
    }

    #[test]
    fn test_curve_d3_new() {
        let d3 = CurveD3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        assert!((d3.point_x - 1.0).abs() < PRECISION);
        assert!((d3.d1_x - 3.0).abs() < PRECISION);
        assert!((d3.d2_x - 5.0).abs() < PRECISION);
        assert!((d3.d3_x - 7.0).abs() < PRECISION);
    }

    #[test]
    fn test_curve_d3_from_d1() {
        let d1 = CurveD1::new(1.0, 2.0, 3.0, 4.0);
        let d3 = CurveD3::from_d1(d1);
        assert!((d3.point_x - 1.0).abs() < PRECISION);
        assert!((d3.d1_x - 3.0).abs() < PRECISION);
        assert!(d3.d2_x.abs() < PRECISION);
        assert!(d3.d3_x.abs() < PRECISION);
    }

    #[test]
    fn test_curve_d3_from_d2() {
        let d2 = CurveD2::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let d3 = CurveD3::from_d2(d2);
        assert!((d3.point_x - 1.0).abs() < PRECISION);
        assert!((d3.d1_x - 3.0).abs() < PRECISION);
        assert!((d3.d2_x - 5.0).abs() < PRECISION);
        assert!(d3.d3_x.abs() < PRECISION);
    }
}
