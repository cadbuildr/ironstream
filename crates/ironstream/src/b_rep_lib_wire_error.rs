// FILE: b_rep_lib_wire_error.rs
// occt: BRepLib_WireError

/// Errors that can occur at wire construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BRepLibWireError {
    /// Wire constructed successfully
    WireDone = 0,
    /// Wire is empty
    EmptyWire = 1,
    /// Wire has disconnected edges
    DisconnectedWire = 2,
    /// Wire is non-manifold (edges connect incorrectly)
    NonManifoldWire = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_error_values() {
        assert_eq!(BRepLibWireError::WireDone as i32, 0);
        assert_eq!(BRepLibWireError::EmptyWire as i32, 1);
        assert_eq!(BRepLibWireError::DisconnectedWire as i32, 2);
        assert_eq!(BRepLibWireError::NonManifoldWire as i32, 3);
    }

    #[test]
    fn test_wire_error_copy() {
        let err1 = BRepLibWireError::DisconnectedWire;
        let err2 = err1;
        assert_eq!(err1, err2);
    }
}
