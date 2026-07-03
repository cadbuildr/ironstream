// FILE: top_ope_b_rep_p_line_inter.rs
// occt: TopOpeBRep_PLineInter

/// Pointer type alias for LineInter.
pub type PLineInter = Option<Box<LineInter>>;

/// Line intersection structure.
pub struct LineInter {
    data: Vec<u8>,
}

impl LineInter {
    /// Create a new LineInter.
    pub fn new() -> Self {
        LineInter { data: Vec::new() }
    }
}

impl Default for LineInter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let line = LineInter::new();
        assert_eq!(line.data.len(), 0);
    }

    #[test]
    fn test_pointer_type() {
        let p: PLineInter = Some(Box::new(LineInter::new()));
        assert!(p.is_some());
    }

    #[test]
    fn test_null_pointer() {
        let p: PLineInter = None;
        assert!(p.is_none());
    }
}
