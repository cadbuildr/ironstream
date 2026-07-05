// FILE: bin_drivers_marker.rs
// occt: BinDrivers_Marker

/// Marker for binary format structure
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinDriversMarker {
    Start,
    End,
    Reference,
    Data,
    Unknown,
}

impl BinDriversMarker {
    /// Creates a marker from a byte value
    pub fn from_byte(value: u8) -> Self {
        match value {
            0 => BinDriversMarker::Start,
            1 => BinDriversMarker::End,
            2 => BinDriversMarker::Reference,
            3 => BinDriversMarker::Data,
            _ => BinDriversMarker::Unknown,
        }
    }

    /// Converts marker to byte value
    pub fn to_byte(&self) -> u8 {
        match self {
            BinDriversMarker::Start => 0,
            BinDriversMarker::End => 1,
            BinDriversMarker::Reference => 2,
            BinDriversMarker::Data => 3,
            BinDriversMarker::Unknown => 255,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_to_byte() {
        assert_eq!(BinDriversMarker::Start.to_byte(), 0);
        assert_eq!(BinDriversMarker::End.to_byte(), 1);
        assert_eq!(BinDriversMarker::Reference.to_byte(), 2);
        assert_eq!(BinDriversMarker::Data.to_byte(), 3);
    }

    #[test]
    fn test_marker_from_byte() {
        assert_eq!(BinDriversMarker::from_byte(0), BinDriversMarker::Start);
        assert_eq!(BinDriversMarker::from_byte(1), BinDriversMarker::End);
        assert_eq!(BinDriversMarker::from_byte(2), BinDriversMarker::Reference);
        assert_eq!(BinDriversMarker::from_byte(3), BinDriversMarker::Data);
        assert_eq!(BinDriversMarker::from_byte(255), BinDriversMarker::Unknown);
    }

    #[test]
    fn test_roundtrip() {
        let marker = BinDriversMarker::Reference;
        let byte = marker.to_byte();
        let restored = BinDriversMarker::from_byte(byte);
        assert_eq!(marker, restored);
    }
}
