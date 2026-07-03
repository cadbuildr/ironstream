// FILE: b_rep_builder_api_transition_mode.rs
// occt: BRepBuilderAPI_TransitionMode

pub struct BrepbuilderapiTransitionmode;

impl BrepbuilderapiTransitionmode {
    pub fn new() -> Self {
        BrepbuilderapiTransitionmode
    }
}

impl Default for BrepbuilderapiTransitionmode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepbuilderapiTransitionmode::new();
    }
}
