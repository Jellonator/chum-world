pub mod bezierpatch;
pub mod dsp;
pub mod idmap;
use std::ops::{Add, Div, Sub};

use crc::crc32;

pub fn round_up(value: usize, mult: usize) -> usize {
    if mult == 0 {
        value
    } else if value % mult == 0 {
        value
    } else {
        value + mult - (value % mult)
    }
}

/// Divide a by b, with the result rounded up (e.g. 7/2 -> 4)
pub fn div_up<T>(a: T, b: T) -> T
where
    T: Div<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + From<u8> + Copy,
{
    (a + (b - 1u8.into())) / b
}

/// Return the nibble values from a. Returns (high, low).
pub fn get_nibbles(a: u8) -> (u8, u8) {
    ((a >> 4) & 0xF, a & 0xF)
}

/// Return the nibble values from a.
pub fn get_high_nibble(a: u8) -> u8 {
    (a >> 4) & 0xF
}

/// Return the nibble values from a.
pub fn get_low_nibble(a: u8) -> u8 {
    a & 0xF
}

/// Get the output file name for the given file string and id
pub fn get_file_string(s: &str, id: u32) -> Vec<String> {
    let fullpath = if let Some(pos) = s.rfind('.') {
        let (left, right) = s.split_at(pos);
        format!("{}[{:08X}]{}", left, id, right)
    } else {
        format!("{}[{:08X}]", s, id)
    };
    let fullpath: &str = if s.starts_with("DB:>") {
        &fullpath[4..]
    } else {
        &fullpath
    };
    fullpath
        .split('>')
        .map(|s| s.replace(|c: char| !c.is_alphanumeric() && c != '.', "_"))
        .collect()
}

/// Get the output file name for the given file string, id, and extension
pub fn get_file_string_ext(s: &str, id: u32, extension: &str) -> Vec<String> {
    let fullpath = if let Some(pos) = s.rfind('.') {
        let (left, _right) = s.split_at(pos);
        format!("{}[{:08X}].{}", left, id, extension)
    } else {
        format!("{}[{:08X}].{}", s, id, extension)
    };
    let fullpath: &str = if s.starts_with("DB:>") {
        &fullpath[4..]
    } else {
        &fullpath
    };
    fullpath
        .split('>')
        .map(|s| s.replace(|c: char| !c.is_alphanumeric() && c != '.', "_"))
        .collect()
}

/// Hash the given name using the crc32 IEEE algorithm.
pub fn hash_name_u32(name: &str) -> u32 {
    crc32::checksum_ieee(&name.as_bytes()) as u32
}

/// Hash the given name using the crc32 IEEE algorithm.
pub fn hash_name_i32(name: &str) -> i32 {
    crc32::checksum_ieee(&name.as_bytes()) as i32
}
