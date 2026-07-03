// FILE: b_rep_builder_api_make_edge2d.rs
// occt: BRepBuilderAPI_MakeEdge2d

pub struct BrepbuilderapiMakeedge2d;

impl BrepbuilderapiMakeedge2d {
    pub fn new() -> Self {
        BrepbuilderapiMakeedge2d
    }
}

impl Default for BrepbuilderapiMakeedge2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepbuilderapiMakeedge2d::new();
    }
}
