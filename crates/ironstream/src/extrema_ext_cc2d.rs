// FILE: extrema_ext_cc2d.rs
// occt: Extrema_ExtCC2d

pub struct ExtremaExtCc2d {
    done: bool,
    nb_extrema: i32,
}

impl ExtremaExtCc2d {
    pub fn new() -> Self { ExtremaExtCc2d { done: false, nb_extrema: 0 } }
    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_extrema(&self) -> i32 { self.nb_extrema }
    pub fn set_done(&mut self, d: bool) { self.done = d; }
    pub fn set_nb_extrema(&mut self, n: i32) { self.nb_extrema = n; }
}
impl Default for ExtremaExtCc2d {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let ext = ExtremaExtCc2d::new();
        assert!(!ext.is_done());
        assert_eq!(ext.nb_extrema(), 0);
    }
}
