// FILE: b_rep_fill_edge_on_surf_law.rs
// occt: Brepfilledgeonsurflaw

#[derive(Clone, Debug)]
pub struct Brepfilledgeonsurflaw;

impl Brepfilledgeonsurflaw {
    pub fn new() -> Self { Brepfilledgeonsurflaw }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = Brepfilledgeonsurflaw::new();
    }
}
