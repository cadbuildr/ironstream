// FILE: b_rep_builder_api_pipe_error.rs
// occt: BRepBuilderAPI_PipeError

pub struct BrepbuilderapiPipeerror;

impl BrepbuilderapiPipeerror {
    pub fn new() -> Self {
        BrepbuilderapiPipeerror
    }
}

impl Default for BrepbuilderapiPipeerror {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepbuilderapiPipeerror::new();
    }
}
