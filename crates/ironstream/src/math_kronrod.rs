// FILE: rust/ironstream/crates/ironstream/src/math_kronrod.rs

// occt: math_KronrodSingleIntegration // — Gauss-Kronrod 15-point quadrature on [a,b]
/// Numerical integration using the G7K15 Gauss-Kronrod quadrature rule.
pub struct MathKronrodIntegration {
    pub value: f64,
    pub error_estimate: f64,
    pub nb_iter: u32,
    pub is_done: bool,
}

// Standard G7K15 abscissas (positive half, 8 values; last entry is 0.0 for the
// centre node).  The full 15-node rule is obtained by reflecting each non-zero node.
const KN_NODES: [f64; 8] = [
    0.991_455_371_120_813,
    0.949_107_912_342_759,
    0.864_864_423_359_769,
    0.741_531_185_599_394,
    0.586_087_235_467_691,
    0.405_845_151_377_397,
    0.207_784_955_007_898,
    0.0,
];

// Kronrod weights for the 8 distinct abscissas listed above.
const KN_WEIGHTS: [f64; 8] = [
    0.022_935_322_010_529,
    0.063_092_092_629_979,
    0.104_790_010_322_250,
    0.140_653_259_715_525,
    0.169_004_726_639_267,
    0.190_350_578_064_785,
    0.204_432_940_075_298,
    0.209_482_141_084_728,
];

// Gauss weights for the embedded 7-point rule.
// The 7 Gauss nodes correspond to KN_NODES at indices 1, 3, 5 (positive side)
// and the centre node at index 7.
// G7_WEIGHTS[0] -> index 1, [1] -> index 3, [2] -> index 5, [3] -> index 7 (centre).
const G7_WEIGHTS: [f64; 4] = [
    0.129_484_966_168_870,
    0.279_705_391_489_277,
    0.381_830_050_505_119,
    0.417_959_183_673_469,
];

impl MathKronrodIntegration {
    /// Return a zero-initialised integrator (all zero / false).
    pub fn new() -> Self {
        Self {
            value: 0.0,
            error_estimate: 0.0,
            nb_iter: 0,
            is_done: false,
        }
    }

    /// Integrate `f` on `[a, b]` using the 15-point Gauss-Kronrod rule.
    ///
    /// Sets `is_done = true` after the single-pass evaluation.
    /// `nb_iter` is set to the number of function evaluations (15).
    /// `error_estimate` is `|K15 - G7|`.
    /// Returns the computed integral value.
    pub fn perform(&mut self, f: &dyn Fn(f64) -> f64, a: f64, b: f64, _tolerance: f64) -> f64 {
        let mid = 0.5 * (b + a);
        let half = 0.5 * (b - a);

        let mut k15_sum = 0.0_f64;
        let mut g7_sum = 0.0_f64;
        let mut calls: u32 = 0;

        for i in 0..8usize {
            let t = KN_NODES[i];

            if t == 0.0 {
                // Centre node — appears once only.
                let fx = f(mid);
                calls += 1;
                k15_sum += KN_WEIGHTS[i] * fx;
                g7_sum += G7_WEIGHTS[3] * fx;
            } else {
                // Symmetric pair of nodes.
                let fxp = f(mid + half * t);
                let fxn = f(mid - half * t);
                calls += 2;

                k15_sum += KN_WEIGHTS[i] * (fxp + fxn);

                // Indices 1, 3, 5 are also Gauss nodes.
                match i {
                    1 => g7_sum += G7_WEIGHTS[0] * (fxp + fxn),
                    3 => g7_sum += G7_WEIGHTS[1] * (fxp + fxn),
                    5 => g7_sum += G7_WEIGHTS[2] * (fxp + fxn),
                    _ => {}
                }
            }
        }

        let result = half * k15_sum;
        let g7_result = half * g7_sum;

        self.value = result;
        self.error_estimate = (result - g7_result).abs();
        self.nb_iter = calls;
        self.is_done = true;

        result
    }

    /// The computed integral value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Error estimate `|K15 - G7|`.
    pub fn error_estimate(&self) -> f64 {
        self.error_estimate
    }

    /// Whether `perform` has been called.
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// Number of function evaluations performed (15 for a single G7K15 pass).
    pub fn nb_iter(&self) -> u32 {
        self.nb_iter
    }
}

impl Default for MathKronrodIntegration {
    fn default() -> Self {
        Self::new()
    }
}
