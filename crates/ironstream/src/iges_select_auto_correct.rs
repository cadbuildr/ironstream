// FILE: iges_select_auto_correct.rs
// occt: IGESSelect_AutoCorrect

pub struct IGESSelectAutoCorrect;

impl IGESSelectAutoCorrect {
    pub fn new() -> Self {
        IGESSelectAutoCorrect
    }
}

impl Default for IGESSelectAutoCorrect {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectAutoCorrect::new();
    }
}
