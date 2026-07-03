// FILE: chord_dev.rs
// occt: GCPnts_AbscissaPoint, GCPnts_UniformAbscissa, GCPnts_TangentialDeflection

/// Algorithm for chord-deviation point sampling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChordAlgoType {
    AbscissaPoint,
    UniformAbscissa,
    TangentialDeflection,
    UniformDeflection,
}

// occt: GCPnts_AbscissaPoint — point at a given curvilinear abscissa
#[derive(Clone, Debug)]
pub struct GcpntsAbscissaPoint {
    pub curve_id: u32,
    pub abscissa: f64,
    pub from_param: f64,
    pub to_param: f64,
    pub result_param: f64,
    pub is_done: bool,
}

impl GcpntsAbscissaPoint {
    pub fn new(curve_id: u32, abscissa: f64, from: f64) -> Self {
        let mut s = Self {
            curve_id,
            abscissa,
            from_param: from,
            to_param: 1.0,
            result_param: 0.0,
            is_done: false,
        };
        s.compute();
        s
    }

    fn compute(&mut self) {
        if self.curve_id == 0 { return; }
        // Stub: linear arc-length parametrization
        self.result_param = self.from_param + self.abscissa;
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn parameter(&self) -> Option<f64> { if self.is_done { Some(self.result_param) } else { None } }

    pub fn length(curve_id: u32, first: f64, last: f64) -> f64 {
        if curve_id == 0 { return 0.0; }
        (last - first).abs()
    }
}

// occt: GCPnts_UniformAbscissa — distributes N points with equal arc-length spacing
#[derive(Clone, Debug)]
pub struct GcpntsUniformAbscissa {
    pub curve_id: u32,
    pub nb_points: usize,
    pub first_param: f64,
    pub last_param: f64,
    pub abscissa: f64,
    pub params: Vec<f64>,
    pub is_done: bool,
}

impl GcpntsUniformAbscissa {
    pub fn new_nb_points(curve_id: u32, nb: usize, first: f64, last: f64) -> Self {
        let mut s = Self {
            curve_id,
            nb_points: nb,
            first_param: first,
            last_param: last,
            abscissa: 0.0,
            params: Vec::new(),
            is_done: false,
        };
        s.compute();
        s
    }

    pub fn new_abscissa(curve_id: u32, abscissa: f64, first: f64, last: f64) -> Self {
        let len = (last - first).abs();
        let nb = if abscissa > 1e-15 { (len / abscissa).round() as usize + 1 } else { 2 };
        let mut s = Self {
            curve_id,
            nb_points: nb,
            first_param: first,
            last_param: last,
            abscissa,
            params: Vec::new(),
            is_done: false,
        };
        s.compute();
        s
    }

    fn compute(&mut self) {
        if self.curve_id == 0 || self.nb_points < 2 { return; }
        let n = self.nb_points;
        let step = (self.last_param - self.first_param) / (n - 1) as f64;
        self.params = (0..n).map(|i| self.first_param + i as f64 * step).collect();
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn nb_points(&self) -> usize { self.params.len() }
    pub fn parameter(&self, i: usize) -> Option<f64> {
        // 1-based access
        if i == 0 { None } else { self.params.get(i - 1).copied() }
    }
    pub fn abscissa(&self) -> f64 {
        if self.params.len() >= 2 { self.params[1] - self.params[0] } else { 0.0 }
    }
}

// occt: GCPnts_TangentialDeflection — adaptive point sampling by chord and angular deviation
#[derive(Clone, Debug)]
pub struct GcpntsTangentialDeflection {
    pub curve_id: u32,
    pub angular_deflection: f64,
    pub curvature_deflection: f64,
    pub minimum_nb_points: usize,
    pub first_param: f64,
    pub last_param: f64,
    pub params: Vec<f64>,
    pub is_done: bool,
}

impl GcpntsTangentialDeflection {
    pub fn new(
        curve_id: u32,
        first: f64,
        last: f64,
        angular: f64,
        curvature: f64,
    ) -> Self {
        let mut s = Self {
            curve_id,
            angular_deflection: angular,
            curvature_deflection: curvature,
            minimum_nb_points: 2,
            first_param: first,
            last_param: last,
            params: Vec::new(),
            is_done: false,
        };
        s.compute();
        s
    }

    fn compute(&mut self) {
        if self.curve_id == 0 { return; }
        // Stub: uniform distribution based on deflection
        let len = (self.last_param - self.first_param).abs();
        let nb = (len / self.curvature_deflection.max(1e-15)).ceil() as usize + 2;
        let nb = nb.max(self.minimum_nb_points);
        let step = len / (nb - 1) as f64;
        self.params = (0..nb).map(|i| self.first_param + i as f64 * step).collect();
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn nb_points(&self) -> usize { self.params.len() }
    pub fn parameter(&self, i: usize) -> Option<f64> {
        if i == 0 { None } else { self.params.get(i - 1).copied() }
    }
    pub fn add_point(&mut self, param: f64) { self.params.push(param); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abscissa_point_basic() {
        let a = GcpntsAbscissaPoint::new(1, 0.3, 0.0);
        assert!(a.is_done());
        let p = a.parameter().unwrap();
        assert!((p - 0.3).abs() < 1e-10);
    }

    #[test]
    fn abscissa_point_length() {
        let len = GcpntsAbscissaPoint::length(1, 0.0, 2.0);
        assert!((len - 2.0).abs() < 1e-10);
    }

    #[test]
    fn uniform_abscissa_nb_points() {
        let u = GcpntsUniformAbscissa::new_nb_points(1, 5, 0.0, 1.0);
        assert!(u.is_done());
        assert_eq!(u.nb_points(), 5);
        let p = u.parameter(1).unwrap();
        assert!((p - 0.0).abs() < 1e-10);
        let p5 = u.parameter(5).unwrap();
        assert!((p5 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn uniform_abscissa_spacing() {
        let u = GcpntsUniformAbscissa::new_nb_points(1, 3, 0.0, 2.0);
        let p2 = u.parameter(2).unwrap();
        assert!((p2 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn tangential_deflection_nb_points() {
        let t = GcpntsTangentialDeflection::new(1, 0.0, 1.0, 0.1, 0.1);
        assert!(t.is_done());
        assert!(t.nb_points() >= 2);
    }

    #[test]
    fn tangential_deflection_null_curve() {
        let t = GcpntsTangentialDeflection::new(0, 0.0, 1.0, 0.1, 0.1);
        assert!(!t.is_done());
    }
}
