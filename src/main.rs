use clap::{arg, Parser};
use std::fs::File;
use std::io::{BufReader, Read};
use std::{io, process};

use pe_fe::sections_header::make_section_header;
use pe_fe::{lib as pefelib, show_headers};
use pefelib::dos_header::{check_for_mz, make_dos_header, print_rich_sha256_hash};

use pefelib::coff_header::make_coff_header;
use pefelib::optional_header::make_optional_header;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(short, long)]
    filename: String,
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

    show_headers(&header_dos.0, &header_coff, &header_opt, &secheader);

    if header_dos.0.has_rich {
        print_rich_sha256_hash(&header_dos.0);
    }

    Ok(())
}
