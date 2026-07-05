// FILE: top_ope_b_rep_sequence_of_point2d.rs
// occt: TopOpeBRep_SequenceOfPoint2d

/// Point2d: 2D point.
#[derive(Clone, Debug)]
pub struct Point2d {
    x: f64,
    y: f64,
}

impl Point2d {
    pub fn new(x: f64, y: f64) -> Self {
        Point2d { x, y }
    }

    pub fn coordinates(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

impl Default for Point2d {
    fn default() -> Self {
        Point2d::new(0.0, 0.0)
    }
}

/// SequenceOfPoint2d: OCCT 1-based sequence of 2D points.
#[derive(Clone, Debug)]
pub struct SequenceOfPoint2d {
    data: Vec<Point2d>,
}

impl SequenceOfPoint2d {
    pub fn new() -> Self {
        SequenceOfPoint2d { data: Vec::new() }
    }

    pub fn append(&mut self, point: Point2d) {
        self.data.push(point);
    }

    pub fn prepend(&mut self, point: Point2d) {
        self.data.insert(0, point);
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Get at 1-based index (OCCT style).
    pub fn value(&self, index_1based: usize) -> Option<&Point2d> {
        if index_1based == 0 {
            None
        } else {
            self.data.get(index_1based - 1)
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Point2d> {
        self.data.iter()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn lower(&self) -> usize {
        1
    }

    pub fn upper(&self) -> usize {
        self.data.len()
    }
}

impl Default for SequenceOfPoint2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2d_new() {
        let pt = Point2d::new(1.5, 2.5);
        assert_eq!(pt.coordinates(), (1.5, 2.5));
    }

    #[test]
    fn test_sequence_append() {
        let mut seq = SequenceOfPoint2d::new();
        seq.append(Point2d::new(1.0, 2.0));
        seq.append(Point2d::new(3.0, 4.0));
        assert_eq!(seq.size(), 2);
    }

    #[test]
    fn test_sequence_value_1based() {
        let mut seq = SequenceOfPoint2d::new();
        seq.append(Point2d::new(1.0, 2.0));
        seq.append(Point2d::new(3.0, 4.0));

        assert!(seq.value(0).is_none());
        assert_eq!(seq.value(1).unwrap().coordinates(), (1.0, 2.0));
        assert_eq!(seq.value(2).unwrap().coordinates(), (3.0, 4.0));
    }

    #[test]
    fn test_sequence_bounds() {
        let mut seq = SequenceOfPoint2d::new();
        seq.append(Point2d::new(0.0, 0.0));
        seq.append(Point2d::new(1.0, 1.0));
        assert_eq!(seq.lower(), 1);
        assert_eq!(seq.upper(), 2);
    }
}
