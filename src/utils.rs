use crate::coff_header::enums::PEFILEERROR;
use std::fs::File;
use std::io::Read;
use std::ops::{BitAnd, Shr};

#[allow(dead_code)]
pub fn u32_to_u16_high_low(inp: u32) -> (u16, u16) {
    let high: u16 = inp.bitand(0xFFFF0000).shr(16) as u16;
    let low: u16 = inp.bitand(0x0000FFFF) as u16;
    (high, low)
}

pub fn get_large_data_chunk(mut filename: File) -> Vec<u8> {
    let mut chunk = [0; 0x900];

    let _filename_check = match filename.read_exact(&mut chunk) {
        Ok(_success) => Ok(()),
        Err(_e) => Err(PEFILEERROR::CouldNotGetData),
    };

    Vec::from(chunk)
}
