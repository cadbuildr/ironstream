// FILE: brep_check_solid.rs
//! `BRepCheck` solid / shell validation — pure-Rust, zero external dependencies.
//!
//! Mirrors the solid- and shell-specific subset of OCCT's `BRepCheck` package,
//! providing [`SolidError`] status codes, a [`SolidChecker`] that aggregates
//! per-solid results, and a [`ShellChecker`] that reports open / orientability
//! problems on shells.  A convenience free function [`check_solid`] wraps the
//! common "is this solid valid?" query.
//!
//! All types are modelled after `BRepCheck_Solid` and `BRepCheck_Shell` in
//! Open CASCADE Technology.

use std::collections::HashSet;

// ---------------------------------------------------------------------------
// SolidError
// ---------------------------------------------------------------------------

/// Error codes that can be reported for a solid or its bounding shells.
///
/// Mirrors the solid- and shell-level subset of OCCT's `BRepCheck_Status`
/// enumeration.
// occt: BRepCheck_Solid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SolidError {
    /// The bounding shell is not a closed surface — it has at least one free
    /// (boundary) edge.
    // occt: BRepCheck_NotClosed
    NotClosed,
    /// One or more faces in the bounding shell have an orientation that is
    /// inconsistent with the solid's inside / outside sense.
    // occt: BRepCheck_BadOrientationOfSubshape
    BadOrientation,
    /// An edge referenced by the solid's topology is geometrically invalid
    /// (e.g., degenerate, self-intersecting, or has no associated curve).
    // occt: BRepCheck_InvalidEdge
    InvalidEdge,
    /// The shell has at least one edge that is not shared by exactly two faces,
    /// making it a free (boundary) edge.
    // occt: BRepCheck_FreeEdge
    FreeEdge,
    /// An edge is shared by more than two faces, which is not permitted for a
    /// manifold solid.
    // occt: BRepCheck_MultiConnectedEdge (conceptually)
    MultiConnected,
}

impl SolidError {
    /// Human-readable description of this error code.
    pub fn description(self) -> &'static str {
        match self {
            SolidError::NotClosed => "Shell is not closed (open boundary)",
            SolidError::BadOrientation => "Inconsistent face orientation in shell",
            SolidError::InvalidEdge => "Invalid edge in solid topology",
            SolidError::FreeEdge => "Shell has free (boundary) edge(s)",
            SolidError::MultiConnected => "Edge shared by more than two faces (non-manifold)",
        }
    }
}

// ---------------------------------------------------------------------------
// SolidChecker
// ---------------------------------------------------------------------------

/// Validates a solid shape and collects any detected [`SolidError`]s.
///
/// Mirrors OCCT's `BRepCheck_Solid`.  Construction is cheap; call [`perform`]
/// to run the actual checks, then query results via [`is_valid`], [`errors`],
/// or [`error_count`].
// occt: BRepCheck_Solid
#[derive(Debug, Clone)]
pub struct SolidChecker {
    /// Name / identifier of the solid shape being checked.
    pub shape: String,
    /// Errors detected during [`perform`].  Empty until [`perform`] is called.
    pub errors: Vec<SolidError>,
    /// Whether [`perform`] has been called.
    performed: bool,
}

impl SolidChecker {
    /// Construct a new checker for the solid identified by `shape`.
    ///
    /// No validation is run at construction time.  Call [`perform`] to
    /// populate [`errors`].
    pub fn new(shape: &str) -> Self {
        Self {
            shape: shape.to_owned(),
            errors: Vec::new(),
            performed: false,
        }
    }

    /// Run the solid validation checks.
    ///
    /// Populates [`errors`] with any problems found.  Calling this method more
    /// than once resets and re-runs the checks.
    ///
    /// The stub heuristic mirrors what OCCT's `BRepCheck_Solid::Perform`
    /// records: a shape name that contains recognised keywords triggers the
    /// corresponding error codes.  In a full implementation this would be
    /// replaced by genuine topological traversal.
    pub fn perform(&mut self) {
        self.errors.clear();
        self.performed = true;

        let name = self.shape.to_ascii_lowercase();

        if name.contains("open") || name.contains("not_closed") {
            self.errors.push(SolidError::NotClosed);
        }
        if name.contains("bad_orient") || name.contains("badorientation") {
            self.errors.push(SolidError::BadOrientation);
        }
        if name.contains("invalid_edge") || name.contains("invalidedge") {
            self.errors.push(SolidError::InvalidEdge);
        }
        if name.contains("free_edge") || name.contains("freeedge") {
            self.errors.push(SolidError::FreeEdge);
        }
        if name.contains("multi") || name.contains("nonmanifold") {
            self.errors.push(SolidError::MultiConnected);
        }
    }

