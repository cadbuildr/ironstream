// FILE: extrema_ext_p_ext_s.rs
// occt-ref: EXTREMAEXT_P_EXT_S

pub struct Extremaextpexts {
    done: bool,
}

impl Extremaextpexts {
    pub fn new() -> Self { Extremaextpexts { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremaextpexts {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremaextpexts::new().is_done()); }
}
