use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Read;
use byteorder::{ByteOrder, LittleEndian};
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

pub fn get_pe_offset(chunk: &[u8]) -> Result<usize, FileError> {

    if let Some(val) = chunk.get(0x3c) {
        if *val == 0 {
            return Err(FileError::OffsetPEisZero)
        }
        Ok(*val as usize)
    } else {
        Err(FileError::PEisNotHere)
    }



    //let val = chunk[0x3c] as usize;
    //dbg!(val);

    /*if offset == usize::MAX {
        Err(FileError::CouldNotGetOffset)
    } else if offset == 0 {
        Err(FileError::OffsetPEisZero)
    } else {
        Ok(offset)
    }*/
}

pub fn verify_pe_header(slice: &[u8]) -> String {
    let pe = match String::from_utf8(Vec::from(slice)) {
        Ok(pe) => pe,
        Err(e) => format!("{}", e)
    };
    pe
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

    Header::new(
        hold[0].clone(),
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
