// FILE: bin_l_drivers_marker.rs
// occt: BinLDrivers_Marker

/// Markers used in binary file format for document structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinLDriversMarker {
    EndAttrList = -1,
    EndLabel = -2,
}

impl BinLDriversMarker {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }

    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            -1 => Some(BinLDriversMarker::EndAttrList),
            -2 => Some(BinLDriversMarker::EndLabel),
            _ => None,
        }
    }

    pub fn is_end_marker(&self) -> bool {
        matches!(self, BinLDriversMarker::EndAttrList | BinLDriversMarker::EndLabel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end_attr_list_value() {
        assert_eq!(BinLDriversMarker::EndAttrList.as_i32(), -1);
    }

    #[test]
    fn test_end_label_value() {
        assert_eq!(BinLDriversMarker::EndLabel.as_i32(), -2);
    }

    #[test]
    fn test_from_i32_end_attr_list() {
        assert_eq!(BinLDriversMarker::from_i32(-1), Some(BinLDriversMarker::EndAttrList));
    }

    #[test]
    fn test_from_i32_end_label() {
        assert_eq!(BinLDriversMarker::from_i32(-2), Some(BinLDriversMarker::EndLabel));
    }

    #[test]
    fn test_from_i32_invalid() {
        assert_eq!(BinLDriversMarker::from_i32(0), None);
        assert_eq!(BinLDriversMarker::from_i32(1), None);
    }

    #[test]
    fn test_is_end_marker() {
        assert!(BinLDriversMarker::EndAttrList.is_end_marker());
        assert!(BinLDriversMarker::EndLabel.is_end_marker());
    }

    #[test]
    fn test_round_trip() {
        let marker = BinLDriversMarker::EndAttrList;
        let value = marker.as_i32();
        let restored = BinLDriversMarker::from_i32(value).unwrap();
        assert_eq!(marker, restored);
    }
}
