// FILE: prs3d_basic_aspect.rs
// occt: Prs3d_BasicAspect

#[derive(Clone, Debug)]
pub struct Prs3dBasicAspect { pub aspect_id: u32, pub aspect_type: u32 }

impl Prs3dBasicAspect {
    pub fn new(aspect_id: u32, aspect_type: u32) -> Self {
        Self { aspect_id, aspect_type }
    }
}

impl Default for Prs3dBasicAspect {
    fn default() -> Self { Self::new(0, 0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let a = Prs3dBasicAspect::new(1, 5);
        assert_eq!(a.aspect_id, 1);
    }
}
