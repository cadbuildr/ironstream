// FILE: f_em_tool_seq_of_lin_constr.rs
// occt: FEmTool_SeqOfLinConstr

#[derive(Clone, Debug)]
pub struct LinConstr {}

#[derive(Clone, Debug)]
pub struct SeqOfLinConstr {
    items: Vec<LinConstr>,
}

impl SeqOfLinConstr {
    pub fn new() -> Self { SeqOfLinConstr { items: Vec::new() } }
    pub fn append(&mut self, item: LinConstr) { self.items.push(item); }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}

impl Default for SeqOfLinConstr {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_seq_creation() {
        let seq = SeqOfLinConstr::new();
        assert!(seq.is_empty());
    }
}
