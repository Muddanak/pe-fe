use std::error::Error;

use crate::lib::coff_header::enums::{CHARACTERISTICS, MACHINE};
use crate::lib::coff_header::structs::CoffHeader;
use crate::lib::utils::match_gen_in_map;
use byteorder::{ByteOrder, LittleEndian};
use chrono::{TimeZone, Utc};

pub(crate) mod enums;
pub(crate) mod structs;

/// Takes in the slice of the buffered data and where the offset for the COFF header is located
///
/// Return a built CoffHeader based on the file opened, if it's a proper image
///
///  # Example
///
/// let coffHead = make_coff_header(&buffer, 0x3c);
///
pub fn make_coff_header(data: &[u8], offset: usize) -> Result<CoffHeader, Box<dyn Error>> {
    let mut coffheader = CoffHeader::new();
    let mut characteristics_vec: Vec<String> = Vec::new();

    let mut cur = offset;

    coffheader.HE_MACHINEINFO = LittleEndian::read_u16(&data[cur..cur + 2]);
    cur += 2;
    coffheader.HE_SECTIONS = LittleEndian::read_u16(&data[cur..cur + 2]);
    cur += 2;
    coffheader.HE_DATESTAMP_UTC = LittleEndian::read_u32(&data[cur..cur + 4]);
    cur += 4;
    coffheader.HE_POINTERTOSYMBOLS = LittleEndian::read_u32(&data[cur..cur + 4]);
    cur += 4;
    coffheader.HE_NUMBEROFSYMBOLS = LittleEndian::read_u32(&data[cur..cur + 4]);
    cur += 4;
    coffheader.HE_OPTIONAL = LittleEndian::read_u16(&data[cur..cur + 2]);
    cur += 2;
    coffheader.HE_CHARACTERISTICS = LittleEndian::read_u16(&data[cur..cur + 2]);

    coffheader.HE_DETAILS.MACHINE = match_gen_in_map(&MACHINE, coffheader.HE_MACHINEINFO);

    for (charac, charac_id) in CHARACTERISTICS.into_iter() {
        if coffheader.HE_CHARACTERISTICS & *charac_id != 0 {
            characteristics_vec.push(String::from(*charac));
        }
    }

    coffheader.HE_DETAILS.CHARACTERISTICS = characteristics_vec.join("|");

    coffheader.HE_DETAILS.DATESTAMP_UTC = format!(
        "UTC: {}",
        Utc.timestamp_opt(i64::from(coffheader.HE_DATESTAMP_UTC), 0)
            .unwrap()
    );

    Ok(coffheader)
}

