// FILE: draw_tr_surf_drawable.rs
// occt: DrawTrSurf_Drawable

//! Base drawable class for tracing surfaces and curves.

#[derive(Clone, Debug)]
pub struct Display;

impl Display {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct DrawTrSurfDrawable;

impl DrawTrSurfDrawable {
    pub fn new() -> Self {
        Self
    }

    pub fn draw_on(&self, _display: &mut Display) {
        // Virtual draw method
    }
}

impl Default for DrawTrSurfDrawable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _drawable = DrawTrSurfDrawable::new();
    }

    #[test]
    fn test_draw() {
        let drawable = DrawTrSurfDrawable::new();
        let mut display = Display::new();
        drawable.draw_on(&mut display);
    }
}
