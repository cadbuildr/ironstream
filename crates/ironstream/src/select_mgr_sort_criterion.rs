// FILE: select_mgr_sort_criterion.rs
// occt: SelectMgr_SortCriterion

/// Sort criterion for selecting entities during interactive selection by mouse click
#[derive(Clone, Debug)]
pub struct SelectMgrSortCriterion {
    /// 3D point of detection
    pub point: (f64, f64, f64),
    /// Surface normal or zero vector if undefined
    pub normal: (f32, f32, f32),
    /// Distance from the view plane to the entity
    pub depth: f64,
    /// Distance from the clicked point to the entity on the view plane
    pub min_dist: f64,
    /// Tolerance used for selecting candidates
    pub tolerance: f64,
    /// Selection priority
    pub selection_priority: i32,
    /// Display priority
    pub display_priority: i32,
    /// ZLayer rendering order index, stronger than depth
    pub z_layer_position: i32,
    /// Overall number of entities collected for the same owner
    pub nb_owner_matches: i32,
    /// Flag to signal comparison to be done over priority
    pub is_prefer_priority: bool,
}

impl SelectMgrSortCriterion {
    /// Empty constructor
    pub fn new() -> Self {
        SelectMgrSortCriterion {
            point: (0.0, 0.0, 0.0),
            normal: (0.0, 0.0, 0.0),
            depth: 0.0,
            min_dist: 0.0,
            tolerance: 0.0,
            selection_priority: 0,
            display_priority: 0,
            z_layer_position: 0,
            nb_owner_matches: 0,
            is_prefer_priority: false,
        }
    }

    /// Compare with another item by depth, priority and minDist
    pub fn is_closer_depth(&self, other: &SelectMgrSortCriterion) -> bool {
        // The object within different ZLayer groups cannot be compared by depth
        if self.z_layer_position != other.z_layer_position {
            return self.z_layer_position > other.z_layer_position;
        }

        // Closest object is selected if their depths are not equal within tolerance
        if (self.depth - other.depth).abs() > self.tolerance + other.tolerance {
            return self.depth < other.depth;
        }

        let a_cos = self.compute_normal_angle(other);

        let a_depth = self.depth - self.tolerance;
        let an_other_depth = other.depth - other.tolerance;

        // Comparison depths taking into account tolerances occurs when the surfaces are parallel
        // or have the same sensitivity and the angle between them is less than 60 degrees.
        const CONFUSION: f64 = 1e-7;
        if (a_depth - an_other_depth).abs() > CONFUSION {
            if (a_cos > 0.5 && (self.tolerance - other.tolerance).abs() < CONFUSION)
                || (a_cos - 1.0).abs() < CONFUSION
            {
                return a_depth < an_other_depth;
            }
        }

        // If two objects have similar depth, select the one with higher priority
        if self.selection_priority > other.selection_priority {
            return true;
        }

        if self.display_priority > other.display_priority {
            return true;
        }

        // If priorities are equal, one closest to the mouse
        self.selection_priority == other.selection_priority && self.min_dist < other.min_dist
    }

    /// Compare with another item using old logic (OCCT version <= 6.3.1)
    /// with priority considered preceding depth
    pub fn is_higher_priority(&self, other: &SelectMgrSortCriterion) -> bool {
        // The object within different ZLayer groups cannot be compared by depth
        if self.z_layer_position != other.z_layer_position {
            return self.z_layer_position > other.z_layer_position;
        }

        if self.selection_priority != other.selection_priority {
            return self.selection_priority > other.selection_priority;
        }

        if self.display_priority != other.display_priority {
            return self.display_priority > other.display_priority;
        }

        const CONFUSION: f64 = 1e-7;
        if (self.depth - other.depth).abs() <= CONFUSION {
            return self.min_dist < other.min_dist;
        }

        self.depth < other.depth
    }

