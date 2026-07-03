// FILE: int_res2d_situation.rs
// occt: IntRes2d_Situation

/// Situation enumeration for curve analysis
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Situation {
    Inside,
    Outside,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_situation_variants() {
        let _s1 = Situation::Inside;
        let _s2 = Situation::Outside;
        let _s3 = Situation::Unknown;
        assert_ne!(Situation::Inside, Situation::Outside);
        assert_ne!(Situation::Outside, Situation::Unknown);
    }
}
