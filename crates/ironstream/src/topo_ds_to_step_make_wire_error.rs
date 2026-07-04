// FILE: topo_ds_to_step_make_wire_error.rs
// occt: TopoDSToStep_MakeWireError

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MakeWireError {
    WireDone,
    NonManifoldWire,
    WireOther,
}

impl MakeWireError {
    pub fn is_success(&self) -> bool {
        matches!(self, MakeWireError::WireDone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_success() {
        assert!(MakeWireError::WireDone.is_success());
        assert!(!MakeWireError::NonManifoldWire.is_success());
    }
}
