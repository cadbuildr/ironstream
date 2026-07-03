// FILE: b_rep_builder_api_fast_sewing.rs
// occt: BRepBuilderAPI_FastSewing

pub struct BrepbuilderapiFastsewing;

impl BrepbuilderapiFastsewing {
    pub fn new() -> Self {
        BrepbuilderapiFastsewing
    }
}

impl Default for BrepbuilderapiFastsewing {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepbuilderapiFastsewing::new();
    }
}
