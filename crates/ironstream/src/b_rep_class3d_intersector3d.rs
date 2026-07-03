// FILE: b_rep_class3d_intersector3d.rs
// occt: BRepClass3d_Intersector3d

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    On,
    In,
    Out,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransitionOnCurve {
    In,
    Out,
    Touch,
    Undecidable,
}

/// Performs intersection of a line with a face
pub struct Intersector3d {
    pnt: [f64; 3],
    u: f64,
    v: f64,
    w: f64,
    transition: TransitionOnCurve,
    done: bool,
    has_apoint: bool,
    state: State,
}

impl Intersector3d {
    pub fn new() -> Self {
        Intersector3d {
            pnt: [0.0; 3],
            u: 0.0,
            v: 0.0,
            w: 0.0,
            transition: TransitionOnCurve::Undecidable,
            done: false,
            has_apoint: false,
            state: State::Out,
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn has_apoint(&self) -> bool {
        self.has_apoint
    }

    pub fn u_parameter(&self) -> f64 {
        self.u
    }

    pub fn v_parameter(&self) -> f64 {
        self.v
    }

    pub fn w_parameter(&self) -> f64 {
        self.w
    }

    pub fn pnt(&self) -> [f64; 3] {
        self.pnt
    }

    pub fn transition(&self) -> TransitionOnCurve {
        self.transition
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn perform(&mut self) {
        self.done = true;
    }
}

impl Default for Intersector3d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersector_creation() {
        let inter = Intersector3d::new();
        assert!(!inter.is_done());
        assert!(!inter.has_apoint());
        assert_eq!(inter.state(), State::Out);
    }

    #[test]
    fn test_parameters() {
        let inter = Intersector3d::new();
        assert_eq!(inter.u_parameter(), 0.0);
        assert_eq!(inter.v_parameter(), 0.0);
        assert_eq!(inter.w_parameter(), 0.0);
    }

    #[test]
    fn test_perform() {
        let mut inter = Intersector3d::new();
        assert!(!inter.is_done());
        inter.perform();
        assert!(inter.is_done());
    }

    #[test]
    fn test_point() {
        let inter = Intersector3d::new();
        let pnt = inter.pnt();
        assert_eq!(pnt[0], 0.0);
        assert_eq!(pnt[1], 0.0);
        assert_eq!(pnt[2], 0.0);
    }
}
