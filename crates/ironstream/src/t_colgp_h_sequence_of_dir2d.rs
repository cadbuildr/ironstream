// FILE: t_colgp_h_sequence_of_dir2d.rs
// occt: TColgp_HSequenceOfDir2d

use std::sync::Arc;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub struct Dir2d { pub x: f64, pub y: f64 }

#[derive(Debug, Clone)]
pub struct TColgpHSequenceOfDir2d {
    data: Arc<Data>,
}

#[derive(Debug)]
struct Data {
    items: VecDeque<Dir2d>,
}

impl TColgpHSequenceOfDir2d {
    pub fn new() -> Self {
        TColgpHSequenceOfDir2d {
            data: Arc::new(Data { items: VecDeque::new() }),
        }
    }
    pub fn len(&self) -> usize { self.data.items.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TColgpHSequenceOfDir2d::new(); }
}
