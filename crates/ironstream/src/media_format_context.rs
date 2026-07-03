// FILE: media_format_context.rs
// occt: Media_FormatContext

//! Media format context.
#[derive(Clone, Debug, Default)]
pub struct FormatContext {
    format_name: String,
    duration: i64,
    bit_rate: i64,
    nb_streams: i32,
}

impl FormatContext {
    pub fn new() -> Self { Self::default() }
    pub fn format_name(&self) -> &str { &self.format_name }
    pub fn duration(&self) -> i64 { self.duration }
    pub fn bit_rate(&self) -> i64 { self.bit_rate }
    pub fn nb_streams(&self) -> i32 { self.nb_streams }
    pub fn set_format_name(&mut self, name: String) { self.format_name = name; }
    pub fn set_duration(&mut self, dur: i64) { self.duration = dur; }
    pub fn set_bit_rate(&mut self, rate: i64) { self.bit_rate = rate; }
    pub fn set_nb_streams(&mut self, nb: i32) { self.nb_streams = nb; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ctx = FormatContext::new();
        assert_eq!(ctx.format_name(), "");
    }
}
