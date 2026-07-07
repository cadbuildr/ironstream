// FILE: step_visual_text_path.rs
// occt: StepVisual_TextPath

//! Faithful port of OCCT `StepVisual_TextPath` (StepVisual_TextPath.hxx):
//! the STEP text_path enumeration — direction in which a text literal
//! flows from its placement point.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StepVisualTextPath {
    /// StepVisual_tpUp
    Up,
    /// StepVisual_tpRight
    Right,
    /// StepVisual_tpDown
    Down,
    /// StepVisual_tpLeft
    Left,
}

impl StepVisualTextPath {
    /// C++ enum numeric value (declaration order in the OCCT header).
    pub fn as_i32(self) -> i32 {
        match self {
            StepVisualTextPath::Up => 0,
            StepVisualTextPath::Right => 1,
            StepVisualTextPath::Down => 2,
            StepVisualTextPath::Left => 3,
        }
    }

    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(StepVisualTextPath::Up),
            1 => Some(StepVisualTextPath::Right),
            2 => Some(StepVisualTextPath::Down),
            3 => Some(StepVisualTextPath::Left),
            _ => None,
        }
    }

    /// STEP file token as written by the STEP writer (.UP., .RIGHT., ...).
    pub fn step_token(self) -> &'static str {
        match self {
            StepVisualTextPath::Up => ".UP.",
            StepVisualTextPath::Right => ".RIGHT.",
            StepVisualTextPath::Down => ".DOWN.",
            StepVisualTextPath::Left => ".LEFT.",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declaration_order_matches_occt() {
        // OCCT header order: tpUp, tpRight, tpDown, tpLeft
        assert_eq!(StepVisualTextPath::Up.as_i32(), 0);
        assert_eq!(StepVisualTextPath::Right.as_i32(), 1);
        assert_eq!(StepVisualTextPath::Down.as_i32(), 2);
        assert_eq!(StepVisualTextPath::Left.as_i32(), 3);
    }

    #[test]
    fn roundtrip() {
        for v in 0..4 {
            assert_eq!(StepVisualTextPath::from_i32(v).unwrap().as_i32(), v);
        }
        assert!(StepVisualTextPath::from_i32(4).is_none());
    }

    #[test]
    fn step_tokens() {
        assert_eq!(StepVisualTextPath::Left.step_token(), ".LEFT.");
        assert_eq!(StepVisualTextPath::Up.step_token(), ".UP.");
    }
}
