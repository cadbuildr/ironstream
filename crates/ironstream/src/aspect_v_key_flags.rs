// FILE: aspect_v_key_flags.rs
// occt: Aspect_VKeyFlags

/// Key modifier flags for combining with general key from Aspect_VKey.
/// These are bit flags that can be combined with keyboard and mouse events.
pub type AspectVKeyFlags = u32;

/// No modifier key flags
pub const ASPECT_VKEY_FLAGS_NONE: AspectVKeyFlags = 0;

/// Shift key modifier flag (bit 8)
pub const ASPECT_VKEY_FLAGS_SHIFT: AspectVKeyFlags = 1 << 8;

/// Control key modifier flag (bit 9)
pub const ASPECT_VKEY_FLAGS_CTRL: AspectVKeyFlags = 1 << 9;

/// Alt key modifier flag (bit 10)
pub const ASPECT_VKEY_FLAGS_ALT: AspectVKeyFlags = 1 << 10;

/// Menu key modifier flag (bit 11)
pub const ASPECT_VKEY_FLAGS_MENU: AspectVKeyFlags = 1 << 11;

/// Meta key modifier flag (bit 12)
pub const ASPECT_VKEY_FLAGS_META: AspectVKeyFlags = 1 << 12;

/// All modifier key flags combined
pub const ASPECT_VKEY_FLAGS_ALL: AspectVKeyFlags =
  ASPECT_VKEY_FLAGS_SHIFT | ASPECT_VKEY_FLAGS_CTRL | ASPECT_VKEY_FLAGS_ALT
  | ASPECT_VKEY_FLAGS_MENU | ASPECT_VKEY_FLAGS_META;

/// Mouse button bitmask type
pub type AspectVKeyMouse = u32;

/// No mouse buttons
pub const ASPECT_VKEY_MOUSE_NONE: AspectVKeyMouse = 0;

/// Mouse left button (bit 13)
pub const ASPECT_VKEY_MOUSE_LEFT_BUTTON: AspectVKeyMouse = 1 << 13;

/// Mouse middle button / scroll (bit 14)
pub const ASPECT_VKEY_MOUSE_MIDDLE_BUTTON: AspectVKeyMouse = 1 << 14;

/// Mouse right button (bit 15)
pub const ASPECT_VKEY_MOUSE_RIGHT_BUTTON: AspectVKeyMouse = 1 << 15;

/// All main mouse buttons combined
pub const ASPECT_VKEY_MOUSE_MAIN_BUTTONS: AspectVKeyMouse =
  ASPECT_VKEY_MOUSE_LEFT_BUTTON | ASPECT_VKEY_MOUSE_MIDDLE_BUTTON | ASPECT_VKEY_MOUSE_RIGHT_BUTTON;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_vkey_flags_bit_positions() {
    // Shift is at bit 8
    assert_eq!(ASPECT_VKEY_FLAGS_SHIFT, 256);

    // Ctrl is at bit 9
    assert_eq!(ASPECT_VKEY_FLAGS_CTRL, 512);

    // Alt is at bit 10
    assert_eq!(ASPECT_VKEY_FLAGS_ALT, 1024);

    // Menu is at bit 11
    assert_eq!(ASPECT_VKEY_FLAGS_MENU, 2048);

    // Meta is at bit 12
    assert_eq!(ASPECT_VKEY_FLAGS_META, 4096);
  }

  #[test]
  fn test_vkey_flags_none() {
    assert_eq!(ASPECT_VKEY_FLAGS_NONE, 0);
  }

  #[test]
  fn test_vkey_flags_all() {
    let expected = ASPECT_VKEY_FLAGS_SHIFT
      | ASPECT_VKEY_FLAGS_CTRL
      | ASPECT_VKEY_FLAGS_ALT
      | ASPECT_VKEY_FLAGS_MENU
      | ASPECT_VKEY_FLAGS_META;
    assert_eq!(ASPECT_VKEY_FLAGS_ALL, expected);
  }

  #[test]
  fn test_mouse_button_bit_positions() {
    // Left button is at bit 13
    assert_eq!(ASPECT_VKEY_MOUSE_LEFT_BUTTON, 8192);

    // Middle button is at bit 14
    assert_eq!(ASPECT_VKEY_MOUSE_MIDDLE_BUTTON, 16384);

    // Right button is at bit 15
    assert_eq!(ASPECT_VKEY_MOUSE_RIGHT_BUTTON, 32768);
  }

  #[test]
  fn test_mouse_button_main_buttons() {
    let expected = ASPECT_VKEY_MOUSE_LEFT_BUTTON
      | ASPECT_VKEY_MOUSE_MIDDLE_BUTTON
      | ASPECT_VKEY_MOUSE_RIGHT_BUTTON;
    assert_eq!(ASPECT_VKEY_MOUSE_MAIN_BUTTONS, expected);
  }

  #[test]
  fn test_mouse_button_none() {
    assert_eq!(ASPECT_VKEY_MOUSE_NONE, 0);
  }

  #[test]
  fn test_flag_combinations() {
    // Test combining flags with bitwise OR
    let shift_and_ctrl = ASPECT_VKEY_FLAGS_SHIFT | ASPECT_VKEY_FLAGS_CTRL;
    assert_eq!(shift_and_ctrl, 768);

    // Test combining all flags
    let all_flags = ASPECT_VKEY_FLAGS_SHIFT
      | ASPECT_VKEY_FLAGS_CTRL
      | ASPECT_VKEY_FLAGS_ALT
      | ASPECT_VKEY_FLAGS_MENU
      | ASPECT_VKEY_FLAGS_META;
    assert_eq!(all_flags, ASPECT_VKEY_FLAGS_ALL);
  }

  #[test]
  fn test_bit_isolation() {
    // Test that flags can be isolated with bitwise AND
    let combined = ASPECT_VKEY_FLAGS_SHIFT | ASPECT_VKEY_FLAGS_CTRL;
    assert_eq!(combined & ASPECT_VKEY_FLAGS_SHIFT, ASPECT_VKEY_FLAGS_SHIFT);
    assert_eq!(combined & ASPECT_VKEY_FLAGS_CTRL, ASPECT_VKEY_FLAGS_CTRL);
    assert_eq!(combined & ASPECT_VKEY_FLAGS_ALT, 0);
  }
}
