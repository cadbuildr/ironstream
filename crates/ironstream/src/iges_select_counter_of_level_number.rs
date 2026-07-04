// FILE: iges_select_counter_of_level_number.rs
// occt: IGESSelect_CounterOfLevelNumber

pub struct IGESSelectCounterOfLevelNumber;

impl IGESSelectCounterOfLevelNumber {
    pub fn new() -> Self {
        IGESSelectCounterOfLevelNumber
    }
}

impl Default for IGESSelectCounterOfLevelNumber {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectCounterOfLevelNumber::new();
    }
}
