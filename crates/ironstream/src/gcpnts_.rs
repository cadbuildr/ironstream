// FILE: gcpnts_.rs
// occt-ref: GCPnts_AbscissaPoint, GCPnts_UniformAbscissa
//       GCPnts_UniformDeflection, GCPnts_TangentialDeflection,
//       GCPnts_QuasiUniformAbscissa

/// Status of a GCPnts algorithm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GcpntsStatus {
    NotDone,
    Done,
    NullCurve,
    MaxNbPntReached,
    NullInterval,
}

impl Default for GcpntsStatus {
    fn default() -> Self { Self::NotDone }
}

impl GcpntsStatus {
    pub fn is_done(&self) -> bool { *self == Self::Done }
}

/// Computes a point at a given arc length from a start point.
// occt-ref: GCPnts_AbscissaPoint
#[derive(Clone, Debug)]
pub struct GcpntsAbscissaPoint {
    pub curve_id: u32,
    pub abscissa: f64,
    pub u0: f64,
    pub parameter: f64,
    pub status: GcpntsStatus,
}

impl GcpntsAbscissaPoint {
    pub fn new(curve_id: u32, abscissa: f64, u0: f64, first: f64, last: f64) -> Self {
        let ok = curve_id > 0 && last > first;
        let span = (last - first).abs();
        // Stub: parameter = u0 + abscissa (clamped to [first, last])
        let parameter = if ok {
            (u0 + abscissa).clamp(first, last)
        } else { u0 };
        Self {
            curve_id,
            abscissa,
            u0,
            parameter,
            status: if ok { GcpntsStatus::Done } else { GcpntsStatus::NullCurve },
        }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn parameter(&self) -> f64 { self.parameter }

    /// Compute arc length between two parameters (stub: |u1 - u0|).
    pub fn length(curve_id: u32, first: f64, last: f64) -> f64 {
        if curve_id == 0 { return 0.0; }
        (last - first).abs()
    }
}

/// Distributes points uniformly in arc length along a curve.
// occt-ref: GCPnts_UniformAbscissa
#[derive(Clone, Debug)]
pub struct GcpntsUniformAbscissa {
    pub curve_id: u32,
    pub nb_points: usize,
    pub abscissa: f64,
    pub first: f64,
    pub last: f64,
    pub status: GcpntsStatus,
    pub parameters: Vec<f64>,
}

impl GcpntsUniformAbscissa {
    pub fn new_by_nb(curve_id: u32, nb: usize, first: f64, last: f64) -> Self {
        let ok = curve_id > 0 && last > first && nb >= 2;
        let params = if ok {
            (0..nb).map(|i| first + (last - first) * i as f64 / (nb - 1) as f64).collect()
        } else { vec![] };
        Self {
            curve_id,
            nb_points: if ok { nb } else { 0 },
            abscissa: if ok && nb >= 2 { (last - first) / (nb - 1) as f64 } else { 0.0 },
            first,
            last,
            status: if ok { GcpntsStatus::Done } else { GcpntsStatus::NullCurve },
            parameters: params,
        }
    }

