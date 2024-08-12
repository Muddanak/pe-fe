use std::error::Error;

use crate::sections_header::enums::CHARACTERISTICS;
use crate::sections_header::structs::{SectionHeader, SectionHeaderInfo};
use byteorder::{ByteOrder, LittleEndian};

pub(crate) mod enums;
pub(crate) mod structs;

/// Takes in the slice of the buffered data and where the offset for the section headers and
/// the number of sections extracted from before.
///
/// Return a built SectionHeader based on the file opened, if it's a proper image
///
///  # Example
///
/// let sectHead = make_section_header(&buffer, 0x180, 0x05);
///
pub fn make_section_header(data: &[u8], offset: usize, num_sections: usize) -> Result<SectionHeader, Box<dyn Error>> {
    let mut secheader = SectionHeader::default();
    let mut sections: Vec<SectionHeaderInfo> = vec![];
    let mut cur = offset;

    for _ in 0..num_sections {
        let mut tmpinfo = SectionHeaderInfo::new();
        tmpinfo.NAME = LittleEndian::read_u64(&data[cur..cur + 8]);
        cur += 8;
        tmpinfo.VIRTUALSIZE = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += 4;
        tmpinfo.VIRTUALADDRESS = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += 4;
        tmpinfo.SIZEOFRAWDATA = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += 4;
        tmpinfo.POINTERTORAWDATA = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += 4;
        tmpinfo.POINTERTORELOCATIONS = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += 4;
        tmpinfo.POINTERTOLINENUMBERS = LittleEndian::read_u32(&data[cur..cur + 4]);
        cur += 4;
        tmpinfo.NUMBEROFRELOCATIONS = LittleEndian::read_u16(&data[cur..cur + 2]);
        cur += 2;
        tmpinfo.NUMBEROFLINENUMBERS = LittleEndian::read_u16(&data[cur..cur + 2]);
        cur += 2;

        let mut tmpvec: Vec<String> = Vec::new();

        let comparator = LittleEndian::read_u32(&data[cur..cur + 4]);
        for (charac, charac_id) in CHARACTERISTICS.into_iter() {
            if (comparator & *charac_id).ne(&0) {
                tmpvec.push(String::from(*charac));
            }
        }
        tmpinfo.CHARACTERISTICS = tmpvec.join(" | ");
        sections.push(tmpinfo);
        cur += 4;
    }
    secheader.HEADER = sections;
    Ok(secheader)
}

///
/// Prints the section headers found within the image
///
/// Called by command line argument
///
/// # Example
///
/// print_section_headers(&sectionHead);
///
pub fn print_section_headers(secheader: &SectionHeader) {
    println!("{}", secheader);
}
