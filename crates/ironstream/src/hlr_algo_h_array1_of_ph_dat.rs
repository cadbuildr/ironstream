// FILE: hlr_algo_h_array1_of_ph_dat.rs
// occt: HLRAlgo_HArray1OfPHDat

//! Deprecated: Use Arc<Vec<PhDat>> directly.
//! Handle-based array of hidden line removal data.

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct PhDat {
    pub id: usize,
    pub value: f64,
}

impl PhDat {
    pub fn new(id: usize, value: f64) -> Self {
        PhDat { id, value }
    }
}

pub type HLRAlgoHArray1OfPhDat = Arc<Vec<PhDat>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_array_creation() {
        let vec = vec![PhDat::new(1, 1.5)];
        let h_array: HLRAlgoHArray1OfPhDat = Arc::new(vec);

        assert_eq!(h_array.len(), 1);
        assert_eq!(h_array[0].value, 1.5);
    }

    #[test]
    fn test_h_array_shared() {
        let vec = vec![PhDat::new(1, 10.0)];
        let h_array1 = Arc::new(vec);
        let h_array2 = Arc::clone(&h_array1);

        assert_eq!(Arc::strong_count(&h_array1), 2);
        assert_eq!(h_array2[0].value, 10.0);
    }

    #[test]
    fn test_h_array_immutable() {
        let vec = vec![
            PhDat::new(1, 1.0),
            PhDat::new(2, 2.0),
        ];
        let h_array: HLRAlgoHArray1OfPhDat = Arc::new(vec);

        assert_eq!(h_array.len(), 2);
        assert_eq!(h_array[1].id, 2);
    }
}
