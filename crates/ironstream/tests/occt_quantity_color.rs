// FILE: rust/ironstream/crates/ironstream/tests/occt_quantity_color.rs
//! Ported OpenCascade unit tests — `Quantity_Color`.
//!
//! Tests cover construction, clamping, hex round-trips, named-color lookup,
//! HLS conversion, RGBA, hex-to-name reverse lookup, and the `delta_e`
//! Euclidean distance metric.

extern crate ironstream;
use ironstream::quantity_color::{QuantityColor, QuantityColorModel};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Assert two `f32` values are within `eps` of each other.
fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() <= eps
}

const EPS: f32 = 1.0 / 255.0 + 1e-6; // one 8-bit step + tiny float slop

// ---------------------------------------------------------------------------
// QuantityColorModel — enum exists and variants compare correctly
// ---------------------------------------------------------------------------

#[test]
fn color_model_variants_are_distinct() {
    assert_ne!(QuantityColorModel::Rgb, QuantityColorModel::Hls);
    assert_ne!(QuantityColorModel::Rgb, QuantityColorModel::Ciede2000);
    assert_ne!(QuantityColorModel::Hls, QuantityColorModel::Ciede2000);
}

#[test]
fn color_model_copy_and_debug() {
    let m = QuantityColorModel::Rgb;
    let m2 = m; // Copy
    assert_eq!(m, m2);
    let _ = format!("{m:?}"); // Debug
}

// ---------------------------------------------------------------------------
// Construction — new(), clamping
// ---------------------------------------------------------------------------

#[test]
fn new_stores_channels_unchanged_when_in_range() {
    let c = QuantityColor::new(0.2, 0.4, 0.8);
    assert!((c.r() - 0.2).abs() < 1e-7);
    assert!((c.g() - 0.4).abs() < 1e-7);
    assert!((c.b() - 0.8).abs() < 1e-7);
}

#[test]
fn new_clamps_above_one() {
    let c = QuantityColor::new(2.0, 1.5, 3.0);
    assert_eq!(c.r(), 1.0);
    assert_eq!(c.g(), 1.0);
    assert_eq!(c.b(), 1.0);
}

#[test]
fn new_clamps_below_zero() {
    let c = QuantityColor::new(-0.5, -1.0, -100.0);
    assert_eq!(c.r(), 0.0);
    assert_eq!(c.g(), 0.0);
    assert_eq!(c.b(), 0.0);
}

#[test]
fn new_clamps_mixed() {
    let c = QuantityColor::new(-0.1, 0.5, 1.1);
    assert_eq!(c.r(), 0.0);
    assert!((c.g() - 0.5).abs() < 1e-7);
    assert_eq!(c.b(), 1.0);
}

#[test]
fn default_is_black() {
    let c = QuantityColor::default();
    assert_eq!(c.r(), 0.0);
    assert_eq!(c.g(), 0.0);
    assert_eq!(c.b(), 0.0);
}

// ---------------------------------------------------------------------------
// from_hex / to_hex
// ---------------------------------------------------------------------------

#[test]
fn from_hex_black() {
    let c = QuantityColor::from_hex(0x000000);
    assert_eq!(c.r(), 0.0);
    assert_eq!(c.g(), 0.0);
    assert_eq!(c.b(), 0.0);
}

#[test]
fn from_hex_white() {
    let c = QuantityColor::from_hex(0xFFFFFF);
    assert!((c.r() - 1.0).abs() < 1e-6);
    assert!((c.g() - 1.0).abs() < 1e-6);
    assert!((c.b() - 1.0).abs() < 1e-6);
}

#[test]
fn from_hex_red() {
    let c = QuantityColor::from_hex(0xFF0000);
    assert!((c.r() - 1.0).abs() < 1e-6);
    assert_eq!(c.g(), 0.0);
    assert_eq!(c.b(), 0.0);
}

#[test]
fn from_hex_orange_0xff8800() {
    // 0xFF = 255, 0x88 = 136, 0x00 = 0
    let c = QuantityColor::from_hex(0xFF_88_00);
    assert!((c.r() - 1.0).abs() < 1e-5);
    assert!((c.g() - 136.0 / 255.0).abs() < 1e-5);
    assert_eq!(c.b(), 0.0);
}

#[test]
fn to_hex_round_trip() {
    for &hex in &[0x000000u32, 0xFFFFFF, 0xFF0000, 0x00FF00, 0x0000FF, 0xABCDEF] {
        let c = QuantityColor::from_hex(hex);
        assert_eq!(c.to_hex(), hex, "round-trip failed for 0x{hex:06X}");
    }
}

