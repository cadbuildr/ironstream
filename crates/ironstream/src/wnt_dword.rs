// FILE: wnt_dword.rs
// occt: WNT_Dword

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Dword(pub u32);

impl Dword {
    pub fn new(val: u32) -> Self { Dword(val) }
    pub fn get(&self) -> u32 { self.0 }
    pub fn set(&mut self, val: u32) { self.0 = val; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let d = Dword::new(0x12345678);
        assert_eq!(d.get(), 0x12345678);
    }
}
