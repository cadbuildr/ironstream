// FILE: b_rep_fill_evolved.rs
// occt: Brepfillevolved

#[derive(Clone, Debug)]
pub struct Brepfillevolved;

impl Brepfillevolved {
    pub fn new() -> Self { Brepfillevolved }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = Brepfillevolved::new();
    }
}
