// FILE: rust/ironstream/crates/ironstream/src/quantity_color.rs
//! `Quantity_Color` — color with multiple representations, mirroring
//! OpenCascade's `Quantity_Color` class (`src/Visualization/TKService/Quantity`).
//!
//! Colors are stored internally as linear-sRGB `(r, g, b)` in `[0, 1]`.
//! Conversion to HLS, hex, and named colors is provided, along with a
//! perceptual distance metric (`delta_e`). Zero external dependencies: `std` only.

// occt: Quantity_TypeOfColor
/// The color model / space used to interpret three float coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuantityColorModel {
    /// Standard red-green-blue, each channel in `[0, 1]`.
    Rgb,
    /// Hue (degrees `[0, 360)`), Lightness `[0, 1]`, Saturation `[0, 1]`.
    Hls,
    /// CIEDE2000 perceptual space (placeholder variant; not used as storage).
    Ciede2000,
}

// occt: Quantity_Color
/// A color stored as `(r, g, b)` in `[0, 1]` linear-sRGB.
///
/// Mirrors `Quantity_Color` from OpenCascade's `Quantity` package. Channels are
/// always clamped to `[0, 1]` on construction. All conversion methods are
/// pure functions with no allocation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuantityColor {
    r: f32,
    g: f32,
    b: f32,
}

// ---------------------------------------------------------------------------
// Named-color table
// ---------------------------------------------------------------------------

/// A single entry in the built-in name table.
struct NamedColor {
    name: &'static str,
    r: f32,
    g: f32,
    b: f32,
}

/// The ten recognised named colors, in the same order used for forward and
/// reverse lookups. Tolerance for reverse lookup is 1/255 per channel.
const NAMED_COLORS: &[NamedColor] = &[
    NamedColor { name: "black",   r: 0.0,            g: 0.0,            b: 0.0 },
    NamedColor { name: "white",   r: 1.0,            g: 1.0,            b: 1.0 },
    NamedColor { name: "red",     r: 1.0,            g: 0.0,            b: 0.0 },
    NamedColor { name: "green",   r: 0.0,            g: 0.5019608,      b: 0.0 },
    NamedColor { name: "blue",    r: 0.0,            g: 0.0,            b: 1.0 },
    NamedColor { name: "yellow",  r: 1.0,            g: 1.0,            b: 0.0 },
    NamedColor { name: "cyan",    r: 0.0,            g: 1.0,            b: 1.0 },
    NamedColor { name: "magenta", r: 1.0,            g: 0.0,            b: 1.0 },
    NamedColor { name: "orange",  r: 1.0,            g: 0.64705884,     b: 0.0 },
    NamedColor { name: "gray",    r: 0.5019608,      g: 0.5019608,      b: 0.5019608 },
];

/// Per-channel tolerance used for the reverse name lookup (`1 / 255`).
const NAME_TOL: f32 = 1.0 / 255.0;

// ---------------------------------------------------------------------------
// HLS helpers
// ---------------------------------------------------------------------------

/// Convert HLS `(hue_deg, lightness, saturation)` to RGB `(r, g, b)`.
///
/// `hue_deg` is in degrees `[0, 360)`, L and S are in `[0, 1]`.
fn hls_to_rgb(h: f32, l: f32, s: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (l, l, l);
    }
    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;
    let r = hue_to_channel(p, q, h + 120.0);
    let g = hue_to_channel(p, q, h);
    let b = hue_to_channel(p, q, h - 120.0);
    (r, g, b)
}

/// Helper used by [`hls_to_rgb`] — maps a single hue sector to a channel value.
fn hue_to_channel(p: f32, q: f32, mut t: f32) -> f32 {
    // Normalise t into [0, 360).
    if t < 0.0 {
        t += 360.0;
    }
    if t > 360.0 {
        t -= 360.0;
    }
    if t < 60.0 {
        p + (q - p) * t / 60.0
    } else if t < 180.0 {
        q
    } else if t < 240.0 {
        p + (q - p) * (240.0 - t) / 60.0
    } else {
        p
    }
}

/// Convert RGB `(r, g, b)` to HLS `(hue_deg, lightness, saturation)`.
///
/// Returns hue in degrees `[0, 360)`.
fn rgb_to_hls(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) * 0.5;

    if (max - min).abs() < f32::EPSILON {
        return (0.0, l, 0.0); // achromatic
    }

    let delta = max - min;
    let s = if l < 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };

    let h_raw = if (max - r).abs() < f32::EPSILON {
        (g - b) / delta + if g < b { 6.0 } else { 0.0 }
    } else if (max - g).abs() < f32::EPSILON {
        (b - r) / delta + 2.0
    } else {
        (r - g) / delta + 4.0
    };

    let h = (h_raw * 60.0).rem_euclid(360.0);
    (h, l, s)
}

