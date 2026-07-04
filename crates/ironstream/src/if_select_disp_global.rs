// FILE: if_select_disp_global.rs
// occt: IFSelect_DispGlobal

/// Dispatches all entities globally
#[derive(Clone, Debug)]
pub struct IfSelectDispGlobal {}

impl IfSelectDispGlobal {
    /// Creates a global dispatcher
    pub fn new() -> Self {
        IfSelectDispGlobal {}
    }

    /// Performs the dispatch
    pub fn dispatch(&self) -> usize {
        1
    }
}

impl Default for IfSelectDispGlobal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let disp = IfSelectDispGlobal::new();
        assert_eq!(disp.dispatch(), 1);
    }
}
