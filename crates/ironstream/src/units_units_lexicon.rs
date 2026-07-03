// FILE: units_units_lexicon.rs
// occt: Units_UnitsLexicon

use std::collections::HashMap;

/// The complete units lexicon for parsing
#[derive(Debug, Clone)]
pub struct UnitsUnitsLexicon {
    simple_units: HashMap<String, String>,
    compound_units: HashMap<String, String>,
}

impl UnitsUnitsLexicon {
    pub fn new() -> Self {
        let mut simple = HashMap::new();
        simple.insert("mm".to_string(), "Millimeter".to_string());
        simple.insert("cm".to_string(), "Centimeter".to_string());
        simple.insert("m".to_string(), "Meter".to_string());
        simple.insert("km".to_string(), "Kilometer".to_string());
        simple.insert("in".to_string(), "Inch".to_string());
        simple.insert("ft".to_string(), "Foot".to_string());

        UnitsUnitsLexicon {
            simple_units: simple,
            compound_units: HashMap::new(),
        }
    }

    pub fn get_unit(&self, symbol: &str) -> Option<&str> {
        self.simple_units
            .get(symbol)
            .map(|s| s.as_str())
            .or_else(|| self.compound_units.get(symbol).map(|s| s.as_str()))
    }

    pub fn add_simple_unit(&mut self, symbol: &str, name: &str) {
        self.simple_units
            .insert(symbol.to_string(), name.to_string());
    }

    pub fn add_compound_unit(&mut self, expr: &str, name: &str) {
        self.compound_units
            .insert(expr.to_string(), name.to_string());
    }

    pub fn number_of_units(&self) -> usize {
        self.simple_units.len() + self.compound_units.len()
    }
}

impl Default for UnitsUnitsLexicon {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_units_lexicon_new() {
        let lex = UnitsUnitsLexicon::new();
        assert!(lex.number_of_units() > 0);
    }

    #[test]
    fn test_units_lexicon_get_unit() {
        let lex = UnitsUnitsLexicon::new();
        assert_eq!(lex.get_unit("mm"), Some("Millimeter"));
        assert_eq!(lex.get_unit("m"), Some("Meter"));
    }

    #[test]
    fn test_units_lexicon_add_simple_unit() {
        let mut lex = UnitsUnitsLexicon::new();
        lex.add_simple_unit("mi", "Mile");
        assert_eq!(lex.get_unit("mi"), Some("Mile"));
    }
}
