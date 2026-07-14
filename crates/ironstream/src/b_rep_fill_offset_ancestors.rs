// FILE: b_rep_fill_offset_ancestors.rs
// occt-ref: Brepfilloffsetancestors

#[derive(Clone, Debug)]
pub struct Brepfilloffsetancestors;

impl Brepfilloffsetancestors {
    pub fn new() -> Self { Brepfilloffsetancestors }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = Brepfilloffsetancestors::new();
    }
}
