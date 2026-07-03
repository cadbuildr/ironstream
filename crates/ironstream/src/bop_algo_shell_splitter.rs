// FILE: bop_algo_shell_splitter.rs
// occt: BOPAlgo_ShellSplitter

pub struct BopAlgoShellSplitter;

impl BopAlgoShellSplitter {
    pub fn new() -> Self {
        BopAlgoShellSplitter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BopAlgoShellSplitter::new();
    }
}
