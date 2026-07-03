// FILE: extrema_ext_flag.rs
// occt: EXTREMAEXT_FLAG

pub struct Extremaextflag {
    done: bool,
}

impl Extremaextflag {
    pub fn new() -> Self { Extremaextflag { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremaextflag {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremaextflag::new().is_done()); }
}
