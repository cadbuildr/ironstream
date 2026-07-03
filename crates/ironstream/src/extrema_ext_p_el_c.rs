// FILE: extrema_ext_p_el_c.rs
// occt: EXTREMAEXT_P_EL_C

pub struct Extremaextpelc {
    done: bool,
}

impl Extremaextpelc {
    pub fn new() -> Self { Extremaextpelc { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremaextpelc {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremaextpelc::new().is_done()); }
}
