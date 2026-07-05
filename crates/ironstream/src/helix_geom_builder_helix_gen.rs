// FILE: helix_geom_builder_helix_gen.rs
// occt: HelixGeom_BuilderHelixGen

/// Represents continuity requirements for curve approximation (re-exported from base).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeomAbsShape {
    C0 = 0,
    C1 = 1,
    C2 = 2,
    C3 = 3,
    CN = 4,
}

/// Represents a BSpline curve (re-exported from base).
#[derive(Clone, Debug)]
pub struct GeomCurve {
    pub id: usize,
}

/// Base approximation curve builder with core functionality.
pub struct HelixGeomBuilderApproxCurveBase {
    error_status: i32,
    warning_status: i32,
    tolerance: f64,
    continuity: GeomAbsShape,
    max_degree: i32,
    max_seg: i32,
    tolerance_reached: f64,
    curves: Vec<GeomCurve>,
}

/// Builder for helix curves with parameter management.
///
/// This class extends the approximation builder by adding helix-specific
/// geometric parameters:
/// - Parameter range (t1, t2) - angular range in radians
/// - Pitch - vertical distance per full turn (2*PI radians)
/// - Start radius (r_start) - radius at parameter t1
/// - Taper angle - angle for radius variation (0 = cylindrical)
/// - Orientation - clockwise or counter-clockwise
pub struct HelixGeomBuilderHelixGen {
    base: HelixGeomBuilderApproxCurveBase,
    t1: f64,
    t2: f64,
    pitch: f64,
    r_start: f64,
    taper_angle: f64,
    is_clockwise: bool,
}

impl HelixGeomBuilderHelixGen {
    /// Creates a new helix builder with default parameters.
    pub fn new() -> Self {
        HelixGeomBuilderHelixGen {
            base: HelixGeomBuilderApproxCurveBase {
                error_status: 0,
                warning_status: 0,
                tolerance: 1e-6,
                continuity: GeomAbsShape::C2,
                max_degree: 8,
                max_seg: 100,
                tolerance_reached: 0.0,
                curves: Vec::new(),
            },
            t1: 0.0,
            t2: 2.0 * std::f64::consts::PI,
            pitch: 10.0,
            r_start: 5.0,
            taper_angle: 0.0,
            is_clockwise: true,
        }
    }

    /// Sets parameters for building helix curves.
    pub fn set_curve_parameters(
        &mut self,
        t1: f64,
        t2: f64,
        pitch: f64,
        r_start: f64,
        taper_angle: f64,
        is_clockwise: bool,
    ) {
        self.t1 = t1;
        self.t2 = t2;
        self.pitch = pitch;
        self.r_start = r_start;
        self.taper_angle = taper_angle;
        self.is_clockwise = is_clockwise;
    }

    /// Gets parameters for building helix curves.
    pub fn curve_parameters(&self) -> (f64, f64, f64, f64, f64, bool) {
        (self.t1, self.t2, self.pitch, self.r_start, self.taper_angle, self.is_clockwise)
    }

    /// Gets the parameter range.
    pub fn param_range(&self) -> (f64, f64) {
        (self.t1, self.t2)
    }

    /// Gets the pitch (vertical distance per full turn).
    pub fn pitch(&self) -> f64 {
        self.pitch
    }

    /// Gets the start radius.
    pub fn start_radius(&self) -> f64 {
        self.r_start
    }

    /// Gets the taper angle.
    pub fn taper_angle(&self) -> f64 {
        self.taper_angle
    }

    /// Checks if the helix is clockwise.
    pub fn is_clockwise(&self) -> bool {
        self.is_clockwise
    }

    /// Gets approximation tolerance.
    pub fn tolerance(&self) -> f64 {
        self.base.tolerance
    }

    /// Sets approximation tolerance.
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.base.tolerance = tolerance;
    }

    /// Returns error status of algorithm.
    pub fn error_status(&self) -> i32 {
        self.base.error_status
    }

    /// Returns warning status of algorithm.
    pub fn warning_status(&self) -> i32 {
        self.base.warning_status
    }

    /// Gets sequence of BSpline curves.
    pub fn curves(&self) -> &[GeomCurve] {
        &self.base.curves
    }
}

impl Default for HelixGeomBuilderHelixGen {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_builder() {
        let builder = HelixGeomBuilderHelixGen::new();
        assert_eq!(builder.t1, 0.0);
        assert_eq!(builder.t2, 2.0 * std::f64::consts::PI);
        assert_eq!(builder.pitch, 10.0);
        assert_eq!(builder.r_start, 5.0);
        assert_eq!(builder.taper_angle, 0.0);
        assert!(builder.is_clockwise);
    }

    #[test]
    fn test_set_curve_parameters() {
        let mut builder = HelixGeomBuilderHelixGen::new();
        builder.set_curve_parameters(0.0, 6.28, 15.0, 8.0, 0.1, false);

        let (t1, t2, pitch, r_start, taper_angle, is_clockwise) = builder.curve_parameters();
        assert_eq!(t1, 0.0);
        assert_eq!(t2, 6.28);
        assert_eq!(pitch, 15.0);
        assert_eq!(r_start, 8.0);
        assert_eq!(taper_angle, 0.1);
        assert!(!is_clockwise);
    }

    #[test]
    fn test_get_curve_parameters() {
        let builder = HelixGeomBuilderHelixGen::new();
        let (t1, t2, pitch, r_start, taper_angle, is_clockwise) = builder.curve_parameters();
        assert_eq!(t1, 0.0);
        assert_eq!(t2, 2.0 * std::f64::consts::PI);
        assert_eq!(pitch, 10.0);
        assert_eq!(r_start, 5.0);
        assert_eq!(taper_angle, 0.0);
        assert!(is_clockwise);
    }

    #[test]
    fn test_param_range() {
        let mut builder = HelixGeomBuilderHelixGen::new();
        builder.t1 = 1.0;
        builder.t2 = 5.0;

        let (t1, t2) = builder.param_range();
        assert_eq!(t1, 1.0);
        assert_eq!(t2, 5.0);
    }

    #[test]
    fn test_pitch() {
        let mut builder = HelixGeomBuilderHelixGen::new();
        builder.pitch = 20.0;
        assert_eq!(builder.pitch(), 20.0);
    }

    #[test]
    fn test_start_radius() {
        let mut builder = HelixGeomBuilderHelixGen::new();
        builder.r_start = 12.5;
        assert_eq!(builder.start_radius(), 12.5);
    }

    #[test]
    fn test_taper_angle() {
        let mut builder = HelixGeomBuilderHelixGen::new();
        builder.taper_angle = 0.2;
        assert_eq!(builder.taper_angle(), 0.2);
    }

    #[test]
    fn test_is_clockwise() {
        let mut builder = HelixGeomBuilderHelixGen::new();
        assert!(builder.is_clockwise());
        builder.is_clockwise = false;
        assert!(!builder.is_clockwise());
    }

    #[test]
    fn test_tolerance_management() {
        let mut builder = HelixGeomBuilderHelixGen::new();
        assert_eq!(builder.tolerance(), 1e-6);
        builder.set_tolerance(0.001);
        assert_eq!(builder.tolerance(), 0.001);
    }

    #[test]
    fn test_error_status() {
        let builder = HelixGeomBuilderHelixGen::new();
        assert_eq!(builder.error_status(), 0);
    }

    #[test]
    fn test_default() {
        let builder = HelixGeomBuilderHelixGen::default();
        assert_eq!(builder.pitch(), 10.0);
        assert!(builder.curves().is_empty());
    }
}
