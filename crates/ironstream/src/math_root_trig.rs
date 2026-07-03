// FILE: math_root_trig.rs
//! Trigonometric equation solver using half-angle substitution.
//!
//! Solve trigonometric equation: a*cos^2(x) + 2*b*cos(x)*sin(x) + c*cos(x) + d*sin(x) + e = 0.
//!
//! Uses half-angle substitution t = tan(x/2) to convert to polynomial:
//! - cos(x) = (1-t^2)/(1+t^2)
//! - sin(x) = 2t/(1+t^2)

use std::f64::consts::PI;

const THE_PI: f64 = PI;
const THE_2PI: f64 = 2.0 * PI;

// occt: MathUtils_Status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    OK,
    NotConverged,
}

// occt: MathRoot_Trig
// occt: MathRoot_TrigResult
#[derive(Debug, Clone)]
pub struct TrigResult {
    pub status: Status,
    pub roots: [f64; 4],
    pub nb_roots: usize,
    pub infinite_roots: bool,
}

impl TrigResult {
    pub fn is_done(&self) -> bool {
        self.status == Status::OK
    }
}

impl Default for TrigResult {
    fn default() -> Self {
        TrigResult {
            status: Status::NotConverged,
            roots: [0.0; 4],
            nb_roots: 0,
            infinite_roots: false,
        }
    }
}

/// Solve a quadratic equation: a*x^2 + b*x + c = 0
fn solve_quadratic(a: f64, b: f64, c: f64) -> (usize, [f64; 2]) {
    if a.abs() < 1.5e-12 {
        // Linear: b*x + c = 0
        if b.abs() < 1.5e-12 {
            // Degenerate
            if c.abs() < 1.5e-12 {
                return (0, [0.0; 2]); // infinite solutions
            }
            return (0, [0.0; 2]); // no solution
        }
        return (1, [-c / b, 0.0]);
    }

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return (0, [0.0; 2]);
    }

    let sqrt_disc = discriminant.sqrt();
    let x1 = (-b + sqrt_disc) / (2.0 * a);
    let x2 = (-b - sqrt_disc) / (2.0 * a);

    if discriminant.abs() < 1.5e-12 {
        (1, [x1, 0.0])
    } else {
        (2, [x1, x2])
    }
}

/// Solve a quartic equation: a*x^4 + b*x^3 + c*x^2 + d*x + e = 0
fn solve_quartic(a: f64, b: f64, c: f64, d: f64, e: f64) -> (usize, [f64; 4]) {
    if a.abs() < 1.5e-12 {
        // Reduce to cubic
        let (nb, mut roots) = solve_cubic(b, c, d, e);
        return (nb, roots);
    }

    // Ferrari's method for general quartic
    // Reduce to depressed quartic: t^4 + p*t^2 + q*t + r = 0
    let p = c / a - 3.0 * b * b / (8.0 * a * a);
    let q = d / a - b * c / (2.0 * a * a) + b.powi(3) / (8.0 * a.powi(3));
    let r = e / a - b * d / (4.0 * a * a) + b * b * c / (16.0 * a.powi(3))
        - 3.0 * b.powi(4) / (256.0 * a.powi(4));

    // Resolvent cubic: y^3 - p*y^2 - 4*r*y + 4*p*r - q^2 = 0
    let (_, y_vals) = solve_cubic(1.0, -p, -4.0 * r, 4.0 * p * r - q * q);

    if y_vals[0] == 0.0 && y_vals[1] == 0.0 && y_vals[2] == 0.0 {
        return (0, [0.0; 4]);
    }

    // Use the first real root
    let y = y_vals[0];

    let sqrt_2y = (2.0 * y).abs().sqrt();
    if (2.0 * y).abs() < 1.5e-12 {
        // Handle case where 2*y ≈ 0
        let (n1, r1) = solve_quadratic(1.0, 0.0, p + y);
        let (n2, r2) = solve_quadratic(1.0, 0.0, p - y);
        let mut roots = [0.0; 4];
        roots[..n1].copy_from_slice(&r1[..n1]);
        if n2 > 0 {
            roots[n1] = r2[0];
        }
        if n2 > 1 {
            roots[n1 + 1] = r2[1];
        }
        let total = n1 + n2;
        return (total, roots);
    }

    // Original quartic as product of two quadratics
    let (n1, mut r1) = solve_quadratic(1.0, sqrt_2y, p + y + q / sqrt_2y);
    let (n2, r2) = solve_quadratic(1.0, -sqrt_2y, p + y - q / sqrt_2y);

    let mut roots = [0.0; 4];
    roots[..n1].copy_from_slice(&r1[..n1]);
    if n2 > 0 {
        roots[n1] = r2[0];
    }
    if n2 > 1 {
        roots[n1 + 1] = r2[1];
    }

    let total = (n1 + n2).min(4);
    (total, roots)
}