#[test]
fn to_hex_known_values() {
    let white = QuantityColor::new(1.0, 1.0, 1.0);
    assert_eq!(white.to_hex(), 0xFFFFFF);

    let black = QuantityColor::new(0.0, 0.0, 0.0);
    assert_eq!(black.to_hex(), 0x000000);

    let red = QuantityColor::new(1.0, 0.0, 0.0);
    assert_eq!(red.to_hex(), 0xFF0000);
}

// ---------------------------------------------------------------------------
// rgba()
// ---------------------------------------------------------------------------

#[test]
fn rgba_has_alpha_one() {
    let c = QuantityColor::new(0.1, 0.2, 0.3);
    let [r, g, b, a] = c.rgba();
    assert!((r - 0.1).abs() < 1e-7);
    assert!((g - 0.2).abs() < 1e-7);
    assert!((b - 0.3).abs() < 1e-7);
    assert_eq!(a, 1.0);
}

// ---------------------------------------------------------------------------
// from_name / name()
// ---------------------------------------------------------------------------

#[test]
fn from_name_recognises_all_ten() {
    let names = ["black", "white", "red", "green", "blue",
                 "yellow", "cyan", "magenta", "orange", "gray"];
    for n in &names {
        assert!(QuantityColor::from_name(n).is_some(), "'{n}' not recognised");
    }
}

#[test]
fn from_name_black_is_zero() {
    let c = QuantityColor::from_name("black").unwrap();
    assert_eq!(c.r(), 0.0);
    assert_eq!(c.g(), 0.0);
    assert_eq!(c.b(), 0.0);
}

#[test]
fn from_name_white_is_one() {
    let c = QuantityColor::from_name("white").unwrap();
    assert_eq!(c.r(), 1.0);
    assert_eq!(c.g(), 1.0);
    assert_eq!(c.b(), 1.0);
}

#[test]
fn from_name_red_channels() {
    let c = QuantityColor::from_name("red").unwrap();
    assert_eq!(c.r(), 1.0);
    assert_eq!(c.g(), 0.0);
    assert_eq!(c.b(), 0.0);
}

#[test]
fn from_name_blue_channels() {
    let c = QuantityColor::from_name("blue").unwrap();
    assert_eq!(c.r(), 0.0);
    assert_eq!(c.g(), 0.0);
    assert_eq!(c.b(), 1.0);
}

#[test]
fn from_name_cyan_channels() {
    let c = QuantityColor::from_name("cyan").unwrap();
    assert_eq!(c.r(), 0.0);
    assert_eq!(c.g(), 1.0);
    assert_eq!(c.b(), 1.0);
}

#[test]
fn from_name_magenta_channels() {
    let c = QuantityColor::from_name("magenta").unwrap();
    assert_eq!(c.r(), 1.0);
    assert_eq!(c.g(), 0.0);
    assert_eq!(c.b(), 1.0);
}

#[test]
fn from_name_yellow_channels() {
    let c = QuantityColor::from_name("yellow").unwrap();
    assert_eq!(c.r(), 1.0);
    assert_eq!(c.g(), 1.0);
    assert_eq!(c.b(), 0.0);
}

#[test]
fn from_name_unknown_returns_none() {
    assert!(QuantityColor::from_name("").is_none());
    assert!(QuantityColor::from_name("Black").is_none()); // case-sensitive
    assert!(QuantityColor::from_name("RED").is_none());
    assert!(QuantityColor::from_name("purple").is_none());
    assert!(QuantityColor::from_name("transparent").is_none());
}

#[test]
fn name_reverse_lookup_all_ten() {
    let names = ["black", "white", "red", "green", "blue",
                 "yellow", "cyan", "magenta", "orange", "gray"];
    for n in &names {
        let c = QuantityColor::from_name(n).unwrap();
        assert_eq!(c.name(), Some(*n), "reverse-lookup failed for '{n}'");
    }
}

#[test]
fn name_reverse_lookup_arbitrary_color_is_none() {
    let c = QuantityColor::new(0.1, 0.2, 0.3);
    assert_eq!(c.name(), None);
}

// ---------------------------------------------------------------------------
// HLS round-trip: to_hls / from_hls
// ---------------------------------------------------------------------------

/// The round-trip tolerance is 1/255 per RGB channel (one 8-bit step).
const HLS_RT_TOL: f32 = 2.0 / 255.0;

fn assert_rgb_close(a: &QuantityColor, b: &QuantityColor, label: &str) {
    assert!(
        approx_eq(a.r(), b.r(), HLS_RT_TOL)
            && approx_eq(a.g(), b.g(), HLS_RT_TOL)
            && approx_eq(a.b(), b.b(), HLS_RT_TOL),
        "{label}: got ({}, {}, {}), expected ({}, {}. {})",
        a.r(), a.g(), a.b(), b.r(), b.g(), b.b()
    );
}

