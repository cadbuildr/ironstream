// FILE: extrema_g_func_ext_pc.rs
// occt: EXTREMAG_FUNC_EXT_PC

pub struct Extremagfuncextpc {
    done: bool,
}

impl Extremagfuncextpc {
    pub fn new() -> Self { Extremagfuncextpc { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremagfuncextpc {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremagfuncextpc::new().is_done()); }
}
