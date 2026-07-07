// FILE: hlr_algo_array1_of_t_data.rs
// occt: HLRAlgo_Array1OfTData

//! Deprecated: Use Vec<TData> directly.
//! Array of transient data for hidden line removal.

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

pub type HLRAlgoArray1OfTData = Vec<TData>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let mut array: HLRAlgoArray1OfTData = Vec::new();
        array.push(TData::new(1, 1.5));

        assert_eq!(array.len(), 1);
        assert_eq!(array[0].id, 1);
        assert_eq!(array[0].data, 1.5);
    }

    #[test]
    fn test_array_operations() {
        let array = vec![
            TData::new(1, 1.0),
            TData::new(2, 2.0),
            TData::new(3, 3.0),
        ];

        assert_eq!(array.len(), 3);
        assert_eq!(array[2].data, 3.0);
    }

    #[test]
    fn test_array_iteration() {
        let array = vec![
            TData::new(10, 100.0),
            TData::new(20, 200.0),
        ];

        let sum: f64 = array.iter().map(|t| t.data).sum();
        assert_eq!(sum, 300.0);
    }
}
