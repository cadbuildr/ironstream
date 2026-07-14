// FILE: extrema_func_ps_norm.rs
// occt-ref: EXTREMAFUNC_PS_NORM

pub struct Extremafuncpsnorm {
    done: bool,
}

impl Extremafuncpsnorm {
    pub fn new() -> Self { Extremafuncpsnorm { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremafuncpsnorm {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremafuncpsnorm::new().is_done()); }
}
