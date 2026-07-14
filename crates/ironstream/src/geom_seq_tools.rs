// occt-ref: TColgp_SequenceOfPnt
/// A sequence of 3D points, modelled after TColgp_SequenceOfPnt.
/// Points are stored as `[f64; 3]` arrays (x, y, z).
pub struct SeqOfPnt {
    pub data: Vec<[f64; 3]>,
}

impl SeqOfPnt {
    /// Create an empty sequence.
    pub fn new() -> Self {
        SeqOfPnt { data: Vec::new() }
    }

    /// Append a point to the end of the sequence.
    pub fn append(&mut self, v: [f64; 3]) {
        self.data.push(v);
    }

    /// Prepend a point to the beginning of the sequence.
    pub fn prepend(&mut self, v: [f64; 3]) {
        self.data.insert(0, v);
    }

    /// Return the first point, or `None` if the sequence is empty.
    pub fn first(&self) -> Option<[f64; 3]> {
        self.data.first().copied()
    }

    /// Return the last point, or `None` if the sequence is empty.
    pub fn last(&self) -> Option<[f64; 3]> {
        self.data.last().copied()
    }

    /// Return the point at index `i` (0-based).
    ///
    /// # Panics
    /// Panics if `i` is out of bounds.
    pub fn value(&self, i: usize) -> [f64; 3] {
        self.data[i]
    }

    /// Remove the point at index `i` (0-based).
    ///
    /// # Panics
    /// Panics if `i` is out of bounds.
    pub fn remove(&mut self, i: usize) {
        self.data.remove(i);
    }

    /// Return the number of points in the sequence.
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Return `true` if the sequence contains no points.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Remove all points from the sequence.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for SeqOfPnt {
    fn default() -> Self {
        Self::new()
    }
}

// occt-ref: TColStd_SequenceOfReal
/// A sequence of real (floating-point) values, modelled after TColStd_SequenceOfReal.
pub struct SeqOfReal {
    pub data: Vec<f64>,
}

impl SeqOfReal {
    /// Create an empty sequence.
    pub fn new() -> Self {
        SeqOfReal { data: Vec::new() }
    }

    /// Append a value to the end of the sequence.
    pub fn append(&mut self, v: f64) {
        self.data.push(v);
    }

    /// Return the first value, or `None` if the sequence is empty.
    pub fn first(&self) -> Option<f64> {
        self.data.first().copied()
    }

    /// Return the last value, or `None` if the sequence is empty.
    pub fn last(&self) -> Option<f64> {
        self.data.last().copied()
    }

    /// Return the value at index `i` (0-based).
    ///
    /// # Panics
    /// Panics if `i` is out of bounds.
    pub fn value(&self, i: usize) -> f64 {
        self.data[i]
    }

    /// Remove the value at index `i` (0-based).
    ///
    /// # Panics
    /// Panics if `i` is out of bounds.
    pub fn remove(&mut self, i: usize) {
        self.data.remove(i);
    }

    /// Return the number of values in the sequence.
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Return `true` if the sequence contains no values.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for SeqOfReal {
    fn default() -> Self {
        Self::new()
    }
}

/// A sequence of integer values, modelled after TColStd_SequenceOfInteger.
pub struct SeqOfInteger {
    pub data: Vec<i32>,
}

impl SeqOfInteger {
    /// Create an empty sequence.
    pub fn new() -> Self {
        SeqOfInteger { data: Vec::new() }
    }

    /// Append a value to the end of the sequence.
    pub fn append(&mut self, v: i32) {
        self.data.push(v);
    }

    /// Return the value at index `i` (0-based).
    ///
    /// # Panics
    /// Panics if `i` is out of bounds.
    pub fn value(&self, i: usize) -> i32 {
        self.data[i]
    }

    /// Return the number of values in the sequence.
    pub fn length(&self) -> usize {
        self.data.len()
    }
}

impl Default for SeqOfInteger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seq_of_pnt_basic() {
        let mut s = SeqOfPnt::new();
        assert!(s.is_empty());
        assert_eq!(s.length(), 0);
        assert_eq!(s.first(), None);
        assert_eq!(s.last(), None);

        s.append([1.0, 2.0, 3.0]);
        s.append([4.0, 5.0, 6.0]);
        assert_eq!(s.length(), 2);
        assert_eq!(s.first(), Some([1.0, 2.0, 3.0]));
        assert_eq!(s.last(), Some([4.0, 5.0, 6.0]));
        assert_eq!(s.value(0), [1.0, 2.0, 3.0]);
        assert_eq!(s.value(1), [4.0, 5.0, 6.0]);

        s.prepend([0.0, 0.0, 0.0]);
        assert_eq!(s.length(), 3);
        assert_eq!(s.value(0), [0.0, 0.0, 0.0]);

        s.remove(0);
        assert_eq!(s.length(), 2);
        assert_eq!(s.value(0), [1.0, 2.0, 3.0]);

        s.clear();
        assert!(s.is_empty());
    }

    #[test]
    fn seq_of_real_basic() {
        let mut s = SeqOfReal::new();
        assert!(s.is_empty());
        assert_eq!(s.first(), None);
        assert_eq!(s.last(), None);

        s.append(1.5);
        s.append(2.5);
        assert_eq!(s.length(), 2);
        assert_eq!(s.first(), Some(1.5));
        assert_eq!(s.last(), Some(2.5));
        assert_eq!(s.value(1), 2.5);

        s.remove(0);
        assert_eq!(s.length(), 1);
        assert_eq!(s.value(0), 2.5);
    }

    #[test]
    fn seq_of_integer_basic() {
        let mut s = SeqOfInteger::new();
        assert_eq!(s.length(), 0);

        s.append(10);
        s.append(20);
        assert_eq!(s.length(), 2);
        assert_eq!(s.value(0), 10);
        assert_eq!(s.value(1), 20);
    }
}
