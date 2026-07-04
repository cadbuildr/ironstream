// FILE: b_rep_to_iges_br_shell.rs
// occt: BRepToIGES_BRShell

/// Class for transferring Shell entities from TopoDS to IGES.
pub struct BRShell {
    base: BREntity,
}

pub struct BREntity;

impl BRShell {
    pub fn new() -> Self {
        BRShell {
            base: BREntity,
        }
    }
}

impl Default for BRShell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let shell = BRShell::new();
        assert!(true);
    }
}
