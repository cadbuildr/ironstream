// FILE: hlr_algo_h_array1_of_pi_seg.rs
// occt: HLRAlgo_HArray1OfPISeg

//! Deprecated: Use Arc<Vec<PISeg>> directly.
//! Handle-based array of segment data for hidden line removal.

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct PISeg {
    pub index: usize,
    pub seg_id: usize,
}

impl PISeg {
    pub fn new(index: usize, seg_id: usize) -> Self {
        PISeg { index, seg_id }
    }
}

pub type HLRAlgoHArray1OfPISeg = Arc<Vec<PISeg>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_array_creation() {
        let vec = vec![PISeg::new(0, 1)];
        let h_array: HLRAlgoHArray1OfPISeg = Arc::new(vec);

        assert_eq!(h_array.len(), 1);
        assert_eq!(h_array[0].seg_id, 1);
    }

    #[test]
    fn test_h_array_shared() {
        let vec = vec![PISeg::new(0, 10)];
        let h_array1 = Arc::new(vec);
        let h_array2 = Arc::clone(&h_array1);

        assert_eq!(Arc::strong_count(&h_array1), 2);
        assert_eq!(h_array2[0].seg_id, 10);
    }

    #[test]
    fn test_h_array_immutable() {
        let vec = vec![
            PISeg::new(0, 100),
            PISeg::new(1, 200),
        ];
        let h_array: HLRAlgoHArray1OfPISeg = Arc::new(vec);

        assert_eq!(h_array.len(), 2);
        assert_eq!(h_array[1].index, 1);
    }
}
