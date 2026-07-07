// FILE: t_colgp_h_sequence_of_pnt.rs
// occt: TColgp_HSequenceOfPnt

use std::sync::Arc;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub struct Pnt { pub x: f64, pub y: f64, pub z: f64 }

#[derive(Debug, Clone)]
pub struct TColgpHSequenceOfPnt {
    data: Arc<Data>,
}

#[derive(Debug)]
struct Data {
    items: VecDeque<Pnt>,
}

impl TColgpHSequenceOfPnt {
    pub fn new() -> Self { TColgpHSequenceOfPnt { data: Arc::new(Data { items: VecDeque::new() }) } }
    pub fn len(&self) -> usize { self.data.items.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TColgpHSequenceOfPnt::new(); }
}
