mod coff_header;
mod dos_header;

use clap::Parser;
use std::fs::File;
use crate::coff_header::{check_for_mz, get_large_data_chunk};
use crate::dos_header::make_dos_header;


#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    if let Ok(success) = File::open(&args.filename) {
        //let test_pe :[u8; 4] = 0x50450000_u32.to_be_bytes();

        //DOS header = 64 bytes, 0x64


        let chunk :Vec<u8> = get_large_data_chunk(success);
        let dos_header_data = &chunk[0..=254];
        match check_for_mz(dos_header_data) {
            Ok(mz_offset) => {
                let header_dos = make_dos_header(dos_header_data, mz_offset);
                println!("{header_dos}");




            }
            Err(e) => println!("{e}")
        }
    } else {
        println!("Could not successfully open the file: {}", &args.filename)
    }

    /*let _input_file = match File::open(args.filename) {
        Ok(success) => {
            info = get_first_kilobyte(success);
            Ok(())
        }
        Err(e) => Err(FileErrors::CouldNotOpenFile(e)),
    };

    match check_for_mz(&info[0..=1]) {
        Ok(()) => {
            let val_for_pe = get_pe_header(&info);
            let tmp = make_header_from_info(&info, val_for_pe);
            println!("{}", tmp)
        }
        Err(e) => println!("{e}"),
    }*/
}
