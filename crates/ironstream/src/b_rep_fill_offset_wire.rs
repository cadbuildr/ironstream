// FILE: b_rep_fill_offset_wire.rs
// occt: Brepfilloffsetwire

#[derive(Clone, Debug)]
pub struct Brepfilloffsetwire;

impl Brepfilloffsetwire {
    pub fn new() -> Self { Brepfilloffsetwire }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = Brepfilloffsetwire::new();
    }
}
