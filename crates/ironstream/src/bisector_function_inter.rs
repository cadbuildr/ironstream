// FILE: bisector_function_inter.rs
// occt: Bisector_FunctionInter

/// Function for computing bisector intersections.
#[derive(Debug, Clone)]
pub struct BisectorFunctionInter;

impl BisectorFunctionInter {
    pub fn new() -> Self {
        BisectorFunctionInter
    }
}

impl Default for BisectorFunctionInter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_inter() {
        let _fi = BisectorFunctionInter::new();
    }
}
