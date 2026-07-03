// FILE: b_rep_builder_api_find_plane.rs
// occt: BRepBuilderAPI_FindPlane

pub struct BrepbuilderapiFindplane;

impl BrepbuilderapiFindplane {
    pub fn new() -> Self {
        BrepbuilderapiFindplane
    }
}

impl Default for BrepbuilderapiFindplane {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepbuilderapiFindplane::new();
    }
}
