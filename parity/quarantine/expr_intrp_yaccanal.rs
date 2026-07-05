// FILE: expr_intrp_yaccanal.rs
// occt: ExprIntrp_yaccanal

use std::sync::{Arc, Mutex};
use std::cell::RefCell;

/// Global YACC analysis receptor for expression parsing.
/// This module provides the extern global instance used during YACC parsing.

thread_local! {
    static EXPR_INTRP_RECEPT: RefCell<ExprIntrpAnalysisState> = RefCell::new(ExprIntrpAnalysisState::new());
}

/// Represents the state of the analysis during YACC parsing
#[derive(Debug, Clone)]
pub struct ExprIntrpAnalysisState {
    data: String,
}

impl ExprIntrpAnalysisState {
    /// Create a new analysis state
    pub fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    /// Get the current analysis data
    pub fn data(&self) -> &str {
        &self.data
    }

    /// Set the analysis data
    pub fn set_data(&mut self, data: impl Into<String>) {
        self.data = data.into();
    }

    /// Clear the analysis state
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for ExprIntrpAnalysisState {
    fn default() -> Self {
        Self::new()
    }
}

/// Get access to the global YACC receptor
pub fn expr_intrp_recept<F, R>(f: F) -> R
where
    F: FnOnce(&mut ExprIntrpAnalysisState) -> R,
{
    EXPR_INTRP_RECEPT.with(|recept| f(&mut recept.borrow_mut()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_analysis_state() {
        let state = ExprIntrpAnalysisState::new();
        assert_eq!(state.data(), "");
    }

    #[test]
    fn test_set_and_get_data() {
        let mut state = ExprIntrpAnalysisState::new();
        state.set_data("test_data");
        assert_eq!(state.data(), "test_data");
    }

    #[test]
    fn test_clear_analysis_state() {
        let mut state = ExprIntrpAnalysisState::new();
        state.set_data("some_data");
        assert_eq!(state.data(), "some_data");
        state.clear();
        assert_eq!(state.data(), "");
    }

    #[test]
    fn test_global_recept() {
        expr_intrp_recept(|state| {
            state.set_data("global_test");
            assert_eq!(state.data(), "global_test");
        });

        expr_intrp_recept(|state| {
            assert_eq!(state.data(), "global_test");
            state.clear();
        });

        expr_intrp_recept(|state| {
            assert_eq!(state.data(), "");
        });
    }
}
