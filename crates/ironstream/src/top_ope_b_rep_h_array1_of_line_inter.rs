// FILE: top_ope_b_rep_h_array1_of_line_inter.rs
// occt: TopOpeBRep_HArray1OfLineInter

use std::sync::Arc;

/// LineInter: Line of intersection.
#[derive(Clone, Debug)]
pub struct LineInter {
    id: usize,
}

impl LineInter {
    pub fn new(id: usize) -> Self {
        LineInter { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// HArray1Content: Inner content for handle-based array.
struct HArray1Content {
    data: Vec<LineInter>,
    lower: usize,
}

/// HArray1OfLineInter: Handle (Arc-based) 1-based array.
#[derive(Clone)]
pub struct HArray1OfLineInter {
    inner: Arc<HArray1Content>,
}

impl HArray1OfLineInter {
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower == 0 {
            panic!("OCCT arrays use 1-based indexing");
        }
        let size = upper - lower + 1;
        HArray1OfLineInter {
            inner: Arc::new(HArray1Content {
                data: (0..size).map(|i| LineInter::new(i)).collect(),
                lower,
            }),
        }
    }

    pub fn lower(&self) -> usize {
        self.inner.lower
    }

    pub fn upper(&self) -> usize {
        self.inner.lower + self.inner.data.len() - 1
    }

    pub fn length(&self) -> usize {
        self.inner.data.len()
    }

    pub fn value(&self, index_1based: usize) -> Option<LineInter> {
        if index_1based < self.lower() {
            None
        } else {
            self.inner.data.get(index_1based - self.lower()).cloned()
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &LineInter> {
        self.inner.data.iter()
    }
}

impl std::fmt::Debug for HArray1OfLineInter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HArray1OfLineInter")
            .field("lower", &self.lower())
            .field("upper", &self.upper())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_inter() {
        let line = LineInter::new(42);
        assert_eq!(line.id(), 42);
    }

    #[test]
    fn test_h_array1_new() {
        let arr = HArray1OfLineInter::new(1, 10);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 10);
        assert_eq!(arr.length(), 10);
    }

    #[test]
    fn test_h_array1_value() {
        let arr = HArray1OfLineInter::new(1, 3);
        assert!(arr.value(1).is_some());
        assert!(arr.value(0).is_none());
    }
}
