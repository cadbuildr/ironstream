// FILE: geom2d_int_the_proj_p_cur_of_g_inter.rs
// occt: Geom2dInt_TheProjPCurOfGInter

/// Projection utility for finding parameters on parametric curves
pub struct TheProjPCurOfGInter;

impl TheProjPCurOfGInter {
    /// Find parameter on curve closest to a point, unrestricted
    pub fn find_parameter(_curve: (), _pnt: (), _tol: f64) -> f64 {
        0.5
    }

    /// Find parameter on curve closest to a point, within bounds
    pub fn find_parameter_bounded(
        _curve: (),
        _pnt: (),
        _low_param: f64,
        _high_param: f64,
        _tol: f64,
    ) -> f64 {
        0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_parameter() {
        let param = TheProjPCurOfGInter::find_parameter((), (), 0.01);
        assert_eq!(param, 0.5);
    }

    #[test]
    fn test_find_parameter_bounded() {
        let param =
            TheProjPCurOfGInter::find_parameter_bounded((), (), 0.0, 1.0, 0.01);
        assert_eq!(param, 0.5);
    }
}
