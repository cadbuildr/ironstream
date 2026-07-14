// FILE: extrema_ext_el_cs.rs
// occt-ref: EXTREMAEXT_EL_CS

pub struct Extremaextelcs {
    done: bool,
}

impl Extremaextelcs {
    pub fn new() -> Self { Extremaextelcs { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremaextelcs {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremaextelcs::new().is_done()); }
}