// ---------------------------------------------------------------------------
// QuantityColor implementation
// ---------------------------------------------------------------------------

impl QuantityColor {
    /// Construct from `(r, g, b)` components; each channel is clamped to
    /// `[0, 1]`.
    // occt: Quantity_Color // ::Quantity_Color(r, g, b, Quantity_TOC_RGB)
    #[inline]
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
        }
    }

    /// Construct from a packed 24-bit hex value `0xRRGGBB`.
    ///
    /// # Example
    /// ```
    /// # use ironstream::quantity_color::QuantityColor;
    /// let orange = QuantityColor::from_hex(0xFF_88_00);
    /// assert!((orange.r() - 1.0).abs() < 1e-6);
    /// ```
    // occt: Quantity_Color // ::Quantity_Color(theHex)
    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8)  & 0xFF) as f32 / 255.0;
        let b = ( hex        & 0xFF) as f32 / 255.0;
        Self::new(r, g, b)
    }

    /// Construct from a CSS-style color name.
    ///
    /// Recognised names (case-sensitive): `"black"`, `"white"`, `"red"`,
    /// `"green"`, `"blue"`, `"yellow"`, `"cyan"`, `"magenta"`, `"orange"`,
    /// `"gray"`.
    ///
    /// Returns `None` for unrecognised names.
    // occt: Quantity_Color // ::ColorFromName
    pub fn from_name(name: &str) -> Option<Self> {
        NAMED_COLORS
            .iter()
            .find(|e| e.name == name)
            .map(|e| Self { r: e.r, g: e.g, b: e.b })
    }

    // -- Accessors -----------------------------------------------------------

    /// Red channel, in `[0, 1]`.
    #[inline]
    pub fn r(&self) -> f32 {
        self.r
    }

    /// Green channel, in `[0, 1]`.
    #[inline]
    pub fn g(&self) -> f32 {
        self.g
    }

    /// Blue channel, in `[0, 1]`.
    #[inline]
    pub fn b(&self) -> f32 {
        self.b
    }

    /// RGBA with `alpha = 1.0`.
    // occt: Quantity_Color // has no built-in alpha; added for convenience.
    #[inline]
    pub fn rgba(&self) -> [f32; 4] {
        [self.r, self.g, self.b, 1.0]
    }

    // -- Conversions ---------------------------------------------------------

    /// Pack as `0xRRGGBB`.
    // occt: Quantity_Color // ::PackedColor / StringName
    pub fn to_hex(&self) -> u32 {
        let r = (self.r * 255.0).round() as u32;
        let g = (self.g * 255.0).round() as u32;
        let b = (self.b * 255.0).round() as u32;
        (r << 16) | (g << 8) | b
    }

    /// Convert to HLS `(hue_deg, lightness, saturation)`.
    ///
    /// Hue is in `[0, 360)`, lightness and saturation are in `[0, 1]`.
    // occt: Quantity_Color // ::HlsValues
    pub fn to_hls(&self) -> (f32, f32, f32) {
        rgb_to_hls(self.r, self.g, self.b)
    }

    /// Construct from HLS `(hue_deg, lightness, saturation)`.
    ///
    /// Channels are clamped after conversion.
    // occt: Quantity_Color // ::Quantity_Color(h, l, s, Quantity_TOC_HLS)
    pub fn from_hls(h: f32, l: f32, s: f32) -> Self {
        let (r, g, b) = hls_to_rgb(h, l.clamp(0.0, 1.0), s.clamp(0.0, 1.0));
        Self::new(r, g, b)
    }

    /// Reverse-lookup: return the canonical name if this color matches one of
    /// the ten named colors within a per-channel tolerance of `1/255`.
    // occt: Quantity_Color // ::StringName
    pub fn name(&self) -> Option<&'static str> {
        NAMED_COLORS.iter().find(|e| {
            (e.r - self.r).abs() <= NAME_TOL
                && (e.g - self.g).abs() <= NAME_TOL
                && (e.b - self.b).abs() <= NAME_TOL
        }).map(|e| e.name)
    }

    /// Perceptual distance to `other` as the Euclidean distance in RGB space
    /// (a simple approximation; the full CIEDE2000 formula requires Lab
    /// conversion and is left to higher-level callers).
    ///
    /// Returns a value in `[0, sqrt(3)]`; identical colors give `0.0`.
    // occt: Quantity_Color // ::Distance (CIEDE2000 note — simplified here)
    pub fn delta_e(&self, other: &QuantityColor) -> f32 {
        let dr = self.r - other.r;
        let dg = self.g - other.g;
        let db = self.b - other.b;
        (dr * dr + dg * dg + db * db).sqrt()
    }
}

impl Default for QuantityColor {
    /// Default is black `(0, 0, 0)`.
    fn default() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0 }
    }
}
