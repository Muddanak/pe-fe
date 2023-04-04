use clap::{arg, Parser};
use std::fs::File;
use std::io::{BufReader, Read};
use std::{io, process};

use pe_fe::sections_header::{make_section_header, print_section_headers};
use pe_fe::{lib as pefelib, show_headers};
use pefelib::dos_header::{check_for_mz, make_dos_header, print_rich_sha256_hash};

use pefelib::coff_header::make_coff_header;
use pefelib::optional_header::make_optional_header;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    ///Filename to analyze
    filename: String,

    #[arg(short, long="rich")]
    ///Show the 'Rich' header hash
    rich_hash: bool,

    #[arg(short, long="section")]
    ///Show the Sections header
    section_header: bool,

    #[arg(short = 'a', long = "all")]
    ///Show all header info parsed
    show_all: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut reader = BufReader::new(File::open(args.filename)?);
    let mut buffer = [0; 0x800];
    let _size_read = reader.read(&mut buffer)?;

    let mz_offset = match check_for_mz(&buffer) {
        Ok(offset) => offset,
        Err(e) => {
            println!("{e}");
            process::exit(1)
        }
    };

    let header_dos = make_dos_header(&buffer, mz_offset);

    let mut cursor = header_dos.0.pe_offset + 4;

    let header_coff = make_coff_header(&buffer, cursor);

    cursor += 20;

    let (header_opt, cursor) = make_optional_header(&buffer, cursor);

    let secheader = make_section_header(&buffer, cursor, header_coff.HE_SECTIONS as usize);

    show_headers(&header_dos.0, &header_coff, &header_opt);

    if args.section_header.eq(&true) || args.show_all.eq(&true) {
        print_section_headers(&secheader);
    }

    if header_dos.0.has_rich && (args.rich_hash.eq(&true) || args.show_all.eq(&true)) {
        print_rich_sha256_hash(&header_dos.0);
    }

    Ok(())
}
