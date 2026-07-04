// FILE: xs_control_vars.rs
// occt: XSControl_Vars

/// Container for variables in the control framework.
/// Stores configuration and state variables for exchange operations.
#[derive(Clone, Debug)]
pub struct XSControlVars {
    /// Integer variables
    int_vars: Vec<i32>,
    /// Float variables
    float_vars: Vec<f64>,
    /// String variables
    str_vars: Vec<String>,
}

impl XSControlVars {
    /// Creates a new variable container.
    pub fn new() -> Self {
        Self {
            int_vars: Vec::new(),
            float_vars: Vec::new(),
            str_vars: Vec::new(),
        }
    }

    /// Adds an integer variable.
    pub fn add_int(&mut self, value: i32) {
        self.int_vars.push(value);
    }

    /// Adds a float variable.
    pub fn add_float(&mut self, value: f64) {
        self.float_vars.push(value);
    }

    /// Adds a string variable.
    pub fn add_string(&mut self, value: &str) {
        self.str_vars.push(String::from(value));
    }

    /// Returns the number of variables.
    pub fn nb_vars(&self) -> usize {
        self.int_vars.len() + self.float_vars.len() + self.str_vars.len()
    }

    /// Clears all variables.
    pub fn clear(&mut self) {
        self.int_vars.clear();
        self.float_vars.clear();
        self.str_vars.clear();
    }
}

impl Default for XSControlVars {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vars = XSControlVars::new();
        assert_eq!(vars.nb_vars(), 0);
    }

    #[test]
    fn test_add_int() {
        let mut vars = XSControlVars::new();
        vars.add_int(42);
        assert_eq!(vars.nb_vars(), 1);
    }

    #[test]
    fn test_add_float() {
        let mut vars = XSControlVars::new();
        vars.add_float(3.14);
        assert_eq!(vars.nb_vars(), 1);
    }

    #[test]
    fn test_add_string() {
        let mut vars = XSControlVars::new();
        vars.add_string("test");
        assert_eq!(vars.nb_vars(), 1);
    }

    #[test]
    fn test_mixed_vars() {
        let mut vars = XSControlVars::new();
        vars.add_int(10);
        vars.add_float(3.14);
        vars.add_string("hello");
        assert_eq!(vars.nb_vars(), 3);
    }

    #[test]
    fn test_clear() {
        let mut vars = XSControlVars::new();
        vars.add_int(10);
        vars.add_float(3.14);
        assert_eq!(vars.nb_vars(), 2);

        vars.clear();
        assert_eq!(vars.nb_vars(), 0);
    }
}
