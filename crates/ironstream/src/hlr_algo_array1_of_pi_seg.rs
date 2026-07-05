// FILE: hlr_algo_array1_of_pi_seg.rs
// occt: HLRAlgo_Array1OfPISeg

//! Deprecated: Use Vec<PISeg> directly.
//! Array of segment data for hidden line removal.

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

pub type HLRAlgoArray1OfPISeg = Vec<PISeg>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let mut array: HLRAlgoArray1OfPISeg = Vec::new();
        array.push(PISeg::new(0, 1));

        assert_eq!(array.len(), 1);
        assert_eq!(array[0].index, 0);
        assert_eq!(array[0].seg_id, 1);
    }

    #[test]
    fn test_array_operations() {
        let array = vec![
            PISeg::new(0, 10),
            PISeg::new(1, 20),
            PISeg::new(2, 30),
        ];

        assert_eq!(array.len(), 3);
        assert_eq!(array[1].seg_id, 20);
    }

    #[test]
    fn test_array_iteration() {
        let array = vec![
            PISeg::new(0, 100),
            PISeg::new(1, 200),
        ];

        let ids: Vec<usize> = array.iter().map(|p| p.seg_id).collect();
        assert_eq!(ids, vec![100, 200]);
    }
}
