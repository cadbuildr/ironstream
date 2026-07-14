// FILE: b_rep_fill_compute_c_line.rs
// occt-ref: Brepfillcomputecline

#[derive(Clone, Debug)]
pub struct Brepfillcomputecline;

impl Brepfillcomputecline {
    pub fn new() -> Self { Brepfillcomputecline }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = Brepfillcomputecline::new();
    }
}
