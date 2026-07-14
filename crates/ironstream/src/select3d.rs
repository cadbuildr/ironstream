// FILE: select3d.rs

// occt: Select3D_TypeOfSensitivity
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Select3dTypeOfSensitivity {
    Interior,
    Boundary,
    Any,
}

pub trait Select3dSensitiveEntity: std::fmt::Debug {
    fn sensitivity_type(&self) -> Select3dTypeOfSensitivity;
    fn nb_sub_elements(&self) -> usize;
}

// occt: Select3D_SensitivePoint
#[derive(Clone, Copy, Debug)]
pub struct Select3dSensitivePoint {
    pub point: [f64; 3],
}

impl Select3dSensitivePoint {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { point: [x, y, z] }
    }
}

impl Select3dSensitiveEntity for Select3dSensitivePoint {
    fn sensitivity_type(&self) -> Select3dTypeOfSensitivity {
        Select3dTypeOfSensitivity::Interior
    }

    fn nb_sub_elements(&self) -> usize {
        1
    }
}

// occt-ref: Select3D_SensitiveBox
#[derive(Clone, Copy, Debug)]
pub struct Select3dSensitiveBox {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

impl Select3dSensitiveBox {
    pub fn new(min: [f64; 3], max: [f64; 3]) -> Self {
        Self { min, max }
    }

    pub fn contains_point(&self, p: [f64; 3]) -> bool {
        p[0] >= self.min[0]
            && p[0] <= self.max[0]
            && p[1] >= self.min[1]
            && p[1] <= self.max[1]
            && p[2] >= self.min[2]
            && p[2] <= self.max[2]
    }
}

impl Select3dSensitiveEntity for Select3dSensitiveBox {
    fn sensitivity_type(&self) -> Select3dTypeOfSensitivity {
        Select3dTypeOfSensitivity::Any
    }

    fn nb_sub_elements(&self) -> usize {
        6
    }
}

// occt-ref: Select3D_SensitiveSphere
#[derive(Clone, Copy, Debug)]
pub struct Select3dSensitiveSphere {
    pub center: [f64; 3],
    pub radius: f64,
}

impl Select3dSensitiveSphere {
    pub fn new(center: [f64; 3], radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn contains_point(&self, p: [f64; 3]) -> bool {
        let dx = p[0] - self.center[0];
        let dy = p[1] - self.center[1];
        let dz = p[2] - self.center[2];
        dx * dx + dy * dy + dz * dz <= self.radius * self.radius
    }
}

impl Select3dSensitiveEntity for Select3dSensitiveSphere {
    fn sensitivity_type(&self) -> Select3dTypeOfSensitivity {
        Select3dTypeOfSensitivity::Boundary
    }

    fn nb_sub_elements(&self) -> usize {
        1
    }
}

// occt-ref: SelectMgr_SelectionManager
pub struct SelectMgrSelectionManager {
    entities: Vec<Box<dyn Select3dSensitiveEntity>>,
}

impl SelectMgrSelectionManager {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn add(&mut self, e: Box<dyn Select3dSensitiveEntity>) {
        self.entities.push(e);
    }

    pub fn count(&self) -> usize {
        self.entities.len()
    }
}

impl Default for SelectMgrSelectionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensitive_point_fields() {
        let p = Select3dSensitivePoint::new(1.0, 2.0, 3.0);
        assert_eq!(p.point, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_sensitive_point_trait() {
        let p = Select3dSensitivePoint::new(0.0, 0.0, 0.0);
        assert_eq!(p.sensitivity_type(), Select3dTypeOfSensitivity::Interior);
        assert_eq!(p.nb_sub_elements(), 1);
    }

    #[test]
    fn test_sensitive_box_contains_inside() {
        let b = Select3dSensitiveBox::new([0.0, 0.0, 0.0], [10.0, 10.0, 10.0]);
        assert!(b.contains_point([5.0, 5.0, 5.0]));
    }

    #[test]
    fn test_sensitive_box_contains_outside() {
        let b = Select3dSensitiveBox::new([0.0, 0.0, 0.0], [10.0, 10.0, 10.0]);
        assert!(!b.contains_point([11.0, 5.0, 5.0]));
    }

    #[test]
    fn test_sensitive_box_contains_on_boundary() {
        let b = Select3dSensitiveBox::new([0.0, 0.0, 0.0], [10.0, 10.0, 10.0]);
        assert!(b.contains_point([0.0, 0.0, 0.0]));
        assert!(b.contains_point([10.0, 10.0, 10.0]));
    }

    #[test]
    fn test_sensitive_box_trait() {
        let b = Select3dSensitiveBox::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        assert_eq!(b.sensitivity_type(), Select3dTypeOfSensitivity::Any);
        assert_eq!(b.nb_sub_elements(), 6);
    }

    #[test]
    fn test_sensitive_sphere_contains_inside() {
        let s = Select3dSensitiveSphere::new([0.0, 0.0, 0.0], 5.0);
        assert!(s.contains_point([3.0, 4.0, 0.0]));
    }

    #[test]
    fn test_sensitive_sphere_contains_outside() {
        let s = Select3dSensitiveSphere::new([0.0, 0.0, 0.0], 5.0);
        assert!(!s.contains_point([3.0, 4.0, 1.0]));
    }

    #[test]
    fn test_sensitive_sphere_on_surface() {
        let s = Select3dSensitiveSphere::new([0.0, 0.0, 0.0], 5.0);
        assert!(s.contains_point([5.0, 0.0, 0.0]));
    }

    #[test]
    fn test_sensitive_sphere_trait() {
        let s = Select3dSensitiveSphere::new([0.0, 0.0, 0.0], 1.0);
        assert_eq!(s.sensitivity_type(), Select3dTypeOfSensitivity::Boundary);
        assert_eq!(s.nb_sub_elements(), 1);
    }

    #[test]
    fn test_selection_manager_empty() {
        let mgr = SelectMgrSelectionManager::new();
        assert_eq!(mgr.count(), 0);
    }

    #[test]
    fn test_selection_manager_add() {
        let mut mgr = SelectMgrSelectionManager::new();
        mgr.add(Box::new(Select3dSensitivePoint::new(0.0, 0.0, 0.0)));
        mgr.add(Box::new(Select3dSensitiveBox::new(
            [0.0, 0.0, 0.0],
            [1.0, 1.0, 1.0],
        )));
        mgr.add(Box::new(Select3dSensitiveSphere::new([0.0, 0.0, 0.0], 1.0)));
        assert_eq!(mgr.count(), 3);
    }

    #[test]
    fn test_type_of_sensitivity_eq() {
        assert_eq!(
            Select3dTypeOfSensitivity::Interior,
            Select3dTypeOfSensitivity::Interior
        );
        assert_ne!(
            Select3dTypeOfSensitivity::Interior,
            Select3dTypeOfSensitivity::Boundary
        );
    }
}
