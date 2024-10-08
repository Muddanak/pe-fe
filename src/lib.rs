pub use self::lib::coff_header;
pub use self::lib::dos_header;
pub use self::lib::optional_header;
pub use self::lib::sections_header;
pub use self::lib::utils;

use crate::lib::coff_header::structs::CoffHeader;
use crate::lib::dos_header::structs::DosHeader;
use crate::lib::optional_header::structs::OptHeader;

pub mod lib {
    pub mod coff_header;
    pub mod dos_header;
    pub mod optional_header;
    pub mod sections_header;
    pub mod utils;
}
/// When given built header structs, this function displays the formatted output from each struct
///
///  # Example
///
/// show_headers(&dosHead, &coffHead, &optHead);
///
pub fn show_headers(dosheader: &DosHeader, coffheader: &CoffHeader, optheader: &OptHeader) {
    println!("{dosheader}{coffheader}{optheader}");
    if optheader.MAGIC.eq(&0x20b) {
        println!("{}", optheader.WINDETAILSPLUS)
    } else if optheader.MAGIC.eq(&0x10b) {
        println!("{}", optheader.WINDETAILS32)
    }
    println!("{}", optheader.DATADIRECTORIES);
}

#[cfg(test)]
mod tests {
    use crate::coff_header::make_coff_header;
    use crate::dos_header::{check_for_mz, make_dos_header};
    use crate::optional_header::make_optional_header;
    use crate::sections_header::make_section_header;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::{env, fs, process};

    #[test]
    fn test_lots() -> Result<(), Box<dyn std::error::Error>> {
        //PEFILES64 is an environmental variable pointing to a large testbed of 64-bit-built IMG files
        let paths = fs::read_dir(env::var("PEFILES64").unwrap())
            .unwrap()
            .filter(|x| x.as_ref().unwrap().path().extension() == Some("dat".as_ref()));

        for item in paths {
            println!("Name: {}", item.as_ref().unwrap().path().display());
            let mut reader = BufReader::new(File::open(item.unwrap().path()).unwrap());
            let mut buffer = [0; 0x5000];
            let _size_read = reader.read(&mut buffer);

            let mz_offset = match check_for_mz(&buffer) {
                Ok(offset) => offset,
                Err(e) => {
                    println!("{e}");
                    process::exit(1)
                }
            };

            let header_dos = make_dos_header(&buffer, mz_offset)?;

            let mut cursor = header_dos.0.pe_offset + 4;

            let header_coff = make_coff_header(&buffer, cursor)?;

            cursor += 20;

            let (_header_opt, cursor) = make_optional_header(&buffer, cursor)?;

            let _secheader = make_section_header(&buffer, cursor, header_coff.HE_SECTIONS as usize);
        }
        Ok(())
    }
}
