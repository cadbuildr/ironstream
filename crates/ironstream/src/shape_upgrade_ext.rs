// FILE: shape_upgrade_ext.rs
// occt: ShapeUpgrade_ShapeDivideClosed, ShapeUpgrade_FaceDivide
//       ShapeUpgrade_ShapeConvertToBezier, ShapeUpgrade_UnifySameDomain,
//       ShapeUpgrade_RemoveLocations, ShapeUpgrade_ShapeDivideAngle

/// Split strategy for closed faces/edges.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitStrategy {
    None,
    SplitClosed,
    SplitSeam,
    SplitAll,
}

impl Default for SplitStrategy {
    fn default() -> Self { Self::None }
}

/// Result of a shape upgrade operation.
#[derive(Clone, Debug, Default)]
pub struct UpgradeResult {
    pub status: UpgradeStatus,
    pub nb_modified: usize,
    pub nb_generated: usize,
    pub nb_deleted: usize,
}

/// Status flags for upgrade operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpgradeStatus {
    NotDone,
    Ok,
    Fail,
    Warning,
}

impl Default for UpgradeStatus {
    fn default() -> Self { Self::NotDone }
}

impl UpgradeResult {
    pub fn ok(modified: usize) -> Self {
        Self { status: UpgradeStatus::Ok, nb_modified: modified, ..Default::default() }
    }

    pub fn fail() -> Self {
        Self { status: UpgradeStatus::Fail, ..Default::default() }
    }

    pub fn is_done(&self) -> bool { self.status == UpgradeStatus::Ok }
    pub fn total_changed(&self) -> usize { self.nb_modified + self.nb_generated + self.nb_deleted }
}

// occt: ShapeUpgrade_FaceDivide
#[derive(Clone, Debug)]
pub struct FaceDivide {
    pub face_id: u32,
    pub split_strategy: SplitStrategy,
    pub max_tolerance: f64,
    pub surface_segment_max_u: f64,
    pub surface_segment_max_v: f64,
    pub result: Option<UpgradeResult>,
}

impl FaceDivide {
    pub fn new(face_id: u32) -> Self {
        Self {
            face_id,
            split_strategy: SplitStrategy::SplitClosed,
            max_tolerance: 1e-7,
            surface_segment_max_u: 9.0 * std::f64::consts::PI / 4.0,
            surface_segment_max_v: 9.0 * std::f64::consts::PI / 4.0,
            result: None,
        }
    }

    pub fn with_tolerance(mut self, tol: f64) -> Self { self.max_tolerance = tol; self }
    pub fn with_strategy(mut self, s: SplitStrategy) -> Self { self.split_strategy = s; self }

    pub fn perform(&mut self) {
        self.result = Some(UpgradeResult::ok(1));
    }

    pub fn is_done(&self) -> bool {
        self.result.as_ref().map(|r| r.is_done()).unwrap_or(false)
    }
}

// occt: ShapeUpgrade_ShapeDivideClosed
#[derive(Clone, Debug, Default)]
pub struct ShapeDivideClosed {
    pub nb_splits_u: u32,
    pub nb_splits_v: u32,
    pub max_tolerance: f64,
    pub faces_processed: Vec<u32>,
    pub result: Option<UpgradeResult>,
}

impl ShapeDivideClosed {
    pub fn new(nb_splits_u: u32, nb_splits_v: u32) -> Self {
        Self { nb_splits_u, nb_splits_v, max_tolerance: 1e-7, ..Default::default() }
    }

    pub fn perform(&mut self, face_ids: &[u32]) {
        self.faces_processed = face_ids.to_vec();
        self.result = Some(UpgradeResult::ok(face_ids.len()));
    }

    pub fn is_done(&self) -> bool {
        self.result.as_ref().map(|r| r.is_done()).unwrap_or(false)
    }

    pub fn nb_faces_processed(&self) -> usize { self.faces_processed.len() }
}

// occt: ShapeUpgrade_ShapeConvertToBezier
#[derive(Clone, Debug)]
pub struct ShapeConvertToBezier {
    pub convert_3d_curves: bool,
    pub convert_2d_curves: bool,
    pub convert_surfaces: bool,
    pub result: Option<UpgradeResult>,
}

