// FILE: t_colgp_h_sequence_of_dir.rs
// occt: TColgp_HSequenceOfDir

use std::sync::Arc;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub struct Dir { pub x: f64, pub y: f64, pub z: f64 }

#[derive(Debug, Clone)]
pub struct TColgpHSequenceOfDir {
    data: Arc<Data>,
}

#[derive(Debug)]
struct Data {
    items: VecDeque<Dir>,
}

impl TColgpHSequenceOfDir {
    pub fn new() -> Self {
        TColgpHSequenceOfDir {
            data: Arc::new(Data { items: VecDeque::new() }),
        }
    }
    pub fn len(&self) -> usize { self.data.items.len() }
    pub fn append(&mut self, item: Dir) {
        Arc::get_mut(&mut self.data).unwrap().items.push_back(item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TColgpHSequenceOfDir::new(); }
}
