// FILE: tcol_geom2d.rs

// occt: TColGeom2d_Array1OfCurve
#[derive(Clone, Debug)]
pub struct ColGeom2d1dCurve {
    data: Vec<Option<String>>,
    lower: i32,
    upper: i32,
}

impl ColGeom2d1dCurve {
    pub fn new(lower: i32, upper: i32) -> Self {
        assert!(
            upper >= lower,
            "TColGeom2d_Array1OfCurve: upper ({upper}) < lower ({lower})"
        );
        let size = (upper - lower + 1) as usize;
        Self {
            data: vec![None; size],
            lower,
            upper,
        }
    }

    pub fn set(&mut self, idx: i32, name: String) {
        self.check_bounds(idx);
        self.data[(idx - self.lower) as usize] = Some(name);
    }

    pub fn get(&self, idx: i32) -> Option<&String> {
        self.check_bounds(idx);
        self.data[(idx - self.lower) as usize].as_ref()
    }

    #[inline]
    pub fn lower(&self) -> i32 {
        self.lower
    }

    #[inline]
    pub fn upper(&self) -> i32 {
        self.upper
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    fn check_bounds(&self, idx: i32) {
        assert!(
            idx >= self.lower && idx <= self.upper,
            "TColGeom2d_Array1OfCurve: index {idx} out of range [{}, {}]",
            self.lower,
            self.upper
        );
    }
}

// occt: TColGeom2d_SequenceOfCurve
#[derive(Clone, Debug, Default)]
pub struct ColGeom2dSeqCurve {
    data: Vec<String>,
}

impl ColGeom2dSeqCurve {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, name: String) {
        self.data.push(name);
    }

    pub fn prepend(&mut self, name: String) {
        self.data.insert(0, name);
    }

    pub fn first(&self) -> &String {
        assert!(!self.data.is_empty(), "TColGeom2d_SequenceOfCurve::first: empty sequence");
        &self.data[0]
    }

    pub fn last(&self) -> &String {
        assert!(!self.data.is_empty(), "TColGeom2d_SequenceOfCurve::last: empty sequence");
        self.data.last().unwrap()
    }

    pub fn get(&self, idx: usize) -> &String {
        self.check_bounds(idx);
        &self.data[idx - 1]
    }

    #[inline]
    pub fn length(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn remove(&mut self, idx: usize) {
        self.check_bounds(idx);
        self.data.remove(idx - 1);
    }

    #[inline]
    fn check_bounds(&self, idx: usize) {
        assert!(
            idx >= 1 && idx <= self.data.len(),
            "TColGeom2d_SequenceOfCurve: index {idx} out of [1, {}]",
            self.data.len()
        );
    }
}

// occt: TColGeom2d_SequenceOfGeometry
#[derive(Clone, Debug, Default)]
pub struct ColGeom2dSeqGeom {
    data: Vec<String>,
}

impl ColGeom2dSeqGeom {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, name: String) {
        self.data.push(name);
    }

    pub fn prepend(&mut self, name: String) {
        self.data.insert(0, name);
    }

    pub fn first(&self) -> &String {
        assert!(!self.data.is_empty(), "TColGeom2d_SequenceOfGeometry::first: empty sequence");
        &self.data[0]
    }

    pub fn last(&self) -> &String {
        assert!(!self.data.is_empty(), "TColGeom2d_SequenceOfGeometry::last: empty sequence");
        self.data.last().unwrap()
    }

    pub fn get(&self, idx: usize) -> &String {
        self.check_bounds(idx);
        &self.data[idx - 1]
    }

    #[inline]
    pub fn length(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn remove(&mut self, idx: usize) {
        self.check_bounds(idx);
        self.data.remove(idx - 1);
    }

    #[inline]
    fn check_bounds(&self, idx: usize) {
        assert!(
            idx >= 1 && idx <= self.data.len(),
            "TColGeom2d_SequenceOfGeometry: index {idx} out of [1, {}]",
            self.data.len()
        );
    }
}

// occt-ref: TColGeom2d_Array1OfBoundedCurve
#[derive(Clone, Debug)]
pub struct ColGeom2d1dBoundedCurve {
    data: Vec<Option<String>>,
    lower: i32,
    upper: i32,
}

impl ColGeom2d1dBoundedCurve {
    pub fn new(lower: i32, upper: i32) -> Self {
        assert!(
            upper >= lower,
            "TColGeom2d_Array1OfBoundedCurve: upper ({upper}) < lower ({lower})"
        );
        let size = (upper - lower + 1) as usize;
        Self {
            data: vec![None; size],
            lower,
            upper,
        }
    }

    pub fn set(&mut self, idx: i32, name: String) {
        self.check_bounds(idx);
        self.data[(idx - self.lower) as usize] = Some(name);
    }

    pub fn get(&self, idx: i32) -> Option<&String> {
        self.check_bounds(idx);
        self.data[(idx - self.lower) as usize].as_ref()
    }

    #[inline]
    pub fn lower(&self) -> i32 {
        self.lower
    }

