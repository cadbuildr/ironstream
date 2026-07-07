// FILE: iges_select_add_group.rs
// occt: IGESSelect_AddGroup

pub struct IGESSelectAddGroup;

impl IGESSelectAddGroup {
    pub fn new() -> Self {
        IGESSelectAddGroup
    }
}

impl Default for IGESSelectAddGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectAddGroup::new();
    }
}
