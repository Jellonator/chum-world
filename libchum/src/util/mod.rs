pub mod bezierpatch;
pub mod dsp;
pub mod idmap;
use std::ops::{Add, Div, Sub};
use std::borrow::Cow;

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

/// Remove n rows from the end of a 2D array.
/// Panics if n is greater than size.1
pub fn remove_rows<T>(data: &mut Vec<T>, size: (usize, usize), n: usize)
where
    T: Default + Clone
{
    if n > size.1 {
        panic!("Trying to remove {} rows from a 2D array of height {}", n, size.1);
    }
    data.resize(size.0 * size.1 - size.0 * n, T::default());
}

/// Add n rows to a 2D array
pub fn add_rows<T>(data: &mut Vec<T>, size: (usize, usize), n: usize)
where
    T: Default + Clone
{
    data.resize(size.0 * size.1 - size.0 * n, T::default());
}


/// Remove n columns from the end of a 2D array
/// Panics if n is greater than size.0
pub fn remove_cols<T>(data: &mut Vec<T>, size: (usize, usize), n: usize)
where
    T: Default + Clone
{
    if n > size.0 {
        panic!("Trying to remove {} columns from a 2D array of width {}", n, size.0);
    }
    let new_w = size.0 - n;
    let old_w = size.0;
    let h = size.1;
    // re-order data
    for iy in 1..h {
        for ix in 0..new_w {
            let new_i = iy * new_w + ix;
            let old_i = iy * old_w + ix;
            data.swap(new_i, old_i);
        }
    }
    // resize data
    data.resize(size.0 * size.1 - size.0 * n, T::default());
}

/// Add n columns to the end of a 2D array
pub fn add_cols<T>(data: &mut Vec<T>, size: (usize, usize), n: usize)
where
    T: Default + Clone
{
    let new_w = size.0 + n;
    let old_w = size.0;
    let h = size.1;
    // resize data
    data.resize(size.0 * size.1 + size.0 * n, T::default());
    //re-order data
    for iy in 1..h {
        for ix in 0..old_w {
            let new_i = iy * new_w + ix;
            let old_i = iy * old_w + ix;
            data.swap(new_i, old_i);
        }
    }
}

/// Resize a 2D array.
/// Returns a copy of the given array.
pub fn resize_2d_cloned<'a, T>(data: &'a [T], old_size: (usize, usize), new_size: (usize, usize)) -> Cow<'a, [T]>
where
    T: Default + Clone
{
    if old_size == new_size {
        return Cow::from(data);
    }
    let (old_w, old_h) = old_size;
    let (new_w, new_h) = new_size;
    let mut v = Vec::with_capacity(new_w * new_h);
    for iy in 0..old_h.min(new_h) {
        for ix in 0..old_w.min(new_w) {
            v.push(data[ix + iy * old_w].clone());
        }
        for _iy in old_w.min(new_w)..new_w {
            v.push(T::default());
        }
    }
    for _iy in old_h.min(new_h)..new_h {
        for _ix in 0..new_w {
            v.push(T::default());
        }
    }
    Cow::from(v)
}

/// Resize a 2D array in-place.
pub fn resize_2d_inplace<T>(data: &mut Vec<T>, old_size: (usize, usize), new_size: (usize, usize))
where
    T: Default + Clone
{
    let (old_w, old_h) = old_size;
    let (new_w, new_h) = new_size;
    if new_w * new_h > old_w * old_h {
        data.reserve(new_w * new_h - old_w * old_h);
    }
    let mut w = old_w;
    let mut h = old_h;
    // remove_rows comes first and add_rows comes last to minimize the amount
    // of work that add_cols/remove_cols must perform
    if new_h < old_h {
        remove_rows(data, (w, h), old_h - new_h);
        h = new_h;
    }
    if new_w > old_w {
        add_cols(data, (w, h), new_w - old_w);
        w = new_w;
    } else if new_w < old_w {
        remove_cols(data, (w, h), old_w - new_w);
        w = new_w;
    }
    if new_h > old_h {
        add_rows(data, (w, h), new_h - old_h);
        // h = new_h;
    }
}
