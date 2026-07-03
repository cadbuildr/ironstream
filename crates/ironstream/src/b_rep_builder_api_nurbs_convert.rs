// FILE: b_rep_builder_api_nurbs_convert.rs
// occt: BRepBuilderAPI_NurbsConvert

pub struct BrepbuilderapiNurbsconvert;

impl BrepbuilderapiNurbsconvert {
    pub fn new() -> Self {
        BrepbuilderapiNurbsconvert
    }
}

impl Default for BrepbuilderapiNurbsconvert {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepbuilderapiNurbsconvert::new();
    }
}
