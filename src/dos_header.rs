use byteorder::{ByteOrder, LittleEndian};
use crate::dos_header::structs::DosHeader;

mod structs;

pub fn make_dos_header(data: &[u8], mzfound: usize) -> DosHeader {
    let mut header = DosHeader::new();
    let cur :usize = 0x3c;

    header.mz_offset = mzfound;
    header.ox3c_offset = LittleEndian::read_u32(&data[cur..cur + 4]) as usize;





    header
}