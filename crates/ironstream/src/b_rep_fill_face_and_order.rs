// FILE: b_rep_fill_face_and_order.rs
// occt-ref: Brepfillfaceandorder

#[derive(Clone, Debug)]
pub struct Brepfillfaceandorder;

impl Brepfillfaceandorder {
    pub fn new() -> Self { Brepfillfaceandorder }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = Brepfillfaceandorder::new();
    }
}
