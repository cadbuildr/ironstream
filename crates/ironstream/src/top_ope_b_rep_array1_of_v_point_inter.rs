// FILE: top_ope_b_rep_array1_of_v_point_inter.rs
// occt: TopOpeBRep_Array1OfVPointInter
// occt-ref: TopOpeBRep_VPointInter

/// VPointInter: Vertex point intersection data.
#[derive(Clone, Debug)]
pub struct VPointInter {
    id: usize,
    parameter: f64,
    vertex_id: usize,
}

impl VPointInter {
    pub fn new(id: usize, parameter: f64, vertex_id: usize) -> Self {
        VPointInter {
            id,
            parameter,
            vertex_id,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    pub fn vertex_id(&self) -> usize {
        self.vertex_id
    }
}

impl Default for VPointInter {
    fn default() -> Self {
        VPointInter::new(0, 0.0, 0)
    }
}

/// Array1OfVPointInter: 1-based array.
#[derive(Clone, Debug)]
pub struct Array1OfVPointInter {
    data: Vec<VPointInter>,
    lower: usize,
}

impl Array1OfVPointInter {
    pub fn new(size: usize) -> Self {
        Array1OfVPointInter {
            data: (0..size).map(|i| VPointInter::new(i, 0.0, i)).collect(),
            lower: 1,
        }
    }

    pub fn new_from_bounds(lower: usize, upper: usize) -> Self {
        if lower == 0 {
            panic!("OCCT arrays use 1-based indexing");
        }
        let size = upper - lower + 1;
        Array1OfVPointInter {
            data: (0..size).map(|i| VPointInter::new(i, 0.0, i)).collect(),
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

    pub fn value(&self, index_1based: usize) -> Option<&VPointInter> {
        if index_1based < self.lower {
            None
        } else {
            self.data.get(index_1based - self.lower)
        }
    }

    pub fn set_value(&mut self, index_1based: usize, value: VPointInter) {
        let idx = index_1based - self.lower;
        if idx >= self.data.len() {
            panic!("Index out of bounds");
        }
        self.data[idx] = value;
    }

    pub fn iter(&self) -> impl Iterator<Item = &VPointInter> {
        self.data.iter()
    }
}

impl Default for Array1OfVPointInter {
    fn default() -> Self {
        Self::new(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v_point_inter() {
        let vpt = VPointInter::new(1, 0.5, 10);
        assert_eq!(vpt.id(), 1);
        assert_eq!(vpt.parameter(), 0.5);
        assert_eq!(vpt.vertex_id(), 10);
    }

    #[test]
    fn test_array1_new() {
        let arr = Array1OfVPointInter::new(5);
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn test_array1_value() {
        let arr = Array1OfVPointInter::new(3);
        assert!(arr.value(1).is_some());
    }
}
