// FILE: hlr_algo.rs
// occt: HLRAlgo // — Hidden Line Removal algorithm data structures.

/// Edge visibility classification for HLR output.
// occt: HLRAlgo_EdgeStatus
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HlrAlgoEdgeStatus {
    AllVisible,
    AllHidden,
    Partial,
}

/// Projection type used by the HLR projector.
// occt: HLRAlgo_Projector
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HlrProjectionType {
    Perspective,
    Orthographic,
}

/// Camera/projector definition for hidden line removal.
// occt: HLRAlgo_Projector
pub struct HlrAlgoProjector {
    projection_type: HlrProjectionType,
    // 4x4 view matrix stored flat row-major
    matrix: [[f64; 4]; 4],
    focal: f64,
}

impl HlrAlgoProjector {
    /// Construct an orthographic projector with an identity view matrix.
    pub fn orthographic() -> Self {
        Self {
            projection_type: HlrProjectionType::Orthographic,
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            focal: 0.0,
        }
    }

    /// Construct a perspective projector with the given focal distance.
    pub fn perspective(focal: f64) -> Self {
        Self {
            projection_type: HlrProjectionType::Perspective,
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            focal,
        }
    }

    pub fn projection_type(&self) -> HlrProjectionType {
        self.projection_type
    }

    /// Returns the focal distance for perspective projectors, `None` for orthographic.
    pub fn focal(&self) -> Option<f64> {
        match self.projection_type {
            HlrProjectionType::Perspective => Some(self.focal),
            HlrProjectionType::Orthographic => None,
        }
    }

    /// Project a 3-D point to 2-D screen coordinates.
    pub fn project(&self, p: [f64; 3]) -> [f64; 2] {
        match self.projection_type {
            HlrProjectionType::Orthographic => [p[0], p[1]],
            // Divide by z then scale by focal; assumes p[2] != 0.
            HlrProjectionType::Perspective => {
                [p[0] / p[2] * self.focal, p[1] / p[2] * self.focal]
            }
        }
    }
}

/// List of parameter-interval edge interferences produced by HLR.
// occt: HLRAlgo_InterferenceList
pub struct HlrAlgoInterferenceList {
    entries: Vec<(f64, f64, HlrAlgoEdgeStatus)>,
}

impl HlrAlgoInterferenceList {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, start: f64, end: f64, status: HlrAlgoEdgeStatus) {
        self.entries.push((start, end, status));
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn get(&self, idx: usize) -> Option<(f64, f64, HlrAlgoEdgeStatus)> {
        self.entries.get(idx).copied()
    }
}

impl Default for HlrAlgoInterferenceList {
    fn default() -> Self {
        Self::new()
    }
}

/// Simplified polygon-based hidden line removal algorithm.
// occt: HLRAlgo_PolyAlgo
pub struct HlrAlgoPolyAlgo {
    projector: HlrAlgoProjector,
    interferences: HlrAlgoInterferenceList,
    tolerance: f64,
}

impl HlrAlgoPolyAlgo {
    pub fn new(projector: HlrAlgoProjector) -> Self {
        Self {
            projector,
            interferences: HlrAlgoInterferenceList::new(),
            tolerance: 1e-7,
        }
    }

    pub fn set_tolerance(&mut self, t: f64) {
        self.tolerance = t;
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// No-op stub — in OCCT this rebuilds the poly data for HLR.
    pub fn update(&mut self) {}

    pub fn interference_count(&self) -> usize {
        self.interferences.count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orthographic_projection_type() {
        let p = HlrAlgoProjector::orthographic();
        assert_eq!(p.projection_type(), HlrProjectionType::Orthographic);
    }

    #[test]
    fn orthographic_focal_is_none() {
        let p = HlrAlgoProjector::orthographic();
        assert!(p.focal().is_none());
    }

    #[test]
    fn perspective_projection_type() {
        let p = HlrAlgoProjector::perspective(10.0);
        assert_eq!(p.projection_type(), HlrProjectionType::Perspective);
    }

    #[test]
    fn perspective_focal_is_some() {
        let p = HlrAlgoProjector::perspective(10.0);
        assert_eq!(p.focal(), Some(10.0));
    }

    #[test]
    fn orthographic_projects_xy() {
        let p = HlrAlgoProjector::orthographic();
        let out = p.project([3.0, 4.0, 5.0]);
        assert_eq!(out, [3.0, 4.0]);
    }

    #[test]
    fn perspective_projects_divide_by_z() {
        let p = HlrAlgoProjector::perspective(2.0);
        let out = p.project([6.0, 4.0, 2.0]);
        let eps = 1e-12;
        assert!((out[0] - 6.0).abs() < eps);
        assert!((out[1] - 4.0).abs() < eps);
    }

    #[test]
    fn interference_list_empty() {
        let list = HlrAlgoInterferenceList::new();
        assert_eq!(list.count(), 0);
        assert!(list.get(0).is_none());
    }

    #[test]
    fn interference_list_add_and_get() {
        let mut list = HlrAlgoInterferenceList::new();
        list.add(0.0, 0.5, HlrAlgoEdgeStatus::AllVisible);
        list.add(0.5, 1.0, HlrAlgoEdgeStatus::AllHidden);
        assert_eq!(list.count(), 2);
        let (s, e, st) = list.get(0).unwrap();
        assert_eq!(s, 0.0);
        assert_eq!(e, 0.5);
        assert_eq!(st, HlrAlgoEdgeStatus::AllVisible);
        let (s2, e2, st2) = list.get(1).unwrap();
        assert_eq!(s2, 0.5);
        assert_eq!(e2, 1.0);
        assert_eq!(st2, HlrAlgoEdgeStatus::AllHidden);
    }

    #[test]
    fn poly_algo_default_tolerance() {
        let algo = HlrAlgoPolyAlgo::new(HlrAlgoProjector::orthographic());
        assert!((algo.tolerance() - 1e-7).abs() < 1e-15);
    }

    #[test]
    fn poly_algo_set_tolerance() {
        let mut algo = HlrAlgoPolyAlgo::new(HlrAlgoProjector::orthographic());
        algo.set_tolerance(1e-4);
        assert!((algo.tolerance() - 1e-4).abs() < 1e-15);
    }

    #[test]
    fn poly_algo_update_is_noop() {
        let mut algo = HlrAlgoPolyAlgo::new(HlrAlgoProjector::orthographic());
        algo.update();
        assert_eq!(algo.interference_count(), 0);
    }

    #[test]
    fn edge_status_partial_variant() {
        let s = HlrAlgoEdgeStatus::Partial;
        assert_eq!(s, HlrAlgoEdgeStatus::Partial);
    }
}
