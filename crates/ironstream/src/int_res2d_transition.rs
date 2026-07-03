// FILE: int_res2d_transition.rs
// occt: IntRes2d_Transition

use crate::int_res2d_position::Position;
use crate::int_res2d_type_trans::TypeTrans;
use crate::int_res2d_situation::Situation;

/// Represents a transition near an intersection point between two curves
#[derive(Clone, Debug, Copy)]
pub struct Transition {
    is_tangent: bool,
    position: Position,
    trans_type: i32, // 0=In, 1=Out, 2=Touch, 3=Undecided
    situation: i32,  // 0=Inside, 1=Outside, 2=Unknown
    is_opposite: bool,
}

impl Transition {
    /// Create an empty transition
    pub fn new() -> Self {
        Transition {
            is_tangent: false,
            position: Position::Head,
            trans_type: 3, // Undecided
            situation: 2,
            is_opposite: false,
        }
    }

    /// Create an IN or OUT transition
    pub fn with_type(tangent: bool, pos: Position, trans_type: TypeTrans) -> Self {
        let tt = match trans_type {
            TypeTrans::In => 0,
            TypeTrans::Out => 1,
            TypeTrans::Touch => 2,
            TypeTrans::Undecided => 3,
        };
        Transition {
            is_tangent: tangent,
            position: pos,
            trans_type: tt,
            situation: 2,
            is_opposite: false,
        }
    }

    /// Create a TOUCH transition
    pub fn touch(tangent: bool, pos: Position, situ: Situation, oppos: bool) -> Self {
        let sit = match situ {
            Situation::Inside => 0,
            Situation::Outside => 1,
            Situation::Unknown => 2,
        };
        Transition {
            is_tangent: tangent,
            position: pos,
            trans_type: 2, // Touch
            situation: sit,
            is_opposite: oppos,
        }
    }

    /// Create an UNDECIDED transition
    pub fn undecided(pos: Position) -> Self {
        Transition {
            is_tangent: false,
            position: pos,
            trans_type: 3,
            situation: 2,
            is_opposite: false,
        }
    }

    /// Set transition with IN or OUT type
    pub fn set_type(&mut self, tangent: bool, pos: Position, trans_type: TypeTrans) {
        self.is_tangent = tangent;
        self.position = pos;
        self.trans_type = match trans_type {
            TypeTrans::In => 0,
            TypeTrans::Out => 1,
            TypeTrans::Touch => 2,
            TypeTrans::Undecided => 3,
        };
    }

    /// Set touch transition
    pub fn set_touch(&mut self, tangent: bool, pos: Position, situ: Situation, oppos: bool) {
        self.is_tangent = tangent;
        self.position = pos;
        self.trans_type = 2;
        self.situation = match situ {
            Situation::Inside => 0,
            Situation::Outside => 1,
            Situation::Unknown => 2,
        };
        self.is_opposite = oppos;
    }

    /// Set undecided transition
    pub fn set_undecided(&mut self, pos: Position) {
        self.position = pos;
        self.trans_type = 3;
    }

    /// Get position on curve
    pub fn position_on_curve(&self) -> Position {
        self.position
    }

    /// Get transition type
    pub fn trans_type(&self) -> TypeTrans {
        match self.trans_type {
            0 => TypeTrans::In,
            1 => TypeTrans::Out,
            2 => TypeTrans::Touch,
            _ => TypeTrans::Undecided,
        }
    }

    /// Check if transition is touch
    pub fn is_touch(&self) -> bool {
        self.trans_type == 2
    }

    /// Check if transition is undecided
    pub fn is_undecided(&self) -> bool {
        self.trans_type == 3
    }

    /// Check if tangent
    pub fn is_tangent_transition(&self) -> bool {
        self.is_tangent
    }

    /// Get situation (for touch transitions)
    pub fn situation(&self) -> Situation {
        match self.situation {
            0 => Situation::Inside,
            1 => Situation::Outside,
            _ => Situation::Unknown,
        }
    }

    /// Check if opposite
    pub fn is_opposite_transition(&self) -> bool {
        self.is_opposite
    }
}

impl Default for Transition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = Transition::new();
        assert!(!t.is_tangent_transition());
        assert!(t.is_undecided());
    }

    #[test]
    fn test_with_type() {
        let t = Transition::with_type(false, Position::Middle, TypeTrans::In);
        assert_eq!(t.trans_type(), TypeTrans::In);
        assert_eq!(t.position_on_curve(), Position::Middle);
    }

    #[test]
    fn test_touch() {
        let t = Transition::touch(true, Position::End, Situation::Inside, false);
        assert!(t.is_touch());
        assert!(t.is_tangent_transition());
        assert_eq!(t.situation(), Situation::Inside);
    }

    #[test]
    fn test_undecided() {
        let t = Transition::undecided(Position::Head);
        assert!(t.is_undecided());
        assert_eq!(t.position_on_curve(), Position::Head);
    }

    #[test]
    fn test_setters() {
        let mut t = Transition::new();
        t.set_type(true, Position::Middle, TypeTrans::Out);
        assert_eq!(t.trans_type(), TypeTrans::Out);
        assert!(t.is_tangent_transition());
    }
}
