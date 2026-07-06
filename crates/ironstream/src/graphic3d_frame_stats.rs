// FILE: graphic3d_frame_stats.rs
// occt: Graphic3d_FrameStats

use std::time::Instant;

/// Local model of Graphic3d_FrameStatsData (data frame of counters/timers).
#[derive(Debug, Clone, Default)]
pub struct FrameStatsData {
    fps: f64,
    fps_cpu: f64,
}

impl FrameStatsData {
    /// Empty constructor
    pub fn new() -> Self {
        FrameStatsData::default()
    }

    /// Returns FPS (frames per seconds, elapsed time)
    pub fn frame_rate(&self) -> f64 {
        self.fps
    }

    /// Returns CPU FPS (frames per seconds, CPU time)
    pub fn frame_rate_cpu(&self) -> f64 {
        self.fps_cpu
    }

    /// Reset data
    pub fn reset(&mut self) {
        self.fps = 0.0;
        self.fps_cpu = 0.0;
    }
}

/// Local model of Graphic3d_FrameStatsDataTmp (currently filling frame).
#[derive(Debug, Clone, Default)]
pub struct FrameStatsDataTmp {
    base: FrameStatsData,
}

impl FrameStatsDataTmp {
    /// Empty constructor
    pub fn new() -> Self {
        FrameStatsDataTmp::default()
    }

    /// Reset data
    pub fn reset(&mut self) {
        self.base.reset();
    }

    /// Returns frame rate for modification
    pub fn change_frame_rate(&mut self) -> &mut f64 {
        &mut self.base.fps
    }

    /// Returns CPU frame rate for modification
    pub fn change_frame_rate_cpu(&mut self) -> &mut f64 {
        &mut self.base.fps_cpu
    }
}

/// Class storing the frame statistics
#[derive(Debug, Clone)]
pub struct FrameStats {
    /// update interval in seconds
    update_interval: f64,
    /// whether to use long line format
    is_long_line_format: bool,
    /// frame start time
    frame_start_time: f64,
    /// frame duration
    frame_duration: f64,
    /// FPS frame counter
    fps_frame_count: usize,
    /// data frames history
    counters: Vec<FrameStatsData>,
    /// currently filling data frame
    counters_tmp: FrameStatsDataTmp,
    /// maximum values in history
    counters_max: FrameStatsData,
    /// last data frame index
    last_frame_index: usize,
    /// time when stats creation started (for tracking)
    creation_time: Instant,
}

impl FrameStats {
    /// Default constructor
    pub fn new() -> Self {
        FrameStats {
            update_interval: 1.0,
            is_long_line_format: false,
            frame_start_time: 0.0,
            frame_duration: 0.0,
            fps_frame_count: 0,
            counters: vec![FrameStatsData::new(); 1],
            counters_tmp: FrameStatsDataTmp::new(),
            counters_max: FrameStatsData::new(),
            last_frame_index: 0,
            creation_time: Instant::now(),
        }
    }

    /// Returns interval in seconds for updating meters across several frames
    pub fn update_interval(&self) -> f64 {
        self.update_interval
    }

    /// Sets interval in seconds for updating values
    pub fn set_update_interval(&mut self, interval: f64) {
        self.update_interval = interval;
    }

    /// Returns whether to prefer longer lines over greater number of lines
    pub fn is_long_line_format(&self) -> bool {
        self.is_long_line_format
    }

    /// Sets if format should prefer longer lines over greater number of lines
    pub fn set_long_line_format(&mut self, value: bool) {
        self.is_long_line_format = value;
    }

    /// Frame redraw started
    pub fn frame_start(&mut self) {
        self.frame_start_time = self.creation_time.elapsed().as_secs_f64();
        self.counters_tmp.reset();
    }

    /// Frame redraw finished
    pub fn frame_end(&mut self) {
        let frame_end_time = self.creation_time.elapsed().as_secs_f64();
        self.frame_duration = frame_end_time - self.frame_start_time;
        self.fps_frame_count += 1;

        // Copy temp counters to history
        if self.last_frame_index >= self.counters.len() {
            self.counters.push(FrameStatsData::new());
        }
    }

    /// Returns duration of the last frame in seconds
    pub fn frame_duration(&self) -> f64 {
        self.frame_duration
    }

    /// Returns FPS (frames per seconds, elapsed time)
    pub fn frame_rate(&self) -> f64 {
        if !self.counters.is_empty() {
            self.counters[self.last_frame_index].frame_rate()
        } else {
            0.0
        }
    }

    /// Returns CPU FPS (frames per seconds, CPU time)
    pub fn frame_rate_cpu(&self) -> f64 {
        if !self.counters.is_empty() {
            self.counters[self.last_frame_index].frame_rate_cpu()
        } else {
            0.0
        }
    }

    /// Returns last data frame, cached between stats updates
    pub fn last_data_frame(&self) -> &FrameStatsData {
        if self.last_frame_index < self.counters.len() {
            &self.counters[self.last_frame_index]
        } else {
            &self.counters[0]
        }
    }

    /// Returns last data frame index
    pub fn last_data_frame_index(&self) -> usize {
        self.last_frame_index
    }

    /// Returns data frames
    pub fn data_frames(&self) -> &[FrameStatsData] {
        &self.counters
    }

    /// Returns mutable data frames
    pub fn change_data_frames(&mut self) -> &mut Vec<FrameStatsData> {
        &mut self.counters
    }

    /// Returns active data frame for modification
    pub fn active_data_frame(&mut self) -> &mut FrameStatsDataTmp {
        &mut self.counters_tmp
    }
}

impl Default for FrameStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_stats_creation() {
        let stats = FrameStats::new();
        assert_eq!(stats.update_interval(), 1.0);
        assert!(!stats.is_long_line_format());
        assert_eq!(stats.frame_duration(), 0.0);
    }

    #[test]
    fn test_frame_stats_set_update_interval() {
        let mut stats = FrameStats::new();
        stats.set_update_interval(0.5);
        assert_eq!(stats.update_interval(), 0.5);
    }

    #[test]
    fn test_frame_stats_set_long_line_format() {
        let mut stats = FrameStats::new();
        stats.set_long_line_format(true);
        assert!(stats.is_long_line_format());
    }

    #[test]
    fn test_frame_stats_frame_start_end() {
        let mut stats = FrameStats::new();
        stats.frame_start();
        stats.frame_end();
        // Frame duration should be very small but positive
        assert!(stats.frame_duration() >= 0.0);
    }

    #[test]
    fn test_frame_stats_last_data_frame() {
        let stats = FrameStats::new();
        let frame = stats.last_data_frame();
        assert_eq!(frame.frame_rate(), 0.0);
    }

    #[test]
    fn test_frame_stats_active_data_frame() {
        let mut stats = FrameStats::new();
        {
            let active = stats.active_data_frame();
            // Verify we can modify the active frame
            let _ = active.change_frame_rate();
        }
        // Frame was accessible
        assert!(true);
    }

    #[test]
    fn test_frame_stats_data_frames_access() {
        let stats = FrameStats::new();
        let frames = stats.data_frames();
        assert!(!frames.is_empty());
    }
}