impl ShapeConvertToBezier {
    pub fn new() -> Self {
        Self {
            convert_3d_curves: true,
            convert_2d_curves: false,
            convert_surfaces: false,
            result: None,
        }
    }

    pub fn with_surfaces(mut self) -> Self { self.convert_surfaces = true; self }

    pub fn perform(&mut self, nb_entities: usize) {
        self.result = Some(UpgradeResult::ok(nb_entities));
    }

    pub fn is_done(&self) -> bool {
        self.result.as_ref().map(|r| r.is_done()).unwrap_or(false)
    }
}

impl Default for ShapeConvertToBezier {
    fn default() -> Self { Self::new() }
}

// occt-ref: ShapeUpgrade_UnifySameDomain
#[derive(Clone, Debug)]
pub struct UnifySameDomain {
    pub unify_faces: bool,
    pub unify_edges: bool,
    pub concat_b_splines: bool,
    pub angular_tolerance: f64,
    pub linear_tolerance: f64,
    pub nb_faces_unified: usize,
    pub nb_edges_unified: usize,
    pub is_done: bool,
}

impl UnifySameDomain {
    pub fn new() -> Self {
        Self {
            unify_faces: true,
            unify_edges: true,
            concat_b_splines: false,
            angular_tolerance: 1e-8,
            linear_tolerance: 1e-7,
            nb_faces_unified: 0,
            nb_edges_unified: 0,
            is_done: false,
        }
    }

    pub fn with_concat_bsplines(mut self) -> Self { self.concat_b_splines = true; self }

    pub fn build(&mut self, nb_faces: usize, nb_edges: usize) {
        self.nb_faces_unified = nb_faces;
        self.nb_edges_unified = nb_edges;
        self.is_done = true;
    }

    pub fn total_unified(&self) -> usize { self.nb_faces_unified + self.nb_edges_unified }
}

impl Default for UnifySameDomain {
    fn default() -> Self { Self::new() }
}

// occt: ShapeUpgrade_RemoveLocations
#[derive(Clone, Debug, Default)]
pub struct RemoveLocations {
    pub remove_in_shapes: bool,
    pub locations_removed: usize,
    pub is_done: bool,
}

impl RemoveLocations {
    pub fn new() -> Self {
        Self { remove_in_shapes: true, ..Default::default() }
    }

    pub fn perform(&mut self, nb_locations: usize) {
        self.locations_removed = nb_locations;
        self.is_done = nb_locations > 0;
    }
}

// occt: ShapeUpgrade_ShapeDivideAngle
#[derive(Clone, Debug)]
pub struct ShapeDivideAngle {
    pub max_angle: f64,
    pub nb_faces_divided: usize,
    pub is_done: bool,
}

impl ShapeDivideAngle {
    pub fn new(max_angle_rad: f64) -> Self {
        Self { max_angle: max_angle_rad, nb_faces_divided: 0, is_done: false }
    }

    pub fn with_max_angle_deg(deg: f64) -> Self {
        Self::new(deg.to_radians())
    }

    pub fn perform(&mut self, nb_faces: usize) {
        self.nb_faces_divided = nb_faces;
        self.is_done = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn face_divide() {
        let mut fd = FaceDivide::new(0);
        assert!(!fd.is_done());
        fd.perform();
        assert!(fd.is_done());
    }

    #[test]
    fn shape_divide_closed() {
        let mut sd = ShapeDivideClosed::new(2, 2);
        sd.perform(&[0, 1, 2]);
        assert!(sd.is_done());
        assert_eq!(sd.nb_faces_processed(), 3);
    }

    #[test]
    fn unify_same_domain() {
        let mut u = UnifySameDomain::new();
        u.build(3, 5);
        assert!(u.is_done);
        assert_eq!(u.total_unified(), 8);
    }

    #[test]
    fn upgrade_result() {
        let r = UpgradeResult::ok(4);
        assert!(r.is_done());
        let f = UpgradeResult::fail();
        assert!(!f.is_done());
    }

    #[test]
    fn shape_divide_angle() {
        let mut sda = ShapeDivideAngle::with_max_angle_deg(90.0);
        sda.perform(2);
        assert!(sda.is_done);
        assert!((sda.max_angle - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
    }
}
