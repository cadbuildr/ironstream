// FILE: geom_eval_sine_wave_curve.rs
// occt: GeomEval_SineWaveCurve

pub struct GeomEvalSineWaveCurve {
    amplitude: f64,
    frequency: f64,
}

impl GeomEvalSineWaveCurve {
    pub fn new(amplitude: f64, frequency: f64) -> Self {
        GeomEvalSineWaveCurve {
            amplitude,
            frequency,
        }
    }

    pub fn evaluate(&self, t: f64) -> (f64, f64, f64) {
        (
            t,
            self.amplitude * (self.frequency * t).sin(),
            0.0,
        )
    }

    pub fn evaluate_d1(&self, t: f64) -> ((f64, f64, f64), (f64, f64, f64)) {
        (
            (t, self.amplitude * (self.frequency * t).sin(), 0.0),
            (1.0, self.amplitude * self.frequency * (self.frequency * t).cos(), 0.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_sine_wave_new() {
        let wave = GeomEvalSineWaveCurve::new(1.0, 2.0);
        assert!((wave.amplitude - 1.0).abs() < PRECISION);
        assert!((wave.frequency - 2.0).abs() < PRECISION);
    }

    #[test]
    fn test_sine_wave_evaluate() {
        let wave = GeomEvalSineWaveCurve::new(1.0, 1.0);
        let (x, y, z) = wave.evaluate(0.0);
        assert!(x.abs() < PRECISION);
        assert!(y.abs() < PRECISION);
        assert!(z.abs() < PRECISION);
    }

    #[test]
    fn test_sine_wave_evaluate_d1() {
        let wave = GeomEvalSineWaveCurve::new(1.0, 1.0);
        let ((_x, _y, _z), (dx, dy, dz)) = wave.evaluate_d1(0.0);
        assert!((dx - 1.0).abs() < PRECISION);
        assert!((dy - 1.0).abs() < PRECISION);
        assert!(dz.abs() < PRECISION);
    }
}
