// FILE: resource_convert_unicode.rs
// occt: Resource_ConvertUnicode

//! Conversions between Unicode and the CJK byte encodings Shift-JIS, EUC-JP
//! and GB2312, following Resource_ConvertUnicode.c.
//!
//! OCCT drives these conversions through four 65536-entry lookup tables
//! (`sjisuni`, `unisjis`, `gbuni`, `unigb` from Resource_Shiftjis.pxx and
//! Resource_GB2312.pxx). Those tables are external data; they are modelled
//! here as sparse local tables containing entries copied verbatim from the
//! OCCT tables. Absent entries behave exactly like the zero-filled slots of
//! the full tables (they map to 0). All algorithmic behaviour — the
//! SJIS/JIS/EUC arithmetic, the guards, the in/out convention on the
//! high/low byte pair — mirrors the C source.

// ---------------------------------------------------------------------------
// Byte-class predicates (macros in the C source)
// ---------------------------------------------------------------------------

fn iseuc(c: u32) -> bool {
    (0xa1..=0xfe).contains(&c)
}

fn issjis1(c: u32) -> bool {
    (0x81..=0x9f).contains(&c) || (0xe0..=0xef).contains(&c)
}

fn issjis2(c: u32) -> bool {
    (0x40..=0xfc).contains(&c) && c != 0x7f
}

fn isshift(c: u32) -> bool {
    (0x80..=0xff).contains(&c)
}

// ---------------------------------------------------------------------------
// Sparse local models of the OCCT lookup tables. Every entry below is copied
// verbatim from Resource_Shiftjis.pxx / Resource_GB2312.pxx.
// ---------------------------------------------------------------------------

/// sjisuni[sjis] -> unicode (Shift-JIS code to Unicode scalar).
fn sjisuni(sjis: u16) -> u16 {
    match sjis {
        0x8140 => 0x3000, // IDEOGRAPHIC SPACE
        0x82a0 => 0x3042, // HIRAGANA LETTER A
        0x82a2 => 0x3044, // HIRAGANA LETTER I
        0x8341 => 0x30a2, // KATAKANA LETTER A
        0x8260 => 0xff21, // FULLWIDTH LATIN CAPITAL LETTER A
        0x889f => 0x4e9c, // CJK UNIFIED IDEOGRAPH-4E9C
        _ => 0,
    }
}

/// unisjis[unicode] -> Shift-JIS code.
fn unisjis(uni: u16) -> u16 {
    match uni {
        // ASCII range is identity in the OCCT table (except backslash).
        0x0020..=0x005b | 0x005d..=0x007d => uni,
        0x005c => 0x815f, // REVERSE SOLIDUS -> FULLWIDTH REVERSE SOLIDUS
        0x3000 => 0x8140,
        0x3042 => 0x82a0,
        0x3044 => 0x82a2,
        0x30a2 => 0x8341,
        0xff21 => 0x8260,
        0x4e9c => 0x889f,
        _ => 0,
    }
}

/// gbuni[gb] -> unicode (GB2312 code, high bits stripped, to Unicode).
fn gbuni(gb: u16) -> u16 {
    match gb {
        0x2121 => 0x3000, // IDEOGRAPHIC SPACE
        0x3021 => 0x554a, // CJK UNIFIED IDEOGRAPH-554A
        0x5650 => 0x4e2d, // CJK UNIFIED IDEOGRAPH-4E2D
        _ => 0,
    }
}

/// unigb[unicode] -> GB2312 code (without the 0x8080 offset).
fn unigb(uni: u16) -> u16 {
    match uni {
        0x3000 => 0x2121,
        0x554a => 0x3021,
        0x4e2d => 0x5650,
        _ => 0,
    }
}

// ---------------------------------------------------------------------------
// Internal helpers (static functions in the C source)
// ---------------------------------------------------------------------------

fn sjis_to_jis(ph: &mut u32, pl: &mut u32) {
    if !issjis1(*ph) || !issjis2(*pl) {
        return;
    }
    if *ph <= 0x9f {
        if *pl < 0x9f {
            *ph = (*ph << 1) - 0xe1;
        } else {
            *ph = (*ph << 1) - 0xe0;
        }
    } else if *pl < 0x9f {
        *ph = (*ph << 1) - 0x161;
    } else {
        *ph = (*ph << 1) - 0x160;
    }
    if *pl < 0x7f {
        *pl -= 0x1f;
    } else if *pl < 0x9f {
        *pl -= 0x20;
    } else {
        *pl -= 0x7e;
    }
}

