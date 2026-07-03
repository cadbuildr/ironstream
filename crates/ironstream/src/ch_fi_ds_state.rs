// FILE: ch_fi_ds_state.rs
// occt: ChFiDS_State

/// Describes the different kinds of extremities of a fillet.
/// For a corner with 3 edges and three faces:
/// - AllSame: the three concavities are on the same side of the shape
/// - OnDiff: the edge of the fillet has a concave side different than the two other edges
/// - OnSame: the edge of the fillet has a concave side different than one other edge
///   and identical to the third edge
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChFiDsState {
    /// Concavity same as one edge, different from another
    OnSame,
    /// Concavity different from all others
    OnDiff,
    /// All concavities on the same side
    AllSame,
    /// Break point configuration
    BreakPoint,
    /// Free boundary
    FreeBoundary,
    /// Closed configuration
    Closed,
    /// Tangent configuration
    Tangent,
}

impl Default for ChFiDsState {
    fn default() -> Self {
        Self::FreeBoundary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        assert_eq!(ChFiDsState::OnSame, ChFiDsState::OnSame);
        assert_ne!(ChFiDsState::OnSame, ChFiDsState::OnDiff);
    }

    #[test]
    fn test_default() {
        assert_eq!(ChFiDsState::default(), ChFiDsState::FreeBoundary);
    }

    #[test]
    fn test_copy_semantics() {
        let state = ChFiDsState::BreakPoint;
        let state2 = state;
        assert_eq!(state, state2);
    }
}