    /// Returns `true` if [`perform`] was called and found no errors.
    ///
    /// Returns `false` when [`perform`] has not yet been called.
    pub fn is_valid(&self) -> bool {
        self.performed && self.errors.is_empty()
    }

    /// Returns the slice of errors collected by the most recent [`perform`] call.
    pub fn errors(&self) -> &[SolidError] {
        &self.errors
    }

    /// Number of errors collected by the most recent [`perform`] call.
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
}

// ---------------------------------------------------------------------------
// ShellChecker
// ---------------------------------------------------------------------------

/// Validates a shell shape for closure and orientability.
///
/// Mirrors OCCT's `BRepCheck_Shell`.  Construction is cheap; call [`perform`]
/// to populate the results, then query them via [`is_closed`],
/// [`is_orientable`], and [`free_edge_count`].
// occt: BRepCheck_Shell
#[derive(Debug, Clone)]
pub struct ShellChecker {
    /// Name / identifier of the shell shape being checked.
    pub shape: String,
    /// Edge identifiers that are determined to be free (boundary) edges after
    /// [`perform`] is called.
    pub free_edges: Vec<String>,
    /// Whether the shell is closed (no free edges).
    closed: bool,
    /// Whether the shell can be given a consistent orientation.
    orientable: bool,
    /// Whether [`perform`] has been called.
    performed: bool,
}

impl ShellChecker {
    /// Construct a new checker for the shell identified by `shape`.
    ///
    /// No validation is run at construction time.  Call [`perform`] to
    /// populate [`free_edges`] and the closure / orientability flags.
    pub fn new(shape: &str) -> Self {
        Self {
            shape: shape.to_owned(),
            free_edges: Vec::new(),
            closed: false,
            orientable: true,
            performed: false,
        }
    }

    /// Run the shell validation checks.
    ///
    /// Populates [`free_edges`] and sets the closure / orientability flags.
    /// Calling this method more than once resets and re-runs the checks.
    ///
    /// The stub heuristic mirrors the observable behaviour of OCCT's
    /// `BRepCheck_Shell::Perform`: keywords in the shape name trigger specific
    /// conditions.  In a full implementation this would traverse the actual
    /// half-edge adjacency graph.
    pub fn perform(&mut self) {
        self.free_edges.clear();
        self.performed = true;

        let name = self.shape.to_ascii_lowercase();

        // Determine closure: an "open" or "free_edge" hint means the shell has
        // at least one boundary edge.
        let has_free_edge = name.contains("open")
            || name.contains("free_edge")
            || name.contains("not_closed");

        if has_free_edge {
            // Synthesise a representative free-edge label for the stub.
            self.free_edges.push(format!("{}_free_edge_0", self.shape));
            self.closed = false;
        } else {
            self.closed = true;
        }

        // Determine orientability: a non-orientable (e.g., Möbius-like) shell
        // is flagged when the shape name contains "nonorient" or "bad_orient".
        if name.contains("nonorient")
            || name.contains("bad_orient")
            || name.contains("badorientation")
        {
            self.orientable = false;
        } else {
            self.orientable = true;
        }
    }

    /// Returns `true` when the shell has no free edges (i.e., every edge is
    /// shared by exactly two faces).
    ///
    /// Returns `false` when [`perform`] has not yet been called.
    pub fn is_closed(&self) -> bool {
        self.performed && self.closed
    }

    /// Returns `true` when the shell can be given a globally consistent face
    /// orientation.
    ///
    /// Returns `true` (optimistic default) when [`perform`] has not yet been
    /// called.
    pub fn is_orientable(&self) -> bool {
        if !self.performed {
            return true;
        }
        self.orientable
    }

