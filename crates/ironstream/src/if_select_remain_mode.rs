// FILE: if_select_remain_mode.rs
// occt: IFSelect_RemainMode

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfSelectRemainMode {
    Void,
    Ignore,
    Tag,
    Count,
}

impl IfSelectRemainMode {
    pub fn is_void(&self) -> bool {
        *self == IfSelectRemainMode::Void
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modes() {
        assert!(IfSelectRemainMode::Void.is_void());
        assert!(!IfSelectRemainMode::Ignore.is_void());
    }
}
