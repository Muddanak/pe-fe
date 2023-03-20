use byteorder::{ByteOrder, LittleEndian};
use crate::optional_header::enums::MAGIC;
use crate::optional_header::structs::OptHeader;
use crate::utils::match_u16_in_map;

pub(crate) mod structs;
pub(crate) mod enums;

pub fn make_optional_header(data: &[u8]) -> OptHeader {
    let mut optheader = OptHeader::new();
    let mut cur: usize = 0;

    optheader.MAGIC = LittleEndian::read_u16(&data[cur..cur+2]);
    optheader.DETAILS.MAGIC = match_u16_in_map(&MAGIC, optheader.MAGIC );
    cur += 2; //cur = 2
    optheader.MAJORLINKER = data[cur];
    cur += 1; //cur = 3
    optheader.MINORLINKER = data[cur];
    cur += 1; //cur = 4
    optheader.SIZEOFCODE = LittleEndian::read_u32(&data[cur..cur+4]);
    cur += 4; //cur = 8
    optheader.SIZEOFINITDATA = LittleEndian::read_u32(&data[cur..cur+4]);
    cur += 4; //cur = 12
    optheader.SIZEOFUNINITDATA = LittleEndian::read_u32(&data[cur..cur+4]);
    cur += 4; //cur = 16
    optheader.BASEOFCODE = LittleEndian::read_u32(&data[cur..cur+4]);



    optheader
}