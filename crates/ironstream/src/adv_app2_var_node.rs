// FILE: adv_app2_var_node.rs
// occt: AdvApp2Var_Node

//! A node in the approximation framework with parameter coordinates and constraints.

pub struct Node {
    pub u: f64,
    pub v: f64,
    pub value: Option<(f64, f64, f64)>,
    pub constraint_type: i32,
}

impl Node {
    pub fn new(u: f64, v: f64) -> Self {
        Node {
            u,
            v,
            value: None,
            constraint_type: 0,
        }
    }

    pub fn with_value(u: f64, v: f64, value: (f64, f64, f64)) -> Self {
        Node {
            u,
            v,
            value: Some(value),
            constraint_type: 0,
        }
    }

    pub fn set_value(&mut self, value: (f64, f64, f64)) {
        self.value = Some(value);
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub fn set_constraint_type(&mut self, ctype: i32) {
        self.constraint_type = ctype;
    }

    pub fn get_constraint_type(&self) -> i32 {
        self.constraint_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let node = Node::new(0.5, 0.5);
        assert_eq!(node.u, 0.5);
        assert_eq!(node.v, 0.5);
        assert!(!node.has_value());
    }

    #[test]
    fn test_with_value() {
        let node = Node::with_value(0.5, 0.5, (1.0, 2.0, 3.0));
        assert!(node.has_value());
        assert_eq!(node.value, Some((1.0, 2.0, 3.0)));
    }

    #[test]
    fn test_set_value() {
        let mut node = Node::new(0.5, 0.5);
        node.set_value((1.0, 2.0, 3.0));
        assert!(node.has_value());
    }

    #[test]
    fn test_constraint_type() {
        let mut node = Node::new(0.5, 0.5);
        node.set_constraint_type(5);
        assert_eq!(node.get_constraint_type(), 5);
    }
}
