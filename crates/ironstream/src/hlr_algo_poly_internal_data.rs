// FILE: hlr_algo_poly_internal_data.rs
// occt: HLRAlgo_PolyInternalData

/// Internal data structure for polygon updates (outline processing).
pub struct HlrAlgoPolyInternalData {
    nb_tdata: usize,
    nb_piseg: usize,
    nb_pinod: usize,
    planar: bool,
    int_outl: bool,
}

impl HlrAlgoPolyInternalData {
    /// Create new internal data with given capacity.
    pub fn new(nb_nod: usize, nb_tri: usize) -> Self {
        HlrAlgoPolyInternalData {
            nb_tdata: nb_tri,
            nb_piseg: nb_tri * 3,
            nb_pinod: nb_nod,
            planar: false,
            int_outl: false,
        }
    }

    /// Get number of triangle data entries.
    pub fn nb_tdata(&self) -> usize {
        self.nb_tdata
    }

    /// Get number of internal segments.
    pub fn nb_piseg(&self) -> usize {
        self.nb_piseg
    }

    /// Get number of internal nodes.
    pub fn nb_pinod(&self) -> usize {
        self.nb_pinod
    }

    /// Check if data is planar.
    pub fn planar(&self) -> bool {
        self.planar
    }

    /// Set planar flag.
    pub fn set_planar(&mut self, b: bool) {
        self.planar = b;
    }

    /// Check if interior outline flag is set.
    pub fn int_outl(&self) -> bool {
        self.int_outl
    }

    /// Set interior outline flag.
    pub fn set_int_outl(&mut self, b: bool) {
        self.int_outl = b;
    }

    /// Decrement triangle data count.
    pub fn dec_tdata(&mut self) {
        if self.nb_tdata > 0 {
            self.nb_tdata -= 1;
        }
    }

    /// Decrement segment count.
    pub fn dec_piseg(&mut self) {
        if self.nb_piseg > 0 {
            self.nb_piseg -= 1;
        }
    }

    /// Decrement node count.
    pub fn dec_pinod(&mut self) {
        if self.nb_pinod > 0 {
            self.nb_pinod -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let data = HlrAlgoPolyInternalData::new(10, 5);
        assert_eq!(data.nb_tdata(), 5);
        assert_eq!(data.nb_piseg(), 15); // 5 * 3
        assert_eq!(data.nb_pinod(), 10);
        assert!(!data.planar());
        assert!(!data.int_outl());
    }

    #[test]
    fn test_planar_flag() {
        let mut data = HlrAlgoPolyInternalData::new(10, 5);
        data.set_planar(true);
        assert!(data.planar());
    }

    #[test]
    fn test_int_outl_flag() {
        let mut data = HlrAlgoPolyInternalData::new(10, 5);
        data.set_int_outl(true);
        assert!(data.int_outl());
    }

    #[test]
    fn test_decrement() {
        let mut data = HlrAlgoPolyInternalData::new(10, 5);
        data.dec_tdata();
        assert_eq!(data.nb_tdata(), 4);
        data.dec_piseg();
        assert_eq!(data.nb_piseg(), 14);
        data.dec_pinod();
        assert_eq!(data.nb_pinod(), 9);
    }
}
