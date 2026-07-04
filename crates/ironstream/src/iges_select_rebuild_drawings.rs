// FILE: iges_select_rebuild_drawings.rs
// occt: IGESSelect_RebuildDrawings

pub struct IGESSelectRebuildDrawings;

impl IGESSelectRebuildDrawings {
    pub fn new() -> Self {
        IGESSelectRebuildDrawings
    }
}

impl Default for IGESSelectRebuildDrawings {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectRebuildDrawings::new();
    }
}
