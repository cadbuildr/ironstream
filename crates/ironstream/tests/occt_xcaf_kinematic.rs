extern crate ironstream;
use ironstream::xcaf_kinematic::*;

#[test]
fn test_xcaf_kinematic_basic() {
    // KinematicPair
    let mut pair = KinematicPair::new(PairType::Revolute, "Base", "Follower");
    assert_eq!(pair.pair_type, PairType::Revolute);
    assert_eq!(pair.base, "Base");
    assert_eq!(pair.follower, "Follower");

    pair.set_limits(-1.57, 1.57);
    assert_eq!(pair.limits, (-1.57, 1.57));
    assert!(pair.is_within_limits(0.0));
    assert!(pair.is_within_limits(-1.57));
    assert!(!pair.is_within_limits(3.14));

    // KinematicTool
    let mut tool = KinematicTool::new();
    assert_eq!(tool.pair_count(), 0);

    let p1 = KinematicPair::new(PairType::Prismatic, "A", "B");
    let p2 = KinematicPair::new(PairType::Rigid, "A", "C");
    tool.add_pair(p1);
    tool.add_pair(p2);
    assert_eq!(tool.pair_count(), 2);

    let found = tool.find_pairs("A");
    assert_eq!(found.len(), 2);

    let none = tool.find_pairs("Z");
    assert_eq!(none.len(), 0);

    // Default
    let _default_tool = KinematicTool::default();
}
