use byteorder::{BigEndian, ByteOrder, LittleEndian};
use sha2::Digest;

use crate::coff_header::enums::PEFILEERROR;
use crate::coff_header::enums::PEFILEERROR::NoMZinFile;

use crate::dos_header::structs::DosHeader;

mod structs;

pub fn make_dos_header(data: &[u8], mz_found: usize) -> DosHeader {
    let mut header = DosHeader::new();
    let mut cur: usize = 0x3c; //cur = 60

    header.mz_offset = mz_found; //should be 0x00
    header.pe_offset = LittleEndian::read_u32(&data[cur..cur + 4]) as usize;
    cur += 0x04; //cur = 64
    header.has_stub = check_for_stub(&data[cur..]); //Read from data[64] to end
    cur += 0x40; //cur = 128

    if header.pe_offset != cur {

        header.has_rich = true;
        header.rich_xor_key = get_rich_xor_key(&data[0x80..header.pe_offset]);
        header.rich_ids = get_rich_data(&data[cur..header.pe_offset], header.rich_xor_key);
    }

    header
}

fn check_for_stub(data: &[u8]) -> bool {
    let (offset, _) = data
        .iter()
        .enumerate()
        .find(|(_, item)| "This program".as_bytes().contains(item))
        .unwrap();

    if offset != 0 {
        return true;
    }

    false
}

fn get_rich_xor_key(data: &[u8]) -> u32 {
    let (offset, _) = data
        .iter()
        .enumerate()
        .find(|(_, item)| "Rich".as_bytes().contains(item))
        .unwrap();

    let tmp = BigEndian::read_u32(&data[offset + 4..offset + 8]);

    tmp
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
    println!("\n---------------------------\nRich ID Information");
    let signature = String::from_utf8(Vec::from(header.rich_ids[0].to_be_bytes())).unwrap();
    println!("Signature Verification (Should be 'DanS'): {}", signature);
    let mut hashvec: Vec<u32> = Vec::new();

    let mut hash = sha2::Sha256::new();

    for idnum in (0..header.rich_ids.len()).step_by(2) {
        hashvec.push(header.rich_ids[idnum]);
        hashvec.push(header.rich_ids[idnum + 1]);
    }

    //println!("{}", hashvec.iter().map(|x| x.to_string()).collect::<String>());
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

    if "MZ".chars().all(|item| chunk.contains(&(item as u8))) {
        let (offset, _) = chunk
            .iter()
            .enumerate()
            .find(|(_, item)| "MZ".as_bytes().contains(item))
            .unwrap();

        Ok(offset)
    } else {
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