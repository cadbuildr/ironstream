// FILE: iges_select_file_modifier.rs
// occt: IGESSelect_FileModifier

pub struct IGESSelectFileModifier;

impl IGESSelectFileModifier {
    pub fn new() -> Self {
        IGESSelectFileModifier
    }
}

impl Default for IGESSelectFileModifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectFileModifier::new();
    }
}
