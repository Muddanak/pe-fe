use crate::sections_header::structs::{SectionHeader, SectionHeaderInfo};

pub(crate) mod structs;
pub(crate) mod enums;

pub fn make_section_header(data: &[u8], offset: usize, numberofsections: usize) -> SectionHeader {
    let mut secheader = SectionHeader::default();
    let mut sections: Vec<SectionHeaderInfo> = vec![];



    secheader
}