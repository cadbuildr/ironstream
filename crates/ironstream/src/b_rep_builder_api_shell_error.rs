// FILE: b_rep_builder_api_shell_error.rs
// occt: BRepBuilderAPI_ShellError

pub struct BrepbuilderapiShellerror;

impl BrepbuilderapiShellerror {
    pub fn new() -> Self {
        BrepbuilderapiShellerror
    }
}

impl Default for BrepbuilderapiShellerror {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepbuilderapiShellerror::new();
    }
}
