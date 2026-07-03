// FILE: aspect_eye.rs
// occt: Aspect_Eye

/// Eye type for stereoscopic rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectEye {
    /// Mono/left eye.
    Left = 0,
    /// Right eye.
    Right = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values() {
        assert_eq!(AspectEye::Left as i32, 0);
        assert_eq!(AspectEye::Right as i32, 1);
    }
}
