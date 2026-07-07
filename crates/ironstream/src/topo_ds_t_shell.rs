// FILE: topo_ds_t_shell.rs
// occt: TopoDS_TShell

//! Topology shape implementation for shell.

/// Internal topology structure for shell
#[derive(Clone)]
pub struct TopoDS_TShell {
    id: usize,
}

impl TopoDS_TShell {
    /// Creates new shell topology
    pub fn new(id: usize) -> Self {
        TopoDS_TShell { id }
    }

    /// Returns topology ID
    pub fn id(&self) -> usize {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_shell_new() {
        let tshape = TopoDS_TShell::new(1);
        assert_eq!(tshape.id(), 1);
    }
}
