// FILE: b_rep_prim_api_make_sweep.rs
// occt: BRepPrimAPI_MakeSweep

/// Abstract root class of swept primitives.
#[derive(Debug, Clone)]
pub struct BRepPrimApiMakeSweep;

impl BRepPrimApiMakeSweep {
    pub fn new() -> Self {
        BRepPrimApiMakeSweep
    }
}

impl Default for BRepPrimApiMakeSweep {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_sweep() {
        let _ms = BRepPrimApiMakeSweep::new();
    }
}
