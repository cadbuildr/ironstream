// FILE: ch_fi_ds_el_spine.rs
// occt: ChFiDS_ElSpine

/// Elementary spine for fillets and chamfers.
/// Represents a curve that guides the fillet construction.
#[derive(Debug, Clone)]
pub struct ChFiDsElSpine {
    /// First parameter
    first_param: f64,
    /// Last parameter
    last_param: f64,
    /// Saved first parameter
    first_param_saved: f64,
    /// Saved last parameter
    last_param_saved: f64,
    /// Origin parameter
    origin: f64,
    /// First point
    pt_first: (f64, f64, f64),
    /// Last point
    pt_last: (f64, f64, f64),
    /// First tangent
    tg_first: (f64, f64, f64),
    /// Last tangent
    tg_last: (f64, f64, f64),
    /// Whether the curve is periodic
    is_periodic: bool,
    /// Period if periodic
    period: f64,
}

impl ChFiDsElSpine {
    /// Create a new ElSpine
    pub fn new() -> Self {
        Self {
            first_param: 0.0,
            last_param: 0.0,
            first_param_saved: 0.0,
            last_param_saved: 0.0,
            origin: 0.0,
            pt_first: (0.0, 0.0, 0.0),
            pt_last: (0.0, 0.0, 0.0),
            tg_first: (0.0, 0.0, 1.0),
            tg_last: (0.0, 0.0, 1.0),
            is_periodic: false,
            period: 0.0,
        }
    }

    /// Get first parameter
    pub fn first_parameter(&self) -> f64 {
        self.first_param
    }

    /// Get last parameter
    pub fn last_parameter(&self) -> f64 {
        self.last_param
    }

    /// Get saved first parameter
    pub fn get_saved_first_parameter(&self) -> f64 {
        self.first_param_saved
    }

    /// Get saved last parameter
    pub fn get_saved_last_parameter(&self) -> f64 {
        self.last_param_saved
    }

    /// Set first parameter
    pub fn set_first_parameter(&mut self, p: f64) {
        self.first_param = p;
    }

    /// Set last parameter
    pub fn set_last_parameter(&mut self, p: f64) {
        self.last_param = p;
    }

    /// Save first parameter
    pub fn save_first_parameter(&mut self) {
        self.first_param_saved = self.first_param;
    }

    /// Save last parameter
    pub fn save_last_parameter(&mut self) {
        self.last_param_saved = self.last_param;
    }

    /// Set origin
    pub fn set_origin(&mut self, o: f64) {
        self.origin = o;
    }

    /// Set first point and tangent
    pub fn set_first_point_and_tgt(&mut self, px: f64, py: f64, pz: f64, tx: f64, ty: f64, tz: f64) {
        self.pt_first = (px, py, pz);
        self.tg_first = (tx, ty, tz);
    }

    /// Set last point and tangent
    pub fn set_last_point_and_tgt(&mut self, px: f64, py: f64, pz: f64, tx: f64, ty: f64, tz: f64) {
        self.pt_last = (px, py, pz);
        self.tg_last = (tx, ty, tz);
    }

    /// Get first point and tangent
    pub fn first_point_and_tgt(&self) -> ((f64, f64, f64), (f64, f64, f64)) {
        (self.pt_first, self.tg_first)
    }

    /// Get last point and tangent
    pub fn last_point_and_tgt(&self) -> ((f64, f64, f64), (f64, f64, f64)) {
        (self.pt_last, self.tg_last)
    }

    /// Check if periodic
    pub fn is_periodic(&self) -> bool {
        self.is_periodic
    }

    /// Set periodic flag
    pub fn set_periodic(&mut self, is_periodic: bool) {
        self.is_periodic = is_periodic;
    }

    /// Get period
    pub fn period(&self) -> f64 {
        self.period
    }

    /// Set period
    pub fn set_period(&mut self, p: f64) {
        self.period = p;
    }
}

impl Default for ChFiDsElSpine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let spine = ChFiDsElSpine::new();
        assert_eq!(spine.first_parameter(), 0.0);
        assert_eq!(spine.last_parameter(), 0.0);
        assert!(!spine.is_periodic());
    }

    #[test]
    fn test_set_parameters() {
        let mut spine = ChFiDsElSpine::new();
        spine.set_first_parameter(1.0);
        spine.set_last_parameter(10.0);
        assert_eq!(spine.first_parameter(), 1.0);
        assert_eq!(spine.last_parameter(), 10.0);
    }

    #[test]
    fn test_save_parameters() {
        let mut spine = ChFiDsElSpine::new();
        spine.set_first_parameter(2.0);
        spine.set_last_parameter(8.0);
        spine.save_first_parameter();
        spine.save_last_parameter();
        assert_eq!(spine.get_saved_first_parameter(), 2.0);
        assert_eq!(spine.get_saved_last_parameter(), 8.0);
    }

    #[test]
    fn test_set_points_and_tangents() {
        let mut spine = ChFiDsElSpine::new();
        spine.set_first_point_and_tgt(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        spine.set_last_point_and_tgt(10.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        let (pt, tg) = spine.first_point_and_tgt();
        assert_eq!(pt, (0.0, 0.0, 0.0));
        assert_eq!(tg, (1.0, 0.0, 0.0));
    }

    #[test]
    fn test_periodic() {
        let mut spine = ChFiDsElSpine::new();
        spine.set_periodic(true);
        spine.set_period(6.28);
        assert!(spine.is_periodic());
        assert_eq!(spine.period(), 6.28);
    }

    #[test]
    fn test_default() {
        let spine = ChFiDsElSpine::default();
        assert_eq!(spine.first_parameter(), 0.0);
    }
}
