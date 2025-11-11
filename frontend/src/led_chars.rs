//! LED character patterns for 5x7 dot matrix display
//!
//! This module contains bitmap patterns for rendering text on LED displays.
//! Each character is represented as a 5-bit wide by 7-bit tall pattern.
//!
//! # Pattern Format
//!
//! Each row is represented as a u8 with 5 significant bits (rightmost):
//!
//! ```text
//! Example 'A':
//! 01110  →  0b01110
//! 10001  →  0b10001
//! 10001  →  0b10001
//! 11111  →  0b11111
//! 10001  →  0b10001
//! 10001  →  0b10001
//! 10001  →  0b10001
//! ```

/// Gets the 5x7 LED pattern for a character
///
/// Returns a 7-element array where each element represents one row of the
/// character pattern. Each row is a 5-bit pattern stored in a u8.
///
/// # Arguments
/// * `c` - Character to get pattern for (case-insensitive)
///
/// # Returns
/// Array of 7 rows (top to bottom), each row is 5 bits (left to right)
///
/// # Examples
/// ```
/// let pattern = get_led_char_pattern('A');
/// // pattern[0] = 0b01110 (top row)
/// // pattern[6] = 0b10001 (bottom row)
/// ```
pub fn get_led_char_pattern(c: char) -> [u8; 7] {
    match c.to_ascii_uppercase() {
        'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'B' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
        ],
        'C' => [
            0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110,
        ],
        'D' => [
            0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        'F' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'G' => [
            0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111,
        ],
        'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'I' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111,
        ],
        'J' => [
            0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100,
        ],
        'K' => [
            0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
        ],
        'L' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        'M' => [
            0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
        ],
        'N' => [
            0b10001, 0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001,
        ],
        'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'P' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'Q' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101,
        ],
        'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        'S' => [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        'T' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'U' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'V' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
        ],
        'W' => [
            0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001,
        ],
        'X' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
        ],
        'Y' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'Z' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111,
        ],
        ' ' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000,
        ],
        _ => [
            // Default box pattern for unknown characters
            0b11111, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11111,
        ],
    }
}

/// Returns whether a character has a defined LED pattern
///
/// # Arguments
/// * `c` - Character to check
///
/// # Returns
/// `true` if the character has a specific pattern, `false` if it uses the default box
pub fn has_pattern(c: char) -> bool {
    matches!(
        c.to_ascii_uppercase(),
        'A'..='Z' | ' '
    )
}

/// Gets the width in pixels of the LED character (always 5)
pub const LED_CHAR_WIDTH: usize = 5;

/// Gets the height in pixels of the LED character (always 7)
pub const LED_CHAR_HEIGHT: usize = 7;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_a_pattern() {
        let pattern = get_led_char_pattern('A');
        assert_eq!(pattern[0], 0b01110); // Top row
        assert_eq!(pattern[3], 0b11111); // Middle bar
    }

    #[test]
    fn test_lowercase_converted_to_uppercase() {
        let upper = get_led_char_pattern('A');
        let lower = get_led_char_pattern('a');
        assert_eq!(upper, lower);
    }

    #[test]
    fn test_space_pattern() {
        let pattern = get_led_char_pattern(' ');
        assert_eq!(pattern, [0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_unknown_char_returns_box() {
        let pattern = get_led_char_pattern('!');
        assert_eq!(pattern[0], 0b11111); // Full top row
        assert_eq!(pattern[6], 0b11111); // Full bottom row
    }

    #[test]
    fn test_has_pattern() {
        assert!(has_pattern('A'));
        assert!(has_pattern('z'));
        assert!(has_pattern(' '));
        assert!(!has_pattern('!'));
        assert!(!has_pattern('1'));
    }
}
