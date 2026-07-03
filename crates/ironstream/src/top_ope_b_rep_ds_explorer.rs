// FILE: top_ope_b_rep_ds_explorer.rs
// occt: TopOpeBRepDS_Explorer

use crate::top_ope_b_rep_ds_kind::Kind;

/// Explorer for all types in a data structure
#[derive(Debug, Clone)]
pub struct Explorer {
    /// Start index
    start_index: usize,
    /// End index
    end_index: usize,
    /// Current index
    current_index: usize,
    /// Kind being explored
    kind: Kind,
}

impl Explorer {
    /// Create new explorer
    pub fn new(kind: Kind, start: usize, end: usize) -> Self {
        Explorer {
            kind,
            start_index: start,
            end_index: end,
            current_index: start,
        }
    }

    /// Check if there are more elements
    pub fn more(&self) -> bool {
        self.current_index < self.end_index
    }

    /// Move to next element
    pub fn next(&mut self) {
        if self.current_index < self.end_index {
            self.current_index += 1;
        }
    }

    /// Get current index
    pub fn current(&self) -> usize {
        self.current_index
    }

    /// Get kind
    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// Reset explorer
    pub fn reset(&mut self) {
        self.current_index = self.start_index;
    }

    /// Initialize with new range
    pub fn init(&mut self, kind: Kind, start: usize, end: usize) {
        self.kind = kind;
        self.start_index = start;
        self.end_index = end;
        self.current_index = start;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explorer_new() {
        let exp = Explorer::new(Kind::Curve, 0, 5);
        assert!(exp.more());
        assert_eq!(exp.current(), 0);
        assert_eq!(exp.kind(), Kind::Curve);
    }

    #[test]
    fn test_explorer_iteration() {
        let mut exp = Explorer::new(Kind::Point, 1, 3);
        assert_eq!(exp.current(), 1);
        exp.next();
        assert_eq!(exp.current(), 2);
        exp.next();
        assert!(!exp.more());
    }

    #[test]
    fn test_explorer_init() {
        let mut exp = Explorer::new(Kind::Point, 0, 2);
        exp.init(Kind::Surface, 5, 10);
        assert_eq!(exp.kind(), Kind::Surface);
        assert_eq!(exp.current(), 5);
        assert!(exp.more());
    }
}
