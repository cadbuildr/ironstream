// FILE: top_ope_b_rep_ds_point_data.rs
// occt: TopOpeBRepDS_PointData

use crate::top_ope_b_rep_ds_point::Point;
use crate::top_ope_b_rep_ds_geometry_data::GeometryData;

/// Point data: extends GeometryData with a Point and associated shape indices
#[derive(Debug, Clone)]
pub struct PointData {
    /// Base geometry data containing interferences
    geometry_data: GeometryData,
    /// The point
    point: Point,
    /// Associated shape index 1
    s1: i32,
    /// Associated shape index 2
    s2: i32,
}

impl PointData {
    /// Create new PointData
    pub fn new() -> Self {
        PointData {
            geometry_data: GeometryData::new(),
            point: Point::new(),
            s1: -1,
            s2: -1,
        }
    }

    /// Create from a Point
    pub fn from_point(point: Point) -> Self {
        PointData {
            geometry_data: GeometryData::new(),
            point,
            s1: -1,
            s2: -1,
        }
    }

    /// Create from Point with shape indices
    pub fn from_point_and_shapes(point: Point, i1: i32, i2: i32) -> Self {
        PointData {
            geometry_data: GeometryData::new(),
            point,
            s1: i1,
            s2: i2,
        }
    }

    /// Set shapes
    pub fn set_shapes(&mut self, i1: i32, i2: i32) {
        self.s1 = i1;
        self.s2 = i2;
    }

    /// Get shapes
    pub fn get_shapes(&self) -> (i32, i32) {
        (self.s1, self.s2)
    }

    /// Get geometry data
    pub fn geometry_data(&self) -> &GeometryData {
        &self.geometry_data
    }

    /// Get mutable geometry data
    pub fn geometry_data_mut(&mut self) -> &mut GeometryData {
        &mut self.geometry_data
    }

    /// Get point
    pub fn point(&self) -> &Point {
        &self.point
    }

    /// Get mutable point
    pub fn point_mut(&mut self) -> &mut Point {
        &mut self.point
    }
}

impl Default for PointData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_data_new() {
        let pd = PointData::new();
        assert_eq!(pd.geometry_data().interferences().len(), 0);
        assert_eq!(pd.point().tolerance(), 0.0);
        let (s1, s2) = pd.get_shapes();
        assert_eq!(s1, -1);
        assert_eq!(s2, -1);
    }

    #[test]
    fn test_point_data_from_point() {
        let p = Point::from_coords(1.0, 2.0, 3.0, 0.5);
        let pd = PointData::from_point(p);
        assert_eq!(pd.point().x(), 1.0);
        assert_eq!(pd.point().tolerance(), 0.5);
    }

    #[test]
    fn test_point_data_from_point_and_shapes() {
        let p = Point::from_coords(1.0, 2.0, 3.0, 0.5);
        let pd = PointData::from_point_and_shapes(p, 5, 10);
        let (s1, s2) = pd.get_shapes();
        assert_eq!(s1, 5);
        assert_eq!(s2, 10);
    }

    #[test]
    fn test_point_data_set_shapes() {
        let mut pd = PointData::new();
        pd.set_shapes(7, 14);
        let (s1, s2) = pd.get_shapes();
        assert_eq!(s1, 7);
        assert_eq!(s2, 14);
    }

    #[test]
    fn test_point_data_mut() {
        let mut pd = PointData::new();
        pd.geometry_data_mut().add_interference(1);
        pd.point_mut().set_tolerance(0.75);
        assert_eq!(pd.geometry_data().interferences().len(), 1);
        assert_eq!(pd.point().tolerance(), 0.75);
    }
}
