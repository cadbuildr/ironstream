// FILE: hlr_algo_array1_of_pi_nod.rs
// occt: HLRAlgo_Array1OfPINod

//! Deprecated: Use Vec<PINod> directly.
//! Array of node data for hidden line removal.

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

pub type HLRAlgoArray1OfPINod = Vec<PINod>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let mut array: HLRAlgoArray1OfPINod = Vec::new();
        array.push(PINod::new(0, 1));

        assert_eq!(array.len(), 1);
        assert_eq!(array[0].index, 0);
        assert_eq!(array[0].node_id, 1);
    }

    #[test]
    fn test_array_operations() {
        let array = vec![
            PINod::new(0, 10),
            PINod::new(1, 20),
            PINod::new(2, 30),
        ];

        assert_eq!(array.len(), 3);
        assert_eq!(array[1].node_id, 20);
    }

    #[test]
    fn test_array_iteration() {
        let array = vec![
            PINod::new(0, 100),
            PINod::new(1, 200),
        ];

        let ids: Vec<usize> = array.iter().map(|p| p.node_id).collect();
        assert_eq!(ids, vec![100, 200]);
    }
}
