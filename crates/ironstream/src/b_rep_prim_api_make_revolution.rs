// FILE: b_rep_prim_api_make_revolution.rs
// occt: BRepPrimAPI_MakeRevolution

/// Describes functions to build revolved shapes.
#[derive(Debug, Clone)]
pub struct BRepPrimApiMakeRevolution;

impl BRepPrimApiMakeRevolution {
    pub fn new() -> Self {
        BRepPrimApiMakeRevolution
    }
}

impl Default for BRepPrimApiMakeRevolution {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_revolution() {
        let _mr = BRepPrimApiMakeRevolution::new();
    }
}
