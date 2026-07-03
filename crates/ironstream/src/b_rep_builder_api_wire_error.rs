// FILE: b_rep_builder_api_wire_error.rs
// occt: BRepBuilderAPI_WireError

/// Indicates the outcome of wire construction, whether successful or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireError {
    /// No error occurred. The wire is correctly built.
    WireDone = 0,
    /// No initialization of the algorithm. Only an empty constructor was used.
    EmptyWire = 1,
    /// The last edge attempted to add was not connected to the wire.
    DisconnectedWire = 2,
    /// The wire with some singularity.
    NonManifoldWire = 3,
}

impl WireError {
    pub fn from_i32(val: i32) -> Option<Self> {
        match val {
            0 => Some(WireError::WireDone),
            1 => Some(WireError::EmptyWire),
            2 => Some(WireError::DisconnectedWire),
            3 => Some(WireError::NonManifoldWire),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_error_values() {
        assert_eq!(WireError::WireDone.as_i32(), 0);
        assert_eq!(WireError::EmptyWire.as_i32(), 1);
        assert_eq!(WireError::DisconnectedWire.as_i32(), 2);
        assert_eq!(WireError::NonManifoldWire.as_i32(), 3);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(WireError::from_i32(0), Some(WireError::WireDone));
        assert_eq!(WireError::from_i32(1), Some(WireError::EmptyWire));
        assert_eq!(WireError::from_i32(2), Some(WireError::DisconnectedWire));
        assert_eq!(WireError::from_i32(3), Some(WireError::NonManifoldWire));
        assert_eq!(WireError::from_i32(4), None);
    }

    #[test]
    fn test_wire_error_copy() {
        let err1 = WireError::WireDone;
        let err2 = err1;
        assert_eq!(err1, err2);
    }
}
