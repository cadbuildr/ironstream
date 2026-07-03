// FILE: b_rep_builder_api.rs
// occt: BRepBuilderAPI

pub struct Brepbuilderapi;

impl Brepbuilderapi {
    pub fn new() -> Self {
        Brepbuilderapi
    }
}

impl Default for Brepbuilderapi {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = Brepbuilderapi::new();
    }
}
