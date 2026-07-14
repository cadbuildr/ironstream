// FILE: top_ope_b_rep_array1_of_line_inter.rs
// occt: TopOpeBRep_Array1OfLineInter
// occt-ref: TopOpeBRep_LineInter

/// LineInter: Represents a line of intersection.
#[derive(Clone, Debug)]
pub struct LineInter {
    id: usize,
    parameter_range: (f64, f64),
}

impl LineInter {
    pub fn new(id: usize) -> Self {
        LineInter {
            id,
            parameter_range: (0.0, 1.0),
        }
    }

    pub fn with_range(id: usize, param_min: f64, param_max: f64) -> Self {
        LineInter {
            id,
            parameter_range: (param_min, param_max),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn parameter_range(&self) -> (f64, f64) {
        self.parameter_range
    }

    pub fn set_parameter_range(&mut self, min: f64, max: f64) {
        self.parameter_range = (min, max);
    }
}

impl Default for LineInter {
    fn default() -> Self {
        LineInter::new(0)
    }
}

/// Array1OfLineInter: 1-based array of LineInter.
#[derive(Clone, Debug)]
pub struct Array1OfLineInter {
    data: Vec<LineInter>,
    lower: usize,
}

impl Array1OfLineInter {
    pub fn new(size: usize) -> Self {
        Array1OfLineInter {
            data: (0..size).map(|i| LineInter::new(i)).collect(),
            lower: 1,
        }
    }

    pub fn new_from_bounds(lower: usize, upper: usize) -> Self {
        if lower == 0 {
            panic!("OCCT arrays use 1-based indexing");
        }
        let size = upper - lower + 1;
        Array1OfLineInter {
            data: (0..size).map(|i| LineInter::new(i)).collect(),
            lower,
        }
    }

    pub fn lower(&self) -> usize {
        self.lower
    }

    pub fn upper(&self) -> usize {
        self.lower + self.data.len() - 1
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }

    pub fn value(&self, index_1based: usize) -> Option<&LineInter> {
        if index_1based < self.lower {
            None
        } else {
            self.data.get(index_1based - self.lower)
        }
    }

    pub fn value_mut(&mut self, index_1based: usize) -> Option<&mut LineInter> {
        if index_1based < self.lower {
            None
        } else {
            self.data.get_mut(index_1based - self.lower)
        }
    }

    pub fn set_value(&mut self, index_1based: usize, value: LineInter) {
        if index_1based < self.lower {
            panic!("Index out of bounds");
        }
        let idx = index_1based - self.lower;
        if idx >= self.data.len() {
            panic!("Index out of bounds");
        }
        self.data[idx] = value;
    }

    pub fn iter(&self) -> impl Iterator<Item = &LineInter> {
        self.data.iter()
    }
}

impl Default for Array1OfLineInter {
    fn default() -> Self {
        Self::new(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_inter_new() {
        let line = LineInter::new(42);
        assert_eq!(line.id(), 42);
    }

    #[test]
    fn test_line_inter_with_range() {
        let line = LineInter::with_range(10, 0.0, 1.0);
        assert_eq!(line.id(), 10);
        assert_eq!(line.parameter_range(), (0.0, 1.0));
    }

    #[test]
    fn test_array1_new() {
        let arr = Array1OfLineInter::new(5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn test_array1_from_bounds() {
        let arr = Array1OfLineInter::new_from_bounds(1, 10);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 10);
    }

    #[test]
    fn test_array1_value() {
        let arr = Array1OfLineInter::new(3);
        assert!(arr.value(0).is_none());
        assert!(arr.value(1).is_some());
        assert!(arr.value(4).is_none());
    }

    #[test]
    fn test_array1_set_value() {
        let mut arr = Array1OfLineInter::new(2);
        let line = LineInter::with_range(99, 0.5, 0.7);
        arr.set_value(1, line);

        let retrieved = arr.value(1).unwrap();
        assert_eq!(retrieved.id(), 99);
        assert_eq!(retrieved.parameter_range(), (0.5, 0.7));
    }

    #[test]
    #[should_panic]
    fn test_array1_zero_lower_panic() {
        let _ = Array1OfLineInter::new_from_bounds(0, 5);
    }
}
