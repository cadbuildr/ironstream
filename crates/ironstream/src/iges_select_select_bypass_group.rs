// FILE: iges_select_select_bypass_group.rs
// occt: IGESSelect_SelectBypassGroup

pub struct IGESSelectSelectBypassGroup;

impl IGESSelectSelectBypassGroup {
    pub fn new() -> Self {
        IGESSelectSelectBypassGroup
    }
}

impl Default for IGESSelectSelectBypassGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectSelectBypassGroup::new();
    }
}
