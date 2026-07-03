// FILE: topo_orient.rs
//
// Orientation and state primitives for boundary-representation topology.
// Models TopAbs_Orientation and BRep_Builder orientation semantics from OCCT.

// occt: TopAbs_Orientation
/// Describes the relative orientation of a topological shape with respect
/// to its parent.  The four values mirror TopAbs_Orientation exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Orientation {
    /// Shape has the same sense as its support (TopAbs_FORWARD).
    Forward,
    /// Shape has the opposite sense to its support (TopAbs_REVERSED).
    Reversed,
    /// Shape is inside its support (TopAbs_INTERNAL).
    Internal,
    /// Shape is outside its support (TopAbs_EXTERNAL).
    External,
}

impl Orientation {
    /// Return the reversed orientation.
    ///
    /// `Forward` <-> `Reversed`; `Internal` and `External` are unchanged.
    /// Mirrors `TopAbs::Reverse` from OCCT.
    pub fn reverse(&self) -> Orientation {
        match self {
            Orientation::Forward => Orientation::Reversed,
            Orientation::Reversed => Orientation::Forward,
            Orientation::Internal => Orientation::Internal,
            Orientation::External => Orientation::External,
        }
    }

    /// Returns `true` when the orientation is `Forward`.
    pub fn is_forward(&self) -> bool {
        matches!(self, Orientation::Forward)
    }

    /// Returns `true` when the orientation is `Reversed`.
    pub fn is_reversed(&self) -> bool {
        matches!(self, Orientation::Reversed)
    }
}

// occt: TopAbs_State
/// Classifies the position of a point or sub-shape relative to a shape.
/// Mirrors `TopAbs_State` from OCCT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    /// Point / shape is inside (TopAbs_IN).
    In,
    /// Point / shape is outside (TopAbs_OUT).
    Out,
    /// Point / shape lies on the boundary (TopAbs_ON).
    On,
    /// Position is indeterminate (TopAbs_UNKNOWN).
    Unknown,
}

// occt: BRep_Builder
/// A named topological shape together with its orientation.
///
/// Models the shape-with-orientation concept used throughout `BRep_Builder`
/// when sub-shapes are added to a compound with an explicit orientation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrientedShape {
    pub name: String,
    pub orientation: Orientation,
}

impl OrientedShape {
    /// Create a new `OrientedShape` with `Forward` orientation.
    pub fn new(name: &str) -> Self {
        OrientedShape {
            name: name.to_owned(),
            orientation: Orientation::Forward,
        }
    }

    /// Create a new `OrientedShape` with the given orientation.
    pub fn with_orientation(name: &str, o: Orientation) -> Self {
        OrientedShape {
            name: name.to_owned(),
            orientation: o,
        }
    }

    /// Return a copy of this shape with its orientation reversed.
    ///
    /// `Forward` <-> `Reversed`; `Internal` / `External` are unchanged.
    pub fn reversed(&self) -> OrientedShape {
        OrientedShape {
            name: self.name.clone(),
            orientation: self.orientation.reverse(),
        }
    }

    /// Return a copy of this shape with its orientation complemented.
    ///
    /// Complement table (mirrors `TopAbs::Complement`):
    /// - `Forward`  -> `Internal`
    /// - `Internal` -> `Forward`
    /// - `Reversed` -> `External`
    /// - `External` -> `Reversed`
    pub fn complement(&self) -> OrientedShape {
        let o = match self.orientation {
            Orientation::Forward => Orientation::Internal,
            Orientation::Internal => Orientation::Forward,
            Orientation::Reversed => Orientation::External,
            Orientation::External => Orientation::Reversed,
        };
        OrientedShape {
            name: self.name.clone(),
            orientation: o,
        }
    }
}

