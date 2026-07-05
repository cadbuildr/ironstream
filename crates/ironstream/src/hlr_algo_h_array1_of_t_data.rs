// FILE: hlr_algo_h_array1_of_t_data.rs
// occt: HLRAlgo_HArray1OfTData

//! Deprecated: Use Arc<Vec<TData>> directly.
//! Handle-based array of transient data for hidden line removal.

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TData {
    pub id: usize,
    pub data: f64,
}

impl TData {
    pub fn new(id: usize, data: f64) -> Self {
        TData { id, data }
    }
}

pub type HLRAlgoHArray1OfTData = Arc<Vec<TData>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_array_creation() {
        let vec = vec![TData::new(1, 1.5)];
        let h_array: HLRAlgoHArray1OfTData = Arc::new(vec);

        assert_eq!(h_array.len(), 1);
        assert_eq!(h_array[0].data, 1.5);
    }

    #[test]
    fn test_h_array_shared() {
        let vec = vec![TData::new(1, 10.0)];
        let h_array1 = Arc::new(vec);
        let h_array2 = Arc::clone(&h_array1);

        assert_eq!(Arc::strong_count(&h_array1), 2);
        assert_eq!(h_array2[0].data, 10.0);
    }

    #[test]
    fn test_h_array_immutable() {
        let vec = vec![
            TData::new(1, 1.0),
            TData::new(2, 2.0),
        ];
        let h_array: HLRAlgoHArray1OfTData = Arc::new(vec);

        assert_eq!(h_array.len(), 2);
        assert_eq!(h_array[1].id, 2);
    }
}
