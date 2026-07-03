// FILE: units_shifted_token.rs
// occt: Units_ShiftedToken

/// A token representing a shifted unit in an expression
#[derive(Debug, Clone)]
pub struct UnitsShiftedToken {
    text: String,
    shift: f64,
}

impl UnitsShiftedToken {
    pub fn new(text: &str, shift: f64) -> Self {
        UnitsShiftedToken {
            text: text.to_string(),
            shift,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn shift(&self) -> f64 {
        self.shift
    }

    pub fn set_shift(&mut self, shift: f64) {
        self.shift = shift;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shifted_token_new() {
        let token = UnitsShiftedToken::new("Celsius", 273.15);
        assert_eq!(token.text(), "Celsius");
        assert_eq!(token.shift(), 273.15);
    }
}
