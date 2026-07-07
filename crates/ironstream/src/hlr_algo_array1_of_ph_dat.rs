// FILE: hlr_algo_array1_of_ph_dat.rs
// occt: HLRAlgo_Array1OfPHDat

//! Deprecated: Use Vec<PhDat> directly.
//! Array of hidden line removal data.

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

pub type HLRAlgoArray1OfPhDat = Vec<PhDat>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let mut array: HLRAlgoArray1OfPhDat = Vec::new();
        array.push(PhDat::new(1, 1.5));

        assert_eq!(array.len(), 1);
        assert_eq!(array[0].id, 1);
        assert_eq!(array[0].value, 1.5);
    }

    #[test]
    fn test_array_operations() {
        let array = vec![
            PhDat::new(1, 1.0),
            PhDat::new(2, 2.0),
            PhDat::new(3, 3.0),
        ];

        assert_eq!(array.len(), 3);
        assert_eq!(array[2].value, 3.0);
    }

    #[test]
    fn test_array_iteration() {
        let array = vec![
            PhDat::new(10, 100.0),
            PhDat::new(20, 200.0),
        ];

        let sum: f64 = array.iter().map(|p| p.value).sum();
        assert_eq!(sum, 300.0);
    }
}
