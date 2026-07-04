// FILE: step_repr_assembly_component_usage_substitute.rs
// occt: StepRepr_AssemblyComponentUsageSubstitute

/// Representation of STEP entity AssemblyComponentUsageSubstitute.
#[derive(Clone, Debug, Default)]
pub struct StepReprAssemblyComponentUsageSubstitute;

impl StepReprAssemblyComponentUsageSubstitute {
    pub fn new() -> Self {
        StepReprAssemblyComponentUsageSubstitute
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _substitute = StepReprAssemblyComponentUsageSubstitute::new();
    }
}
