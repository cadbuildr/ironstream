// FILE: draw_color.rs
// occt: Draw_Color

//! Represents a color for drawing in the Draw application.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawColor {
    kind: DrawColorKind,
}

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

impl DrawColor {
    /// Create a default white color
    pub fn new() -> Self {
        DrawColor {
            kind: DrawColorKind::Blanc,
        }
    }

    /// Create a color from a ColorKind
    pub fn from_kind(kind: DrawColorKind) -> Self {
        DrawColor { kind }
    }

    /// Get the color kind ID
    pub fn id(&self) -> DrawColorKind {
        self.kind
    }

    /// Get the color kind as an integer
    pub fn id_int(&self) -> u32 {
        self.kind as u32
    }
}

impl Default for DrawColor {
    fn default() -> Self {
        Self::new()
    }
}

impl From<DrawColorKind> for DrawColor {
    fn from(kind: DrawColorKind) -> Self {
        DrawColor::from_kind(kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_default() {
        let color = DrawColor::new();
        assert_eq!(color.id(), DrawColorKind::Blanc);
    }

    #[test]
    fn test_color_from_kind() {
        let color = DrawColor::from_kind(DrawColorKind::Rouge);
        assert_eq!(color.id(), DrawColorKind::Rouge);
        assert_eq!(color.id_int(), 1);
    }

    #[test]
    fn test_color_id_int() {
        let color = DrawColor::from_kind(DrawColorKind::Bleu);
        assert_eq!(color.id_int(), 3);
    }

    #[test]
    fn test_color_from_trait() {
        let color: DrawColor = DrawColorKind::Magenta.into();
        assert_eq!(color.id(), DrawColorKind::Magenta);
    }

    #[test]
    fn test_all_colors() {
        let colors = vec![
            (DrawColorKind::Blanc, 0),
            (DrawColorKind::Rouge, 1),
            (DrawColorKind::Vert, 2),
            (DrawColorKind::Bleu, 3),
            (DrawColorKind::Cyan, 4),
            (DrawColorKind::Or, 5),
            (DrawColorKind::Magenta, 6),
            (DrawColorKind::Marron, 7),
            (DrawColorKind::Orange, 8),
            (DrawColorKind::Rose, 9),
            (DrawColorKind::Saumon, 10),
            (DrawColorKind::Violet, 11),
            (DrawColorKind::Jaune, 12),
            (DrawColorKind::Kaki, 13),
            (DrawColorKind::Corail, 14),
        ];

        for (kind, expected_id) in colors {
            let color = DrawColor::from_kind(kind);
            assert_eq!(color.id_int(), expected_id);
        }
    }
}
