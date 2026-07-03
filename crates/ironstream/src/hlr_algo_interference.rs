// FILE: hlr_algo_interference.rs
// occt: HLRAlgo_Interference

use crate::hlr_algo_intersection::HlrAlgoIntersection;
use crate::hlr_algo_coincidence::HlrAlgoCoincidence;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TopAbsOrientation {
    Forward = 0,
    Reversed = 1,
    Internal = 2,
    External = 3,
}

/// Describes interference between an edge and a face at an intersection point.
#[derive(Clone)]
pub struct HlrAlgoInterference {
    intersection: HlrAlgoIntersection,
    boundary: HlrAlgoCoincidence,
    orientation: TopAbsOrientation,
    transition: TopAbsOrientation,
    b_transition: TopAbsOrientation,
}

impl HlrAlgoInterference {
    /// Create a default interference.
    pub fn new() -> Self {
        HlrAlgoInterference {
            intersection: HlrAlgoIntersection::new(),
            boundary: HlrAlgoCoincidence::new(),
            orientation: TopAbsOrientation::Forward,
            transition: TopAbsOrientation::Forward,
            b_transition: TopAbsOrientation::Forward,
        }
    }

    /// Create an interference with all parameters.
    pub fn with_values(
        inters: HlrAlgoIntersection,
        bound: HlrAlgoCoincidence,
        orient: TopAbsOrientation,
        trans: TopAbsOrientation,
        btrans: TopAbsOrientation,
    ) -> Self {
        HlrAlgoInterference {
            intersection: inters,
            boundary: bound,
            orientation: orient,
            transition: trans,
            b_transition: btrans,
        }
    }

    /// Set intersection data.
    pub fn set_intersection(&mut self, i: HlrAlgoIntersection) {
        self.intersection = i;
    }

    /// Get intersection data (immutable).
    pub fn intersection(&self) -> &HlrAlgoIntersection {
        &self.intersection
    }

    /// Get intersection data (mutable).
    pub fn intersection_mut(&mut self) -> &mut HlrAlgoIntersection {
        &mut self.intersection
    }

    /// Set boundary data.
    pub fn set_boundary(&mut self, b: HlrAlgoCoincidence) {
        self.boundary = b;
    }

    /// Get boundary data (immutable).
    pub fn boundary(&self) -> &HlrAlgoCoincidence {
        &self.boundary
    }

    /// Get boundary data (mutable).
    pub fn boundary_mut(&mut self) -> &mut HlrAlgoCoincidence {
        &mut self.boundary
    }

    /// Set orientation.
    pub fn set_orientation(&mut self, o: TopAbsOrientation) {
        self.orientation = o;
    }

    /// Get orientation.
    pub fn orientation(&self) -> TopAbsOrientation {
        self.orientation
    }

    /// Set transition.
    pub fn set_transition(&mut self, tr: TopAbsOrientation) {
        self.transition = tr;
    }

    /// Get transition.
    pub fn transition(&self) -> TopAbsOrientation {
        self.transition
    }

    /// Set boundary transition.
    pub fn set_boundary_transition(&mut self, btr: TopAbsOrientation) {
        self.b_transition = btr;
    }

    /// Get boundary transition.
    pub fn boundary_transition(&self) -> TopAbsOrientation {
        self.b_transition
    }
}

impl Default for HlrAlgoInterference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let intf = HlrAlgoInterference::new();
        assert_eq!(intf.orientation(), TopAbsOrientation::Forward);
        assert_eq!(intf.transition(), TopAbsOrientation::Forward);
    }

    #[test]
    fn test_set_orientation() {
        let mut intf = HlrAlgoInterference::new();
        intf.set_orientation(TopAbsOrientation::Reversed);
        assert_eq!(intf.orientation(), TopAbsOrientation::Reversed);
    }

    #[test]
    fn test_set_transition() {
        let mut intf = HlrAlgoInterference::new();
        intf.set_transition(TopAbsOrientation::Internal);
        assert_eq!(intf.transition(), TopAbsOrientation::Internal);
    }

    #[test]
    fn test_set_boundary_transition() {
        let mut intf = HlrAlgoInterference::new();
        intf.set_boundary_transition(TopAbsOrientation::External);
        assert_eq!(intf.boundary_transition(), TopAbsOrientation::External);
    }
}
