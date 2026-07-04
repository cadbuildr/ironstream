// FILE: step_visual_view_volume.rs
// occt: StepVisual_ViewVolume

/// Represents a STEP ViewVolume entity.
pub struct ViewVolume {
    projection_type: CentralOrParallel,
    projection_point: CartesianPoint,
    view_plane_distance: f64,
    front_plane_distance: f64,
    front_plane_clipping: bool,
    back_plane_distance: f64,
    back_plane_clipping: bool,
    view_volume_sides_clipping: bool,
    view_window: PlanarBox,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CentralOrParallel {
    Central,
    Parallel,
}

pub struct CartesianPoint;
pub struct PlanarBox;

impl ViewVolume {
    /// Creates a new view volume.
    pub fn new() -> Self {
        ViewVolume {
            projection_type: CentralOrParallel::Central,
            projection_point: CartesianPoint,
            view_plane_distance: 0.0,
            front_plane_distance: 0.0,
            front_plane_clipping: false,
            back_plane_distance: 0.0,
            back_plane_clipping: false,
            view_volume_sides_clipping: false,
            view_window: PlanarBox,
        }
    }

    /// Initializes all fields.
    pub fn init(
        &mut self,
        projection_type: CentralOrParallel,
        projection_point: CartesianPoint,
        view_plane_distance: f64,
        front_plane_distance: f64,
        front_plane_clipping: bool,
        back_plane_distance: f64,
        back_plane_clipping: bool,
        view_volume_sides_clipping: bool,
        view_window: PlanarBox,
    ) {
        self.projection_type = projection_type;
        self.projection_point = projection_point;
        self.view_plane_distance = view_plane_distance;
        self.front_plane_distance = front_plane_distance;
        self.front_plane_clipping = front_plane_clipping;
        self.back_plane_distance = back_plane_distance;
        self.back_plane_clipping = back_plane_clipping;
        self.view_volume_sides_clipping = view_volume_sides_clipping;
        self.view_window = view_window;
    }

    pub fn projection_type(&self) -> CentralOrParallel {
        self.projection_type
    }

    pub fn set_projection_type(&mut self, pt: CentralOrParallel) {
        self.projection_type = pt;
    }

    pub fn view_plane_distance(&self) -> f64 {
        self.view_plane_distance
    }

    pub fn set_view_plane_distance(&mut self, dist: f64) {
        self.view_plane_distance = dist;
    }

    pub fn front_plane_distance(&self) -> f64 {
        self.front_plane_distance
    }

    pub fn set_front_plane_distance(&mut self, dist: f64) {
        self.front_plane_distance = dist;
    }

    pub fn front_plane_clipping(&self) -> bool {
        self.front_plane_clipping
    }

    pub fn set_front_plane_clipping(&mut self, clip: bool) {
        self.front_plane_clipping = clip;
    }

    pub fn back_plane_distance(&self) -> f64 {
        self.back_plane_distance
    }

    pub fn set_back_plane_distance(&mut self, dist: f64) {
        self.back_plane_distance = dist;
    }

    pub fn back_plane_clipping(&self) -> bool {
        self.back_plane_clipping
    }

    pub fn set_back_plane_clipping(&mut self, clip: bool) {
        self.back_plane_clipping = clip;
    }

    pub fn view_volume_sides_clipping(&self) -> bool {
        self.view_volume_sides_clipping
    }

    pub fn set_view_volume_sides_clipping(&mut self, clip: bool) {
        self.view_volume_sides_clipping = clip;
    }
}

impl Default for ViewVolume {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vv = ViewVolume::new();
        assert_eq!(vv.projection_type(), CentralOrParallel::Central);
        assert_eq!(vv.view_plane_distance(), 0.0);
    }

    #[test]
    fn test_distances() {
        let mut vv = ViewVolume::new();
        vv.set_view_plane_distance(10.0);
        vv.set_front_plane_distance(5.0);
        vv.set_back_plane_distance(20.0);
        assert_eq!(vv.view_plane_distance(), 10.0);
        assert_eq!(vv.front_plane_distance(), 5.0);
        assert_eq!(vv.back_plane_distance(), 20.0);
    }

    #[test]
    fn test_clipping() {
        let mut vv = ViewVolume::new();
        vv.set_front_plane_clipping(true);
        vv.set_back_plane_clipping(true);
        assert!(vv.front_plane_clipping());
        assert!(vv.back_plane_clipping());
    }
}
