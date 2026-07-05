// FILE: draw_tr_surf_surface.rs
// occt: DrawTrSurf_Surface

//! Base class for drawable surfaces.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawColor {
    White,
    Red,
    Green,
}

impl Default for DrawColor {
    fn default() -> Self {
        DrawColor::White
    }
}

#[derive(Clone, Debug)]
pub struct Display;

#[derive(Clone, Debug)]
pub struct DrawTrSurfSurface {
    color: DrawColor,
    u_samples: usize,
    v_samples: usize,
}

impl DrawTrSurfSurface {
    pub fn new() -> Self {
        Self {
            color: DrawColor::default(),
            u_samples: 50,
            v_samples: 50,
        }
    }

    pub fn draw_on(&self, _display: &mut Display) {}

    pub fn set_color(&mut self, color: DrawColor) {
        self.color = color;
    }

    pub fn color(&self) -> DrawColor {
        self.color
    }

    pub fn set_u_samples(&mut self, count: usize) {
        self.u_samples = count;
    }

    pub fn set_v_samples(&mut self, count: usize) {
        self.v_samples = count;
    }
}

impl Default for DrawTrSurfSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let surface = DrawTrSurfSurface::new();
        assert_eq!(surface.color(), DrawColor::White);
    }

    #[test]
    fn test_color() {
        let mut surface = DrawTrSurfSurface::new();
        surface.set_color(DrawColor::Red);
        assert_eq!(surface.color(), DrawColor::Red);
    }

    #[test]
    fn test_samples() {
        let mut surface = DrawTrSurfSurface::new();
        surface.set_u_samples(100);
        surface.set_v_samples(150);

        assert_eq!(surface.u_samples, 100);
        assert_eq!(surface.v_samples, 150);
    }
}
