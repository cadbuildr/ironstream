// FILE: geom2d_int_pc_loc_f_of_the_locate_ext_pc_of_the_proj_p_cur_of_g_inter.rs
// occt: Geom2dInt_PCLocFOfTheLocateExtPCOfTheProjPCurOfGInter

/// Type alias for 2D curve extremum function using Geom2dCurveTool
/// Maps: Extrema_GFuncExtPC<Adaptor2d_Curve2d, Geom2dInt_Geom2dCurveTool, ...>
pub struct PCLocFOfTheLocateExtPC;

impl PCLocFOfTheLocateExtPC {
    pub fn new() -> Self {
        PCLocFOfTheLocateExtPC
    }
}

impl Default for PCLocFOfTheLocateExtPC {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _f = PCLocFOfTheLocateExtPC::new();
    }

    #[test]
    fn test_default() {
        let _f = PCLocFOfTheLocateExtPC::default();
    }
}
