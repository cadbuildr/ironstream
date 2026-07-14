// FILE: gcpnts.rs

// occt-ref: GCPnts_AbscissaPoint // distribution enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCPntsDistribution {
    Uniform,
    Curvature,
    Combined,
}

// occt-ref: GCPnts_AbscissaPoint
pub struct GCPntsAbscissaPoint {
    parameter: f64,
    is_done: bool,
}

impl GCPntsAbscissaPoint {
    pub fn new(abscissa: f64, u0: f64, _tolerance: f64) -> Self {
        Self {
            parameter: u0 + abscissa,
            is_done: true,
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn parameter(&self) -> Option<f64> {
        if self.is_done {
            Some(self.parameter)
        } else {
            None
        }
    }
}

// occt-ref: GCPnts_UniformAbscissa
pub struct GCPntsUniformAbscissa {
    params: Vec<f64>,
    is_done: bool,
}

impl GCPntsUniformAbscissa {
    pub fn new(first: f64, last: f64, nb_points: usize) -> Self {
        let params = if nb_points == 0 {
            vec![]
        } else if nb_points == 1 {
            vec![first]
        } else {
            (0..nb_points)
                .map(|i| first + (last - first) * (i as f64) / ((nb_points - 1) as f64))
                .collect()
        };
        Self {
            is_done: !params.is_empty(),
            params,
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_points(&self) -> usize {
        self.params.len()
    }

    // 1-based index
    pub fn parameter(&self, idx: usize) -> Option<f64> {
        if idx == 0 {
            return None;
        }
        self.params.get(idx - 1).copied()
    }
}

// occt-ref: GCPnts_UniformDeflection
pub struct GCPntsUniformDeflection {
    params: Vec<f64>,
    is_done: bool,
    deflection: f64,
}

impl GCPntsUniformDeflection {
    pub fn new(first: f64, last: f64, deflection: f64) -> Self {
        let span = last - first;
        let n_intervals = if deflection > 0.0 {
            (span / deflection).ceil() as usize
        } else {
            0
        };
        let nb_points = n_intervals + 1;
        let params: Vec<f64> = if n_intervals == 0 {
            vec![]
        } else {
            (0..nb_points)
                .map(|i| first + span * (i as f64) / (n_intervals as f64))
                .collect()
        };
        Self {
            is_done: !params.is_empty(),
            deflection,
            params,
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn deflection(&self) -> f64 {
        self.deflection
    }

    pub fn nb_points(&self) -> usize {
        self.params.len()
    }

    // 1-based index
    pub fn parameter(&self, idx: usize) -> Option<f64> {
        if idx == 0 {
            return None;
        }
        self.params.get(idx - 1).copied()
    }
}

// occt-ref: GCPnts_TangentialDeflection
pub struct GCPntsTangentialDeflection {
    params: Vec<f64>,
    is_done: bool,
    angular_deflection: f64,
    curvature_deflection: f64,
}

impl GCPntsTangentialDeflection {
    pub fn new(first: f64, last: f64, angular: f64, curvature: f64) -> Self {
        // Use the smaller of the two deflection values to determine step size,
        // so both angular and curvature constraints are satisfied.
        let effective = angular.min(curvature);
        let span = last - first;
        let n_intervals = if effective > 0.0 {
            (span / effective).ceil() as usize
        } else {
            0
        };
        let nb_points = n_intervals + 1;
        let params: Vec<f64> = if n_intervals == 0 {
            vec![]
        } else {
            (0..nb_points)
                .map(|i| first + span * (i as f64) / (n_intervals as f64))
                .collect()
        };
        Self {
            is_done: !params.is_empty(),
            angular_deflection: angular,
            curvature_deflection: curvature,
            params,
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn nb_points(&self) -> usize {
        self.params.len()
    }

    // 1-based index
    pub fn parameter(&self, idx: usize) -> Option<f64> {
        if idx == 0 {
            return None;
        }
        self.params.get(idx - 1).copied()
    }

    pub fn angular_deflection(&self) -> f64 {
        self.angular_deflection
    }

    pub fn curvature_deflection(&self) -> f64 {
        self.curvature_deflection
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abscissa_point_linear_stub() {
        let ap = GCPntsAbscissaPoint::new(0.3, 0.5, 1e-7);
        assert!(ap.is_done());
        let p = ap.parameter().unwrap();
        assert!((p - 0.8).abs() < 1e-12);
    }

    #[test]
    fn abscissa_point_negative_abscissa() {
        let ap = GCPntsAbscissaPoint::new(-0.2, 1.0, 1e-7);
        assert_eq!(ap.parameter(), Some(0.8));
    }

    #[test]
    fn uniform_abscissa_5_points() {
        let ua = GCPntsUniformAbscissa::new(0.0, 1.0, 5);
        assert!(ua.is_done());
        assert_eq!(ua.nb_points(), 5);
        assert!((ua.parameter(1).unwrap() - 0.0).abs() < 1e-12);
        assert!((ua.parameter(3).unwrap() - 0.5).abs() < 1e-12);
        assert!((ua.parameter(5).unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn uniform_abscissa_single_point() {
        let ua = GCPntsUniformAbscissa::new(0.0, 1.0, 1);
        assert!(ua.is_done());
        assert_eq!(ua.nb_points(), 1);
        assert!((ua.parameter(1).unwrap() - 0.0).abs() < 1e-12);
    }

    #[test]
    fn uniform_abscissa_zero_points() {
        let ua = GCPntsUniformAbscissa::new(0.0, 1.0, 0);
        assert!(!ua.is_done());
        assert_eq!(ua.nb_points(), 0);
        assert!(ua.parameter(1).is_none());
    }

    #[test]
    fn uniform_abscissa_oob_index() {
        let ua = GCPntsUniformAbscissa::new(0.0, 1.0, 3);
        assert!(ua.parameter(0).is_none());
        assert!(ua.parameter(4).is_none());
    }

    #[test]
    fn uniform_deflection_basic() {
        // deflection 0.5 over [0,1] => 2 intervals => 3 points
        let ud = GCPntsUniformDeflection::new(0.0, 1.0, 0.5);
        assert!(ud.is_done());
        assert_eq!(ud.nb_points(), 3);
        assert!((ud.parameter(1).unwrap() - 0.0).abs() < 1e-12);
        assert!((ud.parameter(2).unwrap() - 0.5).abs() < 1e-12);
        assert!((ud.parameter(3).unwrap() - 1.0).abs() < 1e-12);
        assert!((ud.deflection() - 0.5).abs() < 1e-12);
    }

    #[test]
    fn uniform_deflection_zero_deflection() {
        let ud = GCPntsUniformDeflection::new(0.0, 1.0, 0.0);
        assert!(!ud.is_done());
        assert_eq!(ud.nb_points(), 0);
    }

    #[test]
    fn tangential_deflection_basic() {
        let td = GCPntsTangentialDeflection::new(0.0, 1.0, 0.5, 0.25);
        assert!(td.is_done());
        // effective = 0.25, ceil(1.0/0.25) = 4 intervals => 5 points
        assert_eq!(td.nb_points(), 5);
        assert!((td.angular_deflection() - 0.5).abs() < 1e-12);
        assert!((td.curvature_deflection() - 0.25).abs() < 1e-12);
    }

    #[test]
    fn tangential_deflection_endpoints() {
        let td = GCPntsTangentialDeflection::new(2.0, 4.0, 0.5, 0.5);
        assert!(td.is_done());
        let n = td.nb_points();
        assert!((td.parameter(1).unwrap() - 2.0).abs() < 1e-12);
        assert!((td.parameter(n).unwrap() - 4.0).abs() < 1e-12);
    }

    #[test]
    fn distribution_enum_eq() {
        assert_eq!(GCPntsDistribution::Uniform, GCPntsDistribution::Uniform);
        assert_ne!(GCPntsDistribution::Curvature, GCPntsDistribution::Combined);
    }
}
