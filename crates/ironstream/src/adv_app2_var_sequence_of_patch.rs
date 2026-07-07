// FILE: adv_app2_var_sequence_of_patch.rs
// occt: AdvApp2Var_SequenceOfPatch

//! Deprecated NCollection alias: Sequence<Patch>

/// Patch (stub).
#[derive(Clone, Debug)]
pub struct Patch {
    pub id: u32,
}

/// Sequence of patches.
pub type AdvApp2VarSequenceOfPatch = Vec<Patch>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        let mut seq: AdvApp2VarSequenceOfPatch = Vec::new();
        seq.push(Patch { id: 1 });
        assert_eq!(seq.len(), 1);
    }
}
