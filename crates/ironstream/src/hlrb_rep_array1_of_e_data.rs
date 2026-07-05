// FILE: hlrb_rep_array1_of_e_data.rs
// occt: HLRBRep_Array1OfEData

//! Deprecated: Use Vec<EData> directly.
//! Array of edge data for HLR.

#[derive(Clone, Debug)]
pub struct EData {
    pub edge_id: usize,
    pub flag: u8,
}

impl EData {
    pub fn new(edge_id: usize, flag: u8) -> Self {
        EData { edge_id, flag }
    }
}

pub type HLRBRepArray1OfEData = Vec<EData>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let mut array: HLRBRepArray1OfEData = Vec::new();
        array.push(EData::new(1, 0));

        assert_eq!(array.len(), 1);
        assert_eq!(array[0].edge_id, 1);
        assert_eq!(array[0].flag, 0);
    }

    #[test]
    fn test_array_operations() {
        let array = vec![
            EData::new(1, 0),
            EData::new(2, 1),
            EData::new(3, 0),
        ];

        assert_eq!(array.len(), 3);
        assert_eq!(array[1].flag, 1);
    }

    #[test]
    fn test_array_iteration() {
        let array = vec![
            EData::new(10, 1),
            EData::new(20, 0),
        ];

        let flags: Vec<u8> = array.iter().map(|e| e.flag).collect();
        assert_eq!(flags, vec![1, 0]);
    }
}
