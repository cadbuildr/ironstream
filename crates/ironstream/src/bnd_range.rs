//! `Bnd_Range` — a 1D range restricted by two real values, mirroring
//! OpenCascade's `Bnd_Range` class (`src/FoundationClasses/TKMath/Bnd`).
//!
//! A range can be *void*, indicating it contains no point. The void state is
//! encoded exactly as in OCCT: `myLast < myFirst` (default `myFirst = 0.0`,
//! `myLast = -1.0`). Semantics and method names follow OCCT (snake_case for
//! Rust idiom). Self-contained: depends only on `std`.

/// OCCT's `RealSmall()` == `DBL_MIN` (smallest positive normal double).
const REAL_SMALL: f64 = f64::MIN_POSITIVE;

/// OCCT's `IsEqual(a, b)` for doubles: `Abs(a - b) < RealSmall()`.
#[inline]
fn occt_is_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < REAL_SMALL
}

/// Bounds of a range: a `(min, max)` pair, mirroring OCCT's nested
/// `Bnd_Range::Bounds` struct (used for "structured-binding"-style access).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bounds {
    /// Minimum value of the range.
    pub min: f64,
    /// Maximum value of the range.
    pub max: f64,
}

/// Status of an intersection check against a (possibly periodic) value,
/// mirroring OCCT's `Bnd_Range::IntersectStatus`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntersectStatus {
    /// No intersection with `theVal + k * thePeriod`.
    Out,
    /// Range strictly contains `theVal + k * thePeriod`.
    In,
    /// Range boundary coincides with `theVal + k * thePeriod`.
    Boundary,
}

/// `Bnd_Range` — a range in 1D space restricted by two real values.
// occt: Bnd_Range
#[derive(Clone, Copy, Debug)]
pub struct BndRange {
    first: f64,
    last: f64,
}

impl BndRange {
    /// Default constructor. Creates a VOID range (`first = 0.0`, `last = -1.0`).
    pub fn new() -> Self {
        Self {
            first: 0.0,
            last: -1.0,
        }
    }

    /// Constructor from `(min, max)`. Never creates a VOID range.
    ///
    /// # Panics
    /// Panics (OCCT throws `Standard_ConstructionError`) if `max < min`.
    pub fn with_bounds(min: f64, max: f64) -> Self {
        if max < min {
            panic!("Bnd_Range: Last < First");
        }
        Self {
            first: min,
            last: max,
        }
    }

