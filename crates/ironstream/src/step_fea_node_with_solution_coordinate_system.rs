// FILE: step_fea_node_with_solution_coordinate_system.rs
// occt: StepFEA_NodeWithSolutionCoordinateSystem

/// Representation of STEP entity NodeWithSolutionCoordinateSystem
#[derive(Debug, Clone)]
pub struct StepFeaNodeWithSolutionCoordinateSystem;

impl StepFeaNodeWithSolutionCoordinateSystem {
    /// Creates a new NodeWithSolutionCoordinateSystem
    pub fn new() -> Self {
        StepFeaNodeWithSolutionCoordinateSystem
    }
}

impl Default for StepFeaNodeWithSolutionCoordinateSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_with_solution_coordinate_system_creation() {
        let node = StepFeaNodeWithSolutionCoordinateSystem::new();
        let _ = node;
    }

    #[test]
    fn test_node_with_solution_coordinate_system_default() {
        let node = StepFeaNodeWithSolutionCoordinateSystem::default();
        let _ = node;
    }
}
