// FILE: draw_color_kind.rs
// occt: Draw_ColorKind

//! Enumeration of standard colors for drawing in the Draw application.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrawColorKind {
    /// White
    Blanc = 0,
    /// Red
    Rouge = 1,
    /// Green
    Vert = 2,
    /// Blue
    Bleu = 3,
    /// Cyan
    Cyan = 4,
    /// Gold
    Or = 5,
    /// Magenta
    Magenta = 6,
    /// Brown
    Marron = 7,
    /// Orange
    Orange = 8,
    /// Pink
    Rose = 9,
    /// Salmon
    Saumon = 10,
    /// Violet
    Violet = 11,
    /// Yellow
    Jaune = 12,
    /// Khaki
    Kaki = 13,
    /// Coral
    Corail = 14,
}

impl DrawColorKind {
    /// Convert from integer representation
    pub fn from_int(value: u32) -> Option<Self> {
        match value {
            0 => Some(DrawColorKind::Blanc),
            1 => Some(DrawColorKind::Rouge),
            2 => Some(DrawColorKind::Vert),
            3 => Some(DrawColorKind::Bleu),
            4 => Some(DrawColorKind::Cyan),
            5 => Some(DrawColorKind::Or),
            6 => Some(DrawColorKind::Magenta),
            7 => Some(DrawColorKind::Marron),
            8 => Some(DrawColorKind::Orange),
            9 => Some(DrawColorKind::Rose),
            10 => Some(DrawColorKind::Saumon),
            11 => Some(DrawColorKind::Violet),
            12 => Some(DrawColorKind::Jaune),
            13 => Some(DrawColorKind::Kaki),
            14 => Some(DrawColorKind::Corail),
            _ => None,
        }
    }

    /// Convert to integer representation
    pub fn to_int(self) -> u32 {
        self as u32
    }
}

impl Default for DrawColorKind {
    fn default() -> Self {
        DrawColorKind::Blanc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_kind_values() {
        assert_eq!(DrawColorKind::Blanc.to_int(), 0);
        assert_eq!(DrawColorKind::Rouge.to_int(), 1);
        assert_eq!(DrawColorKind::Vert.to_int(), 2);
        assert_eq!(DrawColorKind::Bleu.to_int(), 3);
        assert_eq!(DrawColorKind::Cyan.to_int(), 4);
    }

    #[test]
    fn test_color_kind_from_int() {
        assert_eq!(DrawColorKind::from_int(0), Some(DrawColorKind::Blanc));
        assert_eq!(DrawColorKind::from_int(1), Some(DrawColorKind::Rouge));
        assert_eq!(DrawColorKind::from_int(14), Some(DrawColorKind::Corail));
        assert_eq!(DrawColorKind::from_int(15), None);
    }

    #[test]
    fn test_color_kind_round_trip() {
        let color = DrawColorKind::Magenta;
        let as_int = color.to_int();
        assert_eq!(DrawColorKind::from_int(as_int), Some(color));
    }

    #[test]
    fn test_color_kind_default() {
        assert_eq!(DrawColorKind::default(), DrawColorKind::Blanc);
    }
}
