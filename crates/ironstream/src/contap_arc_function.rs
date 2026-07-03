// FILE: contap_arc_function.rs
// occt: Contap_ArcFunction

pub struct ContapArcFunction;

impl ContapArcFunction {
    pub fn new() -> Self {
        ContapArcFunction
    }
}

impl Default for ContapArcFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ContapArcFunction;

    #[test]
    fn test_new() {
        let _ = ContapArcFunction::new();
    }
}
