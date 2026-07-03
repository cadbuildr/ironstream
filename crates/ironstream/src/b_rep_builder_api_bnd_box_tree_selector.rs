// FILE: b_rep_builder_api_bnd_box_tree_selector.rs
// occt: BRepBuilderAPI_BndBoxTreeSelector

pub struct BrepbuilderapiBndboxtreeselector;

impl BrepbuilderapiBndboxtreeselector {
    pub fn new() -> Self {
        BrepbuilderapiBndboxtreeselector
    }
}

impl Default for BrepbuilderapiBndboxtreeselector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepbuilderapiBndboxtreeselector::new();
    }
}
