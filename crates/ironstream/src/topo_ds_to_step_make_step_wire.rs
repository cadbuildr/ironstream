// FILE: topo_ds_to_step_make_step_wire.rs
// occt: TopoDSToStep_MakeStepWire

use super::topo_ds_to_step_make_wire_error::MakeWireError;

pub struct MakeStepWire {
    result: Option<TopologicalRepresentationItem>,
    error: MakeWireError,
}

pub struct TopologicalRepresentationItem;

impl MakeStepWire {
    pub fn new() -> Self {
        MakeStepWire {
            result: None,
            error: MakeWireError::WireDone,
        }
    }

    pub fn value(&self) -> Option<&TopologicalRepresentationItem> {
        self.result.as_ref()
    }

    pub fn error(&self) -> MakeWireError {
        self.error
    }
}

impl Default for MakeStepWire {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let maker = MakeStepWire::new();
        assert!(maker.value().is_none());
    }
}
