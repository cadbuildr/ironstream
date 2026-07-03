// FILE: b_rep_builder_api_collect.rs
// occt: BRepBuilderAPI_Collect

pub struct BrepbuilderapiCollect;

impl BrepbuilderapiCollect {
    pub fn new() -> Self {
        BrepbuilderapiCollect
    }
}

impl Default for BrepbuilderapiCollect {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepbuilderapiCollect::new();
    }
}
