// FILE: b_rep_test.rs
// occt: BRepTest

/// Test command registry for BRep functionality.
pub struct BrepTest;

impl BrepTest {
    /// Registers all topology test commands.
    pub fn all_commands() -> Vec<&'static str> {
        vec![
            "basic",
            "curve",
            "fillet2d",
            "surface",
            "primitive",
            "filling",
            "sweep",
            "topology",
            "fillet",
            "chamfer",
            "gprop",
            "mat",
            "draftangle",
            "feature",
            "other",
            "extrema",
            "check",
            "placement",
            "projection",
            "history",
        ]
    }

    /// Registers basic commands.
    pub fn basic_commands() -> Vec<&'static str> {
        vec!["vertex", "edge", "wire", "face", "shell", "solid"]
    }

    /// Registers curve commands (edges and wires).
    pub fn curve_commands() -> Vec<&'static str> {
        vec!["curve", "wire", "mkedge", "mkwire"]
    }

    /// Registers fillet2d commands.
    pub fn fillet2d_commands() -> Vec<&'static str> {
        vec!["fillet2d", "chamfer2d"]
    }

    /// Registers surface commands (faces and shells).
    pub fn surface_commands() -> Vec<&'static str> {
        vec!["face", "shell", "mkface", "mkshell"]
    }

    /// Registers primitive commands.
    pub fn primitive_commands() -> Vec<&'static str> {
        vec!["box", "sphere", "cylinder", "cone", "torus", "prism", "revol"]
    }

    /// Registers filling commands.
    pub fn filling_commands() -> Vec<&'static str> {
        vec!["filling"]
    }

    /// Registers sweep commands.
    pub fn sweep_commands() -> Vec<&'static str> {
        vec!["sweep", "pipe"]
    }

    /// Registers topology commands.
    pub fn topology_commands() -> Vec<&'static str> {
        vec!["explode", "compound"]
    }

    /// Registers fillet commands.
    pub fn fillet_commands() -> Vec<&'static str> {
        vec!["fillet"]
    }

    /// Registers chamfer commands.
    pub fn chamfer_commands() -> Vec<&'static str> {
        vec!["chamfer"]
    }

    /// Registers geometric property commands.
    pub fn gprop_commands() -> Vec<&'static str> {
        vec!["mass", "area", "inertia"]
    }

    /// Registers medial axis transform (mat) commands.
    pub fn mat_commands() -> Vec<&'static str> {
        vec!["mat"]
    }

    /// Registers draft angle commands.
    pub fn draft_angle_commands() -> Vec<&'static str> {
        vec!["draft"]
    }

    /// Registers feature commands.
    pub fn feature_commands() -> Vec<&'static str> {
        vec!["pocket", "pad", "hole"]
    }

    /// Registers other (auxiliary) commands.
    pub fn other_commands() -> Vec<&'static str> {
        vec!["copy", "transform", "scale"]
    }

    /// Registers extrema commands.
    pub fn extrema_commands() -> Vec<&'static str> {
        vec!["extrema", "distance"]
    }

    /// Registers check commands.
    pub fn check_commands() -> Vec<&'static str> {
        vec!["checkshape", "validate"]
    }

    /// Registers placement commands.
    pub fn placement_commands() -> Vec<&'static str> {
        vec!["placement", "position"]
    }

    /// Registers projection commands.
    pub fn projection_commands() -> Vec<&'static str> {
        vec!["project"]
    }

    /// Registers history commands.
    pub fn history_commands() -> Vec<&'static str> {
        vec!["history", "generated"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_commands() {
        let commands = BrepTest::all_commands();
        assert!(commands.len() > 0);
        assert!(commands.contains(&"basic"));
        assert!(commands.contains(&"curve"));
    }

    #[test]
    fn test_basic_commands() {
        let commands = BrepTest::basic_commands();
        assert!(commands.contains(&"vertex"));
        assert!(commands.contains(&"edge"));
    }

    #[test]
    fn test_curve_commands() {
        let commands = BrepTest::curve_commands();
        assert!(commands.contains(&"mkedge"));
        assert!(commands.contains(&"mkwire"));
    }

    #[test]
    fn test_primitive_commands() {
        let commands = BrepTest::primitive_commands();
        assert!(commands.contains(&"box"));
        assert!(commands.contains(&"sphere"));
    }
}
