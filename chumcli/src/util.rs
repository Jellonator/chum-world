use crc::crc32;

pub fn hash_name_i32(name: &str) -> i32 {
    crc32::checksum_ieee(&name.as_bytes()) as i32
}

pub fn hash_name_u32(name: &str) -> u32 {
    crc32::checksum_ieee(&name.as_bytes())
}
