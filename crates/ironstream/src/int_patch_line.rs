// FILE: int_patch_line.rs
// occt: IntPatch_Line

/// Base class for intersection lines between two surfaces.
/// Can represent lines, circles, ellipses, analytical curves, walking curves, or restrictions.
#[derive(Clone, Debug)]
pub struct IntPatchLine {
    arc_type: i32, // 0=line, 1=circle, 2=ellipse, 3=parabola, 4=hyperbola, 5=analytic, 6=walking, 7=restriction
    is_tangent: bool,
    is_u_iso_s1: bool,
    is_v_iso_s1: bool,
    is_u_iso_s2: bool,
    is_v_iso_s2: bool,
    trans_s1: i32,
    trans_s2: i32,
    situ_s1: i32,
    situ_s2: i32,
}

impl IntPatchLine {
    /// Creates a new intersection line.
    pub fn new(arc_type: i32) -> Self {
        IntPatchLine {
            arc_type,
            is_tangent: false,
            is_u_iso_s1: false,
            is_v_iso_s1: false,
            is_u_iso_s2: false,
            is_v_iso_s2: false,
            trans_s1: 0, // IN
            trans_s2: 0, // IN
            situ_s1: 0,  // UNKNOWN
            situ_s2: 0,  // UNKNOWN
        }
    }

    /// Sets the isoparametric flags.
    pub fn set_value(&mut self, uiso1: bool, viso1: bool, uiso2: bool, viso2: bool) {
        self.is_u_iso_s1 = uiso1;
        self.is_v_iso_s1 = viso1;
        self.is_u_iso_s2 = uiso2;
        self.is_v_iso_s2 = viso2;
    }

    /// Returns the arc type (0=line, 1=circle, ..., 7=restriction).
    pub fn arc_type(&self) -> i32 {
        self.arc_type
    }

    /// Returns whether the line is a tangency line.
    pub fn is_tangent(&self) -> bool {
        self.is_tangent
    }

    /// Returns the transition type on surface 1 (0=IN, 1=OUT, 2=TOUCH, 3=UNDECIDED).
    pub fn transition_on_s1(&self) -> i32 {
        self.trans_s1
    }

    /// Returns the transition type on surface 2.
    pub fn transition_on_s2(&self) -> i32 {
        self.trans_s2
    }

    /// Returns the situation of S1 compared to S2 (0=UNKNOWN, 1=INSIDE, 2=OUTSIDE).
    pub fn situation_s1(&self) -> i32 {
        self.situ_s1
    }

    /// Returns the situation of S2 compared to S1.
    pub fn situation_s2(&self) -> i32 {
        self.situ_s2
    }

    /// Returns whether the line is a U-isoparametric curve on S1.
    pub fn is_u_iso_on_s1(&self) -> bool {
        self.is_u_iso_s1
    }

    /// Returns whether the line is a V-isoparametric curve on S1.
    pub fn is_v_iso_on_s1(&self) -> bool {
        self.is_v_iso_s1
    }

    /// Returns whether the line is a U-isoparametric curve on S2.
    pub fn is_u_iso_on_s2(&self) -> bool {
        self.is_u_iso_s2
    }

    /// Returns whether the line is a V-isoparametric curve on S2.
    pub fn is_v_iso_on_s2(&self) -> bool {
        self.is_v_iso_s2
    }

    /// Sets the tangency flag.
    pub fn set_tangent(&mut self, is_tangent: bool) {
        self.is_tangent = is_tangent;
    }

    /// Sets the transition types.
    pub fn set_transitions(&mut self, trans_s1: i32, trans_s2: i32) {
        self.trans_s1 = trans_s1;
        self.trans_s2 = trans_s2;
    }

    /// Sets the situation values.
    pub fn set_situations(&mut self, situ_s1: i32, situ_s2: i32) {
        self.situ_s1 = situ_s1;
        self.situ_s2 = situ_s2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_line() {
        let line = IntPatchLine::new(0);
        assert_eq!(line.arc_type(), 0);
        assert!(!line.is_tangent());
    }

    #[test]
    fn test_set_value() {
        let mut line = IntPatchLine::new(0);
        line.set_value(true, false, false, true);
        assert!(line.is_u_iso_on_s1());
        assert!(!line.is_v_iso_on_s1());
        assert!(!line.is_u_iso_on_s2());
        assert!(line.is_v_iso_on_s2());
    }

    #[test]
    fn test_set_tangent() {
        let mut line = IntPatchLine::new(0);
        line.set_tangent(true);
        assert!(line.is_tangent());
    }

    #[test]
    fn test_set_transitions() {
        let mut line = IntPatchLine::new(0);
        line.set_transitions(1, 2);
        assert_eq!(line.transition_on_s1(), 1);
        assert_eq!(line.transition_on_s2(), 2);
    }

    #[test]
    fn test_set_situations() {
        let mut line = IntPatchLine::new(0);
        line.set_situations(1, 2);
        assert_eq!(line.situation_s1(), 1);
        assert_eq!(line.situation_s2(), 2);
    }

    #[test]
    fn test_different_arc_types() {
        for arc_type in 0..8 {
            let line = IntPatchLine::new(arc_type);
            assert_eq!(line.arc_type(), arc_type);
        }
    }
}
