// FILE: bisector_function_h.rs
// occt: Bisector_FunctionH

/// Compute function H for bisector calculations.
#[derive(Debug, Clone)]
pub struct BisectorFunctionH;

impl BisectorFunctionH {
    pub fn new() -> Self {
        BisectorFunctionH
    }
}

impl Default for BisectorFunctionH {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_h() {
        let _fh = BisectorFunctionH::new();
    }
}
