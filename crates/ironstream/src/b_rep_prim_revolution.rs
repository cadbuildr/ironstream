// FILE: b_rep_prim_revolution.rs
// occt: BRepPrim_Revolution

/// Implement the OneAxis algorithm for a revolution surface.
#[derive(Debug, Clone)]
pub struct BRepPrimRevolution;

impl BRepPrimRevolution {
    pub fn new() -> Self {
        BRepPrimRevolution
    }
}

impl Default for BRepPrimRevolution {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_revolution() {
        let _rev = BRepPrimRevolution::new();
    }
}
