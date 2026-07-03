// FILE: tk_bool_pch.rs
// occt: TKBool_pch

pub struct TKBoolPch;

impl TKBoolPch {
    pub fn new() -> Self {
        TKBoolPch
    }
}

impl Default for TKBoolPch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pch_creation() {
        let pch = TKBoolPch::new();
        let _pch2 = TKBoolPch::default();
    }

    #[test]
    fn test_pch_default() {
        let pch1 = TKBoolPch::default();
        let pch2 = TKBoolPch::new();
    }
}
