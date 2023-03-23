use crate::coff_header::structs::CoffHeader;
use crate::dos_header::structs::DosHeader;
use crate::optional_header::structs::OptHeader;

pub mod coff_header;
pub mod dos_header;
pub mod optional_header;
pub mod utils;

#[allow(dead_code)]
pub fn show_headers(dosheader: &DosHeader, coffheader: &CoffHeader, optheader: &OptHeader) {
    println!("{dosheader}{coffheader}{optheader}");
    if optheader.MAGIC.eq(&0x20b) {
        println!("{}", optheader.WINDETAILSPLUS)
    }
    else if optheader.MAGIC.eq(&0x10b) {
        println!("{}", optheader.WINDETAILS32)
    }
}