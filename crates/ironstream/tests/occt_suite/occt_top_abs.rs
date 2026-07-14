// FILE: tests/occt_top_abs.rs
extern crate ironstream;
use ironstream::top_abs::*;

// --- ShapeEnum ---

#[test]
fn test_shape_enum_compound() {
    assert_eq!(TopAbs::shape_name(ShapeEnum::Compound), "COMPOUND");
}

#[test]
fn test_shape_enum_compsolid() {
    assert_eq!(TopAbs::shape_name(ShapeEnum::CompSolid), "COMPSOLID");
}

#[test]
fn test_shape_enum_solid() {
    assert_eq!(TopAbs::shape_name(ShapeEnum::Solid), "SOLID");
}

#[test]
fn test_shape_enum_shell() {
    assert_eq!(TopAbs::shape_name(ShapeEnum::Shell), "SHELL");
}

#[test]
fn test_shape_enum_face() {
    assert_eq!(TopAbs::shape_name(ShapeEnum::Face), "FACE");
}

#[test]
fn test_shape_enum_wire() {
    assert_eq!(TopAbs::shape_name(ShapeEnum::Wire), "WIRE");
}

#[test]
fn test_shape_enum_edge() {
    assert_eq!(TopAbs::shape_name(ShapeEnum::Edge), "EDGE");
}

#[test]
fn test_shape_enum_vertex() {
    assert_eq!(TopAbs::shape_name(ShapeEnum::Vertex), "VERTEX");
}

#[test]
fn test_shape_enum_shape() {
    assert_eq!(TopAbs::shape_name(ShapeEnum::Shape), "SHAPE");
}

#[test]
fn test_shape_enum_clone_and_copy() {
    let s = ShapeEnum::Face;
    let s2 = s;
    assert_eq!(s, s2);
    let s3 = s.clone();
    assert_eq!(s, s3);
}

// --- Orientation ---

#[test]
fn test_orientation_name_forward() {
    assert_eq!(TopAbs::orientation_name(Orientation::Forward), "FORWARD");
}

#[test]
fn test_orientation_name_reversed() {
    assert_eq!(TopAbs::orientation_name(Orientation::Reversed), "REVERSED");
}

#[test]
fn test_orientation_name_internal() {
    assert_eq!(TopAbs::orientation_name(Orientation::Internal), "INTERNAL");
}

#[test]
fn test_orientation_name_external() {
    assert_eq!(TopAbs::orientation_name(Orientation::External), "EXTERNAL");
}

// --- State ---

#[test]
fn test_state_in() {
    let s = State::IN;
    assert_eq!(s, State::IN);
}

#[test]
fn test_state_out() {
    let s = State::OUT;
    assert_eq!(s, State::OUT);
}

#[test]
fn test_state_on() {
    let s = State::ON;
    assert_eq!(s, State::ON);
}

#[test]
fn test_state_unknown() {
    let s = State::UNKNOWN;
    assert_eq!(s, State::UNKNOWN);
}

#[test]
fn test_state_all_distinct() {
    assert_ne!(State::IN, State::OUT);
    assert_ne!(State::IN, State::ON);
    assert_ne!(State::IN, State::UNKNOWN);
    assert_ne!(State::OUT, State::ON);
    assert_ne!(State::OUT, State::UNKNOWN);
    assert_ne!(State::ON, State::UNKNOWN);
}

// --- compose ---

#[test]
fn test_compose_ff_is_f() {
    assert_eq!(
        TopAbs::compose(Orientation::Forward, Orientation::Forward),
        Orientation::Forward
    );
}

#[test]
fn test_compose_fr_is_r() {
    assert_eq!(
        TopAbs::compose(Orientation::Forward, Orientation::Reversed),
        Orientation::Reversed
    );
}

#[test]
fn test_compose_rf_is_r() {
    assert_eq!(
        TopAbs::compose(Orientation::Reversed, Orientation::Forward),
        Orientation::Reversed
    );
}

#[test]
fn test_compose_rr_is_f() {
    assert_eq!(
        TopAbs::compose(Orientation::Reversed, Orientation::Reversed),
        Orientation::Forward
    );
}

#[test]
fn test_compose_internal_first_always_internal() {
    for o in [
        Orientation::Forward,
        Orientation::Reversed,
        Orientation::Internal,
        Orientation::External,
    ] {
        assert_eq!(TopAbs::compose(Orientation::Internal, o), Orientation::Internal);
    }
}

#[test]
fn test_compose_external_first_always_external() {
    for o in [
        Orientation::Forward,
        Orientation::Reversed,
        Orientation::Internal,
        Orientation::External,
    ] {
        assert_eq!(TopAbs::compose(Orientation::External, o), Orientation::External);
    }
}

#[test]
fn test_compose_second_internal_overrides_forward() {
    assert_eq!(
        TopAbs::compose(Orientation::Forward, Orientation::Internal),
        Orientation::Internal
    );
}

