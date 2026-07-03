// FILE: extrema_epc_of_elpc_of_locate_ext_pc2d.rs
// occt: Extrema_EPCOfELPCOfLocateExtPC2d

pub struct ExtremaEpcOfElpcOfLocateExtPc2d {
    nb_results: i32,
}

impl ExtremaEpcOfElpcOfLocateExtPc2d {
    pub fn new() -> Self {
        ExtremaEpcOfElpcOfLocateExtPc2d { nb_results: 0 }
    }
    pub fn nb_results(&self) -> i32 { self.nb_results }
    pub fn set_nb_results(&mut self, n: i32) { self.nb_results = n; }
}
impl Default for ExtremaEpcOfElpcOfLocateExtPc2d {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        assert_eq!(ExtremaEpcOfElpcOfLocateExtPc2d::new().nb_results(), 0);
    }
}
