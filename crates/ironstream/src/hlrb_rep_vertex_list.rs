// FILE: hlrb_rep_vertex_list.rs
// occt: HLRBRep_VertexList

/// Iterates over vertices in an edge, handling both boundary points and interferences.
#[derive(Clone, Debug)]
pub struct HLRBRepVertexList {
    index: usize,
    count: usize,
    is_periodic: bool,
}

impl HLRBRepVertexList {
    pub fn new(is_periodic: bool) -> Self {
        HLRBRepVertexList {
            index: 0,
            count: 0,
            is_periodic,
        }
    }

    pub fn is_periodic(&self) -> bool {
        self.is_periodic
    }

    pub fn more(&self) -> bool {
        self.index < self.count
    }

    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn is_boundary(&self) -> bool {
        false
    }

    pub fn is_interference(&self) -> bool {
        false
    }

    pub fn orientation(&self) -> i32 {
        0
    }

    pub fn transition(&self) -> i32 {
        0
    }

    pub fn boundary_transition(&self) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let vl = HLRBRepVertexList::new(false);
        assert!(!vl.is_periodic());
        assert!(!vl.more());
    }

    #[test]
    fn test_periodic() {
        let vl = HLRBRepVertexList::new(true);
        assert!(vl.is_periodic());
    }
}
