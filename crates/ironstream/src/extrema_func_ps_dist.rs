// FILE: extrema_func_ps_dist.rs
// occt-ref: EXTREMAFUNC_PS_DIST

pub struct Extremafuncpsdist {
    done: bool,
}

impl Extremafuncpsdist {
    pub fn new() -> Self { Extremafuncpsdist { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremafuncpsdist {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremafuncpsdist::new().is_done()); }
}
