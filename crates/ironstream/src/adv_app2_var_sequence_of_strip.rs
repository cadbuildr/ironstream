// FILE: adv_app2_var_sequence_of_strip.rs
// occt: AdvApp2Var_SequenceOfStrip

//! Deprecated NCollection alias: Sequence<Strip>

/// Strip (stub).
#[derive(Clone, Debug)]
pub struct Strip {
    pub id: u32,
}

/// Sequence of strips.
pub type AdvApp2VarSequenceOfStrip = Vec<Strip>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        let mut seq: AdvApp2VarSequenceOfStrip = Vec::new();
        seq.push(Strip { id: 1 });
        assert_eq!(seq.len(), 1);
    }
}
