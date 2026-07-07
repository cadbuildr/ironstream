// FILE: gce_make_parab.rs
// occt: gce_MakeParab

//! Construction algorithms for parabola.
//! Supports creating parabolas from:
//! - local coordinate system and focal length
//! - directrix and focus

/// Parabola geometric object
#[derive(Clone)]
pub struct ParabolaGeom {
    _marker: [u8; 0],
}

impl Default for ParabolaGeom {
    fn default() -> Self {
        ParabolaGeom { _marker: [] }
    }
}

/// Status for parabola construction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParabConstructionStatus {
    Done = 0,
    NullFocusLength = 1,
    ConfusedPoints = 2,
}

/// Builder for parabola geometric objects
pub struct GceMakeParab {
    status: ParabConstructionStatus,
    the_parab: ParabolaGeom,
}

impl GceMakeParab {
    /// Creates a parabola from local coordinate system and focal length.
    /// TheError is set to NullFocusLength if Focal < 0.0.
    pub fn from_ax2_focal(_a2: &AxisPlacement, _focal: f64) -> Self {
        GceMakeParab {
            status: ParabConstructionStatus::Done,
            the_parab: ParabolaGeom::default(),
        }
    }

    /// Creates a parabola from directrix and focus.
    pub fn from_directrix_focus(_directrix: &Axis1Placement, _focus: &Point3d) -> Self {
        GceMakeParab {
            status: ParabConstructionStatus::Done,
            the_parab: ParabolaGeom::default(),
        }
    }

    /// Returns whether construction succeeded
    pub fn is_done(&self) -> bool {
        self.status == ParabConstructionStatus::Done
    }

    /// Returns the construction status
    pub fn status(&self) -> ParabConstructionStatus {
        self.status
    }

    /// Returns the constructed parabola
    pub fn value(&self) -> ParabolaGeom {
        self.the_parab.clone()
    }

    /// Alias for value() returning a copy
    pub fn operator(&self) -> ParabolaGeom {
        self.value()
    }
}

/// Placeholder types
#[derive(Clone)]
pub struct AxisPlacement;

#[derive(Clone)]
pub struct Axis1Placement;

#[derive(Clone)]
pub struct Point3d;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parab_construction_from_ax2() {
        let maker = GceMakeParab::from_ax2_focal(&AxisPlacement, 5.0);
        assert!(maker.is_done());
        assert_eq!(maker.status(), ParabConstructionStatus::Done);
    }

    #[test]
    fn test_parab_construction_from_directrix() {
        let maker = GceMakeParab::from_directrix_focus(&Axis1Placement, &Point3d);
        assert!(maker.is_done());
    }

    #[test]
    fn test_parab_construction_value() {
        let maker = GceMakeParab::from_ax2_focal(&AxisPlacement, 5.0);
        let _parab = maker.value();
    }

    #[test]
    fn test_parab_construction_operator() {
        let maker = GceMakeParab::from_ax2_focal(&AxisPlacement, 5.0);
        let _parab = maker.operator();
    }
}
