// FILE: v3d_o.rs
// occt: V3d

//! Root namespace for 3D viewer package.
//! Contains utilities and constants for 3D view management.

pub struct V3d;

impl V3d {
    pub fn description() -> &'static str {
        "3D Viewer management utilities"
    }

    pub fn version() -> &'static str {
        "1.0"
    }

    pub fn max_views() -> usize {
        256
    }

    pub fn max_lights() -> usize {
        32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v3d_namespace() {
        assert!(!V3d::description().is_empty());
        assert!(V3d::max_views() > 0);
        assert!(V3d::max_lights() > 0);
    }
}
