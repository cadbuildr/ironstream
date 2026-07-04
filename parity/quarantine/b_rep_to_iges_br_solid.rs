// FILE: b_rep_to_iges_br_solid.rs
// occt: BRepToIGES_BRSolid

/// Class for transferring Solid entities from TopoDS to IGES.
pub struct BRSolid {
    base: BREntity,
}

pub struct BREntity;

impl BRSolid {
    pub fn new() -> Self {
        BRSolid {
            base: BREntity,
        }
    }
}

impl Default for BRSolid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solid = BRSolid::new();
        assert!(true);
    }
}
