// FILE: media_player_context.rs
// occt: Media_PlayerContext

//! Media player context.
#[derive(Clone, Debug, Default)]
pub struct PlayerContext {
    duration: f64,
    current_time: f64,
    is_playing: bool,
    volume: f32,
}

impl PlayerContext {
    pub fn new() -> Self {
        Self { duration: 0.0, current_time: 0.0, is_playing: false, volume: 1.0 }
    }
    pub fn duration(&self) -> f64 { self.duration }
    pub fn current_time(&self) -> f64 { self.current_time }
    pub fn is_playing(&self) -> bool { self.is_playing }
    pub fn volume(&self) -> f32 { self.volume }
    pub fn set_duration(&mut self, d: f64) { self.duration = d; }
    pub fn set_current_time(&mut self, t: f64) { self.current_time = t; }
    pub fn set_playing(&mut self, p: bool) { self.is_playing = p; }
    pub fn set_volume(&mut self, v: f32) { self.volume = v.max(0.0).min(1.0); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ctx = PlayerContext::new();
        assert!(!ctx.is_playing());
    }
}
