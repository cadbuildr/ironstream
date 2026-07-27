// FILE: top_abs.rs

// occt-ref: TopAbs_ShapeEnum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShapeEnum {
    Compound,
    CompSolid,
    Solid,
    Shell,
    Face,
    Wire,
    Edge,
    Vertex,
    Shape,
}

// occt-ref: TopAbs_Orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Orientation {
    Forward,
    Reversed,
    Internal,
    External,
}

// occt: TopAbs_State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    IN,
    OUT,
    ON,
    UNKNOWN,
}

// occt: TopAbs
pub struct TopAbs;

impl TopAbs {
    /// Compose two orientations.
    /// OCCT composition table:
    ///   Forward  x Forward  = Forward
    ///   Forward  x Reversed = Reversed
    ///   Reversed x Forward  = Reversed
    ///   Reversed x Reversed = Forward
    ///   Internal x *        = Internal
    ///   External x *        = External
    ///   * x Internal        = Internal
    ///   * x External        = External
    pub fn compose(o1: Orientation, o2: Orientation) -> Orientation {
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

    /// Reverse an orientation.
    /// Forward <-> Reversed, Internal and External are unchanged.
    pub fn reverse(o: Orientation) -> Orientation {
        match o {
            Orientation::Forward => Orientation::Reversed,
            Orientation::Reversed => Orientation::Forward,
            Orientation::Internal => Orientation::Internal,
            Orientation::External => Orientation::External,
        }
    }

    /// Complement an orientation.
    /// Forward <-> Internal, Reversed <-> External.
    pub fn complement(o: Orientation) -> Orientation {
        match o {
            Orientation::Forward => Orientation::Internal,
            Orientation::Internal => Orientation::Forward,
            Orientation::Reversed => Orientation::External,
            Orientation::External => Orientation::Reversed,
        }
    }

    /// Returns true if a shape of the given type can be closed.
    /// Closed shapes: Wire, Edge, Vertex (can be degenerate/closed),
    /// Shell (can be closed solid boundary), Solid (always closed).
    /// OCCT: Wire, Edge can be closed; Shell can be closed.
    pub fn is_closed(s: ShapeEnum) -> bool {
        matches!(
            s,
            ShapeEnum::Wire | ShapeEnum::Edge | ShapeEnum::Shell | ShapeEnum::Solid
        )
    }

    /// Return a string name for a ShapeEnum value.
    pub fn shape_name(s: ShapeEnum) -> &'static str {
        match s {
            ShapeEnum::Compound => "COMPOUND",
            ShapeEnum::CompSolid => "COMPSOLID",
            ShapeEnum::Solid => "SOLID",
            ShapeEnum::Shell => "SHELL",
            ShapeEnum::Face => "FACE",
            ShapeEnum::Wire => "WIRE",
            ShapeEnum::Edge => "EDGE",
            ShapeEnum::Vertex => "VERTEX",
            ShapeEnum::Shape => "SHAPE",
        }
    }

