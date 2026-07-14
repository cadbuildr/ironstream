// FILE: rust/ironstream/crates/ironstream/src/message_progress.rs

// occt-ref: Message_ProgressRange // — zero-dependency port of OCCT's progress tracking

// occt-ref: Message_ProgressRange // state — enum NotStarted, InProgress, Completed, Aborted
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageProgressState {
    NotStarted,
    InProgress,
    Completed,
    Aborted,
}

// occt-ref: Message_ProgressRange // — tracks a fraction of work done
#[derive(Debug, Clone)]
pub struct MessageProgressRange {
    name: String,
    min: f64,
    max: f64,
    current: f64,
    state: MessageProgressState,
    nb_steps: u32,
    step_size: f64,
}

impl MessageProgressRange {
    /// Create a new progress range spanning [min, max] divided into nb_steps steps.
    /// step_size = (max - min) / nb_steps
    pub fn new(name: &str, min: f64, max: f64, nb_steps: u32) -> Self {
        let step_size = if nb_steps > 0 {
            (max - min) / nb_steps as f64
        } else {
            0.0
        };
        Self {
            name: name.to_string(),
            min,
            max,
            current: min,
            state: MessageProgressState::NotStarted,
            nb_steps,
            step_size,
        }
    }

    /// Return the range name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return fractional progress in [0, 1]: (current - min) / (max - min), clamped.
    pub fn progress(&self) -> f64 {
        let span = self.max - self.min;
        if span == 0.0 {
            return 1.0;
        }
        let raw = (self.current - self.min) / span;
        raw.clamp(0.0, 1.0)
    }

    /// Advance current by one step_size.
    pub fn advance(&mut self) {
        self.current += self.step_size;
    }

    /// Return the current state.
    pub fn state(&self) -> MessageProgressState {
        self.state
    }

    /// Transition to InProgress.
    pub fn start(&mut self) {
        self.state = MessageProgressState::InProgress;
    }

    /// Transition to Aborted.
    pub fn abort(&mut self) {
        self.state = MessageProgressState::Aborted;
    }

    /// Transition to Completed.
    pub fn complete(&mut self) {
        self.state = MessageProgressState::Completed;
    }

    /// Return true if state is Completed or Aborted.
    pub fn is_done(&self) -> bool {
        matches!(
            self.state,
            MessageProgressState::Completed | MessageProgressState::Aborted
        )
    }
}

// occt-ref: Message_ProgressScope // — nested progress composed of weighted ranges
#[derive(Debug, Clone)]
pub struct MessageProgressScope {
    name: String,
    ranges: Vec<MessageProgressRange>,
    weights: Vec<f64>,
    total_weight: f64,
}

impl MessageProgressScope {
    /// Create a new empty scope.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ranges: Vec::new(),
            weights: Vec::new(),
            total_weight: 0.0,
        }
    }

    /// Add a range to the scope with the given weight.
    pub fn add_range(&mut self, r: MessageProgressRange, weight: f64) {
        self.ranges.push(r);
        self.weights.push(weight);
        self.total_weight += weight;
    }

    /// Return the number of ranges in the scope.
    pub fn nb_ranges(&self) -> usize {
        self.ranges.len()
    }

    /// Return the weighted average of all ranges' progress(), in [0, 1].
    pub fn overall_progress(&self) -> f64 {
        if self.total_weight == 0.0 {
            return 0.0;
        }
        let mut sum = 0.0;
        for (r, &w) in self.ranges.iter().zip(self.weights.iter()) {
            sum += r.progress() * w;
        }
        sum / self.total_weight
    }

    /// Return the scope name.
    pub fn name(&self) -> &str {
        &self.name
    }
}