    /// Helper: compute cosine of angle between normals
    fn compute_normal_angle(&self, other: &SelectMgrSortCriterion) -> f64 {
        let nx1 = f64::from(self.normal.0);
        let ny1 = f64::from(self.normal.1);
        let nz1 = f64::from(self.normal.2);
        let nx2 = f64::from(other.normal.0);
        let ny2 = f64::from(other.normal.1);
        let nz2 = f64::from(other.normal.2);

        let mod1_sq = nx1 * nx1 + ny1 * ny1 + nz1 * nz1;
        let mod2_sq = nx2 * nx2 + ny2 * ny2 + nz2 * nz2;

        if mod1_sq <= 1e-10 || mod2_sq <= 1e-10 {
            return 1.0;
        }

        let dot = nx1 * nx2 + ny1 * ny2 + nz1 * nz2;
        let mod1 = mod1_sq.sqrt();
        let mod2 = mod2_sq.sqrt();
        let cos_angle = dot / (mod1 * mod2);
        cos_angle.abs()
    }
}

impl Default for SelectMgrSortCriterion {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_criterion() {
        let crit = SelectMgrSortCriterion::new();
        assert_eq!(crit.point, (0.0, 0.0, 0.0));
        assert_eq!(crit.depth, 0.0);
        assert_eq!(crit.selection_priority, 0);
        assert!(!crit.is_prefer_priority);
    }

    #[test]
    fn test_is_closer_depth_simple() {
        let mut crit1 = SelectMgrSortCriterion::new();
        crit1.depth = 10.0;
        crit1.z_layer_position = 0;

        let mut crit2 = SelectMgrSortCriterion::new();
        crit2.depth = 20.0;
        crit2.z_layer_position = 0;

        assert!(crit1.is_closer_depth(&crit2));
        assert!(!crit2.is_closer_depth(&crit1));
    }

    #[test]
    fn test_z_layer_comparison() {
        let mut crit1 = SelectMgrSortCriterion::new();
        crit1.z_layer_position = 1;
        crit1.depth = 100.0;

        let mut crit2 = SelectMgrSortCriterion::new();
        crit2.z_layer_position = 0;
        crit2.depth = 1.0;

        assert!(crit1.is_closer_depth(&crit2));
    }

    #[test]
    fn test_priority_comparison() {
        let mut crit1 = SelectMgrSortCriterion::new();
        crit1.depth = 10.0;
        crit1.selection_priority = 5;
        crit1.z_layer_position = 0;

        let mut crit2 = SelectMgrSortCriterion::new();
        crit2.depth = 10.0;
        crit2.selection_priority = 3;
        crit2.z_layer_position = 0;

        assert!(crit1.is_closer_depth(&crit2));
    }

    #[test]
    fn test_min_dist_comparison() {
        let mut crit1 = SelectMgrSortCriterion::new();
        crit1.depth = 10.0;
        crit1.selection_priority = 0;
        crit1.display_priority = 0;
        crit1.min_dist = 5.0;
        crit1.z_layer_position = 0;

        let mut crit2 = SelectMgrSortCriterion::new();
        crit2.depth = 10.0;
        crit2.selection_priority = 0;
        crit2.display_priority = 0;
        crit2.min_dist = 10.0;
        crit2.z_layer_position = 0;

        assert!(crit1.is_closer_depth(&crit2));
    }

    #[test]
    fn test_is_higher_priority() {
        let mut crit1 = SelectMgrSortCriterion::new();
        crit1.selection_priority = 5;
        crit1.z_layer_position = 0;

        let mut crit2 = SelectMgrSortCriterion::new();
        crit2.selection_priority = 3;
        crit2.z_layer_position = 0;

        assert!(crit1.is_higher_priority(&crit2));
        assert!(!crit2.is_higher_priority(&crit1));
    }

    #[test]
    fn test_normal_angle_zero() {
        let crit1 = SelectMgrSortCriterion::new();
        let crit2 = SelectMgrSortCriterion::new();
        let angle = crit1.compute_normal_angle(&crit2);
        assert_eq!(angle, 1.0);
    }

    #[test]
    fn test_normal_angle_perpendicular() {
        let mut crit1 = SelectMgrSortCriterion::new();
        crit1.normal = (1.0, 0.0, 0.0);

        let mut crit2 = SelectMgrSortCriterion::new();
        crit2.normal = (0.0, 1.0, 0.0);

        let angle = crit1.compute_normal_angle(&crit2);
        assert!((angle - 0.0).abs() < 1e-6);
    }
}
