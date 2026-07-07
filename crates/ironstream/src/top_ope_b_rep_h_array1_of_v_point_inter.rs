// FILE: top_ope_b_rep_h_array1_of_v_point_inter.rs
// occt: TopOpeBRep_HArray1OfVPointInter

use std::sync::Arc;

/// VPointInter: Vertex point intersection.
#[derive(Clone, Debug)]
pub struct VPointInter {
    id: usize,
    param: f64,
}

impl VPointInter {
    pub fn new(id: usize, param: f64) -> Self {
        VPointInter { id, param }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn param(&self) -> f64 {
        self.param
    }
}

struct HArray1Content {
    data: Vec<VPointInter>,
    lower: usize,
}

/// HArray1OfVPointInter: Handle-based 1-based array.
#[derive(Clone)]
pub struct HArray1OfVPointInter {
    inner: Arc<HArray1Content>,
}

impl HArray1OfVPointInter {
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower == 0 {
            panic!("OCCT arrays use 1-based indexing");
        }
        let size = upper - lower + 1;
        HArray1OfVPointInter {
            inner: Arc::new(HArray1Content {
                data: (0..size)
                    .map(|i| VPointInter::new(i, 0.0))
                    .collect(),
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

    pub fn value(&self, index_1based: usize) -> Option<VPointInter> {
        if index_1based < self.lower() {
            None
        } else {
            self.inner.data.get(index_1based - self.lower()).cloned()
        }
    }
}

impl std::fmt::Debug for HArray1OfVPointInter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HArray1OfVPointInter")
            .field("lower", &self.lower())
            .field("upper", &self.upper())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v_point_inter() {
        let vpt = VPointInter::new(1, 0.5);
        assert_eq!(vpt.id(), 1);
        assert_eq!(vpt.param(), 0.5);
    }

    #[test]
    fn test_h_array1_new() {
        let arr = HArray1OfVPointInter::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
    }

    #[test]
    fn test_h_array1_value() {
        let arr = HArray1OfVPointInter::new(1, 3);
        assert!(arr.value(1).is_some());
    }
}
