// FILE: ais_base_animation_object.rs
// occt: AIS_BaseAnimationObject

#[derive(Clone, Debug)]
pub struct BaseAnimationObject {
    name: String,
    duration: f64,
    is_looped: bool,
}

impl BaseAnimationObject {
    pub fn new(name: String) -> Self {
        Self { name, duration: 1.0, is_looped: false }
    }
    pub fn name(&self) -> &str { &self.name }
    pub fn duration(&self) -> f64 { self.duration }
    pub fn set_duration(&mut self, d: f64) { self.duration = d.max(0.0); }
    pub fn is_looped(&self) -> bool { self.is_looped }
    pub fn set_looped(&mut self, l: bool) { self.is_looped = l; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a = BaseAnimationObject::new("anim".to_string());
        assert_eq!(a.name(), "anim");
    }
}
