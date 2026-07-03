// FILE: brep_class.rs
//! `BRepClass` — point-in-solid and point-in-face classifiers, mirroring
//! OpenCascade's `BRepClass3d` and `BRepClass` packages.
//!
//! This module provides two classifier types:
//!
//! * [`SolidClassifier`] — classifies a 3-D point against the interior of a
//!   named solid.  Models `BRepClass3d_SolidClassifier`.
//! * [`FaceClassifier`] — classifies a 2-D parameter-space point against the
//!   interior of a named face.  Models `BRepClass_FaceClassifier`.
//!
//! Both types are pure-Rust, zero-dependency stubs that carry the OCCT-shaped
//! API and return sensible placeholder results.  They are intended as scaffolding
//! for a full geometric implementation.

// ─────────────────────────── TopState ────────────────────────────────────────

/// Classification result for a point relative to a topological entity.
///
/// Mirrors the `TopAbs_State` enumeration used throughout OCCT's BRepClass
/// family of classifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopState {
    /// The point is strictly inside the entity.
    In,
    /// The point is strictly outside the entity.
    Out,
    /// The point lies on the boundary of the entity (within tolerance).
    On,
    /// The classification could not be determined.
    Unknown,
}

// ─────────────────────────── SolidClassifier ─────────────────────────────────

// occt: BRepClass3d_SolidClassifier
/// Classifies 3-D points against the interior of a named solid.
///
/// Mirrors `BRepClass3d_SolidClassifier` from the OCCT `BRepClass3d` package.
///
/// # Usage
///
/// ```rust
/// use ironstream::brep_class::{SolidClassifier, TopState};
///
/// let mut clf = SolidClassifier::new("my_box");
/// let state = clf.perform([0.5, 0.5, 0.5], 1e-7);
/// assert_eq!(clf.state(), state);
/// assert!(!clf.is_on_face());
/// ```
#[derive(Debug, Clone)]
pub struct SolidClassifier {
    /// Name (identifier) of the solid being classified against.
    ///
    /// In a full implementation this would be a reference to the actual B-Rep
    /// solid geometry.  Here it is kept as a `String` stub.
    pub solid: String,

    /// Tolerance used for on-boundary detection (set by the last
    /// [`SolidClassifier::perform`] call).
    pub tolerance: f64,

    /// Cached result of the last [`SolidClassifier::perform`] call.
    state: TopState,

    /// Whether the last classified point was found to lie on a face boundary.
    on_face: bool,
}

impl SolidClassifier {
    /// Create a new classifier for the named solid.
    ///
    /// Mirrors `BRepClass3d_SolidClassifier::BRepClass3d_SolidClassifier(shape)`.
    pub fn new(solid: &str) -> Self {
        SolidClassifier {
            solid: solid.to_owned(),
            tolerance: 0.0,
            state: TopState::Unknown,
            on_face: false,
        }
    }

    /// Classify the 3-D point `pt` against the loaded solid using `tol` as the
    /// on-boundary threshold.
    ///
    /// Updates the internal state and returns the [`TopState`] result.
    ///
    /// Mirrors `BRepClass3d_SolidClassifier::Perform(point, tolerance)`.
    ///
    /// # Stub behaviour
    ///
    /// This implementation has no access to real geometry, so it always returns
    /// [`TopState::Unknown`].  Replace the body with actual ray-casting logic
    /// once the geometry representation is available.
    pub fn perform(&mut self, pt: [f64; 3], tol: f64) -> TopState {
        self.tolerance = tol;
        let _ = pt; // geometry not yet available — suppress unused-variable lint
        // TODO: implement ray-casting against `self.solid` geometry.
        self.state = TopState::Unknown;
        self.on_face = false;
        self.state
    }

    /// Return the classification result of the last [`SolidClassifier::perform`]
    /// call.
    ///
    /// Returns [`TopState::Unknown`] if `perform` has not yet been called.
    ///
    /// Mirrors `BRepClass3d_SolidClassifier::State()`.
    pub fn state(&self) -> TopState {
        self.state
    }

    /// Return `true` if the last classified point was found to lie on a face of
    /// the solid boundary (within tolerance).
    ///
    /// Mirrors `BRepClass3d_SolidClassifier::IsOnAFace()`.
    pub fn is_on_face(&self) -> bool {
        self.on_face
    }
}

// ─────────────────────────── FaceClassifier ──────────────────────────────────

// occt: BRepClass_FaceClassifier
/// Classifies 2-D parameter-space points against the interior of a named face.
///
/// Mirrors `BRepClass_FaceClassifier` from the OCCT `BRepClass` package.
///
/// # Usage
///
/// ```rust
/// use ironstream::brep_class::{FaceClassifier, TopState};
///
/// let mut clf = FaceClassifier::new();
/// let state = clf.perform("my_face", [0.3, 0.7], 1e-7);
/// assert_eq!(clf.state(), state);
/// ```
#[derive(Debug, Clone)]
pub struct FaceClassifier {
    /// Name (identifier) of the face most recently passed to
    /// [`FaceClassifier::perform`].
    ///
    /// In a full implementation this would be derived from an actual `TopoDS_Face`
    /// reference.  Kept as a `String` stub here.
    pub face: String,

    /// Cached result of the last [`FaceClassifier::perform`] call.
    state: TopState,
}

impl FaceClassifier {
    /// Create a new, unloaded face classifier.
    ///
    /// Mirrors `BRepClass_FaceClassifier::BRepClass_FaceClassifier()`.
    pub fn new() -> Self {
        FaceClassifier {
            face: String::new(),
            state: TopState::Unknown,
        }
    }

