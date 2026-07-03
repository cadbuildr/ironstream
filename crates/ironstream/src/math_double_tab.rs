//! `math_DoubleTab` -- heap-allocated 1-D array of `f64` with an arbitrary
//! inclusive lower-bound index, mirroring OpenCascade's `math_DoubleTab`
//! (`src/FoundationClasses/TKMath/math/math_DoubleTab.{hxx,cxx}`).
//!
//! OCCT uses `math_DoubleTab` internally for `math_Vector`, `math_Matrix`, and
//! related types to allow arbitrary (possibly negative) index bases while
//! keeping a single heap allocation. The Rust version does the same with no
//! unsafe code.

// occt: math_DoubleTab
/// A heap-allocated array of `f64` whose index domain is the inclusive range
/// `[lower, upper]`.
///
/// All indexing methods panic (OCCT raises `Standard_RangeError`) when an
/// out-of-range index is supplied.
#[derive(Clone, Debug, PartialEq)]
pub struct DoubleTab {
    lower: i32,
    data: Vec<f64>,
}

impl DoubleTab {
    /// `math_DoubleTab(Lower, Upper)` — allocates `upper - lower + 1` elements,
    /// all initialised to `0.0`.
    ///
    /// # Panics
    /// Panics when `upper < lower`.
    pub fn new(lower: i32, upper: i32) -> Self {
        assert!(
            upper >= lower,
            "math_DoubleTab: upper ({upper}) < lower ({lower})"
        );
        let len = (upper - lower + 1) as usize;
        Self {
            lower,
            data: vec![0.0; len],
        }
    }

    /// `math_DoubleTab(Lower, Upper, InitialValue)` — like [`new`](Self::new)
    /// but fills every element with `initial_value`.
    ///
    /// # Panics
    /// Panics when `upper < lower`.
    pub fn with_value(lower: i32, upper: i32, initial_value: f64) -> Self {
        assert!(
            upper >= lower,
            "math_DoubleTab: upper ({upper}) < lower ({lower})"
        );
        let len = (upper - lower + 1) as usize;
        Self {
            lower,
            data: vec![initial_value; len],
        }
    }

    /// `math_DoubleTab::Init(InitialValue)` — overwrites every element with
    /// `value`.
    pub fn init(&mut self, value: f64) {
        self.data.fill(value);
    }

    /// `math_DoubleTab::Lower()` — inclusive lower bound of the index range.
    #[inline]
    pub fn lower(&self) -> i32 {
        self.lower
    }

    /// `math_DoubleTab::Upper()` — inclusive upper bound of the index range.
    #[inline]
    pub fn upper(&self) -> i32 {
        self.lower + self.data.len() as i32 - 1
    }

    /// `math_DoubleTab::Length()` — number of elements (`upper - lower + 1`).
    #[inline]
    pub fn length(&self) -> i32 {
        self.data.len() as i32
    }

    /// `math_DoubleTab::Value(Index)` — read element at `index`.
    ///
    /// # Panics
    /// Panics when `index` is out of `[lower, upper]`.
    #[inline]
    pub fn value(&self, index: i32) -> f64 {
        let i = self.checked_offset(index);
        self.data[i]
    }

    /// `math_DoubleTab::ChangeValue(Index)` — mutable reference to element at
    /// `index`.  Mirrors OCCT's `ChangeValue` which returns a non-const
    /// reference for assignment.
    ///
    /// # Panics
    /// Panics when `index` is out of `[lower, upper]`.
    #[inline]
    pub fn change_value(&mut self, index: i32) -> &mut f64 {
        let i = self.checked_offset(index);
        &mut self.data[i]
    }

    /// Convenience setter — assigns `value` to element `index`.
    ///
    /// Equivalent to `*tab.change_value(index) = value`.
    ///
    /// # Panics
    /// Panics when `index` is out of `[lower, upper]`.
    #[inline]
    pub fn set(&mut self, index: i32, value: f64) {
        let i = self.checked_offset(index);
        self.data[i] = value;
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    #[inline]
    fn checked_offset(&self, index: i32) -> usize {
        assert!(
            index >= self.lower && index <= self.upper(),
            "math_DoubleTab: index {index} out of range [{}, {}]",
            self.lower,
            self.upper()
        );
        (index - self.lower) as usize
    }
}