    /// Return a string name for an Orientation value.
    pub fn orientation_name(o: Orientation) -> &'static str {
        match o {
            Orientation::Forward => "FORWARD",
            Orientation::Reversed => "REVERSED",
            Orientation::Internal => "INTERNAL",
            Orientation::External => "EXTERNAL",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_forward_forward() {
        assert_eq!(
            TopAbs::compose(Orientation::Forward, Orientation::Forward),
            Orientation::Forward
        );
    }

    #[test]
    fn test_compose_forward_reversed() {
        assert_eq!(
            TopAbs::compose(Orientation::Forward, Orientation::Reversed),
            Orientation::Reversed
        );
    }

    #[test]
    fn test_compose_reversed_forward() {
        assert_eq!(
            TopAbs::compose(Orientation::Reversed, Orientation::Forward),
            Orientation::Reversed
        );
    }

    #[test]
    fn test_compose_reversed_reversed() {
        assert_eq!(
            TopAbs::compose(Orientation::Reversed, Orientation::Reversed),
            Orientation::Forward
        );
    }

    #[test]
    fn test_compose_internal_absorbs_all() {
        assert_eq!(
            TopAbs::compose(Orientation::Internal, Orientation::Forward),
            Orientation::Internal
        );
        assert_eq!(
            TopAbs::compose(Orientation::Internal, Orientation::Reversed),
            Orientation::Internal
        );
        assert_eq!(
            TopAbs::compose(Orientation::Internal, Orientation::Internal),
            Orientation::Internal
        );
        assert_eq!(
            TopAbs::compose(Orientation::Internal, Orientation::External),
            Orientation::Internal
        );
    }

    #[test]
    fn test_compose_external_absorbs_all() {
        assert_eq!(
            TopAbs::compose(Orientation::External, Orientation::Forward),
            Orientation::External
        );
        assert_eq!(
            TopAbs::compose(Orientation::External, Orientation::Reversed),
            Orientation::External
        );
        assert_eq!(
            TopAbs::compose(Orientation::External, Orientation::Internal),
            Orientation::External
        );
        assert_eq!(
            TopAbs::compose(Orientation::External, Orientation::External),
            Orientation::External
        );
    }

    #[test]
    fn test_compose_second_internal_wins_over_forward_reversed() {
        assert_eq!(
            TopAbs::compose(Orientation::Forward, Orientation::Internal),
            Orientation::Internal
        );
        assert_eq!(
            TopAbs::compose(Orientation::Reversed, Orientation::Internal),
            Orientation::Internal
        );
        assert_eq!(
            TopAbs::compose(Orientation::Forward, Orientation::External),
            Orientation::External
        );
        assert_eq!(
            TopAbs::compose(Orientation::Reversed, Orientation::External),
            Orientation::External
        );
    }

    #[test]
    fn test_reverse_forward_gives_reversed() {
        assert_eq!(TopAbs::reverse(Orientation::Forward), Orientation::Reversed);
    }

    #[test]
    fn test_reverse_reversed_gives_forward() {
        assert_eq!(TopAbs::reverse(Orientation::Reversed), Orientation::Forward);
    }

    #[test]
    fn test_reverse_internal_unchanged() {
        assert_eq!(
            TopAbs::reverse(Orientation::Internal),
            Orientation::Internal
        );
    }

    #[test]
    fn test_reverse_external_unchanged() {
        assert_eq!(
            TopAbs::reverse(Orientation::External),
            Orientation::External
        );
    }

    #[test]
    fn test_reverse_is_involution() {
        for o in [
            Orientation::Forward,
            Orientation::Reversed,
            Orientation::Internal,
            Orientation::External,
        ] {
            assert_eq!(TopAbs::reverse(TopAbs::reverse(o)), o);
        }
    }

    #[test]
    fn test_complement_forward_gives_internal() {
        assert_eq!(
            TopAbs::complement(Orientation::Forward),
            Orientation::Internal
        );
    }

    #[test]
    fn test_complement_internal_gives_forward() {
        assert_eq!(
            TopAbs::complement(Orientation::Internal),
            Orientation::Forward
        );
    }

    #[test]
    fn test_complement_reversed_gives_external() {
        assert_eq!(
            TopAbs::complement(Orientation::Reversed),
            Orientation::External
        );
    }

    #[test]
    fn test_complement_external_gives_reversed() {
        assert_eq!(
            TopAbs::complement(Orientation::External),
            Orientation::Reversed
        );
    }

    #[test]
    fn test_complement_is_involution() {
        for o in [
            Orientation::Forward,
            Orientation::Reversed,
            Orientation::Internal,
            Orientation::External,
        ] {
            assert_eq!(TopAbs::complement(TopAbs::complement(o)), o);
        }
    }

    #[test]
    fn test_is_closed_wire() {
        assert!(TopAbs::is_closed(ShapeEnum::Wire));
    }

    #[test]
    fn test_is_closed_edge() {
        assert!(TopAbs::is_closed(ShapeEnum::Edge));
    }

    #[test]
    fn test_is_closed_shell() {
        assert!(TopAbs::is_closed(ShapeEnum::Shell));
    }

    #[test]
    fn test_is_closed_solid() {
        assert!(TopAbs::is_closed(ShapeEnum::Solid));
    }

    #[test]
    fn test_not_closed_face() {
        assert!(!TopAbs::is_closed(ShapeEnum::Face));
    }

    #[test]
    fn test_not_closed_vertex() {
        assert!(!TopAbs::is_closed(ShapeEnum::Vertex));
    }

    #[test]
    fn test_not_closed_compound() {
        assert!(!TopAbs::is_closed(ShapeEnum::Compound));
    }

    #[test]
    fn test_not_closed_compsolid() {
        assert!(!TopAbs::is_closed(ShapeEnum::CompSolid));
    }

    #[test]
    fn test_not_closed_shape() {
        assert!(!TopAbs::is_closed(ShapeEnum::Shape));
    }

    #[test]
    fn test_shape_name_all_variants() {
        assert_eq!(TopAbs::shape_name(ShapeEnum::Compound), "COMPOUND");
        assert_eq!(TopAbs::shape_name(ShapeEnum::CompSolid), "COMPSOLID");
        assert_eq!(TopAbs::shape_name(ShapeEnum::Solid), "SOLID");
        assert_eq!(TopAbs::shape_name(ShapeEnum::Shell), "SHELL");
        assert_eq!(TopAbs::shape_name(ShapeEnum::Face), "FACE");
        assert_eq!(TopAbs::shape_name(ShapeEnum::Wire), "WIRE");
        assert_eq!(TopAbs::shape_name(ShapeEnum::Edge), "EDGE");
        assert_eq!(TopAbs::shape_name(ShapeEnum::Vertex), "VERTEX");
        assert_eq!(TopAbs::shape_name(ShapeEnum::Shape), "SHAPE");
    }

    #[test]
    fn test_orientation_name_all_variants() {
        assert_eq!(TopAbs::orientation_name(Orientation::Forward), "FORWARD");
        assert_eq!(TopAbs::orientation_name(Orientation::Reversed), "REVERSED");
        assert_eq!(TopAbs::orientation_name(Orientation::Internal), "INTERNAL");
        assert_eq!(TopAbs::orientation_name(Orientation::External), "EXTERNAL");
    }

    #[test]
    fn test_state_variants_distinct() {
        assert_ne!(State::IN, State::OUT);
        assert_ne!(State::IN, State::ON);
        assert_ne!(State::IN, State::UNKNOWN);
        assert_ne!(State::OUT, State::ON);
        assert_ne!(State::OUT, State::UNKNOWN);
        assert_ne!(State::ON, State::UNKNOWN);
    }

    #[test]
    fn test_shape_enum_ordering_by_debug() {
        // Verify all nine variants exist and are distinct
        let variants = [
            ShapeEnum::Compound,
            ShapeEnum::CompSolid,
            ShapeEnum::Solid,
            ShapeEnum::Shell,
            ShapeEnum::Face,
            ShapeEnum::Wire,
            ShapeEnum::Edge,
            ShapeEnum::Vertex,
            ShapeEnum::Shape,
        ];
        for i in 0..variants.len() {
            for j in 0..variants.len() {
                if i == j {
                    assert_eq!(variants[i], variants[j]);
                } else {
                    assert_ne!(variants[i], variants[j]);
                }
            }
        }
    }

    #[test]
    fn test_compose_associativity_sample() {
        // (F o R) o R should equal F o (R o R)
        let a = Orientation::Forward;
        let b = Orientation::Reversed;
        let c = Orientation::Reversed;
        let lhs = TopAbs::compose(TopAbs::compose(a, b), c);
        let rhs = TopAbs::compose(a, TopAbs::compose(b, c));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_compose_identity_forward() {
        // Forward is the identity for compose over Forward/Reversed
        assert_eq!(
            TopAbs::compose(Orientation::Forward, Orientation::Forward),
            Orientation::Forward
        );
        assert_eq!(
            TopAbs::compose(Orientation::Reversed, Orientation::Forward),
            Orientation::Reversed
        );
    }
}
