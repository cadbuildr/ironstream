// FILE: top_trans_surface_transition.rs
// occt: TopTrans_SurfaceTransition

/// Transition information for surface operations
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionType {
    In = 0,
    Out = 1,
    Touch = 2,
    Undecided = 3,
}

/// Surface transition information
#[derive(Clone, Debug)]
pub struct TopTransSurfaceTransition {
    transition: TransitionType,
}

impl TopTransSurfaceTransition {
    pub fn new() -> Self {
        TopTransSurfaceTransition {
            transition: TransitionType::Undecided,
        }
    }

    pub fn set_transition(&mut self, transition: TransitionType) {
        self.transition = transition;
    }

    pub fn transition(&self) -> TransitionType {
        self.transition
    }

    pub fn is_in(&self) -> bool {
        self.transition == TransitionType::In
    }

    pub fn is_out(&self) -> bool {
        self.transition == TransitionType::Out
    }

    pub fn is_touching(&self) -> bool {
        self.transition == TransitionType::Touch
    }
}

impl Default for TopTransSurfaceTransition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let trans = TopTransSurfaceTransition::new();
        assert_eq!(trans.transition(), TransitionType::Undecided);
    }

    #[test]
    fn test_set_transition() {
        let mut trans = TopTransSurfaceTransition::new();
        trans.set_transition(TransitionType::In);
        assert!(trans.is_in());
    }

    #[test]
    fn test_is_out() {
        let mut trans = TopTransSurfaceTransition::new();
        trans.set_transition(TransitionType::Out);
        assert!(trans.is_out());
        assert!(!trans.is_in());
    }

    #[test]
    fn test_is_touching() {
        let mut trans = TopTransSurfaceTransition::new();
        trans.set_transition(TransitionType::Touch);
        assert!(trans.is_touching());
    }
}
