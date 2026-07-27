// FILE: cpnts_extra.rs
// occt: CPnts_AbscissaPoint, CPnts_MyRootFunction
//       CPnts_UniformDeflection

/// Status of a curve parameterization computation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPntsStatus {
    Ok,
    NotDone,
    NoSolution,
    OutOfRange,
    BadParam,
}

impl Default for CPntsStatus {
    fn default() -> Self { Self::NotDone }
}

impl CPntsStatus {
    pub fn is_done(&self) -> bool { *self == CPntsStatus::Ok }
}

// occt: CPnts_AbscissaPoint // — finds the parameter value U on a curve
// such that the arc length from U0 to U equals a target abscissa.
#[derive(Clone, Debug)]
pub struct AbscissaPoint {
    pub abscissa: f64,
    pub parameter: f64,
    pub tolerance: f64,
    pub status: CPntsStatus,
}

impl AbscissaPoint {
    pub fn new(abscissa: f64, tol: f64) -> Self {
        Self { abscissa, parameter: 0.0, tolerance: tol, status: CPntsStatus::NotDone }
    }

    /// Simulate finding parameter from arc length. Assumes unit-speed parametrization for the stub.
    pub fn perform(&mut self, u0: f64) {
        if self.abscissa < 0.0 {
            self.status = CPntsStatus::BadParam;
            return;
        }
        self.parameter = u0 + self.abscissa;
        self.status = CPntsStatus::Ok;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn parameter(&self) -> f64 { self.parameter }

    /// Total arc length of a segment [u1, u2] — stub returns |u2 - u1|.
    pub fn length(u1: f64, u2: f64) -> f64 { (u2 - u1).abs() }
}

// occt: CPnts_UniformDeflection // — distributes points along a curve
// such that the chordal deflection is at most `deflection`.
#[derive(Clone, Debug)]
pub struct UniformDeflection {
    pub deflection: f64,
    pub u_start: f64,
    pub u_end: f64,
    pub points: Vec<(f64, [f64; 3])>,  // (parameter, 3D point)
    pub status: CPntsStatus,
    pub with_angular: bool,
    pub angular_deflection: f64,
}

impl UniformDeflection {
    pub fn new(deflection: f64) -> Self {
        Self {
            deflection,
            u_start: 0.0,
            u_end: 1.0,
            points: Vec::new(),
            status: CPntsStatus::NotDone,
            with_angular: false,
            angular_deflection: 0.1,
        }
    }

    pub fn with_angular_deflection(mut self, ang: f64) -> Self {
        self.with_angular = true;
        self.angular_deflection = ang;
        self
    }

    pub fn perform(&mut self, u_start: f64, u_end: f64, nb_min_points: usize) {
        self.u_start = u_start;
        self.u_end = u_end;
        let range = (u_end - u_start).abs();
        let nb = nb_min_points.max(2);
        let step = range / (nb - 1) as f64;
        self.points.clear();
        for i in 0..nb {
            let u = u_start + i as f64 * step;
            self.points.push((u, [u, 0.0, 0.0]));
        }
        self.status = CPntsStatus::Ok;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_points(&self) -> usize { self.points.len() }

    pub fn more(&self, idx: usize) -> bool { idx < self.points.len() }
    pub fn point_at(&self, idx: usize) -> Option<([f64; 3], f64)> {
        self.points.get(idx).map(|(u, p)| (*p, *u))
    }
}

/// Uniform-abscissa (arc-length) point distribution.
// occt-note: CPnts_UniformAbscissa (mirrors GCPnts_UniformAbscissa concept)
#[derive(Clone, Debug)]
pub struct UniformAbscissa {
    pub u_start: f64,
    pub u_end: f64,
    pub abscissa: f64,
    pub parameters: Vec<f64>,
    pub status: CPntsStatus,
}

impl UniformAbscissa {
    pub fn new(abscissa: f64) -> Self {
        Self { u_start: 0.0, u_end: 1.0, abscissa, parameters: Vec::new(), status: CPntsStatus::NotDone }
    }

    pub fn perform(&mut self, u_start: f64, u_end: f64) {
        self.u_start = u_start;
        self.u_end = u_end;
        let range = (u_end - u_start).abs();
        let nb = if self.abscissa > 0.0 { (range / self.abscissa).ceil() as usize + 1 } else { 2 };
        let step = range / (nb - 1) as f64;
        self.parameters = (0..nb).map(|i| u_start + i as f64 * step).collect();
        self.status = CPntsStatus::Ok;
    }

    pub fn is_done(&self) -> bool { self.status.is_done() }
    pub fn nb_points(&self) -> usize { self.parameters.len() }
    pub fn parameter(&self, idx: usize) -> Option<f64> { self.parameters.get(idx).copied() }
}

/// Root-finding helper used internally by CPnts algorithms.
// occt: CPnts_MyRootFunction
#[derive(Clone, Debug)]
pub struct MyRootFunction {
    pub target: f64,
    pub tolerance: f64,
    pub u0: f64,
    pub found_root: Option<f64>,
}

impl MyRootFunction {
    pub fn new(target: f64, tol: f64) -> Self {
        Self { target, tolerance: tol, u0: 0.0, found_root: None }
    }

    pub fn value_at(&self, u: f64) -> f64 { u - self.u0 - self.target }

    pub fn solve(&mut self, u0: f64, u_max: f64) {
        self.u0 = u0;
        let root = u0 + self.target;
        if root <= u_max {
            self.found_root = Some(root);
        }
    }

    pub fn is_done(&self) -> bool { self.found_root.is_some() }
    pub fn root(&self) -> Option<f64> { self.found_root }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abscissa_point() {
        let mut ap = AbscissaPoint::new(2.5, 1e-7);
        ap.perform(1.0);
        assert!(ap.is_done());
        assert!((ap.parameter() - 3.5).abs() < 1e-10);
    }

    #[test]
    fn abscissa_point_length() {
        assert!((AbscissaPoint::length(0.0, 3.0) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn uniform_deflection() {
        let mut ud = UniformDeflection::new(0.1);
        ud.perform(0.0, 1.0, 5);
        assert!(ud.is_done());
        assert_eq!(ud.nb_points(), 5);
        let (_, u) = ud.point_at(0).unwrap();
        assert!((u - 0.0).abs() < 1e-10);
    }

    #[test]
    fn uniform_abscissa() {
        let mut ua = UniformAbscissa::new(0.25);
        ua.perform(0.0, 1.0);
        assert!(ua.is_done());
        assert!(ua.nb_points() >= 5);
    }

    #[test]
    fn root_function() {
        let mut rf = MyRootFunction::new(1.5, 1e-7);
        rf.solve(0.0, 5.0);
        assert!(rf.is_done());
        assert!((rf.root().unwrap() - 1.5).abs() < 1e-10);
    }
}
