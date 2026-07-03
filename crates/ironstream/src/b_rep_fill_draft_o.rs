// FILE: b_rep_fill_draft_o.rs
// occt: BRepFill_Draft

/// Constructs a draft surface from a shape, direction, and angle.
/// This mirrors the OpenCascade BRepFill_Draft class for sweep-like draft operations.
pub struct BRepFillDraft {
    direction: [f64; 3],
    angle: f64,
    angle_min: f64,
    angle_max: f64,
    tolerance: f64,
    is_internal: bool,
    is_done: bool,
}

impl BRepFillDraft {
    /// Creates a new BRepFillDraft with the given direction and angle.
    pub fn new(dir: [f64; 3], angle: f64) -> Self {
        Self {
            direction: normalize(dir),
            angle,
            angle_min: 0.01,
            angle_max: 3.0,
            tolerance: 1e-7,
            is_internal: false,
            is_done: false,
        }
    }

    /// Sets the options for the draft operation.
    pub fn set_options(&mut self, angle_min: f64, angle_max: f64) {
        self.angle_min = angle_min;
        self.angle_max = angle_max;
    }

    /// Sets whether the draft is internal.
    pub fn set_draft(&mut self, is_internal: bool) {
        self.is_internal = is_internal;
    }

    /// Returns the direction vector.
    pub fn direction(&self) -> [f64; 3] {
        self.direction
    }

    /// Returns the draft angle in radians.
    pub fn angle(&self) -> f64 {
        self.angle
    }

    /// Returns whether the draft operation was successful.
    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

/// Normalizes a 3D vector.
fn normalize(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len > 1e-10 {
        [v[0] / len, v[1] / len, v[2] / len]
    } else {
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draft_creation() {
        let draft = BRepFillDraft::new([0.0, 0.0, 1.0], std::f64::consts::PI / 6.0);
        assert!(draft.angle() > 0.0);
        assert!(!draft.is_done());
    }

    #[test]
    fn test_direction_normalization() {
        let draft = BRepFillDraft::new([3.0, 4.0, 0.0], 0.1);
        let dir = draft.direction();
        let len_sq = dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2];
        assert!((len_sq - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_set_options() {
        let mut draft = BRepFillDraft::new([0.0, 0.0, 1.0], 0.1);
        draft.set_options(0.05, 2.5);
        assert!((draft.angle_min - 0.05).abs() < 1e-10);
        assert!((draft.angle_max - 2.5).abs() < 1e-10);
    }

    #[test]
    fn test_set_internal() {
        let mut draft = BRepFillDraft::new([0.0, 0.0, 1.0], 0.1);
        draft.set_draft(true);
        assert!(draft.is_internal);
    }
}
