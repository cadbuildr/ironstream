// FILE: aspect_width_of_line.rs
// occt: Aspect_WidthOfLine

/// Definition of line width types.
///
/// - Thin: thin line (1 pixel width)
/// - Medium: medium width of 0.5 MM
/// - Thick: thick width of 0.7 MM
/// - VeryThick: very thick width of 1.5 MM
/// - UserDefined: defined by users
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AspectWidthOfLine {
  /// Thin line (1 pixel width)
  Thin = 0,
  /// Medium width of 0.5 MM
  Medium = 1,
  /// Thick width of 0.7 MM
  Thick = 2,
  /// Very thick width of 1.5 MM
  VeryThick = 3,
  /// User-defined width
  UserDefined = 4,
}

impl AspectWidthOfLine {
  /// Returns the approximate width in millimeters for standard line types.
  /// Returns None for UserDefined since the width is defined by the user.
  pub fn width_mm(&self) -> Option<f64> {
    match self {
      AspectWidthOfLine::Thin => Some(0.0),       // 1 pixel
      AspectWidthOfLine::Medium => Some(0.5),
      AspectWidthOfLine::Thick => Some(0.7),
      AspectWidthOfLine::VeryThick => Some(1.5),
      AspectWidthOfLine::UserDefined => None,
    }
  }

  /// Returns the integer discriminant value (0-4)
  pub fn as_u32(&self) -> u32 {
    *self as u32
  }
}

impl From<u32> for AspectWidthOfLine {
  fn from(value: u32) -> Self {
    match value {
      0 => AspectWidthOfLine::Thin,
      1 => AspectWidthOfLine::Medium,
      2 => AspectWidthOfLine::Thick,
      3 => AspectWidthOfLine::VeryThick,
      4 => AspectWidthOfLine::UserDefined,
      _ => AspectWidthOfLine::Medium, // Default to medium for out-of-range values
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_width_of_line_discriminants() {
    assert_eq!(AspectWidthOfLine::Thin.as_u32(), 0);
    assert_eq!(AspectWidthOfLine::Medium.as_u32(), 1);
    assert_eq!(AspectWidthOfLine::Thick.as_u32(), 2);
    assert_eq!(AspectWidthOfLine::VeryThick.as_u32(), 3);
    assert_eq!(AspectWidthOfLine::UserDefined.as_u32(), 4);
  }

  #[test]
  fn test_width_of_line_mm_values() {
    assert_eq!(AspectWidthOfLine::Thin.width_mm(), Some(0.0));
    assert_eq!(AspectWidthOfLine::Medium.width_mm(), Some(0.5));
    assert_eq!(AspectWidthOfLine::Thick.width_mm(), Some(0.7));
    assert_eq!(AspectWidthOfLine::VeryThick.width_mm(), Some(1.5));
    assert_eq!(AspectWidthOfLine::UserDefined.width_mm(), None);
  }

  #[test]
  fn test_from_u32() {
    assert_eq!(AspectWidthOfLine::from(0), AspectWidthOfLine::Thin);
    assert_eq!(AspectWidthOfLine::from(1), AspectWidthOfLine::Medium);
    assert_eq!(AspectWidthOfLine::from(2), AspectWidthOfLine::Thick);
    assert_eq!(AspectWidthOfLine::from(3), AspectWidthOfLine::VeryThick);
    assert_eq!(AspectWidthOfLine::from(4), AspectWidthOfLine::UserDefined);
  }

  #[test]
  fn test_from_u32_out_of_range() {
    // Out of range values default to Medium
    assert_eq!(AspectWidthOfLine::from(5), AspectWidthOfLine::Medium);
    assert_eq!(AspectWidthOfLine::from(100), AspectWidthOfLine::Medium);
    assert_eq!(AspectWidthOfLine::from(u32::MAX), AspectWidthOfLine::Medium);
  }

  #[test]
  fn test_equality() {
    assert_eq!(AspectWidthOfLine::Thin, AspectWidthOfLine::Thin);
    assert_ne!(AspectWidthOfLine::Thin, AspectWidthOfLine::Medium);
  }

  #[test]
  fn test_clone_copy() {
    let wol1 = AspectWidthOfLine::Thick;
    let wol2 = wol1;
    assert_eq!(wol1, wol2);

    let wol3 = wol1.clone();
    assert_eq!(wol1, wol3);
  }

  #[test]
  fn test_hash_eq() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(AspectWidthOfLine::Thin);
    set.insert(AspectWidthOfLine::Medium);
    set.insert(AspectWidthOfLine::Thin);

    assert_eq!(set.len(), 2);
    assert!(set.contains(&AspectWidthOfLine::Thin));
    assert!(set.contains(&AspectWidthOfLine::Medium));
  }
}
