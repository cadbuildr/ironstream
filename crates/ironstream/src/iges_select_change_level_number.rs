// FILE: iges_select_change_level_number.rs
// occt: IGESSelect_ChangeLevelNumber

pub struct IGESSelectChangeLevelNumber;

impl IGESSelectChangeLevelNumber {
    pub fn new() -> Self {
        IGESSelectChangeLevelNumber
    }
}

impl Default for IGESSelectChangeLevelNumber {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectChangeLevelNumber::new();
    }
}
