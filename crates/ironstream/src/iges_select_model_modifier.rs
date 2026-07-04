// FILE: iges_select_model_modifier.rs
// occt: IGESSelect_ModelModifier

pub struct IGESSelectModelModifier;

impl IGESSelectModelModifier {
    pub fn new() -> Self {
        IGESSelectModelModifier
    }
}

impl Default for IGESSelectModelModifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectModelModifier::new();
    }
}
