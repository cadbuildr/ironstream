// FILE: db_rep_face.rs
// occt: DBRep_Face

use std::cell::RefCell;
use std::rc::Rc;

/// ISO type enumeration for geometric surfaces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeomAbsIsoType {
    UIso = 0,
    VIso = 1,
}

impl From<i32> for GeomAbsIsoType {
    fn from(val: i32) -> Self {
        match val {
            0 => GeomAbsIsoType::UIso,
            _ => GeomAbsIsoType::VIso,
        }
    }
}

impl Into<i32> for GeomAbsIsoType {
    fn into(self) -> i32 {
        match self {
            GeomAbsIsoType::UIso => 0,
            GeomAbsIsoType::VIso => 1,
        }
    }
}

/// Color representation (RGBA)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for DrawColor {
    fn default() -> Self {
        DrawColor {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

/// Simple 2D rectangle representation for a face
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Face {
    pub x: f64,
    pub y: f64,
}

impl Default for Face {
    fn default() -> Self {
        Face { x: 0.0, y: 0.0 }
    }
}

/// Display of a face with ISO curves and color.
/// Face + Array of iso + color.
pub struct DBRepFace {
    face: Face,
    color: DrawColor,
    types: Vec<i32>,
    params: Vec<f64>,
}

impl DBRepFace {
    /// Create a DBRepFace with N iso intervals.
    pub fn new(face: Face, n: usize, color: DrawColor) -> Self {
        DBRepFace {
            face,
            color,
            types: if n > 0 { vec![0; n] } else { Vec::new() },
            params: if n > 0 { vec![0.0; 3 * n] } else { Vec::new() },
        }
    }

    /// Get the face.
    pub fn face(&self) -> Face {
        self.face
    }

    /// Set the face.
    pub fn set_face(&mut self, f: Face) {
        self.face = f;
    }

    /// Get the number of ISO intervals.
    pub fn nb_isos(&self) -> usize {
        self.types.len()
    }

    /// Set ISO information at index I (1-indexed in OCCT, but 0-indexed here).
    pub fn set_iso(&mut self, i: usize, t: GeomAbsIsoType, par: f64, t1: f64, t2: f64) {
        if i < self.types.len() {
            self.types[i] = t.into();
            self.params[3 * i] = par;
            self.params[3 * i + 1] = t1;
            self.params[3 * i + 2] = t2;
        }
    }

    /// Get ISO information at index I (1-indexed in OCCT, but 0-indexed here).
    pub fn get_iso(&self, i: usize) -> Option<(GeomAbsIsoType, f64, f64, f64)> {
        if i < self.types.len() && i < self.params.len() / 3 {
            let t = GeomAbsIsoType::from(self.types[i]);
            let par = self.params[3 * i];
            let t1 = self.params[3 * i + 1];
            let t2 = self.params[3 * i + 2];
            Some((t, par, t1, t2))
        } else {
            None
        }
    }

    /// Get the color.
    pub fn color(&self) -> DrawColor {
        self.color
    }

    /// Set the color.
    pub fn set_color(&mut self, c: DrawColor) {
        self.color = c;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let face = Face { x: 1.0, y: 2.0 };
        let color = DrawColor {
            r: 1.0,
            g: 0.5,
            b: 0.25,
            a: 1.0,
        };
        let dbrep = DBRepFace::new(face, 3, color);

        assert_eq!(dbrep.face(), face);
        assert_eq!(dbrep.nb_isos(), 3);
        assert_eq!(dbrep.color(), color);
    }

    #[test]
    fn test_set_face() {
        let face1 = Face { x: 1.0, y: 2.0 };
        let face2 = Face { x: 3.0, y: 4.0 };
        let color = DrawColor::default();
        let mut dbrep = DBRepFace::new(face1, 1, color);

        dbrep.set_face(face2);
        assert_eq!(dbrep.face(), face2);
    }

    #[test]
    fn test_iso_operations() {
        let face = Face { x: 0.0, y: 0.0 };
        let color = DrawColor::default();
        let mut dbrep = DBRepFace::new(face, 2, color);

        dbrep.set_iso(0, GeomAbsIsoType::UIso, 1.5, 0.0, 10.0);
        let (t, par, t1, t2) = dbrep.get_iso(0).unwrap();
        assert_eq!(t, GeomAbsIsoType::UIso);
        assert_eq!(par, 1.5);
        assert_eq!(t1, 0.0);
        assert_eq!(t2, 10.0);

        dbrep.set_iso(1, GeomAbsIsoType::VIso, 2.5, 1.0, 11.0);
        let (t, par, t1, t2) = dbrep.get_iso(1).unwrap();
        assert_eq!(t, GeomAbsIsoType::VIso);
        assert_eq!(par, 2.5);
        assert_eq!(t1, 1.0);
        assert_eq!(t2, 11.0);
    }

    #[test]
    fn test_color() {
        let face = Face::default();
        let color1 = DrawColor {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 0.8,
        };
        let color2 = DrawColor {
            r: 0.9,
            g: 0.8,
            b: 0.7,
            a: 1.0,
        };
        let mut dbrep = DBRepFace::new(face, 1, color1);

        assert_eq!(dbrep.color(), color1);
        dbrep.set_color(color2);
        assert_eq!(dbrep.color(), color2);
    }

    #[test]
    fn test_zero_isos() {
        let face = Face::default();
        let color = DrawColor::default();
        let dbrep = DBRepFace::new(face, 0, color);

        assert_eq!(dbrep.nb_isos(), 0);
        assert_eq!(dbrep.get_iso(0), None);
    }
}
