
use byteorder::{LittleEndian, ByteOrder};
use chrono::{TimeZone, Utc};
use crate::coff_header::enums::{CHARACTERISTICS, MACHINE};
use crate::coff_header::structs::CoffHeader;
use crate::utils::{match_gen_in_map};

pub mod enums;
pub mod structs;

pub fn make_coff_header(data: &[u8]) -> CoffHeader {
    let mut coffheader = CoffHeader::new();
    let mut characteristics_vec: Vec<String> = Vec::new();

    let mut cur = 4;

    coffheader.HE_MACHINEINFO = LittleEndian::read_u16(&data[cur..cur+2]);
    cur += 2;
    coffheader.HE_SECTIONS = LittleEndian::read_u16(&data[cur..cur+2]);
    cur += 2;
    coffheader.HE_DATESTAMP_UTC = LittleEndian::read_u32(&data[cur..cur+4]);
    cur += 4;
    coffheader.HE_POINTERTOSYMBOLS = LittleEndian::read_u32(&data[cur..cur+4]);
    cur += 4;
    coffheader.HE_NUMBEROFSYMBOLS = LittleEndian::read_u32(&data[cur..cur+4]);
    cur += 4;
    coffheader.HE_OPTIONAL = LittleEndian::read_u16(&data[cur..cur+2]);
    cur += 2;
    coffheader.HE_CHARACTERISTICS = LittleEndian::read_u16(&data[cur..cur+2]);

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

    coffheader
}


/*
pub fn _make_header_from_info(chunk: &[u8], offset: usize) -> CoffHeader {
    let sizes = [2, 2, 4, 4, 4, 2, 2];
    let mut cur = offset + 4;
    let mut hold: Vec<String> = Vec::new();
    let mut offsets: Vec<usize> = vec![];
    let mut characteristics_vec: Vec<String> = Vec::new();
    let mut new_header = CoffHeader::new();

    for size in sizes {
        if size == 2 {
            hold.push(LittleEndian::read_u16(&chunk[cur..cur + size]).to_string());
        } else {
            hold.push(LittleEndian::read_u32(&chunk[cur..cur + size]).to_string());
        }
        offsets.push(cur);
        cur += size;
    }

    //dbg!(offsets);

    let machine_id_num: u16 = hold[0].parse().unwrap();
    let sections_id_num: u16 = hold[1].parse().unwrap();
    let date_time_stamp_id_num: u32 = hold[2].parse().unwrap();
    let pointer_to_symbols_table_id_num: u32 = hold[3].parse().unwrap();
    let number_of_symbols_id_num: u32 = hold[4].parse().unwrap();
    let size_of_optional_header_id_num: u16 = hold[5].parse().unwrap();
    let characteristics_id_num: u16 = hold[6].parse().unwrap();

    //Get the Machine info
    if let Some((machine, _)) = MACHINE
        .into_iter()
        .find(|(_, y)| **y as u16 == machine_id_num)
    {
        new_header.HE_MACHINEINFO = (*machine).to_string();
        new_header.HE_MACHINEINFO_OFFSET = format!("0x{:X}", offsets[0]);
    }

    //Get the Number of Sections
    new_header.HE_SECTIONS = format!("{}", sections_id_num);
    new_header.HE_SECTIONS_OFFSET = format!("0x{:X}", offsets[1]);

    //Get the Date and Timestamp (time is in seconds after UNIX_EPOCH
    new_header.HE_DATESTAMP_UTC = format!(
        "UTC: {}",
        Utc.timestamp_opt(i64::from(date_time_stamp_id_num), 0)
            .unwrap()
    );
    new_header.HE_DATESTAMP_LOC = format!(
        "Local: {}",
        Local
            .timestamp_opt(i64::from(date_time_stamp_id_num), 0)
            .unwrap()
    );
    new_header.HE_DATESTAMP_OFFSET = format!("0x{:X}", offsets[2]);

    //Get Pointer to Table Symbols
    //This may just be flat 0 (zero) as it may have been deprecated
    new_header.HE_POINTERTOSYMBOLS = format!("{}", pointer_to_symbols_table_id_num);
    new_header.HE_POINTERTOSYMBOLS_OFFSET = format!("0x{:X}", offsets[3]);

    //Get Number Symbols
    //This may also be just flat 0 (zero) as it may have also been deprecated
    new_header.HE_NUMBEROFSYMBOLS = format!("{}", number_of_symbols_id_num);
    new_header.HE_NUMBEROFSYMBOLS_OFFSET = format!("0x{:X}", offsets[4]);

    //Get Size of Optional Header
    new_header.HE_OPTIONAL = format!("{}", size_of_optional_header_id_num);
    new_header.HE_OPTIONAL_OFFSET = format!("0x{:X}", offsets[5]);

    //Get Characteristics
    for (charac, charac_id) in CHARACTERISTICS.into_iter() {
        if characteristics_id_num & *charac_id as u16 != 0 {
            characteristics_vec.push(String::from(*charac));
        }
    }
    new_header.HE_CHARACTERISTICS = characteristics_vec.join(", ");
    new_header.HE_CHARACTERISTICS_OFFSET = format!("0x{:X}", offsets[6]);

    new_header
}
*/

