// FILE: extrema_ext_el_ss.rs
// occt: EXTREMAEXT_EL_SS

pub struct Extremaextelss {
    done: bool,
}

impl Extremaextelss {
    pub fn new() -> Self { Extremaextelss { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremaextelss {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremaextelss::new().is_done()); }
}
