// FILE: iges_defs_macro_def.rs
// occt: IGESDefs_MacroDef

//! Macro definition entity for IGES.

#[derive(Clone, Debug)]
pub struct MacroDef {
    name: String,
    macro_id: i32,
}

impl MacroDef {
    pub fn new(name: &str, macro_id: i32) -> Self {
        MacroDef {
            name: name.to_string(),
            macro_id,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn macro_id(&self) -> i32 {
        self.macro_id
    }
}

impl Default for MacroDef {
    fn default() -> Self {
        Self::new("", 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let def = MacroDef::new("macro1", 42);
        assert_eq!(def.name(), "macro1");
        assert_eq!(def.macro_id(), 42);
    }

    #[test]
    fn test_default() {
        let def = MacroDef::default();
        assert_eq!(def.name(), "");
        assert_eq!(def.macro_id(), 0);
    }
}
