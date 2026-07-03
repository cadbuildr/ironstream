// FILE: ais_xr_tracked_device.rs
// occt: AIS_XRTrackedDevice

#[derive(Clone, Debug)]
pub struct AisXrTrackedDevice {
    pub object_id: u32,
    pub device_role: u32,
    pub is_visible: bool,
    pub laser_length: f32,
}

impl AisXrTrackedDevice {
    pub fn new(object_id: u32, device_role: u32) -> Self {
        Self { object_id, device_role, is_visible: true, laser_length: 1.0 }
    }
    pub fn set_visible(&mut self, v: bool) { self.is_visible = v; }
    pub fn set_laser_length(&mut self, len: f32) { self.laser_length = len.max(0.0); }
}

impl Default for AisXrTrackedDevice {
    fn default() -> Self { Self::new(0, 0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let dev = AisXrTrackedDevice::new(1, 2);
        assert_eq!(dev.object_id, 1);
    }
}
