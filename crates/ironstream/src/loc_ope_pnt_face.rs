// FILE: loc_ope_pnt_face.rs
// occt: LocOpe_PntFace

/// Represents a point on a face with orientation and parametric information.
pub struct LocOpePntFace {
    pnt_id: usize,
    face_id: usize,
    orientation: i32,
    param: f64,
    u_param: f64,
    v_param: f64,
}

impl LocOpePntFace {
    /// Creates a new point on face.
    pub fn new() -> Self {
        LocOpePntFace {
            pnt_id: 0,
            face_id: 0,
            orientation: 0,
            param: 0.0,
            u_param: 0.0,
            v_param: 0.0,
        }
    }

    /// Creates a point on face with all parameters.
    pub fn with_params(
        pnt_id: usize,
        face_id: usize,
        orientation: i32,
        param: f64,
        u_param: f64,
        v_param: f64,
    ) -> Self {
        LocOpePntFace {
            pnt_id,
            face_id,
            orientation,
            param,
            u_param,
            v_param,
        }
    }

    /// Returns the point ID.
    pub fn pnt(&self) -> usize {
        self.pnt_id
    }

    /// Returns the face ID.
    pub fn face(&self) -> usize {
        self.face_id
    }

    /// Returns the orientation.
    pub fn orientation(&self) -> i32 {
        self.orientation
    }

    /// Changes the orientation.
    pub fn change_orientation(&mut self) -> &mut i32 {
        &mut self.orientation
    }

    /// Returns the parameter.
    pub fn parameter(&self) -> f64 {
        self.param
    }

    /// Returns the U parameter.
    pub fn u_parameter(&self) -> f64 {
        self.u_param
    }

    /// Returns the V parameter.
    pub fn v_parameter(&self) -> f64 {
        self.v_param
    }
}

impl Default for LocOpePntFace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pnt_face_creation() {
        let pf = LocOpePntFace::new();
        assert_eq!(pf.pnt(), 0);
        assert_eq!(pf.face(), 0);
        assert_eq!(pf.orientation(), 0);
    }

    #[test]
    fn test_pnt_face_with_params() {
        let pf = LocOpePntFace::with_params(1, 2, 1, 0.5, 0.3, 0.4);
        assert_eq!(pf.pnt(), 1);
        assert_eq!(pf.face(), 2);
        assert_eq!(pf.parameter(), 0.5);
        assert_eq!(pf.u_parameter(), 0.3);
        assert_eq!(pf.v_parameter(), 0.4);
    }

    #[test]
    fn test_pnt_face_change_orientation() {
        let mut pf = LocOpePntFace::new();
        *pf.change_orientation() = 1;
        assert_eq!(pf.orientation(), 1);
    }
}
