// FILE: math_utils_bracket.rs
// occt: MathUtils_Bracket

// Result structure for root bracketing
// occt: BracketResult
#[derive(Debug, Clone)]
pub struct BracketResult {
    pub is_valid: bool,
    pub a: f64,
    pub b: f64,
    pub fa: f64,
    pub fb: f64,
}

impl BracketResult {
    pub fn new() -> Self {
        BracketResult {
            is_valid: false,
            a: 0.0,
            b: 0.0,
            fa: 0.0,
            fb: 0.0,
        }
    }
}

impl Default for BracketResult {
    fn default() -> Self {
        Self::new()
    }
}

// Result structure for minimum bracketing
// occt: MinBracketResult
#[derive(Debug, Clone)]
pub struct MinBracketResult {
    pub is_valid: bool,
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub fa: f64,
    pub fb: f64,
    pub fc: f64,
}

impl MinBracketResult {
    pub fn new() -> Self {
        MinBracketResult {
            is_valid: false,
            a: 0.0,
            b: 0.0,
            c: 0.0,
            fa: 0.0,
            fb: 0.0,
            fc: 0.0,
        }
    }
}

impl Default for MinBracketResult {
    fn default() -> Self {
        Self::new()
    }
}

// Options for minimum bracketing
// occt: MinBracketOptions
#[derive(Debug, Clone)]
pub struct MinBracketOptions {
    pub max_iterations: usize,
    pub use_limits: bool,
    pub left_limit: f64,
    pub right_limit: f64,
    pub has_fa: bool,
    pub has_fb: bool,
    pub fa: f64,
    pub fb: f64,
}

impl MinBracketOptions {
    pub fn new() -> Self {
        MinBracketOptions {
            max_iterations: 50,
            use_limits: false,
            left_limit: 0.0,
            right_limit: 0.0,
            has_fa: false,
            has_fb: false,
            fa: 0.0,
            fb: 0.0,
        }
    }
}

impl Default for MinBracketOptions {
    fn default() -> Self {
        Self::new()
    }
}

// Constants (from MathUtils_Core)
const THE_GOLDEN_RATIO: f64 = 1.618033988749895;
const THE_ZERO_TOL: f64 = 1.0e-15;

// Helper: Clamp value to limits
fn limited(value: f64, options: &MinBracketOptions) -> f64 {
    if !options.use_limits {
        value
    } else {
        value.max(options.left_limit).min(options.right_limit)
    }
}

// Helper: Sign transfer function
fn sign_transfer(a: f64, b: f64) -> f64 {
    if b >= 0.0 {
        a.abs()
    } else {
        -a.abs()
    }
}

// Helper: Limit and maybe swap
fn limit_and_maybe_swap(
    func: &mut dyn FnMut(f64) -> Option<f64>,
    options: &MinBracketOptions,
    a: f64,
    b: &mut f64,
    fb: &mut f64,
    c: &mut f64,
    fc: &mut f64,
) -> bool {
    *c = limited(*c, options);
    let b_val = *b;
    let c_val = *c;
    if (b_val - c_val).abs() < THE_ZERO_TOL {
        return false;
    }
    match func(*c) {
        Some(fc_val) => {
            *fc = fc_val;
            // Keep B between A and C
            if (a - b_val) * (b_val - c_val) < 0.0 {
                std::mem::swap(b, c);
                std::mem::swap(fb, fc);
            }
            true
        }
        None => false,
    }
}

// Bracket a root by expanding interval until sign change is found
// occt: BracketRoot
pub fn bracket_root(
    func: &mut dyn FnMut(f64) -> Option<f64>,
    a: f64,
    b: f64,
    max_iter: usize,
) -> BracketResult {
    let mut result = BracketResult::new();
    result.a = a;
    result.b = b;

    match func(result.a) {
        Some(fa) => result.fa = fa,
        None => return result,
    }

    match func(result.b) {
        Some(fb) => result.fb = fb,
        None => return result,
    }

    for _i in 0..max_iter {
        if result.fa * result.fb < 0.0 {
            result.is_valid = true;
            // Ensure A < B
            if result.a > result.b {
                std::mem::swap(&mut result.a, &mut result.b);
                std::mem::swap(&mut result.fa, &mut result.fb);
            }
            return result;
        }

        // Expand the interval using golden ratio
        if result.fa.abs() < result.fb.abs() {
            result.a += THE_GOLDEN_RATIO * (result.a - result.b);
            match func(result.a) {
                Some(fa) => result.fa = fa,
                None => return result,
            }
        } else {
            result.b += THE_GOLDEN_RATIO * (result.b - result.a);
            match func(result.b) {
                Some(fb) => result.fb = fb,
                None => return result,
            }
        }
    }

    result
}

