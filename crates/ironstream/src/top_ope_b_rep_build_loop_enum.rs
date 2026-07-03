// FILE: top_ope_b_rep_build_loop_enum.rs
// occt: TopOpeBRepBuild_LoopEnum

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopOpeBRepBuildLoopEnum {
    AnyLoop = 0,
    Boundary = 1,
    Block = 2,
}

impl Default for TopOpeBRepBuildLoopEnum {
    fn default() -> Self {
        TopOpeBRepBuildLoopEnum::AnyLoop
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_enum_values() {
        assert_eq!(TopOpeBRepBuildLoopEnum::AnyLoop as u32, 0);
        assert_eq!(TopOpeBRepBuildLoopEnum::Boundary as u32, 1);
        assert_eq!(TopOpeBRepBuildLoopEnum::Block as u32, 2);
    }

    #[test]
    fn test_loop_enum_equality() {
        assert_eq!(TopOpeBRepBuildLoopEnum::AnyLoop, TopOpeBRepBuildLoopEnum::AnyLoop);
        assert_ne!(TopOpeBRepBuildLoopEnum::AnyLoop, TopOpeBRepBuildLoopEnum::Boundary);
    }

    #[test]
    fn test_loop_enum_default() {
        assert_eq!(TopOpeBRepBuildLoopEnum::default(), TopOpeBRepBuildLoopEnum::AnyLoop);
    }

    #[test]
    fn test_loop_enum_copy() {
        let le = TopOpeBRepBuildLoopEnum::Block;
        let le_copy = le;
        assert_eq!(le, le_copy);
    }
}
