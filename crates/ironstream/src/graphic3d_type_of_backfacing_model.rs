// FILE: graphic3d_type_of_backfacing_model.rs
// occt: Graphic3d_TypeOfBackfacingModel

/// Enumeration of back-face culling modes for the 3D view.
///
/// Defines how back-facing polygons are rendered in the visualization,
/// controlling culling behavior for opaque and transparent objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Graphic3dTypeOfBackfacingModel {
    /// Automatic back-face culling for opaque closed groups (solids).
    /// Transparent objects are rendered double-sided.
    Auto,
    /// Double-sided shading; no culling applied
    DoubleSided,
    /// Back-face culling enabled
    BackCulled,
    /// Front-face culling enabled (inverse of back-culling)
    FrontCulled,
}

impl Graphic3dTypeOfBackfacingModel {
    /// Returns whether this mode is automatic
    pub fn is_auto(&self) -> bool {
        matches!(self, Graphic3dTypeOfBackfacingModel::Auto)
    }

    /// Returns whether this mode is double-sided
    pub fn is_double_sided(&self) -> bool {
        matches!(self, Graphic3dTypeOfBackfacingModel::DoubleSided)
    }

    /// Returns whether this mode culls back faces
    pub fn is_back_culled(&self) -> bool {
        matches!(self, Graphic3dTypeOfBackfacingModel::BackCulled)
    }

    /// Returns whether this mode culls front faces
    pub fn is_front_culled(&self) -> bool {
        matches!(self, Graphic3dTypeOfBackfacingModel::FrontCulled)
    }

    /// Returns true if any form of culling is enabled
    pub fn has_culling(&self) -> bool {
        matches!(
            self,
            Graphic3dTypeOfBackfacingModel::BackCulled
                | Graphic3dTypeOfBackfacingModel::FrontCulled
        )
    }

    /// Returns a descriptive string for the mode
    pub fn description(&self) -> &'static str {
        match self {
            Graphic3dTypeOfBackfacingModel::Auto => "Automatic back-face culling for closed shapes",
            Graphic3dTypeOfBackfacingModel::DoubleSided => "Double-sided shading (no culling)",
            Graphic3dTypeOfBackfacingModel::BackCulled => "Back-face culling",
            Graphic3dTypeOfBackfacingModel::FrontCulled => "Front-face culling",
        }
    }
}

impl std::fmt::Display for Graphic3dTypeOfBackfacingModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Graphic3dTypeOfBackfacingModel::Auto => "Auto",
                Graphic3dTypeOfBackfacingModel::DoubleSided => "DoubleSided",
                Graphic3dTypeOfBackfacingModel::BackCulled => "BackCulled",
                Graphic3dTypeOfBackfacingModel::FrontCulled => "FrontCulled",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_auto() {
        assert!(Graphic3dTypeOfBackfacingModel::Auto.is_auto());
        assert!(!Graphic3dTypeOfBackfacingModel::DoubleSided.is_auto());
        assert!(!Graphic3dTypeOfBackfacingModel::BackCulled.is_auto());
        assert!(!Graphic3dTypeOfBackfacingModel::FrontCulled.is_auto());
    }

    #[test]
    fn test_is_double_sided() {
        assert!(!Graphic3dTypeOfBackfacingModel::Auto.is_double_sided());
        assert!(Graphic3dTypeOfBackfacingModel::DoubleSided.is_double_sided());
        assert!(!Graphic3dTypeOfBackfacingModel::BackCulled.is_double_sided());
        assert!(!Graphic3dTypeOfBackfacingModel::FrontCulled.is_double_sided());
    }

    #[test]
    fn test_is_back_culled() {
        assert!(!Graphic3dTypeOfBackfacingModel::Auto.is_back_culled());
        assert!(!Graphic3dTypeOfBackfacingModel::DoubleSided.is_back_culled());
        assert!(Graphic3dTypeOfBackfacingModel::BackCulled.is_back_culled());
        assert!(!Graphic3dTypeOfBackfacingModel::FrontCulled.is_back_culled());
    }

    #[test]
    fn test_is_front_culled() {
        assert!(!Graphic3dTypeOfBackfacingModel::Auto.is_front_culled());
        assert!(!Graphic3dTypeOfBackfacingModel::DoubleSided.is_front_culled());
        assert!(!Graphic3dTypeOfBackfacingModel::BackCulled.is_front_culled());
        assert!(Graphic3dTypeOfBackfacingModel::FrontCulled.is_front_culled());
    }

    #[test]
    fn test_has_culling() {
        assert!(!Graphic3dTypeOfBackfacingModel::Auto.has_culling());
        assert!(!Graphic3dTypeOfBackfacingModel::DoubleSided.has_culling());
        assert!(Graphic3dTypeOfBackfacingModel::BackCulled.has_culling());
        assert!(Graphic3dTypeOfBackfacingModel::FrontCulled.has_culling());
    }

    #[test]
    fn test_description() {
        assert!(Graphic3dTypeOfBackfacingModel::Auto.description().contains("Automatic"));
        assert!(Graphic3dTypeOfBackfacingModel::DoubleSided
            .description()
            .contains("Double-sided"));
        assert!(Graphic3dTypeOfBackfacingModel::BackCulled
            .description()
            .contains("Back-face"));
        assert!(Graphic3dTypeOfBackfacingModel::FrontCulled
            .description()
            .contains("Front-face"));
    }

    #[test]
    fn test_display_trait() {
        assert_eq!(format!("{}", Graphic3dTypeOfBackfacingModel::Auto), "Auto");
        assert_eq!(
            format!("{}", Graphic3dTypeOfBackfacingModel::DoubleSided),
            "DoubleSided"
        );
        assert_eq!(
            format!("{}", Graphic3dTypeOfBackfacingModel::BackCulled),
            "BackCulled"
        );
        assert_eq!(
            format!("{}", Graphic3dTypeOfBackfacingModel::FrontCulled),
            "FrontCulled"
        );
    }

    #[test]
    fn test_equality() {
        assert_eq!(
            Graphic3dTypeOfBackfacingModel::Auto,
            Graphic3dTypeOfBackfacingModel::Auto
        );
        assert_ne!(
            Graphic3dTypeOfBackfacingModel::Auto,
            Graphic3dTypeOfBackfacingModel::DoubleSided
        );
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Graphic3dTypeOfBackfacingModel::Auto);
        set.insert(Graphic3dTypeOfBackfacingModel::DoubleSided);
        set.insert(Graphic3dTypeOfBackfacingModel::BackCulled);
        set.insert(Graphic3dTypeOfBackfacingModel::FrontCulled);

        assert_eq!(set.len(), 4);
        assert!(set.contains(&Graphic3dTypeOfBackfacingModel::Auto));
    }

    #[test]
    fn test_copy_clone() {
        let mode = Graphic3dTypeOfBackfacingModel::BackCulled;
        let copied = mode;
        let cloned = mode.clone();

        assert_eq!(mode, copied);
        assert_eq!(mode, cloned);
    }
}
