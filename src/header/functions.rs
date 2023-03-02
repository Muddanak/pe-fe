use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Read;
use byteorder::{ByteOrder, LittleEndian};
use chrono::{DateTime, Utc};
use crate::header::enums::MACHINE;
use crate::header::structs::Header;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum FileError {
    CouldNotGetData,
    OffsetPEisZero,
    NoMZinFile,
    CouldNotGetOffset,
    PEisNotHere,
    PEInvalid,
}

impl Display for FileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::CouldNotGetData => {
                write!(f, "Could not successfully get data from the file")
            }
            FileError::OffsetPEisZero => write!(
                f,
                "The value located at offset 0x3c was zero, file might not be a PE"
            ),
            FileError::NoMZinFile => write!(f, "The identifier 'MZ' was not located in the header"),
            FileError::CouldNotGetOffset => {
                write!(f, "Somehow the offset 0x3c was unable to be found")
            },
            FileError::PEisNotHere => write!(f, "The PE header was not found at the provided offset"),
            FileError::PEInvalid => write!(f, "The PE is invalid, somehow?")
        }
    }
}

impl std::error::Error for FileError {}

pub fn get_first_kilobyte(mut filename: File) -> Vec<u8> {
    let mut chunk = [0; 0x400];

    let _filename_check = match filename.read_exact(&mut chunk) {
        Ok(_success) => Ok(()),
        Err(_e) => Err(FileError::CouldNotGetData),
    };

    Vec::from(chunk)
}

pub fn _get_pe_offset(chunk: &[u8]) -> Result<usize, FileError> {

    let mut val  :usize = *chunk.get(0x3c).unwrap() as usize;
    println!("val first is {val}");

    if val == 0 {
        for slot in 0..chunk.len() {
            if chunk[slot] == 0x50 && chunk[slot+1] == 0x45 && chunk[slot+2] == 0x00 && chunk[slot+3] == 0x00 {
                val = slot;
                break;
            }
        }
    }
    //dbg!(&chunk[val..val+4]);
    Ok(val)
}

pub fn _verify_pe_header(slice: &[u8]) -> String {
    let pe = match String::from_utf8(Vec::from(slice)) {
        Ok(pe) => pe,
        Err(e) => format!("{}", e)
    };
    pe
}

pub fn get_pe_header(chunk: &[u8]) -> usize {

    let mut val  :usize = *chunk.get(0x3c).unwrap() as usize;
    if val == 0 {
        for slot in 0..chunk.len() {
            if chunk[slot] == 0x50 && chunk[slot+1] == 0x45 && chunk[slot+2] == 0x00 && chunk[slot+3] == 0x00 {
                val = slot;
                break;
            }
        }
    }

    val
}

pub fn _usize_to_hex(value: usize) -> usize {
   // if let Ok(val) = usize::from_str_radix(&value.to_string(), 16) {
    let val = match usize::from_str_radix(&value.to_string(), 16) {
        Ok(x) => x,
        Err(_e) => 0,
    };
    val.to_owned()
}

///
///
/// check_for_mz
/// Takes: a reference to a slice of u8
/// Ex: &[u8]
///
/// Returns a Result of either Ok(()) or Errors out with a FileError
/// FileError is of variant: NoMZinFile
///
/// Concept:  Grabs the first two characters of the
///
///
pub fn check_for_mz(chunk: &[u8]) -> Result<(), FileError> {
    if let Ok(first_two) = String::from_utf8(Vec::from(chunk)) {
        //dbg!(&first_two);
        if first_two != "MZ" {
            return Err(FileError::NoMZinFile)
        }
    }

    Ok(())
}


///
/// make_header_from_info
///
pub fn make_header_from_info(chunk: &[u8], offset: usize) -> Header {
    let sizes = [2,2,4,4,4,2,2];
    let mut cur = offset+4;
    let mut hold: Vec<String> = Vec::new();


    for size in sizes {

        if size == 2 {
            hold.push(LittleEndian::read_u16(&chunk[cur..cur+size]).to_string());
        } else {
            hold.push(LittleEndian::read_u32(&chunk[cur..cur+size]).to_string());
        }

        cur += size;
    }


    /*let dt = chrono::NaiveDateTime::from_timestamp_millis("1677768407".parse::<i64>().unwrap() ).unwrap();
    dbg!( DateTime::<Utc>::from_utc(dt, Utc));*/

    let m1: u16 = hold[0].parse().unwrap();
    let m2: u16 = hold[1].parse().unwrap();
    let m3: u32 = hold[2].parse().unwrap();
    let m4: u32 = hold[3].parse().unwrap();
    let m5: u32 = hold[4].parse().unwrap();
    let m6: u16 = hold[5].parse().unwrap();
    let m7: u16 = hold[6].parse().unwrap();

    for (k,v) in MACHINE.into_iter() {
        if *v as u16 == m1 {
            hold.push(String::from(*k));
        }
    }

    //let mw: String = MACHINE.into_iter().filter(|(x, y)| **y as u16 == m1).collect();

    if let Some((mw, me)) = MACHINE.into_iter().find(|(x, y)| **y as u16 == m1){
        println!("machine some {}", *mw);
    }



    Header::new(
        hold[7].clone(),
        hold[1].clone(),
        hold[2].clone(),
        hold[3].clone(),
        hold[4].clone(),
        hold[5].clone(),
        hold[6].clone(),
    )


}

///
///
/// Tests are below
///
///

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_mz() {
        assert_eq!(check_for_mz(&[b'M', b'Z']), Ok(()));
    }
}
