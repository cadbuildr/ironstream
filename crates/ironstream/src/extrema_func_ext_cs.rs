// FILE: extrema_func_ext_cs.rs
// occt-ref: EXTREMAFUNC_EXT_CS

pub struct Extremafuncextcs {
    done: bool,
}

impl Extremafuncextcs {
    pub fn new() -> Self { Extremafuncextcs { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremafuncextcs {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremafuncextcs::new().is_done()); }
}
