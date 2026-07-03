// FILE: extrema_g_func_ext_cc.rs
// occt: EXTREMAG_FUNC_EXT_CC

pub struct Extremagfuncextcc {
    done: bool,
}

impl Extremagfuncextcc {
    pub fn new() -> Self { Extremagfuncextcc { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremagfuncextcc {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremagfuncextcc::new().is_done()); }
}
