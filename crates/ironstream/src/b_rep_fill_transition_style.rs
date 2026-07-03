// FILE: b_rep_fill_transition_style.rs
// occt: BRepFill_TransitionStyle

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BRepFillTransitionStyle {
    Modified = 0,
    Right = 1,
    Round = 2,
}

impl Default for BRepFillTransitionStyle {
    fn default() -> Self {
        BRepFillTransitionStyle::Modified
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition_style_values() {
        assert_eq!(BRepFillTransitionStyle::Modified as u32, 0);
        assert_eq!(BRepFillTransitionStyle::Right as u32, 1);
        assert_eq!(BRepFillTransitionStyle::Round as u32, 2);
    }

    #[test]
    fn test_transition_style_equality() {
        assert_eq!(BRepFillTransitionStyle::Modified, BRepFillTransitionStyle::Modified);
        assert_ne!(BRepFillTransitionStyle::Modified, BRepFillTransitionStyle::Right);
    }

    #[test]
    fn test_transition_style_default() {
        assert_eq!(BRepFillTransitionStyle::default(), BRepFillTransitionStyle::Modified);
    }

    #[test]
    fn test_transition_style_copy() {
        let ts = BRepFillTransitionStyle::Right;
        let ts_copy = ts;
        assert_eq!(ts, ts_copy);
    }
}
