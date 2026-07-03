// FILE: extrema_ext_p_el_c2d.rs
// occt: EXTREMAEXT_P_EL_C2D

pub struct Extremaextpelc2d {
    done: bool,
}

impl Extremaextpelc2d {
    pub fn new() -> Self { Extremaextpelc2d { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremaextpelc2d {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremaextpelc2d::new().is_done()); }
}
