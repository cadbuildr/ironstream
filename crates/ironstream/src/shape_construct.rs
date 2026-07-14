// FILE: shape_construct.rs
// occt: ShapeConstruct_Curve
// occt-ref: ShapeConstruct_Polyline, ShapeConstruct_ProjectionPeriodic

/// Polyline: sequence of connected points.
// occt-ref: ShapeConstruct_Polyline
#[derive(Clone, Debug)]
pub struct ShapeConstructPolyline {
    pub points: Vec<[f64; 3]>,
    pub closed: bool,
}

impl ShapeConstructPolyline {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            closed: false,
        }
    }

    pub fn add_point(&mut self, pt: [f64; 3]) {
        self.points.push(pt);
    }

    pub fn close_polyline(&mut self) {
        self.closed = true;
    }

    pub fn nb_points(&self) -> usize { self.points.len() }

    pub fn point(&self, index: usize) -> Option<[f64; 3]> {
        if index > 0 && index <= self.points.len() {
            Some(self.points[index - 1])
        } else {
            None
        }
    }

    pub fn length(&self) -> f64 {
        let mut len = 0.0;
        for i in 0..self.points.len() - 1 {
            let p1 = self.points[i];
            let p2 = self.points[i + 1];
            let dx = p2[0] - p1[0];
            let dy = p2[1] - p1[1];
            let dz = p2[2] - p1[2];
            len += (dx * dx + dy * dy + dz * dz).sqrt();
        }
        if self.closed && self.points.len() > 1 {
            let p1 = self.points[self.points.len() - 1];
            let p2 = self.points[0];
            let dx = p2[0] - p1[0];
            let dy = p2[1] - p1[1];
            let dz = p2[2] - p1[2];
            len += (dx * dx + dy * dy + dz * dz).sqrt();
        }
        len
    }
}

impl Default for ShapeConstructPolyline {
    fn default() -> Self {
        Self::new()
    }
}

/// Projection: periodic curve parameters.
// occt-ref: ShapeConstruct_ProjectionPeriodic
#[derive(Clone, Debug)]
pub struct ShapeConstructProjectionPeriodic {
    pub start_param: f64,
    pub end_param: f64,
    pub period: f64,
}

impl ShapeConstructProjectionPeriodic {
    pub fn new(start: f64, end: f64, period: f64) -> Self {
        Self {
            start_param: start,
            end_param: end,
            period: period.max(0.001),
        }
    }

    pub fn is_periodic(&self) -> bool { self.period > 0.0 }

    pub fn adjust_parameter(&self, param: f64) -> f64 {
        if !self.is_periodic() {
            param
        } else {
            let normalized = (param - self.start_param) % self.period;
            self.start_param + if normalized < 0.0 { normalized + self.period } else { normalized }
        }
    }

    pub fn distance_in_period(&self, p1: f64, p2: f64) -> f64 {
        if !self.is_periodic() {
            (p2 - p1).abs()
        } else {
            let diff = (p2 - p1).abs();
            let wrapped = self.period - diff;
            diff.min(wrapped)
        }
    }
}

impl Default for ShapeConstructProjectionPeriodic {
    fn default() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
}

/// Curve construction utility.
/// occt: ShapeConstruct_Curve
#[derive(Clone, Debug)]
pub struct ShapeConstructCurve {
    pub curve_id: u32,
    pub degree: usize,
    pub knots: Vec<f64>,
    pub control_points: Vec<[f64; 3]>,
}

impl ShapeConstructCurve {
    pub fn new(curve_id: u32, degree: usize) -> Self {
        Self {
            curve_id,
            degree: degree.max(1),
            knots: Vec::new(),
            control_points: Vec::new(),
        }
    }

    pub fn add_knot(&mut self, knot: f64) {
        self.knots.push(knot);
    }

    pub fn add_control_point(&mut self, pt: [f64; 3]) {
        self.control_points.push(pt);
    }

    pub fn nb_knots(&self) -> usize { self.knots.len() }
    pub fn nb_control_points(&self) -> usize { self.control_points.len() }

    pub fn is_valid(&self) -> bool {
        self.nb_control_points() >= self.degree + 1 && self.nb_knots() >= 2 * (self.degree + 1)
    }

    pub fn arc_length(&self) -> f64 {
        let mut len = 0.0;
        for i in 0..self.control_points.len() - 1 {
            let p1 = self.control_points[i];
            let p2 = self.control_points[i + 1];
            let dx = p2[0] - p1[0];
            let dy = p2[1] - p1[1];
            let dz = p2[2] - p1[2];
            len += (dx * dx + dy * dy + dz * dz).sqrt();
        }
        len
    }
}

impl Default for ShapeConstructCurve {
    fn default() -> Self {
        Self::new(0, 3)
    }
}

/// Curve approximation parameters.
// occt-ref: ShapeConstruct_ApproximationParams
#[derive(Clone, Debug)]
pub struct ShapeConstructApproximationParams {
    pub max_degree: usize,
    pub max_segments: usize,
    pub tolerance: f64,
    pub enforce_continuity: bool,
}

impl ShapeConstructApproximationParams {
    pub fn new(max_deg: usize, max_seg: usize, tol: f64) -> Self {
        Self {
            max_degree: max_deg.max(1),
            max_segments: max_seg.max(1),
            tolerance: tol.max(1e-10),
            enforce_continuity: true,
        }
    }
}

impl Default for ShapeConstructApproximationParams {
    fn default() -> Self {
        Self::new(8, 20, 1e-7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_construct_polyline() {
        let mut p = ShapeConstructPolyline::new();
        p.add_point([0.0, 0.0, 0.0]);
        p.add_point([1.0, 0.0, 0.0]);
        p.add_point([1.0, 1.0, 0.0]);
        assert_eq!(p.nb_points(), 3);
        assert_eq!(p.point(1), Some([0.0, 0.0, 0.0]));
    }

    #[test]
    fn polyline_length() {
        let mut p = ShapeConstructPolyline::new();
        p.add_point([0.0, 0.0, 0.0]);
        p.add_point([1.0, 0.0, 0.0]);
        p.add_point([1.0, 1.0, 0.0]);
        let len = p.length();
        assert!((len - 2.0).abs() < 0.01);
    }

    #[test]
    fn projection_periodic() {
        let proj = ShapeConstructProjectionPeriodic::new(0.0, 1.0, 1.0);
        assert!(proj.is_periodic());
        let adjusted = proj.adjust_parameter(1.5);
        assert!(adjusted >= 0.0 && adjusted < 1.0);
    }

    #[test]
    fn shape_construct_curve() {
        let mut c = ShapeConstructCurve::new(1, 3);
        c.add_control_point([0.0, 0.0, 0.0]);
        c.add_control_point([1.0, 0.0, 0.0]);
        c.add_control_point([1.0, 1.0, 0.0]);
        for _ in 0..10 {
            c.add_knot(0.5);
        }
        assert_eq!(c.nb_control_points(), 3);
        assert!(c.nb_knots() > 0);
    }

    #[test]
    fn approximation_params() {
        let p = ShapeConstructApproximationParams::new(6, 15, 1e-6);
        assert_eq!(p.max_degree, 6);
        assert_eq!(p.max_segments, 15);
        assert!(p.tolerance > 0.0);
    }
}
