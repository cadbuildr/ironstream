// FILE: b_rep_to_iges_br_wire.rs
// occt: BRepToIGES_BRWire

/// Class for transferring Wire entities from TopoDS to IGES.
pub struct BRWire {
    base: BREntity,
}

pub struct BREntity;

impl BRWire {
    pub fn new() -> Self {
        BRWire {
            base: BREntity,
        }
    }
}

impl Default for BRWire {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let wire = BRWire::new();
        assert!(true);
    }
}
