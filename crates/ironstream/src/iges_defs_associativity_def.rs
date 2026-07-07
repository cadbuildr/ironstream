// FILE: iges_defs_associativity_def.rs
// occt: IGESDefs_AssociativityDef

//! Associativity definition entity for IGES.

#[derive(Clone, Debug)]
pub struct AssociativityDef {
    name: String,
    assoc_type: i32,
}

impl AssociativityDef {
    pub fn new(name: &str, assoc_type: i32) -> Self {
        AssociativityDef {
            name: name.to_string(),
            assoc_type,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn assoc_type(&self) -> i32 {
        self.assoc_type
    }
}

impl Default for AssociativityDef {
    fn default() -> Self {
        Self::new("", 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let def = AssociativityDef::new("test", 1);
        assert_eq!(def.name(), "test");
        assert_eq!(def.assoc_type(), 1);
    }

    #[test]
    fn test_default() {
        let def = AssociativityDef::default();
        assert_eq!(def.name(), "");
        assert_eq!(def.assoc_type(), 0);
    }
}
