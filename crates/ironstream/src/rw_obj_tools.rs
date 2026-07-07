// FILE: rw_obj_tools.rs
// occt: RWObj_Tools

//! Auxiliary tools for OBJ format parser (namespace `RWObj_Tools`).
//! Faithful port of the inline helpers:
//! - `ReadVec3` (float and double variants) — parse 3 numbers with
//!   strtod semantics, success iff the third number consumed input;
//! - `ReadName` — trim CR/LF and surrounding blanks, false when empty;
//! - `isSpaceChar` — OBJ white space is space or tab only.

/// strtod-like scan: skips leading whitespace, parses an optional-signed
/// decimal number with optional exponent. Returns the value and the number
/// of bytes consumed (0 when no conversion was performed, like strtod
/// leaving end == start).
fn rwobj_strtod_scan(input: &str) -> (f64, usize) {
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() && (bytes[i] as char).is_whitespace() {
        i += 1;
    }
    let num_start = i;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
    }
    let mut saw_digit = false;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
        saw_digit = true;
    }
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
            saw_digit = true;
        }
    }
    if !saw_digit {
        return (0.0, 0);
    }
    let mut end = i;
    // Optional exponent.
    if i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
        let mut j = i + 1;
        if j < bytes.len() && (bytes[j] == b'+' || bytes[j] == b'-') {
            j += 1;
        }
        let exp_digits_start = j;
        while j < bytes.len() && bytes[j].is_ascii_digit() {
            j += 1;
        }
        if j > exp_digits_start {
            end = j;
        }
    }
    let val: f64 = input[num_start..end].parse().unwrap_or(0.0);
    (val, end)
}

/// `RWObj_Tools::ReadVec3` (double variant): reads 3 values; returns
/// (vec, rest, ok) where ok is true iff the third conversion consumed
/// characters (`aPos != theNext` in OCCT).
pub fn rwobj_read_vec3_f64(pos: &str) -> ([f64; 3], &str, bool) {
    let (x, n1) = rwobj_strtod_scan(pos);
    let s1 = &pos[n1..];
    let (y, n2) = rwobj_strtod_scan(s1);
    let s2 = &s1[n2..];
    let (z, n3) = rwobj_strtod_scan(s2);
    let rest = &s2[n3..];
    ([x, y, z], rest, n3 != 0)
}

/// `RWObj_Tools::ReadVec3` (float variant).
pub fn rwobj_read_vec3_f32(pos: &str) -> ([f32; 3], &str, bool) {
    let ([x, y, z], rest, ok) = rwobj_read_vec3_f64(pos);
    ([x as f32, y as f32, z as f32], rest, ok)
}

/// `RWObj_Tools::ReadName`: strips trailing `\n`/`\r`, right- and
/// left-adjusts blanks; returns None when the result is empty
/// (OCCT returns false and clears the name).
pub fn rwobj_read_name(pos: &str) -> Option<String> {
    let mut tail = pos.len();
    let bytes = pos.as_bytes();
    if tail > 0 && bytes[tail - 1] == b'\n' {
        tail -= 1;
    }
    if tail > 0 && bytes[tail - 1] == b'\r' {
        tail -= 1;
    }
    // RightAdjust / LeftAdjust over whitespace.
    let trimmed = pos[..tail].trim_matches(|c: char| c.is_whitespace());
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// `RWObj_Tools::isSpaceChar`: true only for space and tab.
pub fn rwobj_is_space_char(c: char) -> bool {
    c == ' ' || c == '\t'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_vec3_double_parses_vertex_line_payload() {
        let ([x, y, z], rest, ok) = rwobj_read_vec3_f64(" 1.0 -2.5 3e2\n");
        assert!(ok);
        assert_eq!(x, 1.0);
        assert_eq!(y, -2.5);
        assert_eq!(z, 300.0);
        assert_eq!(rest, "\n");
    }

    #[test]
    fn read_vec3_fails_on_two_components() {
        let (_, _, ok) = rwobj_read_vec3_f64("1.0 2.0");
        assert!(!ok, "third strtod consumed nothing -> failure");
        let (_, _, ok2) = rwobj_read_vec3_f64("");
        assert!(!ok2);
    }

    #[test]
    fn read_vec3_float_variant() {
        let ([x, y, z], _, ok) = rwobj_read_vec3_f32("0.5 0.25 0.125 extra");
        assert!(ok);
        assert_eq!((x, y, z), (0.5f32, 0.25, 0.125));
    }

    #[test]
    fn read_name_trims_crlf_and_blanks() {
        assert_eq!(rwobj_read_name("  my group \r\n").as_deref(), Some("my group"));
        assert_eq!(rwobj_read_name("steel\n").as_deref(), Some("steel"));
        assert_eq!(rwobj_read_name("plain").as_deref(), Some("plain"));
    }

    #[test]
    fn read_name_empty_is_none() {
        assert!(rwobj_read_name("").is_none());
        assert!(rwobj_read_name("   \r\n").is_none());
        assert!(rwobj_read_name("\n").is_none());
    }

    #[test]
    fn space_char_is_space_or_tab_only() {
        assert!(rwobj_is_space_char(' '));
        assert!(rwobj_is_space_char('\t'));
        assert!(!rwobj_is_space_char('\n'));
        assert!(!rwobj_is_space_char('a'));
    }
}