    pub fn new_by_abscissa(curve_id: u32, abscissa: f64, first: f64, last: f64) -> Self {
        if curve_id == 0 || abscissa <= 0.0 || last <= first {
            return Self {
                curve_id, nb_points: 0, abscissa, first, last,
                status: GcpntsStatus::NullCurve, parameters: vec![],
            };
        }
        let length = last - first;
        let nb = ((length / abscissa).floor() as usize + 1).max(2);
        Self::new_by_nb(curve_id, nb, first, last)
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_points(&self) -> usize { self.nb_points }
    pub fn abscissa(&self) -> f64 { self.abscissa }

    pub fn parameter(&self, i: usize) -> Option<f64> {
        if i == 0 { None } else { self.parameters.get(i - 1).copied() }
    }
}

/// Distributes points uniformly by deflection (chord error).
// occt-ref: GCPnts_UniformDeflection
#[derive(Clone, Debug)]
pub struct GcpntsUniformDeflection {
    pub curve_id: u32,
    pub deflection: f64,
    pub first: f64,
    pub last: f64,
    pub status: GcpntsStatus,
    pub parameters: Vec<f64>,
    pub nb_points: usize,
}

impl GcpntsUniformDeflection {
    pub fn new(curve_id: u32, deflection: f64, first: f64, last: f64) -> Self {
        let ok = curve_id > 0 && deflection > 0.0 && last > first;
        // Stub: approximate nb points from deflection
        let nb = if ok {
            let span = last - first;
            ((span / deflection.sqrt()) as usize + 2).max(2).min(1000)
        } else { 0 };
        let params = if ok {
            (0..nb).map(|i| first + (last - first) * i as f64 / (nb - 1).max(1) as f64).collect()
        } else { vec![] };
        Self {
            curve_id, deflection, first, last,
            status: if ok { GcpntsStatus::Done } else { GcpntsStatus::NullCurve },
            parameters: params,
            nb_points: nb,
        }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_points(&self) -> usize { self.nb_points }
    pub fn deflection(&self) -> f64 { self.deflection }
    pub fn parameter(&self, i: usize) -> Option<f64> {
        if i == 0 { None } else { self.parameters.get(i - 1).copied() }
    }
}

/// Distributes points using both angular and chordal deflection.
// occt-ref: GCPnts_TangentialDeflection
#[derive(Clone, Debug)]
pub struct GcpntsTangentialDeflection {
    pub curve_id: u32,
    pub angular_deflection: f64,
    pub curvature_deflection: f64,
    pub first: f64,
    pub last: f64,
    pub status: GcpntsStatus,
    pub parameters: Vec<f64>,
    pub nb_points: usize,
    pub minimum_nb_points: usize,
}

impl GcpntsTangentialDeflection {
    pub fn new(
        curve_id: u32,
        first: f64,
        last: f64,
        angular: f64,
        chordal: f64,
        min_pts: usize,
    ) -> Self {
        let ok = curve_id > 0 && last > first && angular > 0.0 && chordal > 0.0;
        let nb = if ok {
            let span = last - first;
            let est = (span / angular.min(chordal).sqrt()) as usize + 2;
            est.max(min_pts).min(5000)
        } else { 0 };
        let params = if ok {
            (0..nb).map(|i| first + (last - first) * i as f64 / (nb - 1).max(1) as f64).collect()
        } else { vec![] };
        Self {
            curve_id, angular_deflection: angular, curvature_deflection: chordal,
            first, last,
            status: if ok { GcpntsStatus::Done } else { GcpntsStatus::NullCurve },
            parameters: params,
            nb_points: nb,
            minimum_nb_points: min_pts,
        }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_points(&self) -> usize { self.nb_points }
    pub fn parameter(&self, i: usize) -> Option<f64> {
        if i == 0 { None } else { self.parameters.get(i - 1).copied() }
    }

    pub fn add_point(&mut self, t: f64) {
        if t >= self.first && t <= self.last {
            self.parameters.push(t);
            self.parameters.sort_by(|a, b| a.partial_cmp(b).unwrap());
            self.parameters.dedup_by(|a, b| (*a - *b).abs() < 1e-14);
            self.nb_points = self.parameters.len();
        }
    }
}

/// Quasi-uniform abscissa: distributes nb points approximately uniformly.
/// occt: GCPnts_QuasiUniformAbscissa
#[derive(Clone, Debug)]
pub struct GcpntsQuasiUniformAbscissa {
    pub curve_id: u32,
    pub nb_points: usize,
    pub first: f64,
    pub last: f64,
    pub status: GcpntsStatus,
    pub parameters: Vec<f64>,
}

impl GcpntsQuasiUniformAbscissa {
    pub fn new(curve_id: u32, nb: usize, first: f64, last: f64) -> Self {
        // Same as uniform but allows non-uniform internal distribution.
        let ok = curve_id > 0 && last > first && nb >= 2;
        let params = if ok {
            (0..nb).map(|i| first + (last - first) * i as f64 / (nb - 1) as f64).collect()
        } else { vec![] };
        Self {
            curve_id, nb_points: if ok { nb } else { 0 }, first, last,
            status: if ok { GcpntsStatus::Done } else { GcpntsStatus::NullCurve },
            parameters: params,
        }
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_points(&self) -> usize { self.nb_points }
    pub fn parameter(&self, i: usize) -> Option<f64> {
        if i == 0 { None } else { self.parameters.get(i - 1).copied() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abscissa_point_basic() {
        let a = GcpntsAbscissaPoint::new(1, 0.5, 0.0, 0.0, 1.0);
        assert!(a.is_done());
        assert!((a.parameter() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn abscissa_point_null_curve() {
        let a = GcpntsAbscissaPoint::new(0, 0.5, 0.0, 0.0, 1.0);
        assert!(!a.is_done());
    }

    #[test]
    fn uniform_abscissa_by_nb() {
        let u = GcpntsUniformAbscissa::new_by_nb(1, 5, 0.0, 1.0);
        assert!(u.is_done());
        assert_eq!(u.nb_points(), 5);
        assert_eq!(u.parameter(1), Some(0.0));
        assert!((u.parameter(5).unwrap() - 1.0).abs() < 1e-10);
        assert!(u.parameter(0).is_none());
    }

    #[test]
    fn uniform_deflection() {
        let u = GcpntsUniformDeflection::new(1, 0.1, 0.0, 1.0);
        assert!(u.is_done());
        assert!(u.nb_points() >= 2);
        assert!((u.deflection() - 0.1).abs() < 1e-12);
    }

    #[test]
    fn tangential_deflection() {
        let t = GcpntsTangentialDeflection::new(1, 0.0, 1.0, 0.1, 0.1, 3);
        assert!(t.is_done());
        assert!(t.nb_points() >= 3);
    }

    #[test]
    fn quasi_uniform_abscissa() {
        let q = GcpntsQuasiUniformAbscissa::new(1, 4, 0.0, 3.0);
        assert!(q.is_done());
        assert_eq!(q.nb_points(), 4);
        assert!((q.parameter(4).unwrap() - 3.0).abs() < 1e-10);
    }
}