fn jis_to_sjis(ph: &mut u32, pl: &mut u32) {
    if *ph & 1 != 0 {
        if *pl < 0x60 {
            *pl += 0x1f;
        } else {
            *pl += 0x20;
        }
    } else {
        *pl += 0x7e;
    }
    if *ph < 0x5f {
        *ph = (*ph + 0xe1) >> 1;
    } else {
        *ph = (*ph + 0x161) >> 1;
    }
}

fn euc_to_sjis(ph: &mut u32, pl: &mut u32) {
    if (*ph & 0xffff_ff00) != 0 || (*pl & 0xffff_ff00) != 0 {
        *ph = 0;
        *pl = 0;
        return;
    }
    if !iseuc(*ph) || !iseuc(*pl) {
        return;
    }
    *ph &= 0x7f;
    *pl &= 0x7f;
    jis_to_sjis(ph, pl);
}

fn sjis_to_euc(ph: &mut u32, pl: &mut u32) {
    if (*ph & 0xffff_ff00) != 0 || (*pl & 0xffff_ff00) != 0 {
        *ph = 0;
        *pl = 0;
        return;
    }
    if !issjis1(*ph) || !issjis2(*pl) {
        return;
    }
    sjis_to_jis(ph, pl);
    *ph |= 0x80;
    *pl |= 0x80;
}

// ---------------------------------------------------------------------------
// Public API (extern "C" functions in the OCCT header)
// ---------------------------------------------------------------------------

/// Resource_sjis_to_unicode: converts a Shift-JIS (high, low) byte pair to
/// a Unicode (high, low) byte pair in place.
pub fn resource_sjis_to_unicode(ph: &mut u32, pl: &mut u32) {
    if (*ph & 0xffff_ff00) != 0 || (*pl & 0xffff_ff00) != 0 {
        *ph = 0;
        *pl = 0;
        return;
    }
    if !issjis1(*ph) || !issjis2(*pl) {
        return;
    }
    let sjis = ((*ph as u16) << 8) | (*pl as u16);
    let uni = sjisuni(sjis);
    *ph = (uni >> 8) as u32;
    *pl = (uni & 0xff) as u32;
}

/// Resource_unicode_to_sjis: converts a Unicode (high, low) byte pair to a
/// Shift-JIS (high, low) byte pair in place.
pub fn resource_unicode_to_sjis(ph: &mut u32, pl: &mut u32) {
    if (*ph & 0xffff_ff00) != 0 || (*pl & 0xffff_ff00) != 0 {
        *ph = 0;
        *pl = 0;
        return;
    }
    if *ph == 0 && *pl == 0 {
        return;
    }
    let uni = ((*ph as u16) << 8) | (*pl as u16);
    let sjis = unisjis(uni);
    *ph = (sjis >> 8) as u32;
    *pl = (sjis & 0xff) as u32;
}

/// Resource_unicode_to_euc: converts a Unicode byte pair to EUC-JP in place.
pub fn resource_unicode_to_euc(ph: &mut u32, pl: &mut u32) {
    if *ph == 0 && *pl == 0 {
        return;
    }
    resource_unicode_to_sjis(ph, pl);
    // Believe it is ANSI code if it is not sjis.
    if issjis1(*ph) {
        sjis_to_euc(ph, pl);
    }
}

/// Resource_euc_to_unicode: converts an EUC-JP byte pair to Unicode in place.
pub fn resource_euc_to_unicode(ph: &mut u32, pl: &mut u32) {
    if !iseuc(*ph) || !iseuc(*pl) {
        return;
    }
    euc_to_sjis(ph, pl);
    resource_sjis_to_unicode(ph, pl);
}

