// FILE: app_par_curves_sequence_of_multi_b_sp_curve.rs
// occt: AppParCurves_SequenceOfMultiBSpCurve

//! Deprecated NCollection alias: Sequence<MultiBSpCurve>

/// B-spline curve (stub).
#[derive(Clone, Debug)]
pub struct MultiBSpCurve {
    pub id: u32,
}

/// Sequence of B-spline curves.
pub type AppParCurvesSequenceOfMultiBSpCurve = Vec<MultiBSpCurve>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        let mut seq: AppParCurvesSequenceOfMultiBSpCurve = Vec::new();
        seq.push(MultiBSpCurve { id: 1 });
        assert_eq!(seq.len(), 1);
    }
}
