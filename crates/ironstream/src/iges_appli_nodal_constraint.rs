// FILE: iges_appli_nodal_constraint.rs
// occt: IGESAppli_NodalConstraint

/// Defines nodal constraints for FEA.
#[derive(Clone, Debug)]
pub struct IgesAppliNodalConstraint {
    constraint_type: i32,
    node_id: i32,
    constraint_values: Vec<f64>,
}

impl IgesAppliNodalConstraint {
    pub fn new() -> Self {
        Self {
            constraint_type: 0,
            node_id: 0,
            constraint_values: Vec::new(),
        }
    }

    pub fn init(&mut self, ctype: i32, nid: i32, values: Vec<f64>) {
        self.constraint_type = ctype;
        self.node_id = nid;
        self.constraint_values = values;
    }

    pub fn constraint_type(&self) -> i32 {
        self.constraint_type
    }

    pub fn node_id(&self) -> i32 {
        self.node_id
    }

    pub fn constraint_values(&self) -> &[f64] {
        &self.constraint_values
    }
}

impl Default for IgesAppliNodalConstraint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut constraint = IgesAppliNodalConstraint::new();
        constraint.init(1, 100, vec![0.0, 0.0, 0.0]);

        assert_eq!(constraint.constraint_type(), 1);
        assert_eq!(constraint.node_id(), 100);
        assert_eq!(constraint.constraint_values(), &[0.0, 0.0, 0.0]);
    }
}
