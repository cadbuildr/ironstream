// FILE: gce2d_make_mirror.rs
// occt: GCE2d_MakeMirror

/// Creates mirror transformation in 2D.
pub struct Gce2dMakeMirror {
    done: bool,
}

impl Gce2dMakeMirror {
    pub fn new() -> Self {
        Gce2dMakeMirror { done: false }
    }

    pub fn mirror_point(&mut self, px: f64, py: f64, mx: f64, my: f64) -> (f64, f64) {
        let dx = mx - px;
        let dy = my - py;
        let result_x = mx + dx;
        let result_y = my + dy;
        self.done = true;
        (result_x, result_y)
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mirror = Gce2dMakeMirror::new();
        assert!(!mirror.is_done());
    }

    #[test]
    fn test_mirror_point() {
        let mut mirror = Gce2dMakeMirror::new();
        let (x, y) = mirror.mirror_point(1.0, 1.0, 2.0, 2.0);
        assert_eq!(x, 3.0);
        assert_eq!(y, 3.0);
        assert!(mirror.is_done());
    }
}
