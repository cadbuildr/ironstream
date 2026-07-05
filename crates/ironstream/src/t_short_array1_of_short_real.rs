// FILE: t_short_array1_of_short_real.rs
// occt: TShort_Array1OfShortReal

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_Array1<float> TShort_Array1OfShortReal;`
//!
//! Classic OCCT Array1 of ShortReal (f32): fixed bounds chosen at
//! construction (any lower bound, commonly 1), Value/SetValue/Init,
//! historically used to store per-node normals of triangulations.

/// `TShort_Array1OfShortReal`.
pub struct TShortArray1OfShortReal {
    lower: i32,
    upper: i32,
    data: Vec<f32>,
}

impl TShortArray1OfShortReal {
    /// Creates an array with bounds [lower, upper], zero-initialized.
    pub fn new(lower: i32, upper: i32) -> Self {
        assert!(upper >= lower, "Array1: upper must be >= lower");
        TShortArray1OfShortReal {
            lower,
            upper,
            data: vec![0.0f32; (upper - lower + 1) as usize],
        }
    }

    pub fn lower(&self) -> i32 {
        self.lower
    }

    pub fn upper(&self) -> i32 {
        self.upper
    }

    pub fn length(&self) -> i32 {
        self.upper - self.lower + 1
    }

    fn offset(&self, index: i32) -> usize {
        assert!(
            index >= self.lower && index <= self.upper,
            "Array1: index {} out of range [{}, {}]",
            index,
            self.lower,
            self.upper
        );
        (index - self.lower) as usize
    }

    /// Value(index).
    pub fn value(&self, index: i32) -> f32 {
        self.data[self.offset(index)]
    }

    /// SetValue(index, v).
    pub fn set_value(&mut self, index: i32, v: f32) {
        let off = self.offset(index);
        self.data[off] = v;
    }

    /// Init(v) — fill with one value.
    pub fn init(&mut self, v: f32) {
        self.data.fill(v);
    }

    /// ChangeValue(index) — mutable reference.
    pub fn change_value(&mut self, index: i32) -> &mut f32 {
        let off = self.offset(index);
        &mut self.data[off]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_and_zero_init() {
        let arr = TShortArray1OfShortReal::new(1, 9);
        assert_eq!((arr.lower(), arr.upper(), arr.length()), (1, 9, 9));
        assert_eq!(arr.value(5), 0.0);
    }

    #[test]
    fn normal_triplets_storage() {
        // Typical use: 3 floats per node normal, 1-based.
        let mut arr = TShortArray1OfShortReal::new(1, 6);
        let normals = [0.0f32, 0.0, 1.0, 1.0, 0.0, 0.0];
        for (i, n) in normals.iter().enumerate() {
            arr.set_value(1 + i as i32, *n);
        }
        assert_eq!(arr.value(3), 1.0);
        assert_eq!(arr.value(4), 1.0);
        assert_eq!(arr.value(6), 0.0);
    }

    #[test]
    fn custom_lower_bound_and_init() {
        let mut arr = TShortArray1OfShortReal::new(-2, 2);
        assert_eq!(arr.length(), 5);
        arr.init(0.5);
        assert_eq!(arr.value(-2), 0.5);
        assert_eq!(arr.value(2), 0.5);
        *arr.change_value(0) = 7.25;
        assert_eq!(arr.value(0), 7.25);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn above_upper_panics() {
        let arr = TShortArray1OfShortReal::new(1, 3);
        let _ = arr.value(4);
    }
}
