// FILE: geom2d_eval_sine_wave_curve.rs
// occt: Geom2dEval_SineWaveCurve

/// Sinusoidal wave curve
#[derive(Debug, Clone)]
pub struct Geom2dEvalSineWaveCurve {
    amplitude: f64,
    frequency: f64,
}

impl Geom2dEvalSineWaveCurve {
    pub fn new(amplitude: f64, frequency: f64) -> Self {
        Geom2dEvalSineWaveCurve {
            amplitude,
            frequency,
        }
    }

    pub fn amplitude(&self) -> f64 {
        self.amplitude
    }

    pub fn frequency(&self) -> f64 {
        self.frequency
    }

    pub fn value(&self, t: f64) -> (f64, f64) {
        (t, self.amplitude * (self.frequency * t).sin())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sine_wave_new() {
        let wave = Geom2dEvalSineWaveCurve::new(1.0, 1.0);
        assert_eq!(wave.amplitude(), 1.0);
        assert_eq!(wave.frequency(), 1.0);
    }

    #[test]
    fn test_sine_wave_value() {
        let wave = Geom2dEvalSineWaveCurve::new(1.0, 1.0);
        let (x, y) = wave.value(0.0);
        assert_eq!(x, 0.0);
        assert!(y.abs() < 0.0001);
    }
}
