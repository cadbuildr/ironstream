// FILE: extrema_ext_pc2d.rs
// occt: EXTREMAEXT_PC2D

pub struct Extremaextpc2d {
    done: bool,
}

impl Extremaextpc2d {
    pub fn new() -> Self { Extremaextpc2d { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremaextpc2d {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremaextpc2d::new().is_done()); }
}
