mod coff_header;
mod dos_header;

use clap::Parser;
use std::fs::File;
use crate::coff_header::{check_for_mz, get_large_data_chunk};


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

        let chunk :Vec<u8> = get_large_data_chunk(success);
        match check_for_mz(&chunk[..=50]) {
            Ok(()) => {




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
