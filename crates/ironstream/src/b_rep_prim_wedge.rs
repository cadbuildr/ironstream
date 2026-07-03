// FILE: b_rep_prim_wedge.rs
// occt: BRepPrim_Wedge

/// Provides constructors for wedges without Builders.
#[derive(Debug, Clone)]
pub struct BRepPrimWedge;

impl BRepPrimWedge {
    pub fn new() -> Self {
        BRepPrimWedge
    }
}

impl Default for BRepPrimWedge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wedge() {
        let _w = BRepPrimWedge::new();
    }
}
