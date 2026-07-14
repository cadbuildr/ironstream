// FILE: extrema_ext_p_el_s.rs
// occt-ref: EXTREMAEXT_P_EL_S

pub struct Extremaextpels {
    done: bool,
}

impl Extremaextpels {
    pub fn new() -> Self { Extremaextpels { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremaextpels {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremaextpels::new().is_done()); }
}
