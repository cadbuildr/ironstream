// FILE: adv_app2_var_framework.rs
// occt: AdvApp2Var_Framework

//! Approximation framework managing nodes and iso-curves.

use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Node {
    pub u: f64,
    pub v: f64,
    pub value: (f64, f64, f64),
}

impl Node {
    pub fn new(u: f64, v: f64, value: (f64, f64, f64)) -> Self {
        Node { u, v, value }
    }
}

#[derive(Clone, Debug)]
pub struct Iso {
    pub is_u: bool,
    pub constant: f64,
    pub approximated: bool,
}

impl Iso {
    pub fn u_iso(constant: f64) -> Self {
        Iso {
            is_u: true,
            constant,
            approximated: false,
        }
    }

    pub fn v_iso(constant: f64) -> Self {
        Iso {
            is_u: false,
            constant,
            approximated: false,
        }
    }
}

pub struct Framework {
    nodes: Vec<Node>,
    u_isos: Vec<Iso>,
    v_isos: Vec<Iso>,
}

impl Framework {
    pub fn new() -> Self {
        Framework {
            nodes: vec![],
            u_isos: vec![],
            v_isos: vec![],
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn add_u_iso(&mut self, iso: Iso) {
        self.u_isos.push(iso);
    }

    pub fn add_v_iso(&mut self, iso: Iso) {
        self.v_isos.push(iso);
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn first_not_approximated(&self) -> Option<(usize, bool)> {
        for (i, iso) in self.u_isos.iter().enumerate() {
            if !iso.approximated {
                return Some((i, true));
            }
        }
        for (i, iso) in self.v_isos.iter().enumerate() {
            if !iso.approximated {
                return Some((i, false));
            }
        }
        None
    }

    pub fn mark_u_iso_approximated(&mut self, index: usize) {
        if index < self.u_isos.len() {
            self.u_isos[index].approximated = true;
        }
    }

    pub fn mark_v_iso_approximated(&mut self, index: usize) {
        if index < self.v_isos.len() {
            self.v_isos[index].approximated = true;
        }
    }
}

impl Default for Framework {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new(0.5, 0.5, (1.0, 2.0, 3.0));
        assert_eq!(node.u, 0.5);
        assert_eq!(node.v, 0.5);
        assert_eq!(node.value, (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_iso_u() {
        let iso = Iso::u_iso(0.5);
        assert!(iso.is_u);
        assert_eq!(iso.constant, 0.5);
        assert!(!iso.approximated);
    }

    #[test]
    fn test_iso_v() {
        let iso = Iso::v_iso(0.3);
        assert!(!iso.is_u);
        assert_eq!(iso.constant, 0.3);
    }

    #[test]
    fn test_framework() {
        let mut framework = Framework::new();
        let node = Node::new(0.0, 0.0, (0.0, 0.0, 0.0));
        framework.add_node(node);
        assert_eq!(framework.node_count(), 1);
    }

    #[test]
    fn test_first_not_approximated() {
        let mut framework = Framework::new();
        framework.add_u_iso(Iso::u_iso(0.5));
        framework.add_v_iso(Iso::v_iso(0.5));

        let result = framework.first_not_approximated();
        assert!(result.is_some());

        let (idx, is_u) = result.unwrap();
        assert_eq!(idx, 0);
        assert!(is_u);
    }

    #[test]
    fn test_mark_approximated() {
        let mut framework = Framework::new();
        framework.add_u_iso(Iso::u_iso(0.5));
        framework.mark_u_iso_approximated(0);

        assert!(framework.u_isos[0].approximated);
    }
}
