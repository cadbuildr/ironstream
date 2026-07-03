// FILE: hlrb_rep_area_limit.rs
// occt: HLRBRep_AreaLimit

use crate::hlr_algo_intersection::HlrAlgoIntersection;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TopAbsState {
    In = 0,
    Out = 1,
    On = 2,
    Unknown = 3,
}

/// A vertex on an edge with visibility state information.
pub struct HlrbrépAreaLimit {
    vertex: HlrAlgoIntersection,
    boundary: bool,
    interference: bool,
    state_before: TopAbsState,
    state_after: TopAbsState,
    edge_before: TopAbsState,
    edge_after: TopAbsState,
    previous: Option<Box<HlrbrépAreaLimit>>,
    next: Option<Box<HlrbrépAreaLimit>>,
}

impl HlrbrépAreaLimit {
    /// Create an area limit with visibility state.
    pub fn new(
        vertex: HlrAlgoIntersection,
        boundary: bool,
        interference: bool,
        state_bef: TopAbsState,
        state_aft: TopAbsState,
        edge_bef: TopAbsState,
        edge_aft: TopAbsState,
    ) -> Self {
        HlrbrépAreaLimit {
            vertex,
            boundary,
            interference,
            state_before: state_bef,
            state_after: state_aft,
            edge_before: edge_bef,
            edge_after: edge_aft,
            previous: None,
            next: None,
        }
    }

    /// Get the vertex (intersection point).
    pub fn vertex(&self) -> &HlrAlgoIntersection {
        &self.vertex
    }

    /// Check if this is a boundary.
    pub fn is_boundary(&self) -> bool {
        self.boundary
    }

    /// Check if this is an interference.
    pub fn is_interference(&self) -> bool {
        self.interference
    }

    /// Get state before intersection.
    pub fn state_before(&self) -> TopAbsState {
        self.state_before
    }

    /// Set state before.
    pub fn set_state_before(&mut self, s: TopAbsState) {
        self.state_before = s;
    }

    /// Get state after intersection.
    pub fn state_after(&self) -> TopAbsState {
        self.state_after
    }

    /// Set state after.
    pub fn set_state_after(&mut self, s: TopAbsState) {
        self.state_after = s;
    }

    /// Get edge state before.
    pub fn edge_before(&self) -> TopAbsState {
        self.edge_before
    }

    /// Set edge state before.
    pub fn set_edge_before(&mut self, s: TopAbsState) {
        self.edge_before = s;
    }

    /// Get edge state after.
    pub fn edge_after(&self) -> TopAbsState {
        self.edge_after
    }

    /// Set edge state after.
    pub fn set_edge_after(&mut self, s: TopAbsState) {
        self.edge_after = s;
    }

    /// Clear linked nodes.
    pub fn clear(&mut self) {
        self.previous = None;
        self.next = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vertex = HlrAlgoIntersection::new();
        let al = HlrbrépAreaLimit::new(
            vertex,
            true,
            false,
            TopAbsState::In,
            TopAbsState::Out,
            TopAbsState::On,
            TopAbsState::In,
        );
        assert!(al.is_boundary());
        assert!(!al.is_interference());
        assert_eq!(al.state_before(), TopAbsState::In);
        assert_eq!(al.state_after(), TopAbsState::Out);
    }

    #[test]
    fn test_set_state() {
        let vertex = HlrAlgoIntersection::new();
        let mut al = HlrbrépAreaLimit::new(
            vertex,
            false,
            true,
            TopAbsState::Out,
            TopAbsState::In,
            TopAbsState::In,
            TopAbsState::Out,
        );
        al.set_state_before(TopAbsState::In);
        al.set_state_after(TopAbsState::Out);
        assert_eq!(al.state_before(), TopAbsState::In);
        assert_eq!(al.state_after(), TopAbsState::Out);
    }

    #[test]
    fn test_edge_state() {
        let vertex = HlrAlgoIntersection::new();
        let mut al = HlrbrépAreaLimit::new(
            vertex,
            true,
            true,
            TopAbsState::On,
            TopAbsState::In,
            TopAbsState::Out,
            TopAbsState::In,
        );
        al.set_edge_before(TopAbsState::In);
        al.set_edge_after(TopAbsState::Out);
        assert_eq!(al.edge_before(), TopAbsState::In);
        assert_eq!(al.edge_after(), TopAbsState::Out);
    }
}
