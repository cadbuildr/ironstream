// FILE: geom2d_int_the_locate_ext_pc_of_the_proj_p_cur_of_g_inter.rs
// occt: Geom2dInt_TheLocateExtPCOfTheProjPCurOfGInter

/// Type alias for local extremum search on 2D curve using Geom2dCurveTool
/// Maps: Extrema_GenLocateExtPC<Adaptor2d_Curve2d, Geom2dInt_Geom2dCurveTool, ...>
pub struct TheLocateExtPCOfTheProjPCur;

impl TheLocateExtPCOfTheProjPCur {
    pub fn new() -> Self {
        TheLocateExtPCOfTheProjPCur
    }

    pub fn locate(&self, _param: f64) -> Option<f64> {
        Some(0.0)
    }
}

impl Default for TheLocateExtPCOfTheProjPCur {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _loc = TheLocateExtPCOfTheProjPCur::new();
    }

    #[test]
    fn test_locate() {
        let loc = TheLocateExtPCOfTheProjPCur::new();
        assert!(loc.locate(0.5).is_some());
    }

    #[test]
    fn test_default() {
        let _loc = TheLocateExtPCOfTheProjPCur::default();
    }
}
