// FILE: aspect_xr_session.rs
// occt: Aspect_XRSession

use core::fmt;

/// Identifies which style of tracking origin the application wants to use for the poses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackingUniverseOrigin {
    /// Poses are provided relative to the seated zero pose
    Seated = 0,
    /// Poses are provided relative to the safe bounds configured by the user
    Standing = 1,
}

/// Extended Reality (XR) Session interface.
///
/// This is an abstract base class providing the interface for XR session management.
/// Real implementations should inherit from this and provide platform-specific behavior.
pub struct AspectXrSession {
    /// Unit scale factor defined as scale factor for m (meters); 1.0 by default
    unit_factor: f64,
    /// Aspect ratio
    aspect: f64,
    /// Field of view in radians
    field_of_view: f64,
    /// Intra-ocular Distance (IOD) / Interpupillary Distance (IPD) in meters
    iod: f64,
    /// Display frequency in Hz, or 0 if unknown
    display_frequency: f32,
    /// Tracking origin (seated or standing)
    tracking_origin: TrackingUniverseOrigin,
}

impl AspectXrSession {
    /// Create a new XR session with default parameters.
    pub fn new() -> Self {
        Self {
            unit_factor: 1.0,
            aspect: 1.0,
            field_of_view: 0.0,
            iod: 0.0,
            display_frequency: 0.0,
            tracking_origin: TrackingUniverseOrigin::Seated,
        }
    }

    /// Return unit scale factor defined as scale factor for m (meters); 1.0 by default.
    pub fn unit_factor(&self) -> f64 {
        self.unit_factor
    }

    /// Set unit scale factor.
    pub fn set_unit_factor(&mut self, factor: f64) {
        self.unit_factor = factor;
    }

    /// Return aspect ratio.
    pub fn aspect(&self) -> f64 {
        self.aspect
    }

    /// Return field of view.
    pub fn field_of_view(&self) -> f64 {
        self.field_of_view
    }

    /// Return Intra-ocular Distance (IOD); also known as Interpupillary Distance (IPD).
    /// Defined in meters by default.
    pub fn iod(&self) -> f64 {
        self.iod
    }

    /// Return display frequency or 0 if unknown.
    pub fn display_frequency(&self) -> f32 {
        self.display_frequency
    }

    /// Return tracking origin.
    pub fn tracking_origin(&self) -> TrackingUniverseOrigin {
        self.tracking_origin
    }

    /// Set tracking origin.
    pub fn set_tracking_origin(&mut self, origin: TrackingUniverseOrigin) {
        self.tracking_origin = origin;
    }
}

impl Default for AspectXrSession {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for AspectXrSession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AspectXrSession")
            .field("unit_factor", &self.unit_factor)
            .field("aspect", &self.aspect)
            .field("field_of_view", &self.field_of_view)
            .field("iod", &self.iod)
            .field("display_frequency", &self.display_frequency)
            .field("tracking_origin", &self.tracking_origin)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xr_session_default_construction() {
        let session = AspectXrSession::new();
        assert_eq!(session.unit_factor(), 1.0);
        assert_eq!(session.aspect(), 1.0);
        assert_eq!(session.tracking_origin(), TrackingUniverseOrigin::Seated);
    }

    #[test]
    fn test_xr_session_unit_factor() {
        let mut session = AspectXrSession::new();
        assert_eq!(session.unit_factor(), 1.0);
        session.set_unit_factor(2.5);
        assert_eq!(session.unit_factor(), 2.5);
    }

    #[test]
    fn test_xr_session_tracking_origin() {
        let mut session = AspectXrSession::new();
        assert_eq!(session.tracking_origin(), TrackingUniverseOrigin::Seated);
        session.set_tracking_origin(TrackingUniverseOrigin::Standing);
        assert_eq!(session.tracking_origin(), TrackingUniverseOrigin::Standing);
    }

    #[test]
    fn test_tracking_universe_origin_values() {
        assert_eq!(TrackingUniverseOrigin::Seated as i32, 0);
        assert_eq!(TrackingUniverseOrigin::Standing as i32, 1);
    }

    #[test]
    fn test_xr_session_default_trait() {
        let session1 = AspectXrSession::default();
        let session2 = AspectXrSession::new();
        assert_eq!(session1.unit_factor(), session2.unit_factor());
        assert_eq!(session1.tracking_origin(), session2.tracking_origin());
    }

    #[test]
    fn test_xr_session_display_properties() {
        let mut session = AspectXrSession::new();
        assert_eq!(session.field_of_view(), 0.0);
        assert_eq!(session.iod(), 0.0);
        assert_eq!(session.display_frequency(), 0.0);
    }
}
