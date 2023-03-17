use pe_fe::coff_header::enums::PEFILEERROR::NoMZinFile;
use pe_fe::dos_header::{make_dos_header, print_rich_sha256_hash, check_for_mz};
use pe_fe::utils::{get_large_data_chunk};
use clap::Parser;
use std::fs::File;
use std::process;
use pe_fe::coff_header::make_coff_header;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    let success = File::open(&args.filename).unwrap_or_else(|_| {
        println!("Could not open file {}", &args.filename);
        process::exit(1);
    });

    let chunk: Vec<u8> = get_large_data_chunk(success);
    let dos_header_data = &chunk[0..1024];

    let mz_offset = check_for_mz(dos_header_data).unwrap_or_else(|_| {
        println!("{}", NoMZinFile);
        process::exit(1);
    });

    let header_dos = make_dos_header(dos_header_data, mz_offset);
    println!("{}", header_dos);

    if header_dos.has_rich {
        print_rich_sha256_hash(&header_dos);
    }

    //dbg!(header_dos.pe_offset+24);

    let header_coff = make_coff_header(&chunk[header_dos.pe_offset..header_dos.pe_offset+24]);

    println!("{}", header_coff);


}
