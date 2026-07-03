// FILE: b_rep_fill_draft.rs
// occt: Brepfilldraft

#[derive(Clone, Debug)]
pub struct Brepfilldraft;

impl Brepfilldraft {
    pub fn new() -> Self { Brepfilldraft }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = Brepfilldraft::new();
    }
}
