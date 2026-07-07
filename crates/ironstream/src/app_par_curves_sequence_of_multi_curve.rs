// FILE: app_par_curves_sequence_of_multi_curve.rs
// occt: AppParCurves_SequenceOfMultiCurve

//! Deprecated NCollection alias: Sequence<MultiCurve>

/// Multi-curve (stub).
#[derive(Clone, Debug)]
pub struct MultiCurve {
    pub id: u32,
}

/// Sequence of multi-curves.
pub type AppParCurvesSequenceOfMultiCurve = Vec<MultiCurve>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        let mut seq: AppParCurvesSequenceOfMultiCurve = Vec::new();
        seq.push(MultiCurve { id: 1 });
        assert_eq!(seq.len(), 1);
    }
}