// Bracket a minimum
// occt: BracketMinimum
pub fn bracket_minimum(
    func: &mut dyn FnMut(f64) -> Option<f64>,
    a: f64,
    b: f64,
    options: &MinBracketOptions,
) -> MinBracketResult {
    let mut result = MinBracketResult::new();

    if options.max_iterations < 1 {
        return result;
    }

    if options.use_limits && options.left_limit > options.right_limit {
        return result;
    }

    result.a = limited(a, options);
    result.b = limited(b, options);

    if (result.a - result.b).abs() < THE_ZERO_TOL {
        return result;
    }

    // Check if we can use precomputed FA
    let use_fa = options.has_fa && (!options.use_limits || (result.a - a).abs() < THE_ZERO_TOL);
    let use_fb = options.has_fb && (!options.use_limits || (result.b - b).abs() < THE_ZERO_TOL);

    if use_fa {
        result.fa = options.fa;
    } else {
        match func(result.a) {
            Some(fa) => result.fa = fa,
            None => return result,
        }
    }

    if use_fb {
        result.fb = options.fb;
    } else {
        match func(result.b) {
            Some(fb) => result.fb = fb,
            None => return result,
        }
    }

    // Ensure we go downhill from A to B
    if result.fb > result.fa {
        std::mem::swap(&mut result.a, &mut result.b);
        std::mem::swap(&mut result.fa, &mut result.fb);
    }

    // Initial guess for C using golden ratio
    result.c = result.b + THE_GOLDEN_RATIO * (result.b - result.a);

    if options.use_limits {
        if !limit_and_maybe_swap(
            func,
            options,
            result.a,
            &mut result.b,
            &mut result.fb,
            &mut result.c,
            &mut result.fc,
        ) {
            return result;
        }
    } else {
        match func(result.c) {
            Some(fc) => result.fc = fc,
            None => return result,
        }
    }

    // Keep expanding until we bracket a minimum
    for _iter in 0..options.max_iterations {
        if result.fb < result.fc {
            break;
        }

        // Parabolic extrapolation
        let r = (result.b - result.a) * (result.fb - result.fc);
        let q = (result.b - result.c) * (result.fb - result.fa);
        let denom = 2.0 * sign_transfer(
            (q - r).abs().max(THE_ZERO_TOL),
            q - r,
        );

        let mut u = result.b - ((result.b - result.c) * q - (result.b - result.a) * r) / denom;
        let mut u_lim = result.b + 100.0 * (result.c - result.b);

        if options.use_limits {
            u_lim = limited(u_lim, options);
        }

        if (result.b - u) * (u - result.c) > 0.0 {
            // U is between B and C
            let fu_opt = func(u);
            if fu_opt.is_none() {
                return result;
            }
            let fu = fu_opt.unwrap();

            if fu < result.fc {
                result.a = result.b;
                result.b = u;
                result.fa = result.fb;
                result.fb = fu;
                result.is_valid = true;
                return result;
            } else if fu > result.fb {
                result.c = u;
                result.fc = fu;
                result.is_valid = true;
                return result;
            }

            // Parabolic step didn't help, use golden section
            u = result.c + THE_GOLDEN_RATIO * (result.c - result.b);
            if options.use_limits {
                if !limit_and_maybe_swap(
                    func,
                    options,
                    result.b,
                    &mut result.c,
                    &mut result.fc,
                    &mut u,
                    &mut result.fa,
                ) {
                    return result;
                }
            } else {
                match func(u) {
                    Some(fu) => result.fa = fu,
                    None => return result,
                }
            }
        } else if (result.c - u) * (u - u_lim) > 0.0 {
            // U is between C and limit
            if options.use_limits {
                if !limit_and_maybe_swap(
                    func,
                    options,
                    result.b,
                    &mut result.c,
                    &mut result.fc,
                    &mut u,
                    &mut result.fa,
                ) {
                    return result;
                }
            } else {
                match func(u) {
                    Some(fu) => result.fa = fu,
                    None => return result,
                }
            }

            if result.fa < result.fc {
                result.b = result.c;
                result.c = u;
                u = result.c + THE_GOLDEN_RATIO * (result.c - result.b);
                result.fb = result.fc;
                result.fc = result.fa;
                if options.use_limits {
                    if !limit_and_maybe_swap(
                        func,
                        options,
                        result.b,
                        &mut result.c,
                        &mut result.fc,
                        &mut u,
                        &mut result.fa,
                    ) {
                        return result;
                    }
                } else {
                    match func(u) {
                        Some(fu) => result.fa = fu,
                        None => return result,
                    }
                }
            }
        } else if (u - u_lim) * (u_lim - result.c) >= 0.0 {
            // U is beyond limit
            u = u_lim;
            if options.use_limits {
                if !limit_and_maybe_swap(
                    func,
                    options,
                    result.b,
                    &mut result.c,
                    &mut result.fc,
                    &mut u,
                    &mut result.fa,
                ) {
                    return result;
                }
            } else {
                match func(u) {
                    Some(fu) => result.fa = fu,
                    None => return result,
                }
            }
        } else {
            // Default golden section step
            u = result.c + THE_GOLDEN_RATIO * (result.c - result.b);
            if options.use_limits {
                if !limit_and_maybe_swap(
                    func,
                    options,
                    result.b,
                    &mut result.c,
                    &mut result.fc,
                    &mut u,
                    &mut result.fa,
                ) {
                    return result;
                }
            } else {
                match func(u) {
                    Some(fu) => result.fa = fu,
                    None => return result,
                }
            }
        }

        // Shift points
        result.a = result.b;
        result.b = result.c;
        result.c = u;
        result.fa = result.fb;
        result.fb = result.fc;
        result.fc = result.fa;
    }

    result.is_valid = result.fb < result.fa && result.fb < result.fc;

    // Ensure A < B < C ordering
    if result.is_valid && result.a > result.c {
        std::mem::swap(&mut result.a, &mut result.c);
        std::mem::swap(&mut result.fa, &mut result.fc);
    }

    if result.is_valid && !(result.a < result.b && result.b < result.c) {
        result.is_valid = false;
    }

    result
}

