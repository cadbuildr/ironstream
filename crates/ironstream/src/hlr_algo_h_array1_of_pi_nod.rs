// FILE: hlr_algo_h_array1_of_pi_nod.rs
// occt: HLRAlgo_HArray1OfPINod

//! Deprecated: Use Arc<Vec<PINod>> directly.
//! Handle-based array of node data for hidden line removal.

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct PINod {
    pub index: usize,
    pub node_id: usize,
}

impl PINod {
    pub fn new(index: usize, node_id: usize) -> Self {
        PINod { index, node_id }
    }
}

pub type HLRAlgoHArray1OfPINod = Arc<Vec<PINod>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_array_creation() {
        let vec = vec![PINod::new(0, 1)];
        let h_array: HLRAlgoHArray1OfPINod = Arc::new(vec);

        assert_eq!(h_array.len(), 1);
        assert_eq!(h_array[0].node_id, 1);
    }

    #[test]
    fn test_h_array_shared() {
        let vec = vec![PINod::new(0, 10)];
        let h_array1 = Arc::new(vec);
        let h_array2 = Arc::clone(&h_array1);

        assert_eq!(Arc::strong_count(&h_array1), 2);
        assert_eq!(h_array2[0].node_id, 10);
    }

    #[test]
    fn test_h_array_immutable() {
        let vec = vec![
            PINod::new(0, 100),
            PINod::new(1, 200),
        ];
        let h_array: HLRAlgoHArray1OfPINod = Arc::new(vec);

        assert_eq!(h_array.len(), 2);
        assert_eq!(h_array[1].index, 1);
    }
}
