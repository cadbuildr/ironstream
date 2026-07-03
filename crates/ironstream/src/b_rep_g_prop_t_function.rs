// FILE: b_rep_g_prop_t_function.rs
// occt: BRepGProp_TFunction

/// Function for T parameter integration
pub struct TFunction;

impl TFunction {
    pub fn new() -> Self {
        TFunction
    }

    pub fn evaluate(_t: f64) -> f64 {
        0.0
    }
}

impl Default for TFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_function() {
        let _func = TFunction::new();
    }
}
