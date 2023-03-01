use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum FileError {
    CouldNotGetData,
    OffsetPEisZero,
    NoMZinFile,
    CouldNotGetOffset,
    PEisNotHere,
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
    let val = chunk[0x3c] as usize;
    let offset = match chunk.get(val) {
        Some(x) => *x as usize,
        None => usize::MAX,
    };

    if offset == usize::MAX {
        Err(FileError::CouldNotGetOffset)
    } else if offset == 0 {
        Err(FileError::OffsetPEisZero)
    } else {
        Ok(offset)
    }
}

pub fn verify_pe_header(slice: &[u8]) -> Result<(), FileError> {

    if let Ok(pe) = String::from_utf8(Vec::from(slice)) {
        if pe != "PE\0\0" {
            return Err(FileError::PEisNotHere)
        }
    }
    println!("PE header has been verified and found!");
    Ok(())
}

pub fn usize_to_hex(value: usize) -> usize {
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
    //let first_two = chunk[0..=1].iter().map(|x| *x as char).collect::<String>();
    if let Ok(first_two) = String::from_utf8(Vec::from(chunk)) {
        //dbg!(&first_two);
        if first_two != "MZ" {
            return Err(FileError::NoMZinFile)
        }
    }

    Ok(())
}

/*pub fn check_for_mz2(chunk: &str) -> Result<(), FileError> {
    //let first_two = chunk[0..=1].iter().map(|x| *x as char).collect::<String>();
    if chunk != "MZ" {
        return Err(FileError::NoMZinFile)
    }

    Ok(())
}*/




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
