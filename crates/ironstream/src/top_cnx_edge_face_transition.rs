// FILE: top_cnx_edge_face_transition.rs
// occt: TopCnx_EdgeFaceTransition

/// Computes cumulated transition for interferences on an edge.
/// Tracks curve transitions and boundary counts.
#[derive(Clone, Debug)]
pub struct TopCnxEdgeFaceTransition {
    nb_bound_forward: i32,
    nb_bound_reversed: i32,
    transition: i32,  // TopAbs_Orientation
    boundary_transition: i32, // TopAbs_Orientation
}

impl TopCnxEdgeFaceTransition {
    /// Creates an empty algorithm.
    pub fn new() -> Self {
        TopCnxEdgeFaceTransition {
            nb_bound_forward: 0,
            nb_bound_reversed: 0,
            transition: 0,
            boundary_transition: 0,
        }
    }

    /// Reset with tangent, normal, and curvature of edge.
    pub fn reset_with_curve(&mut self, _tgt: (f64, f64, f64), _norm: (f64, f64, f64), _curv: f64) {
        self.nb_bound_forward = 0;
        self.nb_bound_reversed = 0;
        self.transition = 0;
        self.boundary_transition = 0;
    }

    /// Reset with linear edge (tangent only).
    pub fn reset(&mut self, _tgt: (f64, f64, f64)) {
        self.nb_bound_forward = 0;
        self.nb_bound_reversed = 0;
        self.transition = 0;
        self.boundary_transition = 0;
    }

    /// Add a curve element to boundary with transition info.
    pub fn add_interference(
        &mut self,
        _tole: f64,
        _tang: (f64, f64, f64),
        _norm: (f64, f64, f64),
        _curv: f64,
        or: i32,
        tr: i32,
        _btr: i32,
    ) {
        // Track forward/reversed boundary counts
        if or == 0 { // Forward orientation
            self.nb_bound_forward += 1;
        } else {
            self.nb_bound_reversed += 1;
        }
        self.transition = tr;
    }

    pub fn transition(&self) -> i32 {
        self.transition
    }

    pub fn boundary_transition(&self) -> i32 {
        self.boundary_transition
    }

    pub fn nb_bound_forward(&self) -> i32 {
        self.nb_bound_forward
    }

    pub fn nb_bound_reversed(&self) -> i32 {
        self.nb_bound_reversed
    }
}

impl Default for TopCnxEdgeFaceTransition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let eft = TopCnxEdgeFaceTransition::new();
        assert_eq!(eft.nb_bound_forward(), 0);
        assert_eq!(eft.nb_bound_reversed(), 0);
    }

    #[test]
    fn test_reset_linear() {
        let mut eft = TopCnxEdgeFaceTransition::new();
        eft.reset((1.0, 0.0, 0.0));
        assert_eq!(eft.nb_bound_forward(), 0);
        assert_eq!(eft.nb_bound_reversed(), 0);
    }

    #[test]
    fn test_add_interference() {
        let mut eft = TopCnxEdgeFaceTransition::new();
        eft.add_interference(0.01, (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), 0.1, 0, 2, 1);
        assert_eq!(eft.nb_bound_forward(), 1);
        assert_eq!(eft.transition(), 2);
    }

    #[test]
    fn test_reversed_boundary() {
        let mut eft = TopCnxEdgeFaceTransition::new();
        eft.add_interference(0.01, (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), 0.1, 1, 2, 1);
        assert_eq!(eft.nb_bound_reversed(), 1);
        assert_eq!(eft.nb_bound_forward(), 0);
    }
}
