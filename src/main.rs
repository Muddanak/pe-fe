use clap::{arg, Parser};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

use pe_fe::sections_header::{make_section_header, print_section_headers};
use pe_fe::{lib as pefelib, show_headers};
use pefelib::dos_header::{check_for_mz, make_dos_header, print_rich_sha256_hash};

use pefelib::coff_header::make_coff_header;
use pefelib::optional_header::make_optional_header;

#[derive(Parser)]
#[command(author = "Mudd", version, long_about = None)]
#[command(about = "A PE file analyzer written in Rust")]
#[command(arg_required_else_help = true, propagate_version = true)]
struct Args {
    ///Filename to analyze
    filename: String,

    #[arg(short, long = "rich")]
    ///Show the 'Rich' header hash
    rich_hash: bool,

    #[arg(short, long = "section")]
    ///Show the Sections header
    section_header: bool,

    ///Show all header info parsed
    #[arg(short = 'a', long = "all")]
    show_all: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.filename.is_empty() {
        return Ok(())
    }

    let mut reader = BufReader::new(File::open(args.filename)?);
    let mut buffer = [0; 0x800];
    let _size_read = reader.read(&mut buffer)?;

    /* let mz_offset = match check_for_mz(&buffer) {
        Ok(offset) => offset,
        Err(e) => Err(e),
    }; */

    let mz_offset = check_for_mz(&buffer)?;

    let header_dos = make_dos_header(&buffer, mz_offset)?;

    let mut cursor = header_dos.0.pe_offset + 4;

    let header_coff = make_coff_header(&buffer, cursor)?;

    cursor += 20;

    let (header_opt, cursor) = make_optional_header(&buffer, cursor)?;

    let secheader = make_section_header(&buffer, cursor, header_coff.HE_SECTIONS as usize)?;

    show_headers(&header_dos.0, &header_coff, &header_opt);

    if args.section_header.eq(&true) || args.show_all {
        print_section_headers(&secheader);
    }

    if header_dos.0.has_rich && (args.rich_hash.eq(&true) || args.show_all) {
        print_rich_sha256_hash(&header_dos.0);
    }

    Ok(())
}
