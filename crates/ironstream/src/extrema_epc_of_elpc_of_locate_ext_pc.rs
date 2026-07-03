// FILE: extrema_epc_of_elpc_of_locate_ext_pc.rs
// occt: Extrema_EPCOfELPCOfLocateExtPC

/// Extrema elliptic-point curve wrapper.
pub struct ExtremaEpcOfElpcOfLocateExtPc {
    nb_results: i32,
}

impl ExtremaEpcOfElpcOfLocateExtPc {
    pub fn new() -> Self {
        ExtremaEpcOfElpcOfLocateExtPc { nb_results: 0 }
    }
    pub fn nb_results(&self) -> i32 {
        self.nb_results
    }
    pub fn set_nb_results(&mut self, n: i32) {
        self.nb_results = n;
    }
}
impl Default for ExtremaEpcOfElpcOfLocateExtPc {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let epc = ExtremaEpcOfElpcOfLocateExtPc::new();
        assert_eq!(epc.nb_results(), 0);
    }
}
