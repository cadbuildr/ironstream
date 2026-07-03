// FILE: b_rep_prim_api_make_wedge.rs
// occt: BRepPrimAPI_MakeWedge

/// Describes functions to build wedges (boxes with inclined faces).
#[derive(Debug, Clone)]
pub struct BRepPrimApiMakeWedge;

impl BRepPrimApiMakeWedge {
    pub fn new() -> Self {
        BRepPrimApiMakeWedge
    }
}

impl Default for BRepPrimApiMakeWedge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_wedge() {
        let _mw = BRepPrimApiMakeWedge::new();
    }
}
