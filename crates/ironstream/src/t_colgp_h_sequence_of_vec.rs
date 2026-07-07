// FILE: t_colgp_h_sequence_of_vec.rs
// occt: TColgp_HSequenceOfVec

use std::sync::Arc;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub struct Vec { pub x: f64, pub y: f64, pub z: f64 }

#[derive(Debug, Clone)]
pub struct TColgpHSequenceOfVec {
    data: Arc<Data>,
}

#[derive(Debug)]
struct Data {
    items: VecDeque<Vec>,
}

impl TColgpHSequenceOfVec {
    pub fn new() -> Self { TColgpHSequenceOfVec { data: Arc::new(Data { items: VecDeque::new() }) } }
    pub fn len(&self) -> usize { self.data.items.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TColgpHSequenceOfVec::new(); }
}
