// FILE: step_shape_csg_solid.rs
// occt: StepShape_CsgSolid

//! Representation of STEP entity CsgSolid

#[derive(Clone, Debug)]
pub struct CsgSolid {
    name: String,
    tree_root_expression: String, // Placeholder for CsgSelect
}

impl CsgSolid {
    /// Returns a CsgSolid
    pub fn new() -> Self {
        CsgSolid {
            name: String::new(),
            tree_root_expression: String::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, tree_root: String) {
        self.name = name;
        self.tree_root_expression = tree_root;
    }

    /// Set TreeRootExpression
    pub fn set_tree_root_expression(&mut self, tree_root: String) {
        self.tree_root_expression = tree_root;
    }

    /// Returns TreeRootExpression
    pub fn tree_root_expression(&self) -> &str {
        &self.tree_root_expression
    }

    /// Returns name field
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name field
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for CsgSolid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let solid = CsgSolid::new();
        assert_eq!(solid.name(), "");
        assert_eq!(solid.tree_root_expression(), "");
    }

    #[test]
    fn test_init() {
        let mut solid = CsgSolid::new();
        solid.init("CSGSolid1".to_string(), "expr1".to_string());
        assert_eq!(solid.name(), "CSGSolid1");
        assert_eq!(solid.tree_root_expression(), "expr1");
    }

    #[test]
    fn test_set_tree_root_expression() {
        let mut solid = CsgSolid::new();
        solid.set_tree_root_expression("new_expr".to_string());
        assert_eq!(solid.tree_root_expression(), "new_expr");
    }

    #[test]
    fn test_set_name() {
        let mut solid = CsgSolid::new();
        solid.set_name("MyCSG".to_string());
        assert_eq!(solid.name(), "MyCSG");
    }
}
