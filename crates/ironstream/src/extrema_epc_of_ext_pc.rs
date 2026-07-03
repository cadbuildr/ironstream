// FILE: extrema_epc_of_ext_pc.rs
// occt: Extrema_EPCOfExtPC

pub struct ExtremaEpcOfExtPc {
    nb_results: i32,
}

impl ExtremaEpcOfExtPc {
    pub fn new() -> Self { ExtremaEpcOfExtPc { nb_results: 0 } }
    pub fn nb_results(&self) -> i32 { self.nb_results }
    pub fn set_nb_results(&mut self, n: i32) { self.nb_results = n; }
}
impl Default for ExtremaEpcOfExtPc {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() { assert_eq!(ExtremaEpcOfExtPc::new().nb_results(), 0); }
}
