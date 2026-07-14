// FILE: b_rep_fill_draft_law.rs
// occt-ref: Brepfilldraftlaw

#[derive(Clone, Debug)]
pub struct Brepfilldraftlaw;

impl Brepfilldraftlaw {
    pub fn new() -> Self { Brepfilldraftlaw }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = Brepfilldraftlaw::new();
    }
}
