// FILE: units_sentence.rs
// occt: Units_Sentence

use std::collections::HashMap;

/// A sentence representing a compound unit expression
#[derive(Debug, Clone)]
pub struct UnitsSentence {
    expression: String,
    tokens: Vec<String>,
}

impl UnitsSentence {
    pub fn new() -> Self {
        UnitsSentence {
            expression: String::new(),
            tokens: Vec::new(),
        }
    }

    pub fn with_expression(expr: &str) -> Self {
        let tokens = expr.split_whitespace().map(|s| s.to_string()).collect();
        UnitsSentence {
            expression: expr.to_string(),
            tokens,
        }
    }

    pub fn expression(&self) -> &str {
        &self.expression
    }

    pub fn tokens(&self) -> &[String] {
        &self.tokens
    }

    pub fn number_of_tokens(&self) -> usize {
        self.tokens.len()
    }

    pub fn parse(&mut self, expr: &str) {
        self.expression = expr.to_string();
        self.tokens = expr.split_whitespace().map(|s| s.to_string()).collect();
    }
}

impl Default for UnitsSentence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentence_new() {
        let sent = UnitsSentence::new();
        assert!(sent.expression().is_empty());
    }

    #[test]
    fn test_sentence_with_expression() {
        let sent = UnitsSentence::with_expression("mm / s");
        assert_eq!(sent.expression(), "mm / s");
        assert_eq!(sent.number_of_tokens(), 3);
    }

    #[test]
    fn test_sentence_parse() {
        let mut sent = UnitsSentence::new();
        sent.parse("kg * m / s");
        assert_eq!(sent.number_of_tokens(), 5);
    }
}
