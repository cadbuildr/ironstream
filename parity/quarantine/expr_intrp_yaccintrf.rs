// FILE: expr_intrp_yaccintrf.rs
// occt: ExprIntrp_yaccintrf

use std::cell::RefCell;

/// Interface to YACC lexer/parser functions for expression parsing.
/// This module provides wrappers for YACC parsing state management.

thread_local! {
    static PARSING_STATE: RefCell<ExprIntrpParsingState> = RefCell::new(ExprIntrpParsingState::new());
}

/// Represents the parsing state during YACC parsing
#[derive(Debug, Clone)]
pub struct ExprIntrpParsingState {
    input_string: String,
    result: String,
    degree: i32,
}

impl ExprIntrpParsingState {
    /// Create a new parsing state
    pub fn new() -> Self {
        Self {
            input_string: String::new(),
            result: String::new(),
            degree: 0,
        }
    }

    /// Get the input string
    pub fn input_string(&self) -> &str {
        &self.input_string
    }

    /// Set the input string
    pub fn set_input_string(&mut self, s: impl Into<String>) {
        self.input_string = s.into();
    }

    /// Get the result
    pub fn result(&self) -> &str {
        &self.result
    }

    /// Set the result
    pub fn set_result(&mut self, r: impl Into<String>) {
        self.result = r.into();
    }

    /// Get the degree
    pub fn degree(&self) -> i32 {
        self.degree
    }

    /// Set the degree
    pub fn set_degree(&mut self, d: i32) {
        self.degree = d;
    }

    /// Clear the parsing state
    pub fn clear(&mut self) {
        self.input_string.clear();
        self.result.clear();
        self.degree = 0;
    }
}

impl Default for ExprIntrpParsingState {
    fn default() -> Self {
        Self::new()
    }
}

/// Start parsing from a string
pub fn expr_intrp_start_string(s: &str) {
    PARSING_STATE.with(|state| {
        state.borrow_mut().set_input_string(s);
    });
}

/// Stop parsing and clean up
pub fn expr_intrp_stop_string() {
    PARSING_STATE.with(|state| {
        state.borrow_mut().clear();
    });
}

/// Set the parsing result
pub fn expr_intrp_set_result(result: impl Into<String>) {
    PARSING_STATE.with(|state| {
        state.borrow_mut().set_result(result);
    });
}

/// Set the parsing degree
pub fn expr_intrp_set_degree(degree: i32) {
    PARSING_STATE.with(|state| {
        state.borrow_mut().set_degree(degree);
    });
}

/// Get the parsing result
pub fn expr_intrp_get_result() -> String {
    PARSING_STATE.with(|state| state.borrow().result().to_string())
}

/// Get the parsing degree
pub fn expr_intrp_get_degree() -> i32 {
    PARSING_STATE.with(|state| state.borrow().degree())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_parsing_state() {
        let state = ExprIntrpParsingState::new();
        assert_eq!(state.input_string(), "");
        assert_eq!(state.result(), "");
        assert_eq!(state.degree(), 0);
    }

    #[test]
    fn test_start_string() {
        expr_intrp_start_string("x + y");
        PARSING_STATE.with(|state| {
            assert_eq!(state.borrow().input_string(), "x + y");
        });
    }

    #[test]
    fn test_set_and_get_result() {
        expr_intrp_set_result("result_value");
        assert_eq!(expr_intrp_get_result(), "result_value");
    }

    #[test]
    fn test_set_and_get_degree() {
        expr_intrp_set_degree(3);
        assert_eq!(expr_intrp_get_degree(), 3);
    }

    #[test]
    fn test_stop_string() {
        expr_intrp_start_string("test");
        expr_intrp_set_result("result");
        expr_intrp_set_degree(2);
        expr_intrp_stop_string();
        assert_eq!(expr_intrp_get_result(), "");
        assert_eq!(expr_intrp_get_degree(), 0);
    }

    #[test]
    fn test_full_parse_flow() {
        expr_intrp_start_string("x * y + z");
        expr_intrp_set_result("parsed");
        expr_intrp_set_degree(1);
        assert_eq!(expr_intrp_get_result(), "parsed");
        assert_eq!(expr_intrp_get_degree(), 1);
        expr_intrp_stop_string();
    }
}
