// FILE: tkv3d_pch.rs
// occt: TKV3d_pch

//! Precompiled header marker for TKV3d module.

pub struct TKV3dPch;

impl TKV3dPch {
    pub fn module_name() -> &'static str {
        "TKV3d"
    }

    pub fn version() -> &'static str {
        "7.8.0"
    }

    pub fn description() -> &'static str {
        "3D Visualization Toolkit"
    }

    pub fn is_precompiled_header() -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tkv3d_pch_info() {
        assert_eq!(TKV3dPch::module_name(), "TKV3d");
        assert!(!TKV3dPch::version().is_empty());
        assert!(TKV3dPch::is_precompiled_header());
    }
}
