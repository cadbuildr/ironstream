// FILE: ais_animation.rs
// occt: AIS_Animation, AIS_AnimationCamera, AIS_AnimationObject

// occt: AIS_Animation
/// Abstract animation base — defines a time-keyed sequence.
#[derive(Clone, Debug)]
pub struct Animation {
    pub name: String,
    pub start_time: f64,
    pub duration: f64,
    pub children: Vec<String>,
}

impl Animation {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), start_time: 0.0, duration: 0.0, children: Vec::new() }
    }

    pub fn set_start_time(&mut self, t: f64) { self.start_time = t; }
    pub fn set_duration(&mut self, d: f64) { self.duration = d.max(0.0); }
    pub fn end_time(&self) -> f64 { self.start_time + self.duration }

    pub fn add_child(&mut self, name: impl Into<String>) {
        self.children.push(name.into());
    }

    pub fn update(&self, pts: f64) -> f64 {
        if self.duration <= 0.0 { return 0.0; }
        ((pts - self.start_time) / self.duration).clamp(0.0, 1.0)
    }
}

// occt: AIS_AnimationCamera
/// Animates a camera between two poses over time.
#[derive(Clone, Debug)]
pub struct AnimationCamera {
    pub base: Animation,
    pub eye_from: [f64; 3],
    pub eye_to: [f64; 3],
    pub center_from: [f64; 3],
    pub center_to: [f64; 3],
    pub up_from: [f64; 3],
    pub up_to: [f64; 3],
}

impl AnimationCamera {
    pub fn new(
        name: impl Into<String>,
        eye_from: [f64; 3], eye_to: [f64; 3],
        center_from: [f64; 3], center_to: [f64; 3],
        up_from: [f64; 3], up_to: [f64; 3],
    ) -> Self {
        Self {
            base: Animation::new(name),
            eye_from, eye_to, center_from, center_to, up_from, up_to,
        }
    }

    /// Interpolated eye position at pts.
    pub fn eye_at(&self, pts: f64) -> [f64; 3] {
        let t = self.base.update(pts);
        lerp3(self.eye_from, self.eye_to, t)
    }

    /// Interpolated center position at pts.
    pub fn center_at(&self, pts: f64) -> [f64; 3] {
        let t = self.base.update(pts);
        lerp3(self.center_from, self.center_to, t)
    }
}

fn lerp3(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    [a[0] + (b[0]-a[0])*t, a[1] + (b[1]-a[1])*t, a[2] + (b[2]-a[2])*t]
}

// occt: AIS_AnimationObject
/// Animates an AIS object (shape) through a series of transforms.
#[derive(Clone, Debug)]
pub struct AnimationObject {
    pub base: Animation,
    /// Keyframes: (time, translation).
    pub keyframes: Vec<(f64, [f64; 3])>,
}

impl AnimationObject {
    pub fn new(name: impl Into<String>) -> Self {
        Self { base: Animation::new(name), keyframes: Vec::new() }
    }

    pub fn add_keyframe(&mut self, time: f64, translation: [f64; 3]) {
        self.keyframes.push((time, translation));
        self.keyframes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    }

    pub fn translation_at(&self, pts: f64) -> [f64; 3] {
        if self.keyframes.is_empty() { return [0.0; 3]; }
        let i = self.keyframes.partition_point(|k| k.0 <= pts).saturating_sub(1);
        if i + 1 >= self.keyframes.len() { return self.keyframes.last().unwrap().1; }
        let (t0, p0) = self.keyframes[i];
        let (t1, p1) = self.keyframes[i+1];
        let dt = t1 - t0;
        if dt.abs() < 1e-14 { return p0; }
        let frac = (pts - t0) / dt;
        lerp3(p0, p1, frac)
    }
}

// occt: AIS_AnimationTimer
/// Simple elapsed-time tracker for animations.
#[derive(Clone, Debug)]
pub struct AnimationTimer {
    pub elapsed: f64,
    pub speed: f64,
    pub is_playing: bool,
}

impl AnimationTimer {
    pub fn new() -> Self { Self { elapsed: 0.0, speed: 1.0, is_playing: false } }
    pub fn play(&mut self) { self.is_playing = true; }
    pub fn pause(&mut self) { self.is_playing = false; }
    pub fn stop(&mut self) { self.is_playing = false; self.elapsed = 0.0; }
    pub fn advance(&mut self, dt: f64) {
        if self.is_playing { self.elapsed += dt * self.speed; }
    }
}

impl Default for AnimationTimer {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animation_update_midpoint() {
        let mut a = Animation::new("test");
        a.set_start_time(0.0);
        a.set_duration(2.0);
        assert!((a.update(1.0) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn camera_animation_eye_at() {
        let mut cam = AnimationCamera::new(
            "cam",
            [0.0, 0.0, 10.0], [10.0, 0.0, 10.0],
            [0.0, 0.0, 0.0], [10.0, 0.0, 0.0],
            [0.0, 1.0, 0.0], [0.0, 1.0, 0.0],
        );
        cam.base.set_duration(1.0);
        let eye = cam.eye_at(0.5);
        assert!((eye[0] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn animation_object_keyframes() {
        let mut ao = AnimationObject::new("obj");
        ao.add_keyframe(0.0, [0.0, 0.0, 0.0]);
        ao.add_keyframe(1.0, [10.0, 0.0, 0.0]);
        let t = ao.translation_at(0.5);
        assert!((t[0] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn animation_timer() {
        let mut timer = AnimationTimer::new();
        timer.play();
        timer.advance(0.1);
        assert!((timer.elapsed - 0.1).abs() < 1e-14);
        timer.pause();
        timer.advance(0.1);
        assert!((timer.elapsed - 0.1).abs() < 1e-14);
    }
}
