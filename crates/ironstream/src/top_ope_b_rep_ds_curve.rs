// FILE: top_ope_b_rep_ds_curve.rs
// occt: TopOpeBRepDS_Curve

/// A curve geometry with tolerance and associated data
#[derive(Debug, Clone)]
pub struct Curve {
    /// Curve 3D handle (we store as unit: should reference Geom_Curve)
    curve_3d: Option<usize>,

    /// Range bounds [first, last]
    first: f64,
    last: f64,
    range_defined: bool,

    /// Tolerance
    tolerance: f64,

    /// Is this a walking curve
    is_walk: bool,

    /// Associated shapes (stored as shape indices)
    shape1_id: Option<usize>,
    shape2_id: Option<usize>,

    /// Surface-curve interferences (indices)
    sci1: Option<usize>,
    sci2: Option<usize>,

    /// Parametric curves (2D)
    curve_2d_1: Option<usize>,
    curve_2d_2: Option<usize>,

    /// Keep flag
    keep: bool,

    /// Mother curve index
    mother: i32,

    /// Data structure index
    ds_index: i32,
}

impl Curve {
    /// Create a new default Curve
    pub fn new() -> Self {
        Curve {
            curve_3d: None,
            first: 0.0,
            last: 0.0,
            range_defined: false,
            tolerance: 0.0,
            is_walk: false,
            shape1_id: None,
            shape2_id: None,
            sci1: None,
            sci2: None,
            curve_2d_1: None,
            curve_2d_2: None,
            keep: false,
            mother: -1,
            ds_index: -1,
        }
    }

    /// Create a Curve from 3D curve, tolerance, and walk flag
    pub fn with_curve(curve_id: usize, tolerance: f64, is_walk: bool) -> Self {
        Curve {
            curve_3d: Some(curve_id),
            tolerance,
            is_walk,
            ..Default::default()
        }
    }

    /// Define the curve
    pub fn define_curve(&mut self, curve_id: usize, tolerance: f64, is_walk: bool) {
        self.curve_3d = Some(curve_id);
        self.tolerance = tolerance;
        self.is_walk = is_walk;
    }

    /// Set tolerance
    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = tol;
    }

    /// Get tolerance
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Set surface-curve interferences
    pub fn set_sci(&mut self, i1: Option<usize>, i2: Option<usize>) {
        self.sci1 = i1;
        self.sci2 = i2;
    }

    /// Get first surface-curve interference
    pub fn get_sci1(&self) -> Option<usize> {
        self.sci1
    }

    /// Get second surface-curve interference
    pub fn get_sci2(&self) -> Option<usize> {
        self.sci2
    }

    /// Set shapes
    pub fn set_shapes(&mut self, s1_id: Option<usize>, s2_id: Option<usize>) {
        self.shape1_id = s1_id;
        self.shape2_id = s2_id;
    }

    /// Get shapes as indices
    pub fn get_shapes(&self) -> (Option<usize>, Option<usize>) {
        (self.shape1_id, self.shape2_id)
    }

    /// Get shape1 index
    pub fn shape1(&self) -> Option<usize> {
        self.shape1_id
    }

    /// Mutably access shape1 index
    pub fn shape1_mut(&mut self) -> &mut Option<usize> {
        &mut self.shape1_id
    }

    /// Get shape2 index
    pub fn shape2(&self) -> Option<usize> {
        self.shape2_id
    }

    /// Mutably access shape2 index
    pub fn shape2_mut(&mut self) -> &mut Option<usize> {
        &mut self.shape2_id
    }

    /// Get curve 3D index
    pub fn curve(&self) -> Option<usize> {
        self.curve_3d
    }

    /// Set range
    pub fn set_range(&mut self, first: f64, last: f64) {
        self.first = first;
        self.last = last;
        self.range_defined = true;
    }

    /// Get range
    pub fn range(&self) -> Option<(f64, f64)> {
        if self.range_defined {
            Some((self.first, self.last))
        } else {
            None
        }
    }

    /// Mutably access curve 3D
    pub fn curve_mut(&mut self) -> &mut Option<usize> {
        &mut self.curve_3d
    }

    /// Set curve 3D with tolerance
    pub fn set_curve_3d(&mut self, curve_id: Option<usize>, tol: f64) {
        self.curve_3d = curve_id;
        self.tolerance = tol;
    }

    /// Get 2D curve 1
    pub fn curve_2d_1(&self) -> Option<usize> {
        self.curve_2d_1
    }

    /// Set 2D curve 1
    pub fn set_curve_2d_1(&mut self, curve_id: Option<usize>) {
        self.curve_2d_1 = curve_id;
    }

    /// Get 2D curve 2
    pub fn curve_2d_2(&self) -> Option<usize> {
        self.curve_2d_2
    }

    /// Set 2D curve 2
    pub fn set_curve_2d_2(&mut self, curve_id: Option<usize>) {
        self.curve_2d_2 = curve_id;
    }

    /// Check if walk curve
    pub fn is_walk(&self) -> bool {
        self.is_walk
    }

    /// Set walk flag
    pub fn set_is_walk(&mut self, b: bool) {
        self.is_walk = b;
    }

    /// Check keep flag
    pub fn keep(&self) -> bool {
        self.keep
    }

    /// Set keep flag
    pub fn set_keep(&mut self, b: bool) {
        self.keep = b;
    }

    /// Get mother curve index
    pub fn mother(&self) -> i32 {
        self.mother
    }

    /// Set mother curve index
    pub fn set_mother(&mut self, i: i32) {
        self.mother = i;
    }

    /// Get data structure index
    pub fn ds_index(&self) -> i32 {
        self.ds_index
    }

    /// Set data structure index
    pub fn set_ds_index(&mut self, i: i32) {
        self.ds_index = i;
    }
}

