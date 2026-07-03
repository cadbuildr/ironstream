// FILE: contap_surf_function.rs
// occt: Contap_SurfFunction

pub struct ContapSurfFunction;

impl ContapSurfFunction {
    pub fn new() -> Self {
        ContapSurfFunction
    }
}

impl Default for ContapSurfFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ContapSurfFunction;

    #[test]
    fn test_new() {
        let _ = ContapSurfFunction::new();
    }
}
