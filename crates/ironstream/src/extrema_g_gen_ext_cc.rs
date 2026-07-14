// FILE: extrema_g_gen_ext_cc.rs
// occt-ref: EXTREMAG_GEN_EXT_CC

pub struct Extremaggenextcc {
    done: bool,
}

impl Extremaggenextcc {
    pub fn new() -> Self { Extremaggenextcc { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremaggenextcc {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremaggenextcc::new().is_done()); }
}
