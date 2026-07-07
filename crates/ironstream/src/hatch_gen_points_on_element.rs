// FILE: hatch_gen_points_on_element.rs
// occt: HatchGen_PointsOnElement

//! Deprecated: Use Vec<PointOnElement> directly.
//! Points on geometric elements for hatch generation.

#[derive(Clone, Debug)]
pub struct PointOnElement {
    pub point_id: usize,
    pub parameter: f64,
    pub state: PointState,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PointState {
    Unknown,
    Visible,
    Hidden,
}

impl PointOnElement {
    pub fn new(point_id: usize, parameter: f64) -> Self {
        PointOnElement {
            point_id,
            parameter,
            state: PointState::Unknown,
        }
    }

    pub fn set_visible(&mut self) {
        self.state = PointState::Visible;
    }

    pub fn set_hidden(&mut self) {
        self.state = PointState::Hidden;
    }

    pub fn is_visible(&self) -> bool {
        self.state == PointState::Visible
    }
}

pub type HatchGenPointsOnElement = Vec<PointOnElement>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let point = PointOnElement::new(1, 0.5);
        assert_eq!(point.point_id, 1);
        assert_eq!(point.parameter, 0.5);
        assert_eq!(point.state, PointState::Unknown);
    }

    #[test]
    fn test_set_visible() {
        let mut point = PointOnElement::new(1, 0.5);
        point.set_visible();
        assert!(point.is_visible());
        assert_eq!(point.state, PointState::Visible);
    }

    #[test]
    fn test_set_hidden() {
        let mut point = PointOnElement::new(1, 0.5);
        point.set_hidden();
        assert!(!point.is_visible());
        assert_eq!(point.state, PointState::Hidden);
    }

    #[test]
    fn test_points_vector() {
        let mut points: HatchGenPointsOnElement = Vec::new();
        let mut p1 = PointOnElement::new(1, 0.25);
        p1.set_visible();
        points.push(p1);

        let mut p2 = PointOnElement::new(2, 0.75);
        p2.set_hidden();
        points.push(p2);

        assert_eq!(points.len(), 2);
        assert!(points[0].is_visible());
        assert!(!points[1].is_visible());
    }

    #[test]
    fn test_points_iteration() {
        let mut points = vec![
            PointOnElement::new(1, 0.0),
            PointOnElement::new(2, 0.5),
            PointOnElement::new(3, 1.0),
        ];

        for point in &mut points {
            if point.parameter > 0.25 {
                point.set_visible();
            }
        }

        let visible_count = points.iter().filter(|p| p.is_visible()).count();
        assert_eq!(visible_count, 2);
    }
}
