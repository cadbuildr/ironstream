// FILE: extrema_func_ext_ss.rs
// occt-ref: EXTREMAFUNC_EXT_SS

pub struct Extremafuncextss {
    done: bool,
}

impl Extremafuncextss {
    pub fn new() -> Self { Extremafuncextss { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremafuncextss {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremafuncextss::new().is_done()); }
}
