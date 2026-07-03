// FILE: hlr_algo_intersection.rs
// occt: HLRAlgo_Intersection

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TopAbsOrientation {
    Forward = 0,
    Reversed = 1,
    Internal = 2,
    External = 3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TopAbsState {
    In = 0,
    Out = 1,
    On = 2,
    Unknown = 3,
}

/// Describes an intersection on an edge to hide.
/// Contains parameter, level, segment index, and visibility state.
#[derive(Clone, Copy)]
pub struct HlrAlgoIntersection {
    orientation: TopAbsOrientation,
    seg_index: i32,
    index: i32,
    level: i32,
    param: f64,
    tolerance: f32,
    state: TopAbsState,
}

impl HlrAlgoIntersection {
    /// Create a default intersection.
    pub fn new() -> Self {
        HlrAlgoIntersection {
            orientation: TopAbsOrientation::Forward,
            seg_index: 0,
            index: 0,
            level: 0,
            param: 0.0,
            tolerance: 0.0,
            state: TopAbsState::On,
        }
    }

    /// Create an intersection with all parameters.
    pub fn with_values(
        ori: TopAbsOrientation,
        lev: i32,
        seg_ind: i32,
        ind: i32,
        p: f64,
        tol: f32,
        s: TopAbsState,
    ) -> Self {
        HlrAlgoIntersection {
            orientation: ori,
            seg_index: seg_ind,
            index: ind,
            level: lev,
            param: p,
            tolerance: tol,
            state: s,
        }
    }

    /// Set orientation.
    pub fn set_orientation(&mut self, ori: TopAbsOrientation) {
        self.orientation = ori;
    }

    /// Get orientation.
    pub fn orientation(&self) -> TopAbsOrientation {
        self.orientation
    }

    /// Set level.
    pub fn set_level(&mut self, lev: i32) {
        self.level = lev;
    }

    /// Get level.
    pub fn level(&self) -> i32 {
        self.level
    }

    /// Set segment index.
    pub fn set_seg_index(&mut self, seg_ind: i32) {
        self.seg_index = seg_ind;
    }

    /// Get segment index.
    pub fn seg_index(&self) -> i32 {
        self.seg_index
    }

    /// Set index.
    pub fn set_index(&mut self, ind: i32) {
        self.index = ind;
    }

    /// Get index.
    pub fn index(&self) -> i32 {
        self.index
    }

    /// Set parameter.
    pub fn set_parameter(&mut self, p: f64) {
        self.param = p;
    }

    /// Get parameter.
    pub fn parameter(&self) -> f64 {
        self.param
    }

    /// Set tolerance.
    pub fn set_tolerance(&mut self, t: f32) {
        self.tolerance = t;
    }

    /// Get tolerance.
    pub fn tolerance(&self) -> f32 {
        self.tolerance
    }

    /// Set state.
    pub fn set_state(&mut self, s: TopAbsState) {
        self.state = s;
    }

    /// Get state.
    pub fn state(&self) -> TopAbsState {
        self.state
    }
}

impl Default for HlrAlgoIntersection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let inter = HlrAlgoIntersection::new();
        assert_eq!(inter.orientation(), TopAbsOrientation::Forward);
        assert_eq!(inter.level(), 0);
        assert_eq!(inter.parameter(), 0.0);
        assert_eq!(inter.state(), TopAbsState::On);
    }

    #[test]
    fn test_with_values() {
        let inter = HlrAlgoIntersection::with_values(
            TopAbsOrientation::Reversed,
            2,
            5,
            10,
            0.75,
            0.01,
            TopAbsState::In,
        );
        assert_eq!(inter.orientation(), TopAbsOrientation::Reversed);
        assert_eq!(inter.level(), 2);
        assert_eq!(inter.seg_index(), 5);
        assert_eq!(inter.index(), 10);
        assert_eq!(inter.parameter(), 0.75);
        assert_eq!(inter.tolerance(), 0.01);
        assert_eq!(inter.state(), TopAbsState::In);
    }

    #[test]
    fn test_setters() {
        let mut inter = HlrAlgoIntersection::new();
        inter.set_orientation(TopAbsOrientation::Internal);
        inter.set_level(3);
        inter.set_parameter(0.5);
        inter.set_state(TopAbsState::Out);
        assert_eq!(inter.orientation(), TopAbsOrientation::Internal);
        assert_eq!(inter.level(), 3);
        assert_eq!(inter.parameter(), 0.5);
        assert_eq!(inter.state(), TopAbsState::Out);
    }
}