    #[inline]
    pub fn upper(&self) -> i32 {
        self.upper
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    fn check_bounds(&self, idx: i32) {
        assert!(
            idx >= self.lower && idx <= self.upper,
            "TColGeom2d_Array1OfBoundedCurve: index {idx} out of range [{}, {}]",
            self.lower,
            self.upper
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array1_curve_new_bounds() {
        let arr = ColGeom2d1dCurve::new(1, 4);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 4);
        assert_eq!(arr.size(), 4);
        assert!(!arr.is_empty());
    }

    #[test]
    fn array1_curve_set_get() {
        let mut arr = ColGeom2d1dCurve::new(1, 3);
        arr.set(1, "line_a".to_string());
        arr.set(3, "circle_b".to_string());
        assert_eq!(arr.get(1).unwrap(), "line_a");
        assert!(arr.get(2).is_none());
        assert_eq!(arr.get(3).unwrap(), "circle_b");
    }

    #[test]
    fn array1_curve_offset_lower() {
        let mut arr = ColGeom2d1dCurve::new(5, 7);
        arr.set(5, "spline".to_string());
        arr.set(7, "ellipse".to_string());
        assert_eq!(arr.lower(), 5);
        assert_eq!(arr.upper(), 7);
        assert_eq!(arr.size(), 3);
        assert_eq!(arr.get(5).unwrap(), "spline");
        assert_eq!(arr.get(7).unwrap(), "ellipse");
    }

    #[test]
    #[should_panic]
    fn array1_curve_out_of_bounds_panics() {
        let arr = ColGeom2d1dCurve::new(1, 3);
        let _ = arr.get(4);
    }

    #[test]
    fn seq_curve_append_length_get() {
        let mut seq = ColGeom2dSeqCurve::new();
        assert!(seq.is_empty());
        seq.append("line1".to_string());
        seq.append("circle1".to_string());
        seq.append("spline1".to_string());
        assert_eq!(seq.length(), 3);
        assert_eq!(seq.get(1), "line1");
        assert_eq!(seq.get(2), "circle1");
        assert_eq!(seq.get(3), "spline1");
    }

    #[test]
    fn seq_curve_prepend() {
        let mut seq = ColGeom2dSeqCurve::new();
        seq.append("second".to_string());
        seq.prepend("first".to_string());
        assert_eq!(seq.first(), "first");
        assert_eq!(seq.last(), "second");
        assert_eq!(seq.length(), 2);
    }

    #[test]
    fn seq_curve_remove_and_clear() {
        let mut seq = ColGeom2dSeqCurve::new();
        seq.append("a".to_string());
        seq.append("b".to_string());
        seq.append("c".to_string());
        seq.remove(2);
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.get(1), "a");
        assert_eq!(seq.get(2), "c");
        seq.clear();
        assert!(seq.is_empty());
    }

    #[test]
    fn seq_geom_append_and_get() {
        let mut seq = ColGeom2dSeqGeom::new();
        seq.append("geom_curve_a".to_string());
        seq.append("geom_curve_b".to_string());
        assert_eq!(seq.length(), 2);
        assert_eq!(seq.get(1), "geom_curve_a");
        assert_eq!(seq.last(), "geom_curve_b");
    }

    #[test]
    fn seq_geom_prepend_first_last() {
        let mut seq = ColGeom2dSeqGeom::new();
        seq.append("middle".to_string());
        seq.prepend("front".to_string());
        seq.append("back".to_string());
        assert_eq!(seq.first(), "front");
        assert_eq!(seq.last(), "back");
        assert_eq!(seq.length(), 3);
    }

    #[test]
    fn seq_geom_remove() {
        let mut seq = ColGeom2dSeqGeom::new();
        seq.append("x".to_string());
        seq.append("y".to_string());
        seq.append("z".to_string());
        seq.remove(1);
        assert_eq!(seq.first(), "y");
        assert_eq!(seq.length(), 2);
    }

    #[test]
    fn array1_bounded_curve_set_get() {
        let mut arr = ColGeom2d1dBoundedCurve::new(1, 5);
        arr.set(2, "arc".to_string());
        arr.set(4, "bezier".to_string());
        assert!(arr.get(1).is_none());
        assert_eq!(arr.get(2).unwrap(), "arc");
        assert!(arr.get(3).is_none());
        assert_eq!(arr.get(4).unwrap(), "bezier");
        assert!(arr.get(5).is_none());
    }

    #[test]
    fn array1_bounded_curve_bounds_and_size() {
        let arr = ColGeom2d1dBoundedCurve::new(0, 9);
        assert_eq!(arr.lower(), 0);
        assert_eq!(arr.upper(), 9);
        assert_eq!(arr.size(), 10);
        assert!(!arr.is_empty());
    }

    #[test]
    #[should_panic]
    fn array1_bounded_curve_out_of_bounds_panics() {
        let arr = ColGeom2d1dBoundedCurve::new(1, 3);
        let _ = arr.get(0);
    }
}
