// FILE: approx_sequence_of_h_array1_of_real.rs
// occt: Approx_SequenceOfHArray1OfReal

//! Deprecated NCollection alias: Sequence<HArray1<Real>>

/// Handle-based array of reals.
#[derive(Clone, Debug)]
pub struct HArray1OfReal {
    pub data: Vec<f64>,
}

/// Sequence of handle arrays.
pub type ApproxSequenceOfHArray1OfReal = Vec<HArray1OfReal>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        let mut seq: ApproxSequenceOfHArray1OfReal = Vec::new();
        seq.push(HArray1OfReal { data: vec![1.0, 2.0, 3.0] });
        assert_eq!(seq.len(), 1);
    }
}
