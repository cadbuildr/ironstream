// FILE: geom_int_the_multi_line_tool_of_wl_approx.rs
// occt: GeomInt_TheMultiLineToolOfWLApprox

pub struct GeomIntTheMultiLineTool;

impl GeomIntTheMultiLineTool {
    pub fn new() -> Self {
        GeomIntTheMultiLineTool
    }

    pub fn nb_points() -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let _tool = GeomIntTheMultiLineTool::new();
    }

    #[test]
    fn test_nb_points() {
        assert_eq!(GeomIntTheMultiLineTool::nb_points(), 0);
    }
}
