// FILE: extrema_g_gen_ext_pc.rs
// occt-ref: EXTREMAG_GEN_EXT_PC

pub struct Extremaggenextpc {
    done: bool,
}

impl Extremaggenextpc {
    pub fn new() -> Self { Extremaggenextpc { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremaggenextpc {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremaggenextpc::new().is_done()); }
}
