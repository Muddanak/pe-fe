pub use self::lib::utils;
pub use self::lib::coff_header;
pub use self::lib::dos_header;
pub use self::lib::optional_header;

use crate::lib::coff_header::structs::CoffHeader;
use crate::lib::dos_header::structs::DosHeader;
use crate::lib::optional_header::structs::OptHeader;

pub mod lib {
    pub mod coff_header;
    pub mod dos_header;
    pub mod optional_header;
    pub mod utils;
}

#[allow(dead_code)]
pub fn show_headers(dosheader: &DosHeader, coffheader: &CoffHeader, optheader: &OptHeader) {
    println!("{dosheader}{coffheader}{optheader}");
    if optheader.MAGIC.eq(&0x20b) {
        println!("{}", optheader.WINDETAILSPLUS)
    }
    else if optheader.MAGIC.eq(&0x10b) {
        println!("{}", optheader.WINDETAILS32)
    }
    println!("{}", optheader.DATADIRECTORIES);
}