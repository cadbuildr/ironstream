// FILE: extrema_epc_of_ext_pc2d.rs
// occt: Extrema_EPCOfExtPC2d

pub struct ExtremaEpcOfExtPc2d { nb_results: i32 }
impl ExtremaEpcOfExtPc2d {
    pub fn new() -> Self { ExtremaEpcOfExtPc2d { nb_results: 0 } }
    pub fn nb_results(&self) -> i32 { self.nb_results }
    pub fn set_nb_results(&mut self, n: i32) { self.nb_results = n; }
}
impl Default for ExtremaEpcOfExtPc2d {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert_eq!(ExtremaEpcOfExtPc2d::new().nb_results(), 0); }
}
