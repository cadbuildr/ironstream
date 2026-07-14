// FILE: b_rep_fill_location_law.rs
// occt-ref: Brepfilllocationlaw

#[derive(Clone, Debug)]
pub struct Brepfilllocationlaw;

impl Brepfilllocationlaw {
    pub fn new() -> Self { Brepfilllocationlaw }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = Brepfilllocationlaw::new();
    }
}
