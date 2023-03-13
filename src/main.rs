mod coff_header;
mod dos_header;

use clap::Parser;
use std::fs::File;
use std::process;
use crate::coff_header::{check_for_mz, get_large_data_chunk};
use crate::coff_header::enums::PEFILEERROR::NoMZinFile;
use crate::dos_header::make_dos_header;


#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    let success = File::open(&args.filename).unwrap_or_else(|_|{ println!("Could not open file {}", &args.filename); process::exit(1);});

    let chunk :Vec<u8> = get_large_data_chunk(success);
    let dos_header_data = &chunk[0..=254];

    let mz_offset = check_for_mz(dos_header_data).unwrap_or_else(|_| { println!("{}", NoMZinFile); process::exit(1); });
    let header_dos = make_dos_header(dos_header_data, mz_offset);
    println!("{header_dos}");

}