/// Solve a cubic equation: a*x^3 + b*x^2 + c*x + d = 0
fn solve_cubic(a: f64, b: f64, c: f64, d: f64) -> (usize, [f64; 4]) {
    if a.abs() < 1.5e-12 {
        // Reduce to quadratic
        let (nb, roots) = solve_quadratic(b, c, d);
        let mut result = [0.0; 4];
        result[..nb].copy_from_slice(&roots[..nb]);
        return (nb, result);
    }

    // Cardano's formula
    let p = (3.0 * a * c - b * b) / (3.0 * a * a);
    let q = (9.0 * a * b * c - 27.0 * a * a * d - 2.0 * b.powi(3))
        / (27.0 * a.powi(3));
    let disc = -(4.0 * p.powi(3) + 27.0 * q * q);

    // Compute discriminant and use appropriate method
    let shift = -b / (3.0 * a);

    // Use Cardano formula with trigonometric method for three real roots
    if disc > 0.0 {
        // Three distinct real roots
        let m = 2.0 * (-p / 3.0).sqrt();
        let theta = (3.0 * q / (p * m)) / 2.0;
        let theta = theta.max(-1.0).min(1.0); // clamp to [-1, 1]
        let theta = theta.acos() / 3.0;

        let mut roots = [0.0; 4];
        roots[0] = m * (2.0 * (theta)).cos() + shift;
        roots[1] = m * (2.0 * (theta - PI / 3.0)).cos() + shift;
        roots[2] = m * (2.0 * (theta + PI / 3.0)).cos() + shift;
        return (3, roots);
    }

    // Use Cardano for one real root
    let delta = q * q / 4.0 + p.powi(3) / 27.0;
    let sqrt_delta = delta.sqrt();

    let u = (-q / 2.0 + sqrt_delta).cbrt();
    let v = (-q / 2.0 - sqrt_delta).cbrt();

    let mut roots = [0.0; 4];
    if u.abs() > 1.5e-12 {
        roots[0] = u + v + shift;
    } else if v.abs() > 1.5e-12 {
        roots[0] = u + v + shift;
    } else {
        roots[0] = shift;
    }

    (1, roots)
}