    /// Non-panicking variant of [`with_bounds`](Self::with_bounds), returning
    /// `Err` instead of throwing on `max < min`. Useful for tests that assert
    /// the construction error without unwinding.
    pub fn try_with_bounds(min: f64, max: f64) -> Result<Self, &'static str> {
        if max < min {
            return Err("Last < First");
        }
        Ok(Self {
            first: min,
            last: max,
        })
    }

    /// Replaces `self` with the common part of `self` and `other`.
    pub fn common(&mut self, other: &BndRange) {
        if other.is_void() {
            self.set_void();
            return;
        }
        if self.is_void() {
            return;
        }
        self.first = self.first.max(other.first);
        self.last = self.last.min(other.last);
    }

    /// Joins `self` and `other` into one interval, replacing `self` with the
    /// result. Returns `false` (leaving `self` unchanged) if the operation
    /// cannot be done (inputs are void or separated).
    #[must_use]
    pub fn union(&mut self, other: &BndRange) -> bool {
        if self.is_void() || other.is_void() {
            return false;
        }
        if self.last < other.first {
            return false;
        }
        if self.first > other.last {
            return false;
        }
        self.first = self.first.min(other.first);
        self.last = self.last.max(other.last);
        true
    }

    /// Splits `self` into several sub-ranges by `val`, appending them to
    /// `list` (which must be initialized by the caller). If `period != 0.0`,
    /// the splits occur at `val + k * period` for every integer `k` such that
    /// the value lies inside the range.
    pub fn split(&self, val: f64, list: &mut Vec<BndRange>, period: f64) {
        let a_period = period.abs();
        if self.is_intersected(val, a_period) != IntersectStatus::In {
            list.push(*self);
            return;
        }

        let is_periodic = a_period > 0.0;

        if !is_periodic {
            list.push(BndRange::with_bounds(self.first, val));
            list.push(BndRange::with_bounds(val, self.last));
            return;
        }

        let mut a_val_prev = val + a_period * ((self.first - val) / a_period).ceil();

        // Now, (first <= a_val_prev < first + a_period).
        if a_val_prev > self.first {
            list.push(BndRange::with_bounds(self.first, a_val_prev));
        }

        let mut a_val = a_val_prev + a_period;
        while a_val <= self.last {
            list.push(BndRange::with_bounds(a_val_prev, a_val));
            a_val_prev = a_val;
            a_val += a_period;
        }

        if a_val_prev < self.last {
            list.push(BndRange::with_bounds(a_val_prev, self.last));
        }
    }

    /// Checks if `self` intersects values like `val + k * period`, where `k`
    /// is any integer. Mirrors OCCT's `IsIntersected` exactly, including the
    /// floor-based seam-edge logic for the periodic case.
    ///
    /// ATTENTION: if `first == last`, this returns only `Out` or `Boundary`.
    #[must_use]
    pub fn is_intersected(&self, val: f64, period: f64) -> IntersectStatus {
        if self.is_void() {
            return IntersectStatus::Out;
        }

        let a_period = period.abs();
        let a_df = self.first - val;
        let a_dl = self.last - val;

        if a_period <= REAL_SMALL {
            let a_delta = a_df * a_dl;
            if occt_is_equal(a_delta, 0.0) {
                return IntersectStatus::Boundary;
            }
            if a_delta > 0.0 {
                return IntersectStatus::Out;
            }
            return IntersectStatus::In;
        }

        // The interval [aDF/aPeriod, aDL/aPeriod] must contain at least one
        // integer number for an intersection to exist.
        let a_val1 = a_df / a_period;
        let a_val2 = a_dl / a_period;
        let a_par1 = a_val1.floor() as i64;
        let a_par2 = a_val2.floor() as i64;

        if a_par1 != a_par2 {
            // Interval (first, last] intersects a seam-edge.
            if occt_is_equal(a_val2, a_par2 as f64) {
                // aVal2 is integer => last lies ON the seam-edge.
                return IntersectStatus::Boundary;
            }
            return IntersectStatus::In;
        }

        // Here, aPar1 == aPar2.
        if occt_is_equal(a_val1, a_par1 as f64) {
            // aVal1 is integer => first lies ON the seam-edge.
            return IntersectStatus::Boundary;
        }

        IntersectStatus::Out
    }

    /// Convenience: [`is_intersected`](Self::is_intersected) with period `0.0`.
    #[must_use]
    pub fn is_intersected_val(&self, val: f64) -> IntersectStatus {
        self.is_intersected(val, 0.0)
    }

    /// Extends `self` to include `parameter`.
    pub fn add(&mut self, parameter: f64) {
        if self.is_void() {
            self.first = parameter;
            self.last = parameter;
            return;
        }
        self.first = self.first.min(parameter);
        self.last = self.last.max(parameter);
    }

    /// Extends `self` to include both ranges (unconditional merge).
    pub fn add_range(&mut self, range: &BndRange) {
        if range.is_void() {
            return;
        }
        if self.is_void() {
            *self = *range;
            return;
        }
        self.first = self.first.min(range.first);
        self.last = self.last.max(range.last);
    }

    /// Obtain the MIN boundary. Returns `None` if VOID.
    pub fn get_min(&self) -> Option<f64> {
        if self.is_void() {
            None
        } else {
            Some(self.first)
        }
    }

    /// Obtain the MAX boundary. Returns `None` if VOID.
    pub fn get_max(&self) -> Option<f64> {
        if self.is_void() {
            None
        } else {
            Some(self.last)
        }
    }

    /// Obtain `(first, last)` boundaries. Returns `None` if VOID.
    pub fn get_bounds(&self) -> Option<(f64, f64)> {
        if self.is_void() {
            None
        } else {
            Some((self.first, self.last))
        }
    }

    /// Returns the bounds as a [`Bounds`] struct, or `None` if VOID.
    /// Mirrors OCCT's `Get()`.
    pub fn get(&self) -> Option<Bounds> {
        if self.is_void() {
            None
        } else {
            Some(Bounds {
                min: self.first,
                max: self.last,
            })
        }
    }

    /// Obtain the parameter satisfying `(p - MIN)/(MAX - MIN) == lambda`.
    /// Returns `None` if VOID.
    pub fn get_intermediate_point(&self, lambda: f64) -> Option<f64> {
        if self.is_void() {
            None
        } else {
            Some(self.first + lambda * (self.last - self.first))
        }
    }

    /// Returns the center `(min + max) / 2`, or `None` if VOID.
    pub fn center(&self) -> Option<f64> {
        if self.is_void() {
            None
        } else {
            Some(0.5 * (self.first + self.last))
        }
    }

    /// Returns the range value `(MAX - MIN)`. Negative for a VOID range.
    pub fn delta(&self) -> f64 {
        self.last - self.first
    }

    /// Is `self` VOID (uninitialized)?
    pub fn is_void(&self) -> bool {
        self.last < self.first
    }

    /// Resets `self` to the default (VOID) state.
    pub fn set_void(&mut self) {
        self.last = -1.0;
        self.first = 0.0;
    }

    /// Extends `self` by `delta` on both sides. No-op if VOID.
    pub fn enlarge(&mut self, delta: f64) {
        if self.is_void() {
            return;
        }
        self.first -= delta;
        self.last += delta;
    }

    /// Returns a copy of `self` shifted by `val`. VOID stays VOID.
    pub fn shifted(&self, val: f64) -> BndRange {
        if self.is_void() {
            BndRange::new()
        } else {
            BndRange::with_bounds(self.first + val, self.last + val)
        }
    }

    /// Shifts `self` by `val`. No-op if VOID.
    pub fn shift(&mut self, val: f64) {
        if !self.is_void() {
            self.first += val;
            self.last += val;
        }
    }

    /// Trims the first value by a lower limit. Marks the range VOID if
    /// `val_lower` exceeds the current MAX.
    pub fn trim_from(&mut self, val_lower: f64) {
        if !self.is_void() {
            self.first = self.first.max(val_lower);
        }
    }

    /// Trims the last value by an upper limit. Marks the range VOID if
    /// `val_upper` is below the current MIN.
    pub fn trim_to(&mut self, val_upper: f64) {
        if !self.is_void() {
            self.last = self.last.min(val_upper);
        }
    }

    /// Returns `true` if `value` is outside `self` (always `true` if VOID).
    pub fn is_out(&self, value: f64) -> bool {
        self.is_void() || value < self.first || value > self.last
    }

    /// Returns `true` if `range` is entirely outside `self`.
    pub fn is_out_range(&self, range: &BndRange) -> bool {
        self.is_void() || range.is_void() || range.last < self.first || range.first > self.last
    }

    /// Returns `true` if `value` lies within `self`.
    pub fn contains(&self, value: f64) -> bool {
        !self.is_out(value)
    }

    /// Returns `true` if `range` overlaps `self`.
    pub fn intersects(&self, range: &BndRange) -> bool {
        !self.is_out_range(range)
    }

    /// Returns the MIN boundary, or `None` if VOID.
    pub fn min(&self) -> Option<f64> {
        if self.is_void() {
            None
        } else {
            Some(self.first)
        }
    }

    /// Returns the MAX boundary, or `None` if VOID.
    pub fn max(&self) -> Option<f64> {
        if self.is_void() {
            None
        } else {
            Some(self.last)
        }
    }
}

impl Default for BndRange {
    fn default() -> Self {
        Self::new()
    }
}

/// `operator==` — exact equality of both boundaries (OCCT uses `==` on the
/// raw `myFirst`/`myLast` fields, so two VOID ranges compare equal only when
/// their raw fields match; the default-constructed VOID is `(0.0, -1.0)`).
impl PartialEq for BndRange {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.last == other.last
    }
}
