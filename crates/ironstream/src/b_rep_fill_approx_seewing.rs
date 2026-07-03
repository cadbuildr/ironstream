// FILE: b_rep_fill_approx_seewing.rs
// occt: BRepFill_ApproxSeewing

#[derive(Clone, Debug)]
pub struct BRepFillApproxSeewing;

impl BRepFillApproxSeewing {
    pub fn new() -> Self { BRepFillApproxSeewing }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BRepFillApproxSeewing::new();
    }
}
