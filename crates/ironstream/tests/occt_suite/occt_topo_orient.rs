extern crate ironstream;
use ironstream::topo_orient::*;

#[test]
fn test_topo_orient_basic() {
    // Orientation reverse.
    assert_eq!(Orientation::Forward.reverse(), Orientation::Reversed);
    assert_eq!(Orientation::Reversed.reverse(), Orientation::Forward);
    assert_eq!(Orientation::Internal.reverse(), Orientation::Internal);
    assert_eq!(Orientation::External.reverse(), Orientation::External);

    // is_forward / is_reversed.
    assert!(Orientation::Forward.is_forward());
    assert!(!Orientation::Reversed.is_forward());
    assert!(Orientation::Reversed.is_reversed());

    // State variants.
    let s = State::In;
    assert_eq!(s, State::In);
    assert_ne!(s, State::Out);

    // OrientedShape construction.
    let shape = OrientedShape::new("face_1");
    assert_eq!(shape.name, "face_1");
    assert_eq!(shape.orientation, Orientation::Forward);

    let shape_rev = OrientedShape::with_orientation("face_1", Orientation::Reversed);
    assert_eq!(shape_rev.orientation, Orientation::Reversed);

    // reversed() swaps Forward <-> Reversed.
    let back = shape.reversed();
    assert_eq!(back.orientation, Orientation::Reversed);
    let back2 = back.reversed();
    assert_eq!(back2.orientation, Orientation::Forward);

    // complement().
    let comp = shape.complement();
    assert_eq!(comp.orientation, Orientation::Internal);

    // compose_orientations.
    assert_eq!(
        compose_orientations(&Orientation::Forward, &Orientation::Forward),
        Orientation::Forward
    );
    assert_eq!(
        compose_orientations(&Orientation::Forward, &Orientation::Reversed),
        Orientation::Reversed
    );
    assert_eq!(
        compose_orientations(&Orientation::Reversed, &Orientation::Reversed),
        Orientation::Forward
    );
    assert_eq!(
        compose_orientations(&Orientation::Internal, &Orientation::Reversed),
        Orientation::Internal
    );
}