/// Compose two orientations according to the OCCT composition table.
///
/// This mirrors `TopAbs::Compose` / the composition rules used by
/// `BRep_Builder` when propagating orientation through the shape hierarchy.
///
/// Composition table:
/// | o1       | o2       | result   |
/// |----------|----------|----------|
/// | Forward  | Forward  | Forward  |
/// | Forward  | Reversed | Reversed |
/// | Reversed | Forward  | Reversed |
/// | Reversed | Reversed | Forward  |
/// | Internal | *        | Internal |
/// | External | *        | External |
/// | *        | Internal | Internal |
/// | *        | External | External |
///
/// `Internal` / `External` always take precedence, with `o1` checked first.
// occt: TopAbs_Orientation
pub fn compose_orientations(o1: &Orientation, o2: &Orientation) -> Orientation {
    match (o1, o2) {
        (Orientation::Internal, _) => Orientation::Internal,
        (Orientation::External, _) => Orientation::External,
        (_, Orientation::Internal) => Orientation::Internal,
        (_, Orientation::External) => Orientation::External,
        (Orientation::Forward, Orientation::Forward) => Orientation::Forward,
        (Orientation::Forward, Orientation::Reversed) => Orientation::Reversed,
        (Orientation::Reversed, Orientation::Forward) => Orientation::Reversed,
        (Orientation::Reversed, Orientation::Reversed) => Orientation::Forward,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_swaps_forward_reversed() {
        assert_eq!(Orientation::Forward.reverse(), Orientation::Reversed);
        assert_eq!(Orientation::Reversed.reverse(), Orientation::Forward);
    }

    #[test]
    fn reverse_leaves_internal_external() {
        assert_eq!(Orientation::Internal.reverse(), Orientation::Internal);
        assert_eq!(Orientation::External.reverse(), Orientation::External);
    }

    #[test]
    fn is_forward_is_reversed() {
        assert!(Orientation::Forward.is_forward());
        assert!(!Orientation::Forward.is_reversed());
        assert!(Orientation::Reversed.is_reversed());
        assert!(!Orientation::Reversed.is_forward());
    }

    #[test]
    fn oriented_shape_new_defaults_to_forward() {
        let s = OrientedShape::new("edge1");
        assert_eq!(s.orientation, Orientation::Forward);
        assert_eq!(s.name, "edge1");
    }

    #[test]
    fn oriented_shape_with_orientation() {
        let s = OrientedShape::with_orientation("face1", Orientation::Reversed);
        assert_eq!(s.orientation, Orientation::Reversed);
    }

    #[test]
    fn oriented_shape_reversed() {
        let s = OrientedShape::new("wire1");
        let r = s.reversed();
        assert_eq!(r.orientation, Orientation::Reversed);
        assert_eq!(r.name, "wire1");
    }

    #[test]
    fn oriented_shape_complement() {
        assert_eq!(
            OrientedShape::with_orientation("s", Orientation::Forward).complement().orientation,
            Orientation::Internal
        );
        assert_eq!(
            OrientedShape::with_orientation("s", Orientation::Internal).complement().orientation,
            Orientation::Forward
        );
        assert_eq!(
            OrientedShape::with_orientation("s", Orientation::Reversed).complement().orientation,
            Orientation::External
        );
        assert_eq!(
            OrientedShape::with_orientation("s", Orientation::External).complement().orientation,
            Orientation::Reversed
        );
    }

    #[test]
    fn compose_orientations_table() {
        use Orientation::*;
        assert_eq!(compose_orientations(&Forward, &Forward), Forward);
        assert_eq!(compose_orientations(&Forward, &Reversed), Reversed);
        assert_eq!(compose_orientations(&Reversed, &Forward), Reversed);
        assert_eq!(compose_orientations(&Reversed, &Reversed), Forward);
        assert_eq!(compose_orientations(&Internal, &Reversed), Internal);
        assert_eq!(compose_orientations(&External, &Forward), External);
        assert_eq!(compose_orientations(&Forward, &Internal), Internal);
        assert_eq!(compose_orientations(&Reversed, &External), External);
    }
}