impl Default for Curve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_new() {
        let c = Curve::new();
        assert!(c.curve().is_none());
        assert_eq!(c.tolerance(), 0.0);
        assert!(!c.is_walk());
        assert!(!c.keep());
        assert_eq!(c.mother(), -1);
        assert_eq!(c.ds_index(), -1);
    }

    #[test]
    fn test_curve_with_curve() {
        let c = Curve::with_curve(42, 0.5, true);
        assert_eq!(c.curve(), Some(42));
        assert_eq!(c.tolerance(), 0.5);
        assert!(c.is_walk());
    }

    #[test]
    fn test_curve_define() {
        let mut c = Curve::new();
        c.define_curve(10, 0.1, false);
        assert_eq!(c.curve(), Some(10));
        assert_eq!(c.tolerance(), 0.1);
        assert!(!c.is_walk());
    }

    #[test]
    fn test_curve_tolerance() {
        let mut c = Curve::new();
        c.set_tolerance(0.75);
        assert_eq!(c.tolerance(), 0.75);
    }

    #[test]
    fn test_curve_sci() {
        let mut c = Curve::new();
        c.set_sci(Some(1), Some(2));
        assert_eq!(c.get_sci1(), Some(1));
        assert_eq!(c.get_sci2(), Some(2));
    }

    #[test]
    fn test_curve_shapes() {
        let mut c = Curve::new();
        c.set_shapes(Some(5), Some(6));
        let (s1, s2) = c.get_shapes();
        assert_eq!(s1, Some(5));
        assert_eq!(s2, Some(6));
        assert_eq!(c.shape1(), Some(5));
        assert_eq!(c.shape2(), Some(6));
    }

    #[test]
    fn test_curve_range() {
        let mut c = Curve::new();
        assert!(c.range().is_none());
        c.set_range(1.0, 5.0);
        assert_eq!(c.range(), Some((1.0, 5.0)));
    }

    #[test]
    fn test_curve_2d_curves() {
        let mut c = Curve::new();
        c.set_curve_2d_1(Some(11));
        c.set_curve_2d_2(Some(22));
        assert_eq!(c.curve_2d_1(), Some(11));
        assert_eq!(c.curve_2d_2(), Some(22));
    }

    #[test]
    fn test_curve_flags() {
        let mut c = Curve::new();
        c.set_is_walk(true);
        assert!(c.is_walk());
        c.set_keep(true);
        assert!(c.keep());
    }

    #[test]
    fn test_curve_mother_and_ds_index() {
        let mut c = Curve::new();
        c.set_mother(7);
        c.set_ds_index(99);
        assert_eq!(c.mother(), 7);
        assert_eq!(c.ds_index(), 99);
    }

    #[test]
    fn test_curve_default() {
        let c = Curve::default();
        assert!(c.curve().is_none());
        assert_eq!(c.tolerance(), 0.0);
    }
}
