// FILE: interface_h_graph.rs
// occt: Interface_HGraph

use std::sync::Arc;

/// This class allows to store a redefinable Graph, via a Handle
pub struct InterfaceHGraph {
    thegraph: InterfaceGraph,
}

/// Placeholder for Interface_Graph
#[derive(Clone)]
pub struct InterfaceGraph {
    // TODO: Implement graph structure
}

impl InterfaceHGraph {
    /// Creates an HGraph directly from a Graph
    pub fn from_graph(agraph: InterfaceGraph) -> Self {
        InterfaceHGraph { thegraph: agraph }
    }

    /// Creates an HGraph with a Graph created from a model and library
    pub fn from_model_and_lib(_amodel: Arc<dyn std::any::Any>, _lib: &InterfaceGeneralLib) -> Self {
        InterfaceHGraph {
            thegraph: InterfaceGraph {},
        }
    }

    /// Creates an HGraph with a graph created from a model and protocol
    pub fn from_model_and_protocol(_amodel: Arc<dyn std::any::Any>, _protocol: Arc<dyn std::any::Any>) -> Self {
        InterfaceHGraph {
            thegraph: InterfaceGraph {},
        }
    }

    /// Returns the Graph contained in me, for Read Only Operations
    pub fn graph(&self) -> &InterfaceGraph {
        &self.thegraph
    }

    /// Same as above, but for Read-Write Operations
    pub fn cgraph(&mut self) -> &mut InterfaceGraph {
        &mut self.thegraph
    }
}

/// Placeholder for Interface_GeneralLib
pub struct InterfaceGeneralLib;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_from_graph() {
        let graph = InterfaceGraph {};
        let hgraph = InterfaceHGraph::from_graph(graph);
        let _ref_graph = hgraph.graph();
        // Verify we can read the graph
    }

    #[test]
    fn test_cgraph() {
        let graph = InterfaceGraph {};
        let mut hgraph = InterfaceHGraph::from_graph(graph);
        let _mutable_ref = hgraph.cgraph();
        // Verify we can mutate the graph
    }

    #[test]
    fn test_create_from_model() {
        let model = Arc::new(());
        let lib = InterfaceGeneralLib;
        let _hgraph = InterfaceHGraph::from_model_and_lib(model, &lib);
    }
}
