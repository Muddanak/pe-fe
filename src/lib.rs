use crate::coff_header::structs::CoffHeader;
use crate::dos_header::structs::DosHeader;

pub mod coff_header;
pub mod dos_header;
pub mod optional_header;
pub mod utils;

#[allow(dead_code)]
pub fn show_headers(_dosheader: DosHeader, _coffheader: CoffHeader) {

}