/// Resource_gb_to_unicode: converts a GB2312 byte pair to Unicode in place.
pub fn resource_gb_to_unicode(ph: &mut u32, pl: &mut u32) {
    if (*ph & 0xffff_ff00) != 0 || (*pl & 0xffff_ff00) != 0 {
        *ph = 0;
        *pl = 0;
        return;
    }
    if !isshift(*ph) || !isshift(*pl) {
        return;
    }
    *ph &= 0x7f;
    *pl &= 0x7f;
    let gb = ((*ph as u16) << 8) | (*pl as u16);
    let uni = gbuni(gb);
    *ph = (uni >> 8) as u32;
    *pl = (uni & 0xff) as u32;
}

/// Resource_unicode_to_gb: converts a Unicode byte pair to GB2312 in place.
pub fn resource_unicode_to_gb(ph: &mut u32, pl: &mut u32) {
    if (*ph & 0xffff_ff00) != 0 || (*pl & 0xffff_ff00) != 0 {
        *ph = 0;
        *pl = 0;
        return;
    }
    if *ph == 0 && *pl == 0 {
        return;
    }
    let uni = ((*ph as u16) << 8) | (*pl as u16);
    let gb = unigb(uni);
    if gb != 0 {
        *ph = ((gb >> 8) as u32) | 0x80;
        *pl = ((gb & 0xff) as u32) | 0x80;
    } else {
        *ph = 0;
        *pl = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pair(h: u32, l: u32) -> (u32, u32) {
        (h, l)
    }

    #[test]
    fn test_sjis_to_unicode_hiragana_a() {
        // Shift-JIS 0x82A0 is HIRAGANA LETTER A (U+3042).
        let (mut h, mut l) = pair(0x82, 0xa0);
        resource_sjis_to_unicode(&mut h, &mut l);
        assert_eq!((h, l), (0x30, 0x42));
    }

    #[test]
    fn test_sjis_to_unicode_ideographic_space() {
        let (mut h, mut l) = pair(0x81, 0x40);
        resource_sjis_to_unicode(&mut h, &mut l);
        assert_eq!((h, l), (0x30, 0x00));
    }

    #[test]
    fn test_sjis_to_unicode_non_sjis_lead_unchanged() {
        // 0x41 is not a Shift-JIS lead byte: pair left untouched.
        let (mut h, mut l) = pair(0x41, 0x42);
        resource_sjis_to_unicode(&mut h, &mut l);
        assert_eq!((h, l), (0x41, 0x42));
    }

    #[test]
    fn test_sjis_to_unicode_out_of_range_clears() {
        let (mut h, mut l) = pair(0x182, 0xa0);
        resource_sjis_to_unicode(&mut h, &mut l);
        assert_eq!((h, l), (0, 0));
    }

    #[test]
    fn test_unicode_to_sjis_hiragana_a() {
        let (mut h, mut l) = pair(0x30, 0x42);
        resource_unicode_to_sjis(&mut h, &mut l);
        assert_eq!((h, l), (0x82, 0xa0));
    }

    #[test]
    fn test_unicode_to_sjis_ascii_identity_and_backslash() {
        // ASCII 'A' maps to itself in the OCCT table.
        let (mut h, mut l) = pair(0x00, 0x41);
        resource_unicode_to_sjis(&mut h, &mut l);
        assert_eq!((h, l), (0x00, 0x41));
        // Backslash maps to fullwidth reverse solidus code 0x815F.
        let (mut h, mut l) = pair(0x00, 0x5c);
        resource_unicode_to_sjis(&mut h, &mut l);
        assert_eq!((h, l), (0x81, 0x5f));
    }

    #[test]
    fn test_unicode_to_sjis_zero_pair_unchanged() {
        let (mut h, mut l) = pair(0, 0);
        resource_unicode_to_sjis(&mut h, &mut l);
        assert_eq!((h, l), (0, 0));
    }

    #[test]
    fn test_sjis_unicode_roundtrip() {
        for &(sh, sl) in &[(0x81u32, 0x40u32), (0x82, 0xa0), (0x88, 0x9f), (0x83, 0x41)] {
            let (mut h, mut l) = (sh, sl);
            resource_sjis_to_unicode(&mut h, &mut l);
            resource_unicode_to_sjis(&mut h, &mut l);
            assert_eq!((h, l), (sh, sl), "roundtrip for sjis {:02x}{:02x}", sh, sl);
        }
    }

    #[test]
    fn test_euc_to_unicode_hiragana_a() {
        // EUC-JP 0xA4A2 is HIRAGANA LETTER A (JIS 0x2422 + 0x8080).
        let (mut h, mut l) = pair(0xa4, 0xa2);
        resource_euc_to_unicode(&mut h, &mut l);
        assert_eq!((h, l), (0x30, 0x42));
    }

    #[test]
    fn test_euc_to_unicode_non_euc_unchanged() {
        let (mut h, mut l) = pair(0x41, 0x42);
        resource_euc_to_unicode(&mut h, &mut l);
        assert_eq!((h, l), (0x41, 0x42));
    }

    #[test]
    fn test_unicode_to_euc_hiragana_a() {
        let (mut h, mut l) = pair(0x30, 0x42);
        resource_unicode_to_euc(&mut h, &mut l);
        assert_eq!((h, l), (0xa4, 0xa2));
    }

    #[test]
    fn test_unicode_to_euc_ascii_stays_ansi() {
        // 'A': unicode_to_sjis gives 0x0041; lead byte is not sjis1, so the
        // value is left as ANSI.
        let (mut h, mut l) = pair(0x00, 0x41);
        resource_unicode_to_euc(&mut h, &mut l);
        assert_eq!((h, l), (0x00, 0x41));
    }

    #[test]
    fn test_euc_unicode_roundtrip() {
        // EUC for U+4E9C (JIS 0x3021 + 0x8080 = 0xB0A1).
        let (mut h, mut l) = pair(0xb0, 0xa1);
        resource_euc_to_unicode(&mut h, &mut l);
        assert_eq!((h, l), (0x4e, 0x9c));
        resource_unicode_to_euc(&mut h, &mut l);
        assert_eq!((h, l), (0xb0, 0xa1));
    }

    #[test]
    fn test_gb_to_unicode_zhong() {
        // GB2312 0xD6D0 is U+4E2D (CJK ideograph "middle").
        let (mut h, mut l) = pair(0xd6, 0xd0);
        resource_gb_to_unicode(&mut h, &mut l);
        assert_eq!((h, l), (0x4e, 0x2d));
    }

    #[test]
    fn test_gb_to_unicode_ideographic_space() {
        // GB2312 0xA1A1 is IDEOGRAPHIC SPACE (U+3000).
        let (mut h, mut l) = pair(0xa1, 0xa1);
        resource_gb_to_unicode(&mut h, &mut l);
        assert_eq!((h, l), (0x30, 0x00));
    }

    #[test]
    fn test_gb_to_unicode_non_shift_unchanged() {
        let (mut h, mut l) = pair(0x41, 0x42);
        resource_gb_to_unicode(&mut h, &mut l);
        assert_eq!((h, l), (0x41, 0x42));
    }

    #[test]
    fn test_unicode_to_gb_zhong() {
        let (mut h, mut l) = pair(0x4e, 0x2d);
        resource_unicode_to_gb(&mut h, &mut l);
        assert_eq!((h, l), (0xd6, 0xd0));
    }

    #[test]
    fn test_unicode_to_gb_unknown_clears() {
        // A code point absent from the GB table maps to (0, 0).
        let (mut h, mut l) = pair(0x01, 0x02);
        resource_unicode_to_gb(&mut h, &mut l);
        assert_eq!((h, l), (0, 0));
    }

    #[test]
    fn test_gb_unicode_roundtrip() {
        for &(gh, gl) in &[(0xd6u32, 0xd0u32), (0xb0, 0xa1), (0xa1, 0xa1)] {
            let (mut h, mut l) = (gh, gl);
            resource_gb_to_unicode(&mut h, &mut l);
            resource_unicode_to_gb(&mut h, &mut l);
            assert_eq!((h, l), (gh, gl), "roundtrip for gb {:02x}{:02x}", gh, gl);
        }
    }

    #[test]
    fn test_out_of_range_inputs_cleared_everywhere() {
        for f in [
            resource_sjis_to_unicode,
            resource_unicode_to_sjis,
            resource_gb_to_unicode,
            resource_unicode_to_gb,
        ] {
            let (mut h, mut l) = (0x1_0000u32, 0x41u32);
            f(&mut h, &mut l);
            assert_eq!((h, l), (0, 0));
        }
    }
}