/// Trigonometric equation solver for: a*cos^2(x) + 2*b*cos(x)*sin(x) + c*cos(x) + d*sin(x) + e = 0
pub fn trigonometric(
    the_a: f64,
    the_b: f64,
    the_c: f64,
    the_d: f64,
    the_e: f64,
    the_inf_bound: f64,
    the_sup_bound: f64,
    the_eps: f64,
) -> TrigResult {
    let mut result = TrigResult::default();
    result.status = Status::OK;

    // Compute working interval
    let (a_my_borne_inf, a_delta, a_mod) = if the_inf_bound <= f64::MIN / 2.0
        && the_sup_bound >= f64::MAX / 2.0
    {
        (0.0, THE_2PI, 0.0)
    } else if the_sup_bound >= f64::MAX / 2.0 {
        (the_inf_bound, THE_2PI, the_inf_bound / THE_2PI)
    } else if the_inf_bound <= f64::MIN / 2.0 {
        (the_sup_bound - THE_2PI, THE_2PI, (the_sup_bound - THE_2PI) / THE_2PI)
    } else {
        let delta = if the_sup_bound - the_inf_bound > THE_2PI {
            THE_2PI
        } else {
            the_sup_bound - the_inf_bound
        };
        (the_inf_bound, delta, the_inf_bound / THE_2PI)
    };

    let a_delta_eps = f64::EPSILON * a_delta.abs();

    // Case: A = B = 0 (degree <= 2 in cos/sin)
    if the_a.abs() <= the_eps && the_b.abs() <= the_eps {
        if the_c.abs() <= the_eps {
            if the_d.abs() <= the_eps {
                if the_e.abs() <= the_eps {
                    result.infinite_roots = true;
                    return result;
                }
                result.nb_roots = 0;
                return result;
            } else {
                // d*sin(x) + e = 0
                let a_val = -the_e / the_d;
                if a_val.abs() > 1.0 {
                    result.nb_roots = 0;
                    return result;
                }
                let mut roots = [a_val.asin(), THE_PI - a_val.asin()];
                for root in &mut roots {
                    if *root <= -the_eps {
                        *root = THE_2PI - root.abs();
                    }
                    *root += a_mod.trunc() * THE_2PI;
                    let ax = *root - a_my_borne_inf;
                    if ax >= -a_delta_eps && ax <= a_delta + a_delta_eps {
                        result.roots[result.nb_roots] = *root;
                        result.nb_roots += 1;
                    }
                }
                return result;
            }
        } else if the_d.abs() <= the_eps {
            // c*cos(x) + e = 0
            let a_val = -the_e / the_c;
            if a_val.abs() > 1.0 {
                result.nb_roots = 0;
                return result;
            }
            let a_principal = a_val.acos();
            let angles = [a_principal, -a_principal];

            for angle in &angles {
                let a_k = ((the_inf_bound - angle) / THE_2PI).floor();
                let mut a_angle = angle + (a_k + 1.0) * THE_2PI;

                for _ in 0..2 {
                    if a_angle >= the_inf_bound - a_delta_eps && a_angle <= the_sup_bound + a_delta_eps
                    {
                        // Check for duplicates
                        let mut is_dup = false;
                        for k in 0..result.nb_roots {
                            if (a_angle - result.roots[k]).abs() < the_eps {
                                is_dup = true;
                                break;
                            }
                        }
                        if !is_dup && result.nb_roots < 4 {
                            result.roots[result.nb_roots] = a_angle;
                            result.nb_roots += 1;
                        }
                    }
                    a_angle += THE_2PI;
                }
            }
            return result;
        } else {
            // c*cos(x) + d*sin(x) + e = 0
            // Using t = tan(x/2): (e-c)*t^2 + 2d*t + (e+c) = 0
            let aa = the_e - the_c;
            let bb = 2.0 * the_d;
            let cc = the_e + the_c;

            let (n_zer, zer) = solve_quadratic(aa, bb, cc);
            if n_zer == 0 {
                return result;
            }
            if aa.abs() < the_eps && bb.abs() < the_eps {
                result.infinite_roots = true;
                return result;
            }

            for i in 0..n_zer {
                let t_val = zer[i];
                let mut a_teta = 2.0 * t_val.atan();
                if t_val <= -the_eps {
                    a_teta = THE_2PI - a_teta.abs();
                }
                a_teta += a_mod.trunc() * THE_2PI;

                if a_teta - a_my_borne_inf < 0.0 {
                    a_teta += THE_2PI;
                }

                let ax = a_teta - a_my_borne_inf;
                if ax >= -a_delta_eps && ax <= a_delta + a_delta_eps {
                    result.roots[result.nb_roots] = a_teta;
                    result.nb_roots += 1;
                }
            }
            return result;
        }
    }

    // General case: degree 4 polynomial using half-angle substitution
    let ko0 = the_a - the_c + the_e;
    let ko1 = 2.0 * the_d - 4.0 * the_b;
    let ko2 = 2.0 * the_e - 2.0 * the_a;
    let ko3 = 4.0 * the_b + 2.0 * the_d;
    let ko4 = the_a + the_c + the_e;

    let (n_zer, zer) = solve_quartic(ko0, ko1, ko2, ko3, ko4);

    if n_zer == 0 {
        return result;
    }

    // Convert t values to angles and filter by bounds
    let mut t_angles = Vec::new();
    for i in 0..n_zer {
        let mut a_teta = 2.0 * zer[i].atan();
        if zer[i] <= -the_eps {
            a_teta = THE_2PI - a_teta.abs();
        }
        a_teta += a_mod.trunc() * THE_2PI;
        if a_teta - a_my_borne_inf < 0.0 {
            a_teta += THE_2PI;
        }

        let ax = a_teta - a_my_borne_inf;
        if ax >= -a_delta_eps && ax <= a_delta + a_delta_eps {
            t_angles.push(a_teta);
        }
    }

    // Refine roots using Newton with Halley fallback for double roots
    for a_teta in t_angles {
        let mut a_teta_refined = a_teta;

        for _ in 0..20 {
            let a_cos = a_teta_refined.cos();
            let a_sin = a_teta_refined.sin();
            let a_cos2 = a_cos * a_cos;
            let a_sin2 = a_sin * a_sin;
            let a_cs = a_cos * a_sin;

            let a_f = the_a * a_cos2 + 2.0 * the_b * a_cs + the_c * a_cos
                + the_d * a_sin + the_e;
            let a_df = -2.0 * the_a * a_cs + 2.0 * the_b * (a_cos2 - a_sin2) - the_c * a_sin
                + the_d * a_cos;

            if a_f.abs() < 1.0e-15 {
                break;
            }

            let a_delta = if a_df.abs() < 1.0e-10 * (a_f.abs() + 1.0) {
                // Use Halley's method
                let a_d2f = -2.0 * the_a * (a_cos2 - a_sin2) - 4.0 * the_b * a_cs
                    - the_c * a_cos - the_d * a_sin;
                let a_denom = 2.0 * a_df * a_df - a_f * a_d2f;
                if a_denom.abs() < 1.0e-30 {
                    break;
                }
                2.0 * a_f * a_df / a_denom
            } else {
                a_f / a_df
            };

            let max_step = 0.5;
            let a_delta_clamped = if a_delta.abs() > max_step {
                if a_delta > 0.0 { max_step } else { -max_step }
            } else {
                a_delta
            };

            a_teta_refined -= a_delta_clamped;

            if a_delta_clamped.abs() < 1.0e-14 {
                break;
            }
        }

        let a_delta_newton = (a_teta_refined - a_teta).abs();
        let a_supm_infs_100 = (the_sup_bound - the_inf_bound) * 0.01;
        if a_delta_newton <= a_supm_infs_100 {
            a_teta_refined = a_teta;
        }

        // Insert in sorted order, avoiding duplicates
        let mut a_found = false;
        for k in 0..result.nb_roots {
            if (a_teta_refined - result.roots[k]).abs() < the_eps {
                a_found = true;
                break;
            }
        }

        if !a_found && result.nb_roots < 4 {
            // Find insertion position
            let mut a_pos = result.nb_roots;
            for k in 0..result.nb_roots {
                if a_teta_refined < result.roots[k] {
                    a_pos = k;
                    break;
                }
            }
            for k in (a_pos + 1..=result.nb_roots).rev() {
                result.roots[k] = result.roots[k - 1];
            }
            result.roots[a_pos] = a_teta_refined;
            result.nb_roots += 1;
        }
    }

    // Special case: check if PI is a root (when A - C + E = 0)
    if result.nb_roots < 4 && (the_a - the_c + the_e).abs() <= the_eps {
        let a_teta = THE_PI + a_mod.trunc() * THE_2PI;
        let ax = a_teta - a_my_borne_inf;
        if ax >= -a_delta_eps && ax <= a_delta + a_delta_eps {
            let mut a_found = false;
            for k in 0..result.nb_roots {
                if (a_teta - result.roots[k]).abs() <= the_eps {
                    a_found = true;
                    break;
                }
            }
            if !a_found {
                // Insert sorted
                let mut a_pos = result.nb_roots;
                for k in 0..result.nb_roots {
                    if a_teta < result.roots[k] {
                        a_pos = k;
                        break;
                    }
                }
                for k in (a_pos + 1..=result.nb_roots).rev() {
                    result.roots[k] = result.roots[k - 1];
                }
                result.roots[a_pos] = a_teta;
                result.nb_roots += 1;
            }
        }
    }

    result
}

