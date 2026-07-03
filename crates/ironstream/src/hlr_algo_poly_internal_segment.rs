// FILE: hlr_algo_poly_internal_segment.rs
// occt: HLRAlgo_PolyInternalSegment

/// An internal segment for polygon outline linking.
#[derive(Clone, Copy)]
pub struct HlrAlgoPolyInternalSegment {
    pub lst_sg1: i32,   // List segment 1
    pub lst_sg2: i32,   // List segment 2
    pub nxt_sg1: i32,   // Next segment 1
    pub nxt_sg2: i32,   // Next segment 2
    pub conex1: i32,    // Connection 1
    pub conex2: i32,    // Connection 2
}

impl HlrAlgoPolyInternalSegment {
    /// Create a default segment.
    pub fn new() -> Self {
        HlrAlgoPolyInternalSegment {
            lst_sg1: 0,
            lst_sg2: 0,
            nxt_sg1: 0,
            nxt_sg2: 0,
            conex1: 0,
            conex2: 0,
        }
    }

    /// Set list segment 1.
    pub fn set_lst_sg1(&mut self, v: i32) {
        self.lst_sg1 = v;
    }

    /// Set list segment 2.
    pub fn set_lst_sg2(&mut self, v: i32) {
        self.lst_sg2 = v;
    }

    /// Set next segment 1.
    pub fn set_nxt_sg1(&mut self, v: i32) {
        self.nxt_sg1 = v;
    }

    /// Set next segment 2.
    pub fn set_nxt_sg2(&mut self, v: i32) {
        self.nxt_sg2 = v;
    }

    /// Set connection 1.
    pub fn set_conex1(&mut self, v: i32) {
        self.conex1 = v;
    }

    /// Set connection 2.
    pub fn set_conex2(&mut self, v: i32) {
        self.conex2 = v;
    }
}

impl Default for HlrAlgoPolyInternalSegment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let seg = HlrAlgoPolyInternalSegment::new();
        assert_eq!(seg.lst_sg1, 0);
        assert_eq!(seg.nxt_sg2, 0);
        assert_eq!(seg.conex1, 0);
    }

    #[test]
    fn test_setters() {
        let mut seg = HlrAlgoPolyInternalSegment::new();
        seg.set_lst_sg1(1);
        seg.set_nxt_sg2(2);
        seg.set_conex1(3);
        assert_eq!(seg.lst_sg1, 1);
        assert_eq!(seg.nxt_sg2, 2);
        assert_eq!(seg.conex1, 3);
    }
}
