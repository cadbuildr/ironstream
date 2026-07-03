// FILE: b_rep_prim_api_make_one_axis.rs
// occt: BRepPrimAPI_MakeOneAxis

/// Abstract root class of algorithms for constructing rotational primitives.
#[derive(Debug, Clone)]
pub struct BRepPrimApiMakeOneAxis;

impl BRepPrimApiMakeOneAxis {
    pub fn new() -> Self {
        BRepPrimApiMakeOneAxis
    }
}

impl Default for BRepPrimApiMakeOneAxis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_one_axis() {
        let _mo = BRepPrimApiMakeOneAxis::new();
    }
}
