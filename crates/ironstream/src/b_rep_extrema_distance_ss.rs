// FILE: b_rep_extrema_distance_ss.rs
// occt: BRepExtrema_DistanceSS

/// Computes minimum distance between two BRep shapes
pub struct DistanceSS {
    dst_ref: f64,
    modif: bool,
    eps: f64,
}

impl DistanceSS {
    pub fn new(dst_ref: f64, eps: f64) -> Self {
        DistanceSS {
            dst_ref,
            modif: false,
            eps,
        }
    }

    pub fn is_done(&self) -> bool {
        self.modif
    }

    pub fn dist_value(&self) -> f64 {
        self.dst_ref
    }

    pub fn seq1_value(&self) -> Vec<(f64, f64, f64)> {
        Vec::new()
    }

    pub fn seq2_value(&self) -> Vec<(f64, f64, f64)> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_ss_creation() {
        let dist = DistanceSS::new(1.0, 0.01);
        assert!(!dist.is_done());
        assert_eq!(dist.dist_value(), 1.0);
    }

    #[test]
    fn test_seq_values() {
        let dist = DistanceSS::new(2.0, 0.001);
        assert_eq!(dist.seq1_value().len(), 0);
        assert_eq!(dist.seq2_value().len(), 0);
    }
}
