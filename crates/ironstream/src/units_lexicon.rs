// FILE: units_lexicon.rs
// occt: Units_Lexicon

use std::collections::HashMap;

/// A lexicon for parsing compound units
#[derive(Debug, Clone)]
pub struct UnitsLexicon {
    tokens: HashMap<String, String>,
}

impl UnitsLexicon {
    pub fn new() -> Self {
        UnitsLexicon {
            tokens: HashMap::new(),
        }
    }

    pub fn add_token(&mut self, name: &str, value: &str) {
        self.tokens.insert(name.to_string(), value.to_string());
    }

    pub fn get_token(&self, name: &str) -> Option<&str> {
        self.tokens.get(name).map(|s| s.as_str())
    }

    pub fn number_of_tokens(&self) -> usize {
        self.tokens.len()
    }

    pub fn clear(&mut self) {
        self.tokens.clear();
    }
}

impl Default for UnitsLexicon {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexicon_new() {
        let lex = UnitsLexicon::new();
        assert_eq!(lex.number_of_tokens(), 0);
    }

    #[test]
    fn test_lexicon_add_token() {
        let mut lex = UnitsLexicon::new();
        lex.add_token("mm", "millimeter");
        assert_eq!(lex.number_of_tokens(), 1);
    }

    #[test]
    fn test_lexicon_get_token() {
        let mut lex = UnitsLexicon::new();
        lex.add_token("cm", "centimeter");
        assert_eq!(lex.get_token("cm"), Some("centimeter"));
    }
}