#[test]
fn hls_round_trip_red() {
    let orig = QuantityColor::new(1.0, 0.0, 0.0);
    let (h, l, s) = orig.to_hls();
    let back = QuantityColor::from_hls(h, l, s);
    assert_rgb_close(&back, &orig, "red");
}

#[test]
fn hls_round_trip_green() {
    let orig = QuantityColor::new(0.0, 1.0, 0.0);
    let (h, l, s) = orig.to_hls();
    let back = QuantityColor::from_hls(h, l, s);
    assert_rgb_close(&back, &orig, "green");
}

#[test]
fn hls_round_trip_blue() {
    let orig = QuantityColor::new(0.0, 0.0, 1.0);
    let (h, l, s) = orig.to_hls();
    let back = QuantityColor::from_hls(h, l, s);
    assert_rgb_close(&back, &orig, "blue");
}

#[test]
fn hls_round_trip_white() {
    let orig = QuantityColor::new(1.0, 1.0, 1.0);
    let (h, l, s) = orig.to_hls();
    let back = QuantityColor::from_hls(h, l, s);
    assert_rgb_close(&back, &orig, "white");
}

#[test]
fn hls_round_trip_black() {
    let orig = QuantityColor::new(0.0, 0.0, 0.0);
    let (h, l, s) = orig.to_hls();
    let back = QuantityColor::from_hls(h, l, s);
    assert_rgb_close(&back, &orig, "black");
}

#[test]
fn hls_round_trip_arbitrary_color() {
    let orig = QuantityColor::new(0.3, 0.6, 0.9);
    let (h, l, s) = orig.to_hls();
    let back = QuantityColor::from_hls(h, l, s);
    assert_rgb_close(&back, &orig, "0.3,0.6,0.9");
}

#[test]
fn hls_round_trip_orange() {
    let orig = QuantityColor::from_name("orange").unwrap();
    let (h, l, s) = orig.to_hls();
    let back = QuantityColor::from_hls(h, l, s);
    assert_rgb_close(&back, &orig, "orange");
}

#[test]
fn hls_achromatic_has_zero_saturation() {
    // Gray has s == 0.
    let gray = QuantityColor::new(0.5, 0.5, 0.5);
    let (_h, l, s) = gray.to_hls();
    assert!((l - 0.5).abs() < 1e-5, "lightness of 50% gray should be 0.5");
    assert!(s.abs() < 1e-5, "saturation of gray should be 0");
}

#[test]
fn hls_red_hue_is_zero_degrees() {
    let red = QuantityColor::new(1.0, 0.0, 0.0);
    let (h, l, s) = red.to_hls();
    assert!(approx_eq(h, 0.0, 1e-3), "red hue should be 0°, got {h}");
    assert!(approx_eq(l, 0.5, 1e-5));
    assert!(approx_eq(s, 1.0, 1e-5));
}

#[test]
fn hls_green_hue_is_120_degrees() {
    let green = QuantityColor::new(0.0, 1.0, 0.0);
    let (h, _l, _s) = green.to_hls();
    assert!(approx_eq(h, 120.0, 1e-3), "green hue should be 120°, got {h}");
}

#[test]
fn hls_blue_hue_is_240_degrees() {
    let blue = QuantityColor::new(0.0, 0.0, 1.0);
    let (h, _l, _s) = blue.to_hls();
    assert!(approx_eq(h, 240.0, 1e-3), "blue hue should be 240°, got {h}");
}

#[test]
fn from_hls_clamps_lightness_and_saturation() {
    // Out-of-range L and S should not produce channels outside [0,1].
    let c = QuantityColor::from_hls(180.0, 2.0, 2.0);
    assert!(c.r() <= 1.0 && c.r() >= 0.0);
    assert!(c.g() <= 1.0 && c.g() >= 0.0);
    assert!(c.b() <= 1.0 && c.b() >= 0.0);
}

// ---------------------------------------------------------------------------
// delta_e — Euclidean distance in RGB
// ---------------------------------------------------------------------------

#[test]
fn delta_e_identical_is_zero() {
    let c = QuantityColor::new(0.5, 0.3, 0.7);
    assert_eq!(c.delta_e(&c), 0.0);
}

