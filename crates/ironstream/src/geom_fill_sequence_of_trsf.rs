// FILE: geom_fill_sequence_of_trsf.rs
// occt: GeomFill_SequenceOfTrsf

#[derive(Clone, Debug)]
pub struct Trsf {}

#[derive(Clone, Debug)]
pub struct SequenceOfTrsf {
    items: Vec<Trsf>,
}

impl SequenceOfTrsf {
    pub fn new() -> Self { SequenceOfTrsf { items: Vec::new() } }
    pub fn append(&mut self, item: Trsf) { self.items.push(item); }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}

impl Default for SequenceOfTrsf {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sequence_creation() {
        let seq = SequenceOfTrsf::new();
        assert!(seq.is_empty());
    }
}
