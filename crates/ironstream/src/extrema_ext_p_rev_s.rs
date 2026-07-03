// FILE: extrema_ext_p_rev_s.rs
// occt: EXTREMAEXT_P_REV_S

pub struct Extremaextprevs {
    done: bool,
}

impl Extremaextprevs {
    pub fn new() -> Self { Extremaextprevs { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremaextprevs {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremaextprevs::new().is_done()); }
}
