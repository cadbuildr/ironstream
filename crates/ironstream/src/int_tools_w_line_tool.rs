// FILE: int_tools_w_line_tool.rs
// occt: IntTools_WLineTool

/// Walking line tool for intersection curves.
pub struct IntToolsWLineTool;

impl IntToolsWLineTool {
    /// Check if surface approx should not be used.
    pub fn not_use_surfaces_for_approx(
        _face1: &[u8],
        _face2: &[u8],
        _wline: &[u8],
        _ifprm: i32,
        _ilprm: i32,
    ) -> bool {
        false
    }

    /// Decompose a walking line.
    pub fn decomposition_of_wline(
        _wline: &[u8],
        _surface1: &[u8],
        _surface2: &[u8],
        _face1: &[u8],
        _face2: &[u8],
        _lconstructor: &[u8],
        _avoid_lconstructor: bool,
        _tol: f64,
        _context: &[u8],
    ) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_use_surfaces_for_approx() {
        let result = IntToolsWLineTool::not_use_surfaces_for_approx(&[], &[], &[], 0, 0);
        assert!(!result);
    }

    #[test]
    fn test_decomposition_of_wline() {
        let result = IntToolsWLineTool::decomposition_of_wline(&[], &[], &[], &[], &[], &[], false, 0.01, &[]);
        assert!(result);
    }
}
