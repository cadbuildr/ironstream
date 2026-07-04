// FILE: iges_select_rebuild_groups.rs
// occt: IGESSelect_RebuildGroups

pub struct IGESSelectRebuildGroups;

impl IGESSelectRebuildGroups {
    pub fn new() -> Self {
        IGESSelectRebuildGroups
    }
}

impl Default for IGESSelectRebuildGroups {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectRebuildGroups::new();
    }
}
