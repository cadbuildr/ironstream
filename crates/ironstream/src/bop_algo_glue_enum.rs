// FILE: bop_algo_glue_enum.rs
// occt: BOPAlgo_GlueEnum

/// Enumeration for glue types in Boolean operations
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BopAlgoGlueType {
    /// No glue
    Off = 0,
    /// Glue fully coinciding faces
    Full = 1,
    /// Glue faces that are coinciding in some area
    Shift = 2,
}

impl From<i32> for BopAlgoGlueType {
    fn from(val: i32) -> Self {
        match val {
            0 => BopAlgoGlueType::Off,
            1 => BopAlgoGlueType::Full,
            2 => BopAlgoGlueType::Shift,
            _ => BopAlgoGlueType::Off,
        }
    }
}

impl Into<i32> for BopAlgoGlueType {
    fn into(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glue_type_off() {
        let glue = BopAlgoGlueType::Off;
        assert_eq!(glue as i32, 0);
    }

    #[test]
    fn test_glue_type_full() {
        let glue = BopAlgoGlueType::Full;
        assert_eq!(glue as i32, 1);
    }

    #[test]
    fn test_glue_type_from() {
        let glue = BopAlgoGlueType::from(1);
        assert_eq!(glue, BopAlgoGlueType::Full);
    }

    #[test]
    fn test_glue_type_invalid() {
        let glue = BopAlgoGlueType::from(999);
        assert_eq!(glue, BopAlgoGlueType::Off);
    }
}
