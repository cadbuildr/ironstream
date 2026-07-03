// FILE: b_rep_builder_api_modify_shape.rs
// occt: BRepBuilderAPI_ModifyShape

pub struct BrepbuilderapiModifyshape;

impl BrepbuilderapiModifyshape {
    pub fn new() -> Self {
        BrepbuilderapiModifyshape
    }
}

impl Default for BrepbuilderapiModifyshape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepbuilderapiModifyshape::new();
    }
}