    /// Classify the 2-D parameter-space point `pt` against `face` using `tol`
    /// as the on-boundary threshold.
    ///
    /// Updates the internal state and returns the [`TopState`] result.
    ///
    /// Mirrors `BRepClass_FaceClassifier::Perform(face, point, tolerance)`.
    ///
    /// # Stub behaviour
    ///
    /// This implementation has no access to real face geometry, so it always
    /// returns [`TopState::Unknown`].  Replace the body with actual winding-number
    /// or ray-casting logic once the 2-D wire representation is available.
    pub fn perform(&mut self, face: &str, pt: [f64; 2], tol: f64) -> TopState {
        self.face = face.to_owned();
        let _ = (pt, tol); // geometry not yet available
        // TODO: implement 2-D winding-number / ray-casting against face wires.
        self.state = TopState::Unknown;
        self.state
    }

    /// Return the classification result of the last [`FaceClassifier::perform`]
    /// call.
    ///
    /// Returns [`TopState::Unknown`] if `perform` has not yet been called.
    ///
    /// Mirrors `BRepClass_FaceClassifier::State()`.
    pub fn state(&self) -> TopState {
        self.state
    }
}

impl Default for FaceClassifier {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────── free functions ───────────────────────────────────

/// Convenience function: classify 3-D point `pt` against the solid named
/// `solid` using `tol` as the on-boundary threshold.
///
/// Equivalent to constructing a [`SolidClassifier`] and calling
/// [`SolidClassifier::perform`].
///
/// Mirrors the intent of `BRepClass3d_SolidClassifier` used as a one-shot
/// helper.
pub fn point_in_solid(pt: [f64; 3], solid: &str, tol: f64) -> TopState {
    let mut clf = SolidClassifier::new(solid);
    clf.perform(pt, tol)
}

/// Convenience function: classify 2-D parameter-space point `pt` against the
/// face named `face` using `tol` as the on-boundary threshold.
///
/// Equivalent to constructing a [`FaceClassifier`] and calling
/// [`FaceClassifier::perform`].
///
/// Mirrors the intent of `BRepClass_FaceClassifier` used as a one-shot helper.
pub fn point_in_face(pt: [f64; 2], face: &str, tol: f64) -> TopState {
    let mut clf = FaceClassifier::new();
    clf.perform(face, pt, tol)
}

// ─────────────────────────── unit tests ──────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── SolidClassifier ───────────────────────────────────────────────────────

    #[test]
    fn solid_classifier_new_stores_name() {
        let clf = SolidClassifier::new("box_solid");
        assert_eq!(clf.solid, "box_solid");
        assert_eq!(clf.tolerance, 0.0);
    }

    #[test]
    fn solid_classifier_initial_state_is_unknown() {
        let clf = SolidClassifier::new("s");
        assert_eq!(clf.state(), TopState::Unknown);
    }

    #[test]
    fn solid_classifier_perform_returns_state() {
        let mut clf = SolidClassifier::new("s");
        let s = clf.perform([1.0, 2.0, 3.0], 1e-6);
        assert_eq!(s, clf.state());
    }

    #[test]
    fn solid_classifier_tolerance_updated_by_perform() {
        let mut clf = SolidClassifier::new("s");
        clf.perform([0.0, 0.0, 0.0], 1e-5);
        assert!((clf.tolerance - 1e-5).abs() < f64::EPSILON);
    }

    #[test]
    fn solid_classifier_is_on_face_false_by_default() {
        let mut clf = SolidClassifier::new("s");
        clf.perform([0.0, 0.0, 0.0], 1e-7);
        assert!(!clf.is_on_face());
    }

    // ── FaceClassifier ────────────────────────────────────────────────────────

    #[test]
    fn face_classifier_new_has_empty_face() {
        let clf = FaceClassifier::new();
        assert!(clf.face.is_empty());
    }

    #[test]
    fn face_classifier_initial_state_is_unknown() {
        let clf = FaceClassifier::new();
        assert_eq!(clf.state(), TopState::Unknown);
    }

    #[test]
    fn face_classifier_perform_stores_face_name() {
        let mut clf = FaceClassifier::new();
        clf.perform("my_face", [0.5, 0.5], 1e-7);
        assert_eq!(clf.face, "my_face");
    }

    #[test]
    fn face_classifier_perform_returns_state() {
        let mut clf = FaceClassifier::new();
        let s = clf.perform("f", [0.1, 0.2], 1e-6);
        assert_eq!(s, clf.state());
    }

    #[test]
    fn face_classifier_default_equals_new() {
        let a = FaceClassifier::new();
        let b = FaceClassifier::default();
        assert_eq!(a.face, b.face);
        assert_eq!(a.state(), b.state());
    }

    // ── free functions ────────────────────────────────────────────────────────

    #[test]
    fn point_in_solid_returns_top_state() {
        let s = point_in_solid([0.5, 0.5, 0.5], "cube", 1e-7);
        // Stub always returns Unknown; the important thing is that it compiles
        // and runs without panicking.
        let _ = s;
    }

    #[test]
    fn point_in_face_returns_top_state() {
        let s = point_in_face([0.3, 0.7], "face_1", 1e-7);
        let _ = s;
    }

    #[test]
    fn top_state_equality() {
        assert_eq!(TopState::In, TopState::In);
        assert_ne!(TopState::In, TopState::Out);
        assert_ne!(TopState::On, TopState::Unknown);
    }

    #[test]
    fn top_state_copy() {
        let a = TopState::On;
        let b = a; // Copy
        assert_eq!(a, b);
    }
}
