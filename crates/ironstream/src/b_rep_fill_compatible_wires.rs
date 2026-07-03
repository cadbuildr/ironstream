// FILE: b_rep_fill_compatible_wires.rs
// occt: Brepfillcompatiblewires

#[derive(Clone, Debug)]
pub struct Brepfillcompatiblewires;

impl Brepfillcompatiblewires {
    pub fn new() -> Self { Brepfillcompatiblewires }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = Brepfillcompatiblewires::new();
    }
}
