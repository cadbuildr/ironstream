// FILE: mesh_vs_drawer_attribute.rs
// occt: MeshVS_DrawerAttribute

//! Enumeration of drawer attributes for mesh presentation styling.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DrawerAttribute {
    InteriorStyle = 0,
    InteriorColor = 1,
    BackInteriorColor = 2,
    EdgeColor = 3,
    EdgeType = 4,
    EdgeWidth = 5,
    HatchStyle = 6,
    FrontMaterial = 7,
    BackMaterial = 8,
    BeamType = 9,
    BeamWidth = 10,
    BeamColor = 11,
    MarkerType = 12,
    MarkerColor = 13,
    MarkerScale = 14,
    TextColor = 15,
    TextHeight = 16,
    TextFont = 17,
    TextExpansionFactor = 18,
    TextSpace = 19,
    TextStyle = 20,
    TextDisplayType = 21,
    TextTexFont = 22,
    TextFontAspect = 23,
    VectorColor = 24,
    VectorMaxLength = 25,
    VectorArrowPart = 26,
    IsAllowOverlapped = 27,
    Reflection = 28,
    ColorReflection = 29,
    ShrinkCoeff = 30,
    MaxFaceNodes = 31,
    ComputeTime = 32,
    ComputeSelectionTime = 33,
    DisplayNodes = 34,
    SelectableAuto = 35,
    ShowEdges = 36,
    SmoothShading = 37,
    SuppressBackFaces = 38,
    User = 39,
}

impl DrawerAttribute {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(DrawerAttribute::InteriorStyle),
            1 => Some(DrawerAttribute::InteriorColor),
            2 => Some(DrawerAttribute::BackInteriorColor),
            3 => Some(DrawerAttribute::EdgeColor),
            4 => Some(DrawerAttribute::EdgeType),
            5 => Some(DrawerAttribute::EdgeWidth),
            6 => Some(DrawerAttribute::HatchStyle),
            7 => Some(DrawerAttribute::FrontMaterial),
            8 => Some(DrawerAttribute::BackMaterial),
            9 => Some(DrawerAttribute::BeamType),
            10 => Some(DrawerAttribute::BeamWidth),
            11 => Some(DrawerAttribute::BeamColor),
            12 => Some(DrawerAttribute::MarkerType),
            13 => Some(DrawerAttribute::MarkerColor),
            14 => Some(DrawerAttribute::MarkerScale),
            15 => Some(DrawerAttribute::TextColor),
            16 => Some(DrawerAttribute::TextHeight),
            17 => Some(DrawerAttribute::TextFont),
            18 => Some(DrawerAttribute::TextExpansionFactor),
            19 => Some(DrawerAttribute::TextSpace),
            20 => Some(DrawerAttribute::TextStyle),
            21 => Some(DrawerAttribute::TextDisplayType),
            22 => Some(DrawerAttribute::TextTexFont),
            23 => Some(DrawerAttribute::TextFontAspect),
            24 => Some(DrawerAttribute::VectorColor),
            25 => Some(DrawerAttribute::VectorMaxLength),
            26 => Some(DrawerAttribute::VectorArrowPart),
            27 => Some(DrawerAttribute::IsAllowOverlapped),
            28 => Some(DrawerAttribute::Reflection),
            29 => Some(DrawerAttribute::ColorReflection),
            30 => Some(DrawerAttribute::ShrinkCoeff),
            31 => Some(DrawerAttribute::MaxFaceNodes),
            32 => Some(DrawerAttribute::ComputeTime),
            33 => Some(DrawerAttribute::ComputeSelectionTime),
            34 => Some(DrawerAttribute::DisplayNodes),
            35 => Some(DrawerAttribute::SelectableAuto),
            36 => Some(DrawerAttribute::ShowEdges),
            37 => Some(DrawerAttribute::SmoothShading),
            38 => Some(DrawerAttribute::SuppressBackFaces),
            39 => Some(DrawerAttribute::User),
            _ => None,
        }
    }

    pub fn is_color_attribute(&self) -> bool {
        matches!(
            self,
            DrawerAttribute::InteriorColor
                | DrawerAttribute::BackInteriorColor
                | DrawerAttribute::EdgeColor
                | DrawerAttribute::BeamColor
                | DrawerAttribute::MarkerColor
                | DrawerAttribute::TextColor
                | DrawerAttribute::VectorColor
        )
    }

    pub fn is_material_attribute(&self) -> bool {
        matches!(self, DrawerAttribute::FrontMaterial | DrawerAttribute::BackMaterial)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drawer_attribute_conversions() {
        assert_eq!(DrawerAttribute::InteriorStyle.as_u32(), 0);
        assert_eq!(DrawerAttribute::InteriorColor.as_u32(), 1);
        assert_eq!(DrawerAttribute::User.as_u32(), 39);
    }

    #[test]
    fn drawer_attribute_from_u32() {
        assert_eq!(DrawerAttribute::from_u32(0), Some(DrawerAttribute::InteriorStyle));
        assert_eq!(DrawerAttribute::from_u32(15), Some(DrawerAttribute::TextColor));
        assert_eq!(DrawerAttribute::from_u32(39), Some(DrawerAttribute::User));
        assert_eq!(DrawerAttribute::from_u32(99), None);
    }

    #[test]
    fn drawer_attribute_predicates() {
        assert!(DrawerAttribute::InteriorColor.is_color_attribute());
        assert!(DrawerAttribute::FrontMaterial.is_material_attribute());
        assert!(!DrawerAttribute::InteriorStyle.is_color_attribute());
    }

    #[test]
    fn drawer_attribute_ordering() {
        assert!(DrawerAttribute::InteriorStyle < DrawerAttribute::InteriorColor);
        assert!(DrawerAttribute::TextColor < DrawerAttribute::VectorColor);
    }
}
