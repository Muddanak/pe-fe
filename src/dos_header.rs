
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use crate::dos_header::structs::DosHeader;

mod structs;

pub fn make_dos_header(data: &[u8], mz_found: usize) -> DosHeader {
    let mut header = DosHeader::new();
    let mut cur :usize = 0x3c;

    header.mz_offset = mz_found;
    header.ox3c_offset = LittleEndian::read_u32(&data[cur..cur + 4]) as usize;
    cur += 0x04;
    header.has_stub = check_for_stub(&data[cur..]);
    if header.ox3c_offset != 0x80 {
        header.has_rich = true;
        println!("{:#04X}", header.ox3c_offset);
        header.rich_xor_key = get_rich_xor(&data[0x80..header.ox3c_offset])
    }


    header
}

fn check_for_stub(data: &[u8]) -> bool {

    let strdata = String::from_utf8_lossy(data);
    if strdata.contains("This program") {
        return true
    }
    false
}

fn get_rich_xor(data: &[u8]) -> u32 {

    //The word "Rich" in hex
    let rich:[u8;4] = 0x52696368_u32.to_be_bytes();

    let (offset, _) = data.iter()
        .enumerate()
        .find(|(_, item)| rich.contains(item) )
        .unwrap();
    BigEndian::read_u32(&data[offset+4..offset+8])
}