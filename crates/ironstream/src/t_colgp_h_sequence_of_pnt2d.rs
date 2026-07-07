// FILE: t_colgp_h_sequence_of_pnt2d.rs
// occt: TColgp_HSequenceOfPnt2d

use std::sync::Arc;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub struct Pnt2d { pub x: f64, pub y: f64 }

#[derive(Debug, Clone)]
pub struct TColgpHSequenceOfPnt2d {
    data: Arc<Data>,
}

#[derive(Debug)]
struct Data {
    items: VecDeque<Pnt2d>,
}

impl TColgpHSequenceOfPnt2d {
    pub fn new() -> Self { TColgpHSequenceOfPnt2d { data: Arc::new(Data { items: VecDeque::new() }) } }
    pub fn len(&self) -> usize { self.data.items.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TColgpHSequenceOfPnt2d::new(); }
}
