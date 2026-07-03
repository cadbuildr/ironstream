// FILE: b_rep_fill_edge3_d_law.rs
// occt: Brepfilledge3dlaw

#[derive(Clone, Debug)]
pub struct Brepfilledge3dlaw;

impl Brepfilledge3dlaw {
    pub fn new() -> Self { Brepfilledge3dlaw }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = Brepfilledge3dlaw::new();
    }
}
