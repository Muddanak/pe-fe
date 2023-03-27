
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use sha2::Digest;

use crate::lib as crlib;

use crlib::coff_header::enums::PEFILEERROR;
use crlib::coff_header::enums::PEFILEERROR::NoMZinFile;
use crlib::dos_header::structs::DosHeader;
use crlib::utils::{bytes_to_hex_string, index_hex_string_in_hex_data};

pub mod structs;
pub mod enums;

pub fn make_dos_header(data: &[u8], mz_found: usize) -> (DosHeader, usize) {

    let mut header = DosHeader::new();
    let mut cur: usize = 0x3c; //cur = 60

    header.mz_offset = mz_found; //should be 0x00
    header.pe_offset = LittleEndian::read_u32(&data[cur..cur+4]) as usize;
    //println!("Cursor: {:#04x}", cur);
    cur += 0x04; //cur = 64/0x40
    header.has_stub = check_for_stub(&data[cur..header.pe_offset]); //Read from data[64] to end
    if header.has_stub {
        cur += 0x40;
    }

    //println!("PE offset at {:#x}", header.pe_offset);
    //println!("Offset is currently at {:#x}", cur);
    //println!("Made it past stub, got {} and PE is {} and cursor is {}", header.has_stub, header.pe_offset, cur);

    header.rich_xor_key = get_rich_xor_key(&data[cur..header.pe_offset]);
    if header.rich_xor_key != 0 {
        header.has_rich = true;
        header.rich_ids = get_rich_data(&data[cur..header.pe_offset], header.rich_xor_key);
    }




    (header, cur)
}

fn check_for_stub(data: &[u8]) -> bool {

    //let offset = index_of_string_in_u8(data, "This program");

    "This program".bytes().all(|x|data.contains(&x))
}

fn get_rich_xor_key(data: &[u8]) -> u32 {

    //let offset = index_of_string_in_u8(data, "Rich");
    let newdata = bytes_to_hex_string(data);
    //println!("newdata is {newdata}");
    let offset = index_hex_string_in_hex_data(newdata, bytes_to_hex_string(b"Rich"));
    //println!("Rich may be at {:#x}", offset);
    /*let mut offset = 0;
    if "Rich".chars().all(|x| data.contains(&(x as u8)))
    {
        data.iter().find(|x| "Rich".as_bytes().contains())
    }*/
    //println!("Rich: {:#04x}", offset);

    if offset != 0 {
        BigEndian::read_u32(&data[offset + 4..offset + 8])
    } else {
        0
    }
}

fn get_rich_data(data: &[u8], key: u32) -> Vec<u32> {
    let key: [u8; 4] = key.to_be_bytes();
    let mut cur = 0;
    let mut rich_ids: Vec<u32> = vec![];

    let mut hold: [u8; 4] = [0; 4];

    for _ in 0..(data.len() / 4) {
        for index in 0..4 {
            hold[index] = data[cur] ^ key[index];
            cur += 1;
        }
        rich_ids.push(u32::from_be_bytes(hold));
    }
    
    rich_ids
}

pub fn print_rich_sha256_hash(header: &DosHeader) {
    //println!("\n---------------------------\nRich ID Information");
    let val = String::from_utf8(Vec::from(header.rich_ids[0].to_be_bytes()));
    let mut signature = String::new();
    if val.is_ok() {
        signature = val.unwrap();
    }

    println!("\n---------Rich Signature ('DanS')--------\n{}", signature);
    let mut hashvec: Vec<u32> = Vec::new();

    let mut hash = sha2::Sha256::new();

    for id_num in (0..header.rich_ids.len()).step_by(2) {
        hashvec.push(header.rich_ids[id_num]);
        hashvec.push(header.rich_ids[id_num + 1]);
    }

    hash.update(
        hashvec.iter().map(|x| x.to_string()).collect::<String>()
    );

    println!("SHA-256 of Rich Header:\n\t{:#04x}\n", hash.finalize());
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
pub fn check_for_mz(chunk: &[u8]) -> Result<usize, PEFILEERROR> {

    let data = &chunk[..2];
    if "MZ".chars().all(|item| data.contains(&(item as u8))) {
        Ok(0x00)
    }else {
        Err(NoMZinFile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_mz() {
        assert_eq!(check_for_mz(&[b'M', b'Z']), Ok(0));
    }
}