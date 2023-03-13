
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
    cur += 0x40;
    if header.ox3c_offset != cur {
        header.has_rich = true;
        let data_to_end = header.ox3c_offset / 4;
        println!("{cur:#04X} {:#04X} {}", header.ox3c_offset, (header.ox3c_offset/4)/4);
        header.rich_xor_key = get_rich_xor_key(&data[0x80..header.ox3c_offset]);
        let test = get_rich_data(&data[cur..data_to_end], (data_to_end / 4) as u32, header.rich_xor_key);
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

fn get_rich_xor_key(data: &[u8]) -> u32 {
    let (offset, _) = data.iter()
        .enumerate()
        .find(|(_, item)| "Rich".as_bytes().contains(item) )
        .unwrap();
    BigEndian::read_u32(&data[offset+4..offset+8])
}

fn get_rich_data(data: &[u8], iters: u32, key: u32) -> u32 {
    let key : [u8;4] = key.to_be_bytes();

    let mut hold: [u8;4] = [0;4];

    for i in 0..4 {
        hold[i] = &data[i] ^ key[i];
    }

    dbg!(hold);

    for _ in 3..iters {

    }

    0
}