#[test]
fn test_compose_second_internal_overrides_reversed() {
    assert_eq!(
        TopAbs::compose(Orientation::Reversed, Orientation::Internal),
        Orientation::Internal
    );
}

#[test]
fn test_compose_second_external_overrides_forward() {
    assert_eq!(
        TopAbs::compose(Orientation::Forward, Orientation::External),
        Orientation::External
    );
}

#[test]
fn test_compose_second_external_overrides_reversed() {
    assert_eq!(
        TopAbs::compose(Orientation::Reversed, Orientation::External),
        Orientation::External
    );
}

// --- reverse ---

#[test]
fn test_reverse_forward() {
    assert_eq!(TopAbs::reverse(Orientation::Forward), Orientation::Reversed);
}

#[test]
fn test_reverse_reversed() {
    assert_eq!(TopAbs::reverse(Orientation::Reversed), Orientation::Forward);
}

#[test]
fn test_reverse_internal_unchanged() {
    assert_eq!(TopAbs::reverse(Orientation::Internal), Orientation::Internal);
}

#[test]
fn test_reverse_external_unchanged() {
    assert_eq!(TopAbs::reverse(Orientation::External), Orientation::External);
}

#[test]
fn test_reverse_double_is_identity() {
    for o in [
        Orientation::Forward,
        Orientation::Reversed,
        Orientation::Internal,
        Orientation::External,
    ] {
        assert_eq!(TopAbs::reverse(TopAbs::reverse(o)), o);
    }
}

// --- complement ---

#[test]
fn test_complement_forward_is_internal() {
    assert_eq!(
        TopAbs::complement(Orientation::Forward),
        Orientation::Internal
    );
}

#[test]
fn test_complement_internal_is_forward() {
    assert_eq!(
        TopAbs::complement(Orientation::Internal),
        Orientation::Forward
    );
}

#[test]
fn test_complement_reversed_is_external() {
    assert_eq!(
        TopAbs::complement(Orientation::Reversed),
        Orientation::External
    );
}

#[test]
fn test_complement_external_is_reversed() {
    assert_eq!(
        TopAbs::complement(Orientation::External),
        Orientation::Reversed
    );
}

#[test]
fn test_complement_double_is_identity() {
    for o in [
        Orientation::Forward,
        Orientation::Reversed,
        Orientation::Internal,
        Orientation::External,
    ] {
        assert_eq!(TopAbs::complement(TopAbs::complement(o)), o);
    }
}

// --- is_closed ---

#[test]
fn test_is_closed_wire_true() {
    assert!(TopAbs::is_closed(ShapeEnum::Wire));
}

#[test]
fn test_is_closed_edge_true() {
    assert!(TopAbs::is_closed(ShapeEnum::Edge));
}

#[test]
fn test_is_closed_shell_true() {
    assert!(TopAbs::is_closed(ShapeEnum::Shell));
}

#[test]
fn test_is_closed_solid_true() {
    assert!(TopAbs::is_closed(ShapeEnum::Solid));
}

#[test]
fn test_is_closed_face_false() {
    assert!(!TopAbs::is_closed(ShapeEnum::Face));
}

#[test]
fn test_is_closed_vertex_false() {
    assert!(!TopAbs::is_closed(ShapeEnum::Vertex));
}

#[test]
fn test_is_closed_compound_false() {
    assert!(!TopAbs::is_closed(ShapeEnum::Compound));
}

#[test]
fn test_is_closed_compsolid_false() {
    assert!(!TopAbs::is_closed(ShapeEnum::CompSolid));
}

#[test]
fn test_is_closed_shape_false() {
    assert!(!TopAbs::is_closed(ShapeEnum::Shape));
}

// --- interaction between compose/reverse/complement ---

#[test]
fn test_reverse_then_compose_vs_complement() {
    // reverse(Reversed) = Forward; composing Forward with itself = Forward
    let o = Orientation::Reversed;
    let rev = TopAbs::reverse(o);
    let result = TopAbs::compose(rev, rev);
    assert_eq!(result, Orientation::Forward);
}

#[test]
fn test_complement_then_reverse_cycle() {
    // complement(reverse(Forward)) = complement(Reversed) = External
    let result = TopAbs::complement(TopAbs::reverse(Orientation::Forward));
    assert_eq!(result, Orientation::External);
}

#[test]
fn test_compose_with_reversed_twice_returns_original() {
    // For Forward/Reversed: composing with Reversed twice is identity
    let o = Orientation::Forward;
    let once = TopAbs::compose(o, Orientation::Reversed);
    let twice = TopAbs::compose(once, Orientation::Reversed);
    assert_eq!(twice, o);
}

#[test]
fn test_debug_format_shape_enum() {
    let s = format!("{:?}", ShapeEnum::Solid);
    assert_eq!(s, "Solid");
}

#[test]
fn test_debug_format_orientation() {
    let s = format!("{:?}", Orientation::Internal);
    assert_eq!(s, "Internal");
}

#[test]
fn test_debug_format_state() {
    let s = format!("{:?}", State::UNKNOWN);
    assert_eq!(s, "UNKNOWN");
}
