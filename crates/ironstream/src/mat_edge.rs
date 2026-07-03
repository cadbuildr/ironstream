// FILE: mat_edge.rs
// occt: MAT_Edge

/// Edge of a MAT graph connecting two bisectors.
#[derive(Debug, Clone)]
pub struct MatEdge {
    edge_number: i32,
    distance: f64,
    intersection_point: i32,
}

impl MatEdge {
    pub fn new() -> Self {
        MatEdge {
            edge_number: 0,
            distance: 0.0,
            intersection_point: 0,
        }
    }

    pub fn edge_number(&self) -> i32 {
        self.edge_number
    }

    pub fn set_edge_number(&mut self, num: i32) {
        self.edge_number = num;
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn set_distance(&mut self, dist: f64) {
        self.distance = dist;
    }
}

impl Default for MatEdge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge() {
        let mut e = MatEdge::new();
        e.set_edge_number(5);
        assert_eq!(e.edge_number(), 5);
    }
}