// Convenience overload with just max iterations
pub fn bracket_minimum_iter(
    func: &mut dyn FnMut(f64) -> Option<f64>,
    a: f64,
    b: f64,
    max_iter: usize,
) -> MinBracketResult {
    let mut options = MinBracketOptions::new();
    options.max_iterations = max_iter;
    bracket_minimum(func, a, b, &options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_minimum_with_limits_succeeds() {
        let mut func = |x: f64| {
            let result = (x - 2.0) * (x - 2.0);
            Some(result)
        };

        let mut options = MinBracketOptions::new();
        options.max_iterations = 50;
        options.use_limits = true;
        options.left_limit = 0.0;
        options.right_limit = 5.0;

        let result = bracket_minimum(&mut func, 0.0, 1.0, &options);
        assert!(result.is_valid);
        assert!(result.a >= options.left_limit);
        assert!(result.c <= options.right_limit);
        assert!(result.fb < result.fa);
        assert!(result.fb < result.fc);
    }

    #[test]
    fn bracket_minimum_with_restrictive_limits_fails() {
        let mut func = |x: f64| {
            let result = (x - 2.0) * (x - 2.0);
            Some(result)
        };

        let mut options = MinBracketOptions::new();
        options.max_iterations = 50;
        options.use_limits = true;
        options.left_limit = 0.0;
        options.right_limit = 1.0;

        let result = bracket_minimum(&mut func, 0.0, 0.5, &options);
        assert!(!result.is_valid);
    }

    #[test]
    fn bracket_minimum_uses_precomputed_endpoint_values() {
        let mut func = |x: f64| {
            // Return None for precomputed values
            if (x - 0.0).abs() < 1.0e-15 || (x - 1.0).abs() < 1.0e-15 {
                return None;
            }
            let result = (x - 2.0) * (x - 2.0);
            Some(result)
        };

        let mut options = MinBracketOptions::new();
        options.max_iterations = 50;
        options.has_fa = true;
        options.has_fb = true;
        options.fa = 4.0;
        options.fb = 1.0;

        let result = bracket_minimum(&mut func, 0.0, 1.0, &options);
        assert!(result.is_valid);
        assert!(result.fb < result.fa);
        assert!(result.fb < result.fc);
    }
}
