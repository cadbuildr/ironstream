// FILE: b_rep_extrema_self_intersection.rs
// occt: BRepExtrema_SelfIntersection

/// Self-intersection detection tool
pub struct SelfIntersection;

impl SelfIntersection {
    pub fn new() -> Self {
        SelfIntersection
    }
}

impl Default for SelfIntersection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_intersection_creation() {
        let _tool = SelfIntersection::new();
    }
}
