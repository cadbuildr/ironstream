// FILE: contap_t_function.rs
// occt: Contap_TFunction

pub struct ContapTFunction;

impl ContapTFunction {
    pub fn new() -> Self {
        ContapTFunction
    }
}

impl Default for ContapTFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ContapTFunction;

    #[test]
    fn test_new() {
        let _ = ContapTFunction::new();
    }
}
