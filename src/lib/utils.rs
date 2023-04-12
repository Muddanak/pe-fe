
use std::fmt::Write;
use std::ops::{BitAnd, Shr};
use std::process::exit;

///
/// Converts an array of u8 into a String of hex digits
///
pub(crate) fn bytes_to_hex_string(data: &[u8]) -> String {
    let mut buffer = String::new();

    for &x in data {
        if let Err(e) = write!(&mut buffer, "{:02X?}", x) {
            println!("Failed to convert byte to hex! Got Error: {e}");
            exit(1)
        }
    }

    buffer
}
///
/// Gets the index of a String o hex digits inside another String of hex digits
///
pub(crate) fn index_hex_string_in_hex_data(data: String, find: String) -> usize {
    if !data.contains(&find) {
        return 0;
    }

    if let Some(index) = data.find(&find) {
        return index / 2;
    }

    0
}

///
/// Generic to match a given item, T, to the str result from the map, map_name
///
pub(crate) fn match_gen_in_map<T: PartialEq>(map_name: &phf::Map<&str, T>, item: T) -> String {
    match map_name.into_iter().find(|(_, y)| **y == item) {
        Some((word, _y)) => String::from(*word),
        None => String::from("None"),
    }
}

///
/// Converts a u64 to a tuple of (u32, u32)
///
pub(crate) fn u64_to_u32(inp: u64) -> (u32, u32) {
    let high: u32 = inp.bitand(0xFFFFFFFF00000000).shr(32) as u32;
    let low: u32 = inp.bitand(0x00000000FFFFFFFF) as u32;
    (high, low)
}

// Tests are below

#[cfg(test)]
mod tests {
    use super::*;
    use phf::phf_map;

    static MAP_TEST_1: phf::Map<&str, u16> = phf_map!(
        "Test" => 0x0101,
        "Test Other" => 0x0202,
    );

    #[test]
    fn test_match_gen_in_map() {
        assert_eq!(match_gen_in_map(&MAP_TEST_1, 0x0101), "Test");
    }

    #[test]
    fn test_u64_to_u32() {
        assert_eq!(u64_to_u32(0xDEADBEEFDEADBEEF), (0xDEADBEEF, 0xDEADBEEF))
    }

    #[test]
    fn test_bytes_to_hex_string() {
        assert_eq!(bytes_to_hex_string(b"TestData"), "5465737444617461");
        assert_eq!(bytes_to_hex_string(b"Data"), "44617461")
    }

    #[test]
    fn test_index_hex_string_in_hex_data() {
        assert_eq!(index_hex_string_in_hex_data("5465737444617461".to_string(),"44617461".to_string()), 4)
    }
}
