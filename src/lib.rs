// Copyright (c) 2024 Experimental-Projects-NAS
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

const CHARSETS: [char; 256] = [
    'à', 'á', 'â', 'ã', 'ä', 'å', 'æ', 'ç',
    'è', 'é', 'ê', 'ë', 'ì', 'í', 'î', 'ï',
    'ð', 'ñ', 'ò', 'ó', 'ô', 'õ', 'ö', '÷',
    'ø', 'ù', 'ú', 'û', 'ü', 'ý', 'þ', 'ÿ', 
    'Ā', 'ā', 'Ă', 'ă', 'Ą', 'ą', 'Ć', 'ć',
    'Ĉ', 'ĉ', 'Ċ', 'ċ', 'Č', 'č', 'Ď', 'ď',
    'Đ', 'đ', 'Ē', 'ē', 'Ĕ', 'ĕ', 'Ė', 'ė',
    'Ę', 'ę', 'Ě', 'ě', 'Ĝ', 'ĝ', 'Ğ', 'ğ', 
    'Ġ', 'ġ', 'Ģ', 'ģ', 'Ĥ', 'ĥ', 'Ħ', 'ħ',
    'Ĩ', 'ĩ', 'Ī', 'ī', 'Ĭ', 'ĭ', 'Į', 'į',
    'İ', 'ı', 'Ĳ', 'ĳ', 'Ĵ', 'ĵ', 'Ķ', 'ķ',
    'ĸ', 'Ĺ', 'ĺ', 'Ļ', 'ļ', 'Ľ', 'ľ', 'Ŀ', 
    'ŀ', 'Ł', 'ł', 'Ń', 'ń', 'Ņ', 'ņ', 'Ň',
    'ň', 'ŉ', 'Ŋ', 'ŋ', 'Ō', 'ō', 'Ŏ', 'ŏ',
    'Ő', 'ő', 'Œ', 'œ', 'Ŕ', 'ŕ', 'Ŗ', 'ŗ',
    'Ř', 'ř', 'Ś', 'ś', 'Ŝ', 'ŝ', 'Ş', 'ş', 
    'Š', 'š', 'Ţ', 'ţ', 'Ť', 'ť', 'Ŧ', 'ŧ',
    'Ũ', 'ũ', 'Ū', 'ū', 'Ŭ', 'ŭ', 'Ů', 'ů',
    'Ű', 'ű', 'Ų', 'ų', 'Ŵ', 'ŵ', 'Ŷ', 'ŷ',
    'Ÿ', 'Ź', 'ź', 'Ż', 'ż', 'Ž', 'ž', 'ſ', 
    'ƀ', 'Ɓ', 'Ƃ', 'ƃ', 'Ƅ', 'ƅ', 'Ɔ', 'Ƈ',
    'ƈ', 'Ɖ', '=', '_', '-', '`', '~', '|',
    '[', ']', '{', '}', 'ƞ', '?', ',', '(',
    ')', '^', '*', '$', '%', '!', '#', '.', 
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z', '0', '1', '2', '3',
    '4', '5', '6', '7', '8', '9', '+', '/'
];

/// Converts a slice of u8 values into a base256 encoded string using the CHARSETS constant.
///
/// # Parameters
///
/// * `bytes` - A slice of u8 values to be encoded.
///
/// # Returns
///
/// A String containing the base256 encoded representation of the input bytes.
///
/// # Example
///
/// ```
/// use base256_lib::{u82base256};
///
/// let bytes: &[u8] = b"Hello, World!";
/// let result = u82base256(bytes);
/// assert_eq!(result, "ĨŅŌŌŏČĀķŏŒŌńā");
/// ```
pub fn u82base256(bytes: &[u8]) -> String {
    let mut result: String = String::new();
    for &byte in bytes {
        result.push(byte2base256(byte));
    }
    result
}

/// Converts a base256 encoded string into a slice of u8 values using the CHARSETS constant.
///
/// # Parameters
///
/// * `input` - A reference to a string containing the base256 encoded values to be converted.
///
/// # Returns
///
/// A vector of u8 values containing the decoded representation of the input string.
///
/// # Example
///
/// ```
/// use base256_lib::{base2562u8};
///
/// let input: &str = "ĨŅŌŌŏČĀķŏŒŌńā";
/// let result = base2562u8(input);
/// assert_eq!(result, vec![
///     72, 101, 108, 108, 111, 44,
///     32, 87, 111, 114, 108, 100, 33
/// ]);
/// ```
pub fn base2562u8(input: &str) -> Vec<u8> {
    let bytes: Vec<u8> = input.chars().map(|c: char| base2562byte(c)).collect();
    bytes
}

/// Converts a single byte value into its corresponding character representation in the CHARSETS constant.
///
/// # Parameters
///
/// * `byte` - A single byte value to be converted. The value must be within the range of 0 to 255.
///
/// # Returns
///
/// A character from the CHARSETS constant that corresponds to the input byte value.
///
/// # Example
///
/// ```
/// use base256_lib::{byte2base256};
///
/// let byte: u8 = 97;
/// let result: char = byte2base256(byte);
/// assert_eq!(result, 'Ł');
/// ```
pub fn byte2base256(byte: u8) -> char {
    CHARSETS[byte as usize]
}

/// Converts a single character from the CHARSETS constant into its corresponding byte value.
///
/// # Parameters
///
/// * `char` - A single character from the CHARSETS constant to be converted.
///
/// # Returns
///
/// A byte value from the range of 0 to 255 that corresponds to the input character.
///
/// # Panics
///
/// This function will panic if the input character is not found in the CHARSETS constant.
///
/// # Example
///
/// ```
/// use base256_lib::{base2562byte};
///
/// let char: char = 'Ł';
/// let result: u8 = base2562byte(char);
/// assert_eq!(result, 97);
/// ```
pub fn base2562byte(char: char) -> u8 {
    let index: usize = CHARSETS.iter().position(|&c| c == char).unwrap();
    index as u8
}

#[cfg(test)]
mod tests {
    use core::str;

    use super::*;

    #[test]
    fn u82base256_test() {
        let input: &str = "Hello, World!";
        let bytes: &[u8] = input.as_bytes();
        let result: String = u82base256(bytes);
        let expected_result: &str = "ĨŅŌŌŏČĀķŏŒŌńā";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn base2562u8_test() {
        let input: &str = "ĨŅŌŌŏČĀķŏŒŌńā";
        let result: Vec<u8> = base2562u8(input);
        let expected_result: Vec<u8> = vec![
            72, 101, 108, 108, 111, 44,
            32, 87, 111, 114, 108, 100, 33
        ];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn byte2base256_test() {
        let byte: u8 = 97;
        let result: char = byte2base256(byte);
        let expected_result: char = 'Ł';
        assert_eq!(result, expected_result);
    }

    #[test]
    fn base2562byte_test() {
        let char: char = 'Ł';
        let result: u8 = base2562byte(char);
        let expected_result: u8 = 97;
        assert_eq!(result, expected_result);
    }
}
