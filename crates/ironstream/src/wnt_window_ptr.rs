// FILE: wnt_window_ptr.rs
// occt: WNT_WindowPtr

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WindowPtr(pub usize);

impl WindowPtr {
    pub fn new(ptr: usize) -> Self { WindowPtr(ptr) }
    pub fn get(&self) -> usize { self.0 }
    pub fn is_null(&self) -> bool { self.0 == 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = WindowPtr::new(0x12345678);
        assert_eq!(p.get(), 0x12345678);
    }
}
