// FILE: ais_mouse_gesture.rs
// occt: AIS_MouseGesture

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseGesture {
    None = 0, SelectRectangle = 1, SelectLasso = 2, Zoom = 3,
    ZoomVertical = 4, ZoomWindow = 5, Pan = 6, RotateOrbit = 7,
    RotateView = 8, Drag = 9,
}

impl MouseGesture {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val { 0 => Some(MouseGesture::None), _ => None, }
    }
    pub fn to_u32(self) -> u32 { self as u32 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_variants() { assert_eq!(MouseGesture::None as u32, 0); }
}
