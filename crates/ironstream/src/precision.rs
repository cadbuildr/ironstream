//! `Precision` — tolerance constants, mirroring OpenCascade's `Precision`
//! package. Used throughout the kernel and the ported OCCT unit tests.

// occt: Precision

/// `Precision::Confusion()` — the standard linear coincidence tolerance.
pub const CONFUSION: f64 = 1.0e-7;

/// `Precision::SquareConfusion()`.
pub const SQUARE_CONFUSION: f64 = CONFUSION * CONFUSION;

/// `Precision::Angular()` — the standard angular tolerance (radians).
pub const ANGULAR: f64 = 1.0e-12;

/// `Precision::Intersection()`.
pub const INTERSECTION: f64 = CONFUSION * 0.01;

/// `Precision::Approximation()`.
pub const APPROXIMATION: f64 = CONFUSION * 10.0;

/// `Precision::Infinite()`.
pub const INFINITE: f64 = 2.0e108;

/// `Precision::IsInfinite(value)`.
pub fn is_infinite(v: f64) -> bool {
    v.abs() >= INFINITE * 0.5
}
