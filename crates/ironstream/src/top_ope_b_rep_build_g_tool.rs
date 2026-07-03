// FILE: top_ope_b_rep_build_g_tool.rs
// occt: TopOpeBRepBuild_GTool

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildGTopo {
    cases: [[bool; 3]; 3],
    type1: u32,
    type2: u32,
}

pub struct TopOpeBRepBuildGTool;

impl TopOpeBRepBuildGTool {
    pub fn new() -> Self {
        TopOpeBRepBuildGTool
    }

    pub fn g_fus_unsh(shape_type1: u32, shape_type2: u32) -> TopOpeBRepBuildGTopo {
        TopOpeBRepBuildGTopo {
            cases: [[false; 3]; 3],
            type1: shape_type1,
            type2: shape_type2,
        }
    }

    pub fn g_fus_same(shape_type1: u32, shape_type2: u32) -> TopOpeBRepBuildGTopo {
        TopOpeBRepBuildGTopo {
            cases: [[false; 3]; 3],
            type1: shape_type1,
            type2: shape_type2,
        }
    }

    pub fn g_fus_diff(shape_type1: u32, shape_type2: u32) -> TopOpeBRepBuildGTopo {
        TopOpeBRepBuildGTopo {
            cases: [[false; 3]; 3],
            type1: shape_type1,
            type2: shape_type2,
        }
    }

    pub fn g_cut_unsh(shape_type1: u32, shape_type2: u32) -> TopOpeBRepBuildGTopo {
        TopOpeBRepBuildGTopo {
            cases: [[false; 3]; 3],
            type1: shape_type1,
            type2: shape_type2,
        }
    }

    pub fn g_cut_same(shape_type1: u32, shape_type2: u32) -> TopOpeBRepBuildGTopo {
        TopOpeBRepBuildGTopo {
            cases: [[false; 3]; 3],
            type1: shape_type1,
            type2: shape_type2,
        }
    }

    pub fn g_cut_diff(shape_type1: u32, shape_type2: u32) -> TopOpeBRepBuildGTopo {
        TopOpeBRepBuildGTopo {
            cases: [[false; 3]; 3],
            type1: shape_type1,
            type2: shape_type2,
        }
    }

    pub fn g_com_unsh(shape_type1: u32, shape_type2: u32) -> TopOpeBRepBuildGTopo {
        TopOpeBRepBuildGTopo {
            cases: [[false; 3]; 3],
            type1: shape_type1,
            type2: shape_type2,
        }
    }

    pub fn g_com_same(shape_type1: u32, shape_type2: u32) -> TopOpeBRepBuildGTopo {
        TopOpeBRepBuildGTopo {
            cases: [[false; 3]; 3],
            type1: shape_type1,
            type2: shape_type2,
        }
    }

    pub fn g_com_diff(shape_type1: u32, shape_type2: u32) -> TopOpeBRepBuildGTopo {
        TopOpeBRepBuildGTopo {
            cases: [[false; 3]; 3],
            type1: shape_type1,
            type2: shape_type2,
        }
    }

    pub fn dump() {
        println!("TopOpeBRepBuild_GTool");
    }
}

impl Default for TopOpeBRepBuildGTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_g_tool_new() {
        let _gt = TopOpeBRepBuildGTool::new();
    }

    #[test]
    fn test_g_tool_g_fus_unsh() {
        let gt = TopOpeBRepBuildGTool::g_fus_unsh(1, 2);
        assert_eq!(gt.type1, 1);
        assert_eq!(gt.type2, 2);
    }

    #[test]
    fn test_g_tool_g_fus_same() {
        let gt = TopOpeBRepBuildGTool::g_fus_same(2, 2);
        assert_eq!(gt.type1, 2);
        assert_eq!(gt.type2, 2);
    }

    #[test]
    fn test_g_tool_g_cut_unsh() {
        let gt = TopOpeBRepBuildGTool::g_cut_unsh(1, 2);
        assert_eq!(gt.type1, 1);
    }

    #[test]
    fn test_g_tool_g_com_same() {
        let gt = TopOpeBRepBuildGTool::g_com_same(3, 3);
        assert_eq!(gt.type1, 3);
    }

    #[test]
    fn test_g_tool_dump() {
        TopOpeBRepBuildGTool::dump();
    }
}