/// Solve linear trigonometric equation: d*sin(x) + e = 0
pub fn trigonometric_linear(
    the_d: f64,
    the_e: f64,
    the_inf_bound: f64,
    the_sup_bound: f64,
) -> TrigResult {
    trigonometric(0.0, 0.0, 0.0, the_d, the_e, the_inf_bound, the_sup_bound, 1.5e-12)
}

/// Solve trigonometric equation: c*cos(x) + d*sin(x) + e = 0
pub fn trigonometric_cde(
    the_c: f64,
    the_d: f64,
    the_e: f64,
    the_inf_bound: f64,
    the_sup_bound: f64,
) -> TrigResult {
    trigonometric(0.0, 0.0, the_c, the_d, the_e, the_inf_bound, the_sup_bound, 1.5e-12)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evaluate_equation(the_a: f64, the_b: f64, the_c: f64, the_d: f64, the_e: f64, the_x: f64) -> f64 {
        let cos_x = the_x.cos();
        let sin_x = the_x.sin();
        the_a * cos_x * cos_x + 2.0 * the_b * cos_x * sin_x + the_c * cos_x
            + the_d * sin_x + the_e
    }

    fn verify_roots(
        result: &TrigResult,
        the_a: f64,
        the_b: f64,
        the_c: f64,
        the_d: f64,
        the_e: f64,
        tol: f64,
    ) {
        for i in 0..result.nb_roots {
            let val = evaluate_equation(the_a, the_b, the_c, the_d, the_e, result.roots[i]);
            assert!(
                val.abs() < tol,
                "Root {} = {} gives f(x) = {}",
                i, result.roots[i], val
            );
        }
    }

    #[test]
    fn linear_sin_zero_constant() {
        // sin(x) = 0 => x = 0, PI
        let result = trigonometric_linear(1.0, 0.0, 0.0, THE_2PI);
        assert!(result.is_done());
        assert_eq!(result.nb_roots, 2);
        verify_roots(&result, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0e-10);
    }

    #[test]
    fn linear_sin_half_value() {
        // sin(x) = 0.5 => x = PI/6, 5*PI/6
        let result = trigonometric_linear(1.0, -0.5, 0.0, THE_2PI);
        assert!(result.is_done());
        assert_eq!(result.nb_roots, 2);
        verify_roots(&result, 0.0, 0.0, 0.0, 1.0, -0.5, 1.0e-10);
    }

    #[test]
    fn linear_sin_no_solution() {
        // sin(x) = 2 => no solution
        let result = trigonometric_linear(1.0, -2.0, 0.0, THE_2PI);
        assert!(result.is_done());
        assert_eq!(result.nb_roots, 0);
    }

    #[test]
    fn linear_cos_zero_constant() {
        // cos(x) = 0 => x = PI/2, 3*PI/2
        let result = trigonometric(0.0, 0.0, 1.0, 0.0, 0.0, 0.0, THE_2PI, 1.5e-12);
        assert!(result.is_done());
        assert!(result.nb_roots >= 2);
        verify_roots(&result, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0e-10);
    }

    #[test]
    fn cde_cos_plus_sin() {
        // cos(x) + sin(x) = 0 => tan(x) = -1 => x = 3*PI/4, 7*PI/4
        let result = trigonometric_cde(1.0, 1.0, 0.0, 0.0, THE_2PI);
        assert!(result.is_done());
        assert_eq!(result.nb_roots, 2);
        verify_roots(&result, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0e-10);
    }

    #[test]
    fn degenerate_all_zero() {
        // 0 = 0 => infinite solutions
        let result = trigonometric(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, THE_2PI, 1.5e-12);
        assert!(result.infinite_roots);
    }

    #[test]
    fn degenerate_constant_nonzero() {
        // e != 0 => no solutions
        let result = trigonometric(0.0, 0.0, 0.0, 0.0, 1.0, 0.0, THE_2PI, 1.5e-12);
        assert!(result.is_done());
        assert_eq!(result.nb_roots, 0);
    }

    #[test]
    fn result_bool_conversion() {
        let result = trigonometric_linear(1.0, 0.0, 0.0, THE_2PI);
        assert!(result.is_done());
    }
}
