// FILE: b_rep_fill_advanced_evolved.rs
// occt: BRepFill_AdvancedEvolved

#[derive(Clone, Debug)]
pub struct BRepFillAdvancedEvolved;

impl BRepFillAdvancedEvolved {
    pub fn new() -> Self { BRepFillAdvancedEvolved }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BRepFillAdvancedEvolved::new();
    }
}
