// FILE: iges_select_change_level_list.rs
// occt: IGESSelect_ChangeLevelList

pub struct IGESSelectChangeLevelList;

impl IGESSelectChangeLevelList {
    pub fn new() -> Self {
        IGESSelectChangeLevelList
    }
}

impl Default for IGESSelectChangeLevelList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectChangeLevelList::new();
    }
}
