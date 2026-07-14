// FILE: extrema_ext_el_c2d.rs
// occt-ref: EXTREMAEXT_EL_C2D

pub struct Extremaextelc2d {
    done: bool,
}

impl Extremaextelc2d {
    pub fn new() -> Self { Extremaextelc2d { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremaextelc2d {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremaextelc2d::new().is_done()); }
}
