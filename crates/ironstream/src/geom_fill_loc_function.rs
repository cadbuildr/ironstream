// FILE: geom_fill_loc_function.rs
// occt: GeomFill_LocFunction

/// Location function for sweep surfaces
pub struct LocFunction;

impl LocFunction {
    pub fn new() -> Self {
        LocFunction
    }
}

impl Default for LocFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _func = LocFunction::new();
    }

    #[test]
    fn test_default() {
        let _func = LocFunction::default();
    }
}
