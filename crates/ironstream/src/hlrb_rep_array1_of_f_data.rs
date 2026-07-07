// FILE: hlrb_rep_array1_of_f_data.rs
// occt: HLRBRep_Array1OfFData

//! Deprecated: Use Vec<FData> directly.
//! Array of face data for HLR.

#[derive(Clone, Debug)]
pub struct FData {
    pub face_id: usize,
    pub visible: bool,
}

impl FData {
    pub fn new(face_id: usize, visible: bool) -> Self {
        FData { face_id, visible }
    }
}

pub type HLRBRepArray1OfFData = Vec<FData>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let mut array: HLRBRepArray1OfFData = Vec::new();
        array.push(FData::new(1, true));

        assert_eq!(array.len(), 1);
        assert_eq!(array[0].face_id, 1);
        assert!(array[0].visible);
    }

    #[test]
    fn test_array_operations() {
        let array = vec![
            FData::new(1, true),
            FData::new(2, false),
            FData::new(3, true),
        ];

        assert_eq!(array.len(), 3);
        assert!(!array[1].visible);
    }

    #[test]
    fn test_array_iteration() {
        let array = vec![
            FData::new(10, true),
            FData::new(20, false),
        ];

        let visible_count = array.iter().filter(|f| f.visible).count();
        assert_eq!(visible_count, 1);
    }
}
