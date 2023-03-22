use crate::coff_header::enums::PEFILEERROR;
use std::fs::File;
use std::io::Read;



/*#[allow(dead_code)]
pub fn u32_to_u16_high_low(inp: u32) -> (u16, u16) {
    let high: u16 = inp.bitand(0xFFFF0000).shr(16) as u16;
    let low: u16 = inp.bitand(0x0000FFFF) as u16;
    (high, low)
}*/

pub fn get_large_data_chunk(mut filename: File) -> Vec<u8> {
    let mut chunk = [0; 0x900];

    let _filename_check = match filename.read_exact(&mut chunk) {
        Ok(_success) => Ok(()),
        Err(_e) => Err(PEFILEERROR::CouldNotGetData),
    };

    Vec::from(chunk)
}

pub fn index_of_string_in_u8(data: &[u8], text_to_find: &str) -> usize {

    for (x, y) in data.iter().enumerate() {
        if *y == text_to_find.as_bytes()[0] {
            return x
        }
    }
    0

    /*let offset = match data.iter()
        .enumerate()
        .find(|(_, item)| text_to_find.chars().a == item) {
        Some( (offset, _) ) => offset,
        _ => 0,
    };

    let offset = data.iter()
        .enumerate()
        .all
    0*/
}

pub fn match_u16_in_map(map_name: &phf::Map<&str, u16>, item: u16) -> String {

    String::from(*map_name.into_iter()
        .find(|(_, y)| **y == item).unwrap().0)
}

pub fn match_gen_in_map<T: PartialEq>(map_name: &phf::Map<&str, T>, item: T) -> String {
    /*String::from(*map_name.into_iter()
        .find(|(_, y)| **y == item).unwrap().0)*/
    match map_name.into_iter().find(|(_, y)| **y == item) {
        Some( (word, _y) ) => String::from(*word),
        None => String::from("None"),
    }
}


///
///
/// Tests are below
///
///

#[cfg(test)]
mod tests {
    use phf::phf_map;
    use super::*;

    static MAP_TEST_1: phf::Map<&str, u16> = phf_map!(
        "Test" => 0x0101,
        "Test Other" => 0x0202,
    );


    #[test]
    fn test_match_u16_in_map(){
        assert_eq!(match_u16_in_map(&MAP_TEST_1, 0x0101), "Test");
    }
}