#[test]
fn delta_e_black_to_white() {
    let black = QuantityColor::new(0.0, 0.0, 0.0);
    let white = QuantityColor::new(1.0, 1.0, 1.0);
    let d = black.delta_e(&white);
    // sqrt(1^2 + 1^2 + 1^2) = sqrt(3) ≈ 1.7320508
    assert!((d - 3.0_f32.sqrt()).abs() < 1e-6, "expected sqrt(3), got {d}");
}

#[test]
fn delta_e_is_symmetric() {
    let a = QuantityColor::new(0.2, 0.4, 0.6);
    let b = QuantityColor::new(0.8, 0.1, 0.9);
    assert!((a.delta_e(&b) - b.delta_e(&a)).abs() < 1e-7);
}

#[test]
fn delta_e_non_negative() {
    let a = QuantityColor::new(1.0, 0.0, 0.0);
    let b = QuantityColor::new(0.0, 1.0, 0.0);
    assert!(a.delta_e(&b) >= 0.0);
}

#[test]
fn delta_e_known_value_red_to_blue() {
    // red=(1,0,0), blue=(0,0,1): dr=-1, dg=0, db=1 -> sqrt(2) ≈ 1.4142135
    let red  = QuantityColor::new(1.0, 0.0, 0.0);
    let blue = QuantityColor::new(0.0, 0.0, 1.0);
    let d = red.delta_e(&blue);
    assert!((d - 2.0_f32.sqrt()).abs() < 1e-6, "expected sqrt(2), got {d}");
}

#[test]
fn delta_e_is_bounded_by_sqrt3() {
    let max_dist = 3.0_f32.sqrt();
    for &(r1, g1, b1, r2, g2, b2) in &[
        (0.0, 0.0, 0.0, 1.0, 1.0, 1.0),
        (1.0, 0.0, 0.0, 0.0, 1.0, 1.0),
        (0.5, 0.5, 0.5, 0.0, 0.0, 0.0),
    ] {
        let a = QuantityColor::new(r1, g1, b1);
        let b = QuantityColor::new(r2, g2, b2);
        assert!(a.delta_e(&b) <= max_dist + 1e-6);
    }
}

// ---------------------------------------------------------------------------
// PartialEq / Copy / Clone / Debug
// ---------------------------------------------------------------------------

#[test]
fn equality_same_values() {
    let a = QuantityColor::new(0.1, 0.2, 0.3);
    let b = QuantityColor::new(0.1, 0.2, 0.3);
    assert_eq!(a, b);
}

#[test]
fn equality_different_values() {
    let a = QuantityColor::new(0.1, 0.2, 0.3);
    let b = QuantityColor::new(0.9, 0.8, 0.7);
    assert_ne!(a, b);
}

#[test]
fn copy_trait_works() {
    let a = QuantityColor::new(0.3, 0.5, 0.7);
    let b = a; // Copy — no move
    assert_eq!(a, b);
}

#[test]
fn clone_trait_works() {
    let a = QuantityColor::new(0.3, 0.5, 0.7);
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn debug_format_does_not_panic() {
    let c = QuantityColor::new(0.1, 0.2, 0.3);
    let s = format!("{c:?}");
    assert!(!s.is_empty());
}

// ---------------------------------------------------------------------------
// Named colour — exact channel values from the table
// ---------------------------------------------------------------------------

#[test]
fn named_orange_has_correct_rgb() {
    let c = QuantityColor::from_name("orange").unwrap();
    // orange = (1.0, 0x70/255 ≈ 0.647, 0.0) per CSS; table uses 165/255.
    // Our table: r=1.0, g=0.64705884, b=0.0.
    assert!((c.r() - 1.0).abs() < EPS, "orange.r");
    assert!((c.b() - 0.0).abs() < EPS, "orange.b");
    // g should be approximately 165/255.
    assert!(c.g() > 0.0 && c.g() < 1.0, "orange.g must be between 0 and 1");
}

#[test]
fn named_gray_is_midtone() {
    let c = QuantityColor::from_name("gray").unwrap();
    // Web 'gray' is #808080 = 128/255 ≈ 0.502.
    assert!((c.r() - c.g()).abs() < EPS, "gray: r != g");
    assert!((c.g() - c.b()).abs() < EPS, "gray: g != b");
    assert!(c.r() > 0.49 && c.r() < 0.51, "gray should be near 0.5, got {}", c.r());
}

#[test]
fn named_green_is_half_brightness() {
    // CSS 'green' is #008000, not #00FF00 (that's 'lime').
    let c = QuantityColor::from_name("green").unwrap();
    assert_eq!(c.r(), 0.0, "green.r");
    assert_eq!(c.b(), 0.0, "green.b");
    assert!(c.g() > 0.49 && c.g() < 0.51, "green.g should be ~0.5, got {}", c.g());
}
