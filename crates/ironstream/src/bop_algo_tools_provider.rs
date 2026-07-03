// FILE: bop_algo_tools_provider.rs
// occt: BOPAlgo_ToolsProvider

pub struct BopAlgoToolsProvider;

impl BopAlgoToolsProvider {
    pub fn new() -> Self {
        BopAlgoToolsProvider
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BopAlgoToolsProvider::new();
    }
}
