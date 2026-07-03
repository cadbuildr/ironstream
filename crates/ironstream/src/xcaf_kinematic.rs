// FILE: xcaf_kinematic.rs

// occt: XCAFKinematics_PairObject
/// The kind of kinematic joint between two bodies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairType {
    /// All six DOF are locked; no relative motion is permitted.
    Rigid,
    /// One translational DOF along a shared axis.
    Prismatic,
    /// One rotational DOF about a shared axis.
    Revolute,
    /// Combined translational and rotational DOF about/along a shared axis.
    Cylindrical,
    /// Three rotational DOF about a shared point (ball-and-socket).
    Spherical,
    /// Two translational and one rotational DOF in a shared plane.
    Planar,
}

// occt: XCAFKinematics_PairObject
/// A single kinematic pair (joint) connecting two named bodies.
///
/// `base` is the driving body and `follower` is the driven body.
/// `limits` defines the inclusive `[lo, hi]` range of the joint value
/// (linear displacement in mm for prismatic joints, angle in radians for
/// revolute/cylindrical joints, etc.).  The default range is `(0.0, 0.0)`.
#[derive(Clone, Debug, PartialEq)]
pub struct KinematicPair {
    pub pair_type: PairType,
    pub base: String,
    pub follower: String,
    pub limits: (f64, f64),
}

impl KinematicPair {
    /// Create a new pair with the given type and body names.
    /// Limits are initialised to `(0.0, 0.0)`.
    pub fn new(t: PairType, base: &str, follower: &str) -> Self {
        Self {
            pair_type: t,
            base: base.to_owned(),
            follower: follower.to_owned(),
            limits: (0.0, 0.0),
        }
    }

    /// Set the joint value range to `[lo, hi]`.
    pub fn set_limits(&mut self, lo: f64, hi: f64) {
        self.limits = (lo, hi);
    }

    /// Return `true` when `val` lies within the closed interval `[limits.0, limits.1]`.
    pub fn is_within_limits(&self, val: f64) -> bool {
        val >= self.limits.0 && val <= self.limits.1
    }
}

// occt: XCAFKinematics_Tool
/// A container for all kinematic pairs defined in an assembly document.
///
/// Mirrors the role of `XCAFKinematics_Tool` from Open CASCADE Technology:
/// it owns a flat list of [`KinematicPair`] objects and exposes query
/// helpers for navigating the kinematic graph.
#[derive(Clone, Debug, Default)]
pub struct KinematicTool {
    pub pairs: Vec<KinematicPair>,
}

impl KinematicTool {
    /// Create an empty tool with no pairs.
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }

    /// Append a kinematic pair to the tool.
    pub fn add_pair(&mut self, p: KinematicPair) {
        self.pairs.push(p);
    }

    /// Return all pairs whose `base` field matches `base`.
    pub fn find_pairs(&self, base: &str) -> Vec<&KinematicPair> {
        self.pairs.iter().filter(|p| p.base == base).collect()
    }

    /// Return the total number of pairs currently registered.
    pub fn pair_count(&self) -> usize {
        self.pairs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pair_defaults() {
        let p = KinematicPair::new(PairType::Revolute, "link0", "link1");
        assert_eq!(p.pair_type, PairType::Revolute);
        assert_eq!(p.base, "link0");
        assert_eq!(p.follower, "link1");
        assert_eq!(p.limits, (0.0, 0.0));
    }

    #[test]
    fn test_set_and_check_limits() {
        let mut p = KinematicPair::new(PairType::Prismatic, "base", "slider");
        p.set_limits(-0.05, 0.15);
        assert!(p.is_within_limits(0.0));
        assert!(p.is_within_limits(-0.05));
        assert!(p.is_within_limits(0.15));
        assert!(!p.is_within_limits(-0.06));
        assert!(!p.is_within_limits(0.16));
    }

    #[test]
    fn test_tool_add_and_count() {
        let mut tool = KinematicTool::new();
        assert_eq!(tool.pair_count(), 0);
        tool.add_pair(KinematicPair::new(PairType::Revolute, "A", "B"));
        tool.add_pair(KinematicPair::new(PairType::Rigid, "B", "C"));
        assert_eq!(tool.pair_count(), 2);
    }

    #[test]
    fn test_find_pairs() {
        let mut tool = KinematicTool::new();
        tool.add_pair(KinematicPair::new(PairType::Revolute, "shoulder", "elbow"));
        tool.add_pair(KinematicPair::new(PairType::Revolute, "shoulder", "clavicle"));
        tool.add_pair(KinematicPair::new(PairType::Prismatic, "elbow", "wrist"));

        let shoulder_pairs = tool.find_pairs("shoulder");
        assert_eq!(shoulder_pairs.len(), 2);
        assert!(shoulder_pairs.iter().all(|p| p.base == "shoulder"));

        let elbow_pairs = tool.find_pairs("elbow");
        assert_eq!(elbow_pairs.len(), 1);
        assert_eq!(elbow_pairs[0].follower, "wrist");

        assert!(tool.find_pairs("nonexistent").is_empty());
    }

    #[test]
    fn test_all_pair_types_round_trip() {
        let types = [
            PairType::Rigid,
            PairType::Prismatic,
            PairType::Revolute,
            PairType::Cylindrical,
            PairType::Spherical,
            PairType::Planar,
        ];
        for t in types {
            let p = KinematicPair::new(t, "a", "b");
            assert_eq!(p.pair_type, t);
        }
    }
}
