// FILE: b_rep_fill_n_sections.rs
// occt-ref: Brepfillnsections

#[derive(Clone, Debug)]
pub struct Brepfillnsections;

impl Brepfillnsections {
    pub fn new() -> Self { Brepfillnsections }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = Brepfillnsections::new();
    }
}
