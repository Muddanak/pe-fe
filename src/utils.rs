use crate::coff_header::enums::PEFILEERROR;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::fmt::Write;


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
}

#[allow(dead_code)]
pub fn bytes_to_hex_string(data: &[u8]) -> String {

    let mut buffer = String::new();

    for &x in data {
        if let Err(e) = write!(&mut buffer, "{:x?}", x) {
            println!("Failed to convert byte to hex! Got Error: {e}");
            exit(1)
        }
    }

    buffer
}

pub fn index_hex_string_in_hex_data(data: String, find: String) -> usize {

    if !data.contains(&find) {
        println!("The string to find was not in the data");
        return 0
    }

    if let Some(index) = data.find(&find) {
        return index/2
    }

    0
}


/*pub fn match_exact_string(inp: &[u8], pattern: &str) -> usize {

    let size = pattern.len();

    let mut data = inp.bytes();
    let counter:usize = 0;
    'upper: for (x,y) in data.enumerate() {
        if y.unwrap() == *pattern.as_bytes().get(counter).unwrap() {
            for z in 1..size {
                if pattern.as_bytes().get(z) != *data[x+z..x+z] {

                }
            }
        }
    }
    0
}*/

pub fn match_u16_in_map(map_name: &phf::Map<&str, u16>, item: u16) -> String {

    String::from(*map_name.into_iter()
        .find(|(_, y)| **y == item).unwrap().0)
}

pub fn match_gen_in_map<T: PartialEq>(map_name: &phf::Map<&str, T>, item: T) -> String {

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
    use std::io::Bytes;
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

    #[test]
    fn test_index_find() {
        let data: Bytes<&[u8]> = b"This is Rich DOS MODE".bytes();
        println!("{data:#?}");
    }
}
