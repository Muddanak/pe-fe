use pe_fe::coff_header::enums::PEFILEERROR::NoMZinFile;
use pe_fe::dos_header::{make_dos_header, print_rich_sha256_hash, check_for_mz};
use pe_fe::utils::{get_large_data_chunk};
use clap::Parser;
use std::fs::File;
use std::process;
use pe_fe::coff_header::make_coff_header;
use pe_fe::optional_header::make_optional_header;
use pe_fe::show_headers;

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
    let mut cursor :usize;

    let mz_offset = check_for_mz(dos_header_data).unwrap_or_else(|_| {
        println!("{}", NoMZinFile);
        process::exit(1);
    });

    let header_dos = make_dos_header(dos_header_data, mz_offset);
    //println!("{}\n", header_dos);

    if header_dos.has_rich {
        print_rich_sha256_hash(&header_dos);
    }

    //dbg!(header_dos.pe_offset+24);

    cursor = header_dos.pe_offset;

    let header_coff = make_coff_header(&chunk[cursor..cursor+24]);

    //println!("{}\n", header_coff);

    cursor += 24;

    let header_opt = make_optional_header(&chunk[cursor..cursor+92]);


    //println!("{}", opt_header);

    show_headers(header_dos, header_coff, header_opt);


}
