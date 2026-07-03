// FILE: top_ope_b_rep_build_define.rs
// occt: TopOpeBRepBuild_define

pub const TOP_OPE_B_REP_BUILD_VERSION: &str = "1.0.0";

pub struct TopOpeBRepBuildDefine;

impl TopOpeBRepBuildDefine {
    pub fn version() -> &'static str {
        TOP_OPE_B_REP_BUILD_VERSION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_version() {
        assert_eq!(TopOpeBRepBuildDefine::version(), "1.0.0");
    }

    #[test]
    fn test_version_const() {
        assert!(!TOP_OPE_B_REP_BUILD_VERSION.is_empty());
    }
}
