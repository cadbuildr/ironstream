// FILE: extrema_g_curve_locator.rs
// occt: EXTREMAG_CURVE_LOCATOR

pub struct Extremagcurvelocator {
    done: bool,
}

impl Extremagcurvelocator {
    pub fn new() -> Self { Extremagcurvelocator { done: false } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
}
impl Default for Extremagcurvelocator {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert!(!Extremagcurvelocator::new().is_done()); }
}
