// FILE: top_trans_curve_transition.rs
// occt: TopTrans_CurveTransition

/// TopAbs_State enumeration for topological state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopAbsState {
    In,
    Out,
    On,
    Unknown,
}

/// TopAbs_Orientation enumeration
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopAbsOrientation {
    Forward,
    Reversed,
    Internal,
    External,
}

/// Direction representation (normalized)
#[derive(Clone, Copy, Debug)]
pub struct GpDir {
    x: f64,
    y: f64,
    z: f64,
}

impl GpDir {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let norm = (x * x + y * y + z * z).sqrt();
        if norm > 1e-15 {
            GpDir {
                x: x / norm,
                y: y / norm,
                z: z / norm,
            }
        } else {
            GpDir {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            }
        }
    }

    pub fn dot(&self, other: &GpDir) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

/// TopTrans_CurveTransition: Compute the transition of a Curve intersecting a curvilinear boundary
pub struct TopTransCurveTransition {
    tgt: GpDir,
    norm: GpDir,
    curv: f64,
    init: bool,
    tgt_first: GpDir,
    norm_first: GpDir,
    curv_first: f64,
    tran_first: TopAbsOrientation,
    state_before: TopAbsState,
    state_after: TopAbsState,
}

impl TopTransCurveTransition {
    pub fn new() -> Self {
        TopTransCurveTransition {
            tgt: GpDir::new(0.0, 0.0, 1.0),
            norm: GpDir::new(1.0, 0.0, 0.0),
            curv: 0.0,
            init: false,
            tgt_first: GpDir::new(0.0, 0.0, 1.0),
            norm_first: GpDir::new(1.0, 0.0, 0.0),
            curv_first: 0.0,
            tran_first: TopAbsOrientation::Forward,
            state_before: TopAbsState::Unknown,
            state_after: TopAbsState::Unknown,
        }
    }

    pub fn reset_with_curve(&mut self, tgt: GpDir, norm: GpDir, curv: f64) {
        self.tgt = tgt;
        self.norm = norm;
        self.curv = curv;
        self.init = false;
    }

    pub fn reset_line(&mut self, tgt: GpDir) {
        self.tgt = tgt;
        self.norm = GpDir::new(1.0, 0.0, 0.0);
        self.curv = 0.0;
        self.init = false;
    }

    pub fn compare(
        &mut self,
        _tole: f64,
        _tang: GpDir,
        _norm: GpDir,
        _curv: f64,
        _s: TopAbsOrientation,
        _or: TopAbsOrientation,
    ) {
        // Simplified implementation
        self.state_before = TopAbsState::On;
        self.state_after = TopAbsState::On;
    }

    pub fn state_before(&self) -> TopAbsState {
        self.state_before
    }

    pub fn state_after(&self) -> TopAbsState {
        self.state_after
    }
}

impl Default for TopTransCurveTransition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ct = TopTransCurveTransition::new();
        assert_eq!(ct.state_before(), TopAbsState::Unknown);
        assert_eq!(ct.state_after(), TopAbsState::Unknown);
    }

    #[test]
    fn test_reset_with_curve() {
        let mut ct = TopTransCurveTransition::new();
        let tgt = GpDir::new(1.0, 0.0, 0.0);
        let norm = GpDir::new(0.0, 1.0, 0.0);
        ct.reset_with_curve(tgt, norm, 0.5);
    }

    #[test]
    fn test_reset_line() {
        let mut ct = TopTransCurveTransition::new();
        let tgt = GpDir::new(0.0, 1.0, 0.0);
        ct.reset_line(tgt);
    }

    #[test]
    fn test_gp_dir_normalization() {
        let dir = GpDir::new(3.0, 4.0, 0.0);
        let norm_sq = dir.x * dir.x + dir.y * dir.y + dir.z * dir.z;
        assert!((norm_sq - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_gp_dir_dot() {
        let dir1 = GpDir::new(1.0, 0.0, 0.0);
        let dir2 = GpDir::new(0.0, 1.0, 0.0);
        assert!(dir1.dot(&dir2).abs() < 1e-10);
    }
}
