// FILE: b_rep_fill_multi_line.rs
// occt: Brepfillmultiline

#[derive(Clone, Debug)]
pub struct Brepfillmultiline;

impl Brepfillmultiline {
    pub fn new() -> Self { Brepfillmultiline }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = Brepfillmultiline::new();
    }
}
