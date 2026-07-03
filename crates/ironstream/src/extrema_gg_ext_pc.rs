// FILE: extrema_gg_ext_pc.rs
// occt: EXTREMAGG_EXT_PC

pub struct Extremaggextpc {
    done: bool,
}

impl Extremaggextpc {
    pub fn new() -> Self { Extremaggextpc { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremaggextpc {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremaggextpc::new().is_done()); }
}
