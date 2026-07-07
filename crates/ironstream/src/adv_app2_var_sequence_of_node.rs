// FILE: adv_app2_var_sequence_of_node.rs
// occt: AdvApp2Var_SequenceOfNode

//! Deprecated NCollection alias: Sequence<Node>

/// Node (stub).
#[derive(Clone, Debug)]
pub struct Node {
    pub id: u32,
}

/// Sequence of nodes.
pub type AdvApp2VarSequenceOfNode = Vec<Node>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_ops() {
        let mut seq: AdvApp2VarSequenceOfNode = Vec::new();
        seq.push(Node { id: 1 });
        seq.push(Node { id: 2 });
        assert_eq!(seq.len(), 2);
    }
}