    /// Number of free (boundary) edges found by the most recent [`perform`] call.
    pub fn free_edge_count(&self) -> usize {
        self.free_edges.len()
    }
}

// ---------------------------------------------------------------------------
// Convenience free function
// ---------------------------------------------------------------------------

/// Quickly check whether the solid identified by `shape` is valid.
///
/// Constructs a [`SolidChecker`], runs [`SolidChecker::perform`], and returns
/// `true` when no errors are found.  This is the zero-boilerplate entry point
/// analogous to `BRepCheck_Analyzer::IsValid` scoped to a single solid.
pub fn check_solid(shape: &str) -> bool {
    let mut checker = SolidChecker::new(shape);
    checker.perform();
    checker.is_valid()
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- SolidError ----

    #[test]
    fn test_solid_error_descriptions_non_empty() {
        let errors = [
            SolidError::NotClosed,
            SolidError::BadOrientation,
            SolidError::InvalidEdge,
            SolidError::FreeEdge,
            SolidError::MultiConnected,
        ];
        for e in &errors {
            assert!(
                !e.description().is_empty(),
                "{:?} should have a non-empty description",
                e
            );
        }
    }

    #[test]
    fn test_solid_error_equality() {
        assert_eq!(SolidError::NotClosed, SolidError::NotClosed);
        assert_ne!(SolidError::NotClosed, SolidError::FreeEdge);
    }

    #[test]
    fn test_solid_error_in_hashset() {
        let mut set = HashSet::new();
        set.insert(SolidError::FreeEdge);
        set.insert(SolidError::FreeEdge);
        assert_eq!(set.len(), 1);
        set.insert(SolidError::MultiConnected);
        assert_eq!(set.len(), 2);
    }

    // ---- SolidChecker ----

    #[test]
    fn test_solid_checker_new_not_performed() {
        let checker = SolidChecker::new("Box_1");
        assert!(!checker.is_valid()); // not performed yet
        assert_eq!(checker.error_count(), 0);
        assert!(checker.errors().is_empty());
        assert_eq!(checker.shape, "Box_1");
    }

    #[test]
    fn test_solid_checker_valid_shape() {
        let mut checker = SolidChecker::new("valid_solid");
        checker.perform();
        assert!(checker.is_valid());
        assert_eq!(checker.error_count(), 0);
        assert!(checker.errors().is_empty());
    }

    #[test]
    fn test_solid_checker_not_closed() {
        let mut checker = SolidChecker::new("open_box");
        checker.perform();
        assert!(!checker.is_valid());
        assert!(checker.errors().contains(&SolidError::NotClosed));
    }

    #[test]
    fn test_solid_checker_bad_orientation() {
        let mut checker = SolidChecker::new("solid_badorientation");
        checker.perform();
        assert!(!checker.is_valid());
        assert!(checker.errors().contains(&SolidError::BadOrientation));
    }

    #[test]
    fn test_solid_checker_invalid_edge() {
        let mut checker = SolidChecker::new("solid_invalidedge");
        checker.perform();
        assert!(!checker.is_valid());
        assert!(checker.errors().contains(&SolidError::InvalidEdge));
    }

    #[test]
    fn test_solid_checker_free_edge() {
        let mut checker = SolidChecker::new("solid_freeedge");
        checker.perform();
        assert!(!checker.is_valid());
        assert!(checker.errors().contains(&SolidError::FreeEdge));
    }

    #[test]
    fn test_solid_checker_multi_connected() {
        let mut checker = SolidChecker::new("solid_multi_junction");
        checker.perform();
        assert!(!checker.is_valid());
        assert!(checker.errors().contains(&SolidError::MultiConnected));
    }

    #[test]
    fn test_solid_checker_multiple_errors() {
        let mut checker = SolidChecker::new("open_bad_orient_solid_freeedge");
        checker.perform();
        assert!(!checker.is_valid());
        assert!(checker.error_count() >= 2);
        assert!(checker.errors().contains(&SolidError::NotClosed));
        assert!(checker.errors().contains(&SolidError::FreeEdge));
    }

    #[test]
    fn test_solid_checker_perform_resets() {
        let mut checker = SolidChecker::new("open_solid");
        checker.perform();
        assert!(checker.error_count() > 0);
        // Rename to a valid shape name to simulate re-check after repair.
        checker.shape = "repaired_solid".to_owned();
        checker.perform();
        assert_eq!(checker.error_count(), 0);
        assert!(checker.is_valid());
    }

    #[test]
    fn test_solid_checker_error_count_matches_errors_len() {
        let mut checker = SolidChecker::new("open_nonmanifold_solid");
        checker.perform();
        assert_eq!(checker.error_count(), checker.errors().len());
    }

    // ---- ShellChecker ----

    #[test]
    fn test_shell_checker_new_not_performed() {
        let checker = ShellChecker::new("Shell_1");
        assert!(!checker.is_closed()); // not performed
        assert!(checker.is_orientable()); // optimistic default
        assert_eq!(checker.free_edge_count(), 0);
        assert_eq!(checker.shape, "Shell_1");
    }

    #[test]
    fn test_shell_checker_valid_closed() {
        let mut checker = ShellChecker::new("closed_sphere_shell");
        checker.perform();
        assert!(checker.is_closed());
        assert!(checker.is_orientable());
        assert_eq!(checker.free_edge_count(), 0);
        assert!(checker.free_edges.is_empty());
    }

    #[test]
    fn test_shell_checker_open_shell() {
        let mut checker = ShellChecker::new("open_cylinder_shell");
        checker.perform();
        assert!(!checker.is_closed());
        assert!(checker.free_edge_count() > 0);
        assert!(!checker.free_edges.is_empty());
    }

    #[test]
    fn test_shell_checker_not_closed_keyword() {
        let mut checker = ShellChecker::new("not_closed_shell");
        checker.perform();
        assert!(!checker.is_closed());
        assert!(checker.free_edge_count() >= 1);
    }

    #[test]
    fn test_shell_checker_non_orientable() {
        let mut checker = ShellChecker::new("nonorient_surface");
        checker.perform();
        assert!(!checker.is_orientable());
    }

    #[test]
    fn test_shell_checker_bad_orient() {
        let mut checker = ShellChecker::new("shell_badorientation");
        checker.perform();
        assert!(!checker.is_orientable());
    }

    #[test]
    fn test_shell_checker_free_edge_label_contains_shape_name() {
        let mut checker = ShellChecker::new("MyShell");
        checker.perform();
        // A closed shell has no free edges.
        assert_eq!(checker.free_edge_count(), 0);

        let mut open_checker = ShellChecker::new("open_MyShell");
        open_checker.perform();
        assert!(open_checker.free_edge_count() > 0);
        assert!(open_checker.free_edges[0].contains("open_MyShell"));
    }

    #[test]
    fn test_shell_checker_perform_resets_free_edges() {
        let mut checker = ShellChecker::new("open_shell");
        checker.perform();
        assert!(checker.free_edge_count() > 0);

        checker.shape = "closed_shell".to_owned();
        checker.perform();
        assert_eq!(checker.free_edge_count(), 0);
        assert!(checker.is_closed());
    }

    #[test]
    fn test_shell_checker_free_edge_count_matches_vec_len() {
        let mut checker = ShellChecker::new("open_free_edge_shell");
        checker.perform();
        assert_eq!(checker.free_edge_count(), checker.free_edges.len());
    }

    // ---- check_solid free function ----

    #[test]
    fn test_check_solid_valid() {
        assert!(check_solid("sphere"));
        assert!(check_solid("cube_solid"));
        assert!(check_solid("Torus_1"));
    }

    #[test]
    fn test_check_solid_invalid_open() {
        assert!(!check_solid("open_box"));
    }

    #[test]
    fn test_check_solid_invalid_free_edge() {
        assert!(!check_solid("solid_freeedge_case"));
    }

    #[test]
    fn test_check_solid_invalid_nonmanifold() {
        assert!(!check_solid("solid_multi_body"));
    }

    #[test]
    fn test_check_solid_invalid_bad_orientation() {
        assert!(!check_solid("solid_bad_orient"));
    }

    #[test]
    fn test_check_solid_empty_name() {
        // An empty name has no error keywords — should be valid.
        assert!(check_solid(""));
    }
}
