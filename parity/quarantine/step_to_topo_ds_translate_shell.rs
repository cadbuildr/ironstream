// FILE: step_to_topo_ds_translate_shell.rs
// occt: StepToTopoDS_TranslateShell

use crate::step_to_topo_ds_root::StepToTopoDS_Root;

/// Translate STEP shell to TopoDS_Shell
pub struct StepToTopoDS_TranslateShell {
    root: StepToTopoDS_Root,
    shell: Option<String>,
}

impl StepToTopoDS_TranslateShell {
    pub fn new() -> Self {
        StepToTopoDS_TranslateShell {
            root: StepToTopoDS_Root::new(),
            shell: None,
        }
    }

    pub fn init(&mut self, shell_key: &str) -> bool {
        self.shell = Some(shell_key.to_string());
        self.root.set_done(true);
        true
    }

    pub fn value(&self) -> Option<&String> {
        self.shell.as_ref()
    }

    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }
}

impl Default for StepToTopoDS_TranslateShell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ts = StepToTopoDS_TranslateShell::new();
        assert!(!ts.is_done());
    }

    #[test]
    fn test_init() {
        let mut ts = StepToTopoDS_TranslateShell::new();
        assert!(ts.init("shell1"));
        assert!(ts.is_done());
        assert_eq!(ts.value(), Some(&"shell1".to_string()));
    }
}
