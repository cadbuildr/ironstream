// FILE: ais_media_player.rs
// occt: AIS_MediaPlayer

#[derive(Clone, Debug)]
pub struct MediaPlayer {
    player_id: u32,
    width: u32,
    height: u32,
    is_playing: bool,
}

impl MediaPlayer {
    pub fn new(player_id: u32) -> Self {
        Self { player_id, width: 640, height: 480, is_playing: false }
    }
    pub fn player_id(&self) -> u32 { self.player_id }
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn is_playing(&self) -> bool { self.is_playing }
    pub fn set_dimensions(&mut self, w: u32, h: u32) { self.width = w; self.height = h; }
    pub fn set_playing(&mut self, p: bool) { self.is_playing = p; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = MediaPlayer::new(1);
        assert_eq!(p.player_id(), 1);
    }
